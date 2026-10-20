//! Phase 2.3 semantics for `AssetHandle` / generation tracking / async preload.
//!
//! These tests define the contract for the upgraded `AssetRegistry`:
//! - cheap handle with name + generation + slot bytes (`Deref`)
//! - replacing an asset under the same name bumps its generation
//! - old handles detect staleness without registry mutation
//! - async preload path is executor-free, threaded, and deterministic

use cnc_asset::{AssetHandle, AssetRegistry};

#[test]
fn legacy_register_get_api_still_works() {
    let mut registry = AssetRegistry::new();
    registry.register("units/e1.shp".to_string(), vec![1, 2, 3, 4]);
    assert_eq!(registry.get("units/e1.shp"), Some(&vec![1, 2, 3, 4]));
    assert_eq!(registry.get("missing"), None);
    assert_eq!(registry.len(), 1);
    assert!(!registry.is_empty());
}

#[test]
fn handle_reports_name_and_derefs_to_loaded_bytes() {
    let mut registry = AssetRegistry::new();
    let data = vec![7u8; 42];
    registry.register("gfx/base.shp".to_string(), data.clone());
    let handle = registry.handle("gfx/base.shp").expect("handle");
    assert_eq!(handle.name(), "gfx/base.shp");
    assert_eq!(&handle[..], &data[..]);
    assert_eq!(handle.bytes(), &data);
}

#[test]
fn first_registration_generation_is_zero() {
    let mut registry = AssetRegistry::new();
    registry.register("a.bin".to_string(), b"one".to_vec());
    assert_eq!(registry.generation("a.bin"), Some(0));
    assert_eq!(registry.generation("missing"), None);

    let (data, generation) = registry
        .get_with_generation("a.bin")
        .expect("slot with generation");
    assert_eq!(data, &b"one".to_vec());
    assert_eq!(generation, 0);
}

#[test]
fn replacing_same_name_bumps_generation() {
    let mut registry = AssetRegistry::new();
    registry.register("hot.bin".to_string(), b"v1".to_vec());
    registry.register("hot.bin".to_string(), b"v2".to_vec());
    assert_eq!(registry.generation("hot.bin"), Some(1));
    registry.register("hot.bin".to_string(), b"v3".to_vec());
    assert_eq!(registry.generation("hot.bin"), Some(2));
    // Distinct names keep independent counters.
    registry.register("other.bin".to_string(), b"x".to_vec());
    assert_eq!(registry.generation("other.bin"), Some(0));
}

#[test]
fn stale_handle_detects_replace_but_keeps_snapshot_bytes() {
    let mut registry = AssetRegistry::new();
    registry.register("hot.bin".to_string(), b"v1".to_vec());
    let stale_handle = registry.handle("hot.bin").expect("handle before reload");

    registry.register("hot.bin".to_string(), b"v2".to_vec());

    assert!(stale_handle.is_stale(&registry), "stale after replace");
    assert_eq!(registry.generation("hot.bin"), Some(1));

    // Snapshot semantics: the old handle still derefs to the bytes it holds.
    assert_eq!(&stale_handle[..], b"v1");

    // A fresh handle resolves the reloaded bytes and is not stale.
    let fresh = registry.handle("hot.bin").expect("handle after reload");
    assert!(!fresh.is_stale(&registry));
    assert_eq!(fresh.generation(), 1);
    assert_eq!(&fresh[..], b"v2");
    assert_ne!(stale_handle, fresh);
}

#[test]
fn equal_handles_compare_equal_for_same_slot() {
    let mut registry = AssetRegistry::new();
    registry.register("same.bin".to_string(), b"data".to_vec());
    let h1 = registry.handle("same.bin").unwrap();
    let h2 = registry.handle("same.bin").unwrap();
    assert_eq!(h1, h2);
    assert_ne!(h1, AssetHandle::missing("other.bin"));
}

#[test]
fn registry_keeps_index_ordered_names() {
    let mut registry = AssetRegistry::new();
    assert!(registry.is_empty());
    registry.register("x".to_string(), vec![0]);
    registry.register("z".to_string(), vec![0]);
    registry.register("y".to_string(), vec![0]);
    // Index-based determinism: names come back in first-registration order.
    assert_eq!(registry.names(), vec!["x", "z", "y"]);
    assert_eq!(registry.index_of("y"), Some(2));
}

#[test]
fn get_async_resolves_registered_bytes_without_executor() {
    let mut registry = AssetRegistry::new();
    registry.register("one.bin".to_string(), vec![1]);
    registry.register("two.bin".to_string(), vec![2, 2]);

    let task = registry.get_async("two.bin");
    let loaded = task.wait_blocking();
    assert_eq!(loaded.get("two.bin"), Some(&[2u8, 2][..]));
    assert!(loaded.missing().is_empty());
}

#[test]
fn preload_reports_missing_names_deterministically() {
    let mut registry = AssetRegistry::new();
    registry.register("known.bin".to_string(), b"known".to_vec());

    let task = registry.get_async("known.bin");
    let loaded = task.wait_blocking();
    assert_eq!(loaded.get("known.bin"), Some(&b"known"[..]));

    let task = registry.get_async("ghost.bin");
    let loaded = task.wait_blocking();
    assert_eq!(loaded.get("ghost.bin"), None);
    assert_eq!(loaded.missing(), vec!["ghost.bin"]);
}

#[test]
fn handle_missing_at_creation_is_stale() {
    let registry = AssetRegistry::new();
    let handle = AssetHandle::missing("void.bin");
    assert!(handle.is_stale(&registry), "missing asset counts as stale");
    assert!(handle.bytes().is_empty());
}
