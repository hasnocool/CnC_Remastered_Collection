//! Asset registry with generational handles and executor-free async preloading.
//!
//! # Handles and hot-reload
//!
//! [`AssetRegistry`] keeps the original keyed-bytes API (`register`/`get`).
//! Registering bytes under a name creates a slot; *re-registering the same
//! name replaces the slot's data and bumps its generation* — that is the hot
//! reload path. [`AssetHandle`] is a cheap, `Clone`-able handle that pins the
//! slot's bytes behind an `Arc` (snapshot semantics) plus name and generation;
//! [`AssetHandle::is_stale`] detects that the registry moved on. Handles are
//! index-keyed: the per-name index is assigned at first registration and
//! never changes, so ordering is deterministic.
//!
//! # Async path (executor-free)
//!
//! [`AssetRegistry::spawn_preload`] / [`AssetRegistry::get_async`] start a
//! worker thread that resolves asset bytes and expose the result as a
//! [`PreloadTask`] implementing `std::future::Future` (`Output =
//! [`PreloadedAssets`]). No async runtime is required:
//! [`PreloadTask::wait_blocking`] polls-and-parks internally, and real
//! executors work too (the final completion wakes the registered waker).
//! std-only; no heavyweight dependency.

use std::collections::HashMap;
use std::future::Future;
use std::ops::Deref;
use std::panic::catch_unwind;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

/// How long a [`PreloadTask::poll`] parks between progress checks when driven
/// by a naive poll loop (keeps `wait_blocking` CPU-cheap without a runtime).
const PRELOAD_PARK: Duration = Duration::from_millis(10);

#[derive(Debug)]
struct Slot {
    data: Vec<u8>,
    generation: u64,
}

/// Registry of named asset byte-slots with generation tracking and orderly
/// indices (see [`AssetRegistry::register`] for the hot-reload rule). Not
/// internally synchronised; wrap in a lock if shared across threads.
#[derive(Debug, Clone, Default)]
pub struct AssetRegistry {
    slots: HashMap<String, Arc<Slot>>,
    /// Registration order (index → name). Grows monotonically; names are
    /// never removed, so indices are stable for the registry's lifetime.
    by_index: Vec<String>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register (or hot-reload) an asset under `name`. Re-registering bumps
    /// the slot's generation; prior handles become stale. Returns a handle to
    /// the freshly registered slot.
    pub fn register(&mut self, name: String, data: Vec<u8>) -> AssetHandle {
        let generation = match self.slots.get(&name) {
            Some(slot) => slot.generation + 1,
            None => {
                self.by_index.push(name.clone());
                0
            }
        };
        let slot = Arc::new(Slot { data, generation });
        self.slots.insert(name.clone(), Arc::clone(&slot));
        let index = self.index_of(&name).expect("index inserted above");
        AssetHandle {
            name: Arc::from(name.as_str()),
            index,
            slot,
        }
    }

    /// Existing byte access, unchanged for callers.
    pub fn get(&self, name: &str) -> Option<&Vec<u8>> {
        self.slots.get(name).map(|slot| &slot.data)
    }

    /// Bytes plus the current generation for a name.
    pub fn get_with_generation(&self, name: &str) -> Option<(&Vec<u8>, u64)> {
        self.slots
            .get(name)
            .map(|slot| (&slot.data, slot.generation))
    }

    /// Current generation of a name (`0` after first registration, +1 per
    /// re-registration).
    pub fn generation(&self, name: &str) -> Option<u64> {
        self.slots.get(name).map(|slot| slot.generation)
    }

    /// Cheap handle pinning the *current* slot's bytes.
    pub fn handle(&self, name: &str) -> Option<AssetHandle> {
        let slot = self.slots.get(name)?;
        Some(AssetHandle {
            name: Arc::from(name),
            index: self.index_of(name)?,
            slot: Arc::clone(slot),
        })
    }

    /// Stable registration index of a name (first-registration order).
    pub fn index_of(&self, name: &str) -> Option<u32> {
        let pos = self.by_index.iter().position(|n| n == name)?;
        Some(pos as u32)
    }

    /// All registered names in first-registration order (deterministic).
    pub fn names(&self) -> Vec<&str> {
        self.by_index.iter().map(String::as_str).collect()
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    // ---- async preload path ---------------------------------------------

    /// Start a worker thread resolving each name through `load` and return an
    /// [`PreloadTask`] future. `load` returning `None` marks the asset as
    /// missing. Results are reported in input order regardless of completion
    /// interleaving (single worker thread: deterministic).
    pub fn spawn_preload<F>(&self, names: Vec<String>, load: F) -> PreloadTask
    where
        F: Fn(&str) -> Option<Vec<u8>> + Send + 'static,
    {
        let order = names.clone();
        let shared = Arc::new(PreloadShared {
            state: Mutex::new(PreloadState {
                pending: names.len(),
                order,
                results: HashMap::with_capacity(names.len()),
                waker: None,
            }),
            ready: Condvar::new(),
        });

        let worker_shared = Arc::clone(&shared);
        let worker = move || {
            for name in names {
                let outcome = catch_unwind(std::panic::AssertUnwindSafe(|| load(&name)))
                    .unwrap_or_else(|_| {
                        eprintln!("cnc-asset: preload loader panicked for {name}");
                        None
                    });
                let mut to_wake = None;
                {
                    let mut state = worker_shared.state.lock().expect("preload state mutex");
                    state.results.insert(name, outcome);
                    state.pending = state.pending.saturating_sub(1);
                    if state.pending == 0 {
                        to_wake = state.waker.take();
                    }
                }
                worker_shared.ready.notify_all();
                if let Some(waker) = to_wake {
                    waker.wake();
                }
            }
        };
        let _worker_thread = std::thread::Builder::new()
            .name("cnc-asset-preload".to_string())
            .spawn(worker)
            .expect("failed to spawn preload worker thread");

        PreloadTask { shared }
    }

    /// Async convenience for a single already-registered name: resolves the
    /// slot's bytes (snapshot at call time) on a worker thread.
    pub fn get_async(&self, name: &str) -> PreloadTask {
        let snapshot: Option<Vec<u8>> = self.get(name).cloned();
        self.spawn_preload(vec![name.to_string()], move |_| snapshot.clone())
    }
}

/// Cheap generational handle to a registered asset slot.
///
/// - `Deref` to the pinned byte snapshot (survives a subsequent registry
///   replace at the same name — that is the stale snapshot).
/// - [`is_stale`] reports that the registry generation for this name moved
///   past the handle's generation (or the name is gone).
/// - Cheap to clone: `Arc<str>` + `u32` + `Arc<Slot>`.
///
/// [`is_stale`]: AssetHandle::is_stale
#[derive(Clone, Debug)]
pub struct AssetHandle {
    name: Arc<str>,
    index: u32,
    slot: Arc<Slot>,
}

impl AssetHandle {
    /// Construct a handle to a slot that may not exist (used for lookups of
    /// assets expected to arrive later). Always stale against a registry
    /// lacking the name; derefs to empty bytes.
    pub fn missing(name: &str) -> Self {
        Self {
            name: Arc::from(name),
            index: u32::MAX,
            slot: Arc::new(Slot {
                data: Vec::new(),
                generation: u64::MAX,
            }),
        }
    }

    /// Asset name this handle was created for.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Stable registration index (deterministic, first-registration order).
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Generation pinned by this handle. `u64::MAX` marks a missing-at-creation
    /// handle.
    pub fn generation(&self) -> u64 {
        self.slot.generation
    }

    /// Pinned byte snapshot (same as derefing).
    pub fn bytes(&self) -> &[u8] {
        &self.slot.data
    }

    /// True when the registry's current slot for this name no longer matches
    /// the pinned slot (hot-reload happened, or the name vanished). A
    /// `missing` handle is always stale.
    pub fn is_stale(&self, registry: &AssetRegistry) -> bool {
        match registry.slots.get(&*self.name) {
            Some(current) => !Arc::ptr_eq(current, &self.slot),
            None => true,
        }
    }
}

impl Deref for AssetHandle {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.slot.data
    }
}

impl PartialEq for AssetHandle {
    /// Two handles are equal when they pin the same backing slot (same name,
    /// index, generation and bytes).
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.index == other.index
            && self.slot.generation == other.slot.generation
            && self.slot.data == other.slot.data
    }
}

impl Eq for AssetHandle {}

/// Resolved bytes for a preloaded asset. `None` data means the loader
/// reported the name as missing (or the loader panicked).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PreloadResult {
    pub name: String,
    pub data: Option<Vec<u8>>,
}

/// Ordered, completed preload output.
#[derive(Clone, Debug, Default)]
pub struct PreloadedAssets {
    results: Vec<PreloadResult>,
}

impl PreloadedAssets {
    /// Bytes for a resolved name.
    pub fn get(&self, name: &str) -> Option<&[u8]> {
        self.results
            .iter()
            .find(|r| r.name == name)
            .and_then(|r| r.data.as_deref())
    }

    /// Names the loader could not resolve, in input order.
    pub fn missing(&self) -> Vec<&str> {
        self.results
            .iter()
            .filter(|r| r.data.is_none())
            .map(|r| r.name.as_str())
            .collect()
    }

    /// All results, in requested order.
    pub fn results(&self) -> &[PreloadResult] {
        &self.results
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}

struct PreloadState {
    pending: usize,
    order: Vec<String>,
    results: HashMap<String, Option<Vec<u8>>>,
    waker: Option<Waker>,
}

struct PreloadShared {
    state: Mutex<PreloadState>,
    ready: Condvar,
}

/// `Future` resolving to [`PreloadedAssets`] once the worker thread finished.
/// Executor-free by design: [`PreloadTask::wait_blocking`] polls and parks
/// internally; real executors also work via the completion waker.
pub struct PreloadTask {
    shared: Arc<PreloadShared>,
}

impl PreloadTask {
    /// Non-blocking result check.
    pub fn try_take(&self) -> Option<PreloadedAssets> {
        let mut state = self.shared.state.lock().expect("preload state mutex");
        collect_finished(&mut state)
    }

    /// Poll-and-park until the worker thread completed, then return the
    /// ordered results. No async runtime needed.
    pub fn wait_blocking(mut self) -> PreloadedAssets {
        // Waker that discards wake-ups: completion is detected by re-polling,
        // and `poll` parks on the condvar so this loop stays CPU-cheap.
        // (Waker::noop() returns &'static Waker, stable since 1.85.)
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        loop {
            if let Poll::Ready(assets) = self.poll(&mut cx) {
                return assets;
            }
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<PreloadedAssets> {
        let mut state = self.shared.state.lock().expect("preload state mutex");
        if let Some(assets) = collect_finished(&mut state) {
            return Poll::Ready(assets);
        }
        // Park briefly so a naive poll loop does not spin; a waker is kept
        // for executor-driven polling (woken on completion).
        state.waker = Some(cx.waker().clone());
        // The returned guard is intentionally dropped: each poll iteration
        // re-locks the state mutex (the guard was consumed by wait_timeout).
        let _ = self
            .shared
            .ready
            .wait_timeout(state, PRELOAD_PARK)
            .expect("preload condvar");
        Poll::Pending
    }
}

impl Future for PreloadTask {
    type Output = PreloadedAssets;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().poll(cx)
    }
}

fn collect_finished(state: &mut PreloadState) -> Option<PreloadedAssets> {
    if state.pending != 0 {
        return None;
    }
    let order = std::mem::take(&mut state.order);
    let results = order
        .into_iter()
        .map(|name| PreloadResult {
            data: state.results.remove(&name).flatten(),
            name,
        })
        .collect();
    Some(PreloadedAssets { results })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_replaces_bytes_and_preserves_index() {
        let mut registry = AssetRegistry::new();
        handle_smoke(&mut registry);
    }

    fn handle_smoke(registry: &mut AssetRegistry) {
        let first = registry.register("a.bin".to_string(), b"one".to_vec());
        let index = first.index();
        assert_eq!(first.generation(), 0);
        let second = registry.register("a.bin".to_string(), b"two".to_vec());
        assert_eq!(second.index(), index, "index stable across reload");
        assert_eq!(second.generation(), 1);
        assert!(first.is_stale(registry));
        assert_eq!(&first[..], b"one", "snapshot survives reload");
        assert!(!second.is_stale(registry));
    }
}
