//! Pointer fixup for save/load (CodePointers / DecodePointers)

use anyhow::{bail, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;

/// Object identifier for pointer fixup
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct ObjectId(pub u32);

impl ObjectId {
    pub const INVALID: Self = Self(u32::MAX);
    pub const NULL: Self = Self(0);

    #[inline]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub fn is_valid(self) -> bool {
        self.0 != u32::MAX && self.0 != 0
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Pointer to an object (serialized as ObjectId)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ObjectPtr<T> {
    id: ObjectId,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ObjectPtr<T> {
    #[inline]
    pub const fn null() -> Self {
        Self {
            id: ObjectId::NULL,
            _phantom: std::marker::PhantomData,
        }
    }
    #[inline]
    pub const fn invalid() -> Self {
        Self {
            id: ObjectId::INVALID,
            _phantom: std::marker::PhantomData,
        }
    }
    #[inline]
    pub fn new(id: ObjectId) -> Self {
        Self {
            id,
            _phantom: std::marker::PhantomData,
        }
    }
    #[inline]
    pub fn id(self) -> ObjectId {
        self.id
    }
    #[inline]
    pub fn is_null(self) -> bool {
        self.id == ObjectId::NULL
    }
    #[inline]
    pub fn is_valid(self) -> bool {
        self.id.is_valid()
    }
}

impl<T> Default for ObjectPtr<T> {
    fn default() -> Self {
        Self::invalid()
    }
}

impl<T> fmt::Display for ObjectPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectPtr({:?})", self.id)
    }
}

/// Context for encoding pointers (saving)
pub struct CodePointers {
    object_to_id: HashMap<*const u8, ObjectId>,
    next_id: u32,
}

impl CodePointers {
    pub fn new() -> Self {
        Self {
            object_to_id: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn get_id(&mut self, obj: *const u8) -> ObjectId {
        if obj.is_null() {
            return ObjectId::NULL;
        }
        *self.object_to_id.entry(obj).or_insert_with(|| {
            let id = ObjectId::new(self.next_id);
            self.next_id += 1;
            id
        })
    }

    pub fn encode_ptr<T>(&mut self, ptr: *const T) -> ObjectId {
        self.get_id(ptr as *const u8)
    }
    pub fn encode_box<T>(&mut self, boxed: &Option<Box<T>>) -> ObjectId {
        match boxed {
            Some(b) => self.encode_ptr(b.as_ref()),
            None => ObjectId::NULL,
        }
    }
    pub fn encode_ref<T>(&mut self, reference: &T) -> ObjectId {
        self.encode_ptr(reference as *const T)
    }
    pub fn object_count(&self) -> u32 {
        self.next_id - 1
    }

    pub fn all_ids(&self) -> Vec<ObjectId> {
        let mut ids: Vec<_> = self.object_to_id.values().copied().collect();
        ids.sort_by_key(|id| id.0);
        ids
    }
}

impl Default for CodePointers {
    fn default() -> Self {
        Self::new()
    }
}

/// Context for decoding pointers (loading)
pub struct DecodePointers {
    id_to_object: HashMap<ObjectId, *mut u8>,
}

impl DecodePointers {
    pub fn new() -> Self {
        Self {
            id_to_object: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, id: ObjectId, obj: *mut T) {
        if id.is_valid() {
            self.id_to_object.insert(id, obj as *mut u8);
        }
    }

    pub fn decode_ptr<T>(&self, id: ObjectId) -> *mut T {
        if id == ObjectId::NULL {
            return std::ptr::null_mut();
        }
        self.id_to_object
            .get(&id)
            .copied()
            .unwrap_or(std::ptr::null_mut()) as *mut T
    }

    /// Resolve an object id back to a mutable reference registered at load
    /// time, mirroring the C++ `DecodePointers` fixup.
    ///
    /// Takes `&mut self` so the borrow checker enforces exclusive access to
    /// the registry for the duration of the resolve, and so callers cannot
    /// hand out two `&mut T` from the same registry simultaneously.
    ///
    /// # Safety
    ///
    /// The id must have been registered with a pointer to an initialized
    /// object of exactly type `T` (the registry stores raw `*mut u8`, so a
    /// mismatched id is silently type-punned); the object must remain valid
    /// and not be aliased for mutation elsewhere for as long as the returned
    /// reference is used, and must only be registered once. The C++ fixup
    /// API this mirrors has the same contract.
    pub unsafe fn decode_ref<T>(&mut self, id: ObjectId) -> &mut T {
        let ptr: *mut T = self.decode_ptr(id);
        assert!(!ptr.is_null(), "Invalid object ID: {:?}", id);
        unsafe { &mut *ptr }
    }

    /// Resolve an object id to a shared reference.
    ///
    /// # Safety
    ///
    /// The registry stores raw pointers with no lifetime tying them to the
    /// objects, so this is exactly as unsafe as [`DecodePointers::decode_ref`]:
    /// the id must have been registered with a pointer to a live, initialised
    /// object of type `T`, that object must not have been moved or dropped
    /// since, and no `&mut T` to it may be alive for the duration of the
    /// returned reference. This function was previously safe, which allowed a
    /// dangling `&T` to be produced without `unsafe` at the call site.
    pub unsafe fn decode_const_ref<T>(&self, id: ObjectId) -> &T {
        let ptr: *mut T = self.decode_ptr(id);
        assert!(!ptr.is_null(), "Invalid object ID: {:?}", id);
        unsafe { &*ptr }
    }

    pub fn try_decode<T>(&self, id: ObjectId) -> Option<*mut T> {
        if id == ObjectId::NULL {
            return None;
        }
        self.id_to_object.get(&id).copied().map(|p| p as *mut T)
    }

    pub fn registered_count(&self) -> usize {
        self.id_to_object.len()
    }

    pub fn verify_complete(&self, expected_count: u32) -> Result<()> {
        if self.id_to_object.len() != expected_count as usize {
            bail!(
                "Incomplete decode: expected {} objects, got {}",
                expected_count,
                self.id_to_object.len()
            );
        }
        Ok(())
    }
}

impl Default for DecodePointers {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for objects that participate in pointer fixup
pub trait Fixup {
    fn code_pointers(&self, coder: &mut CodePointers);
    fn decode_pointers(&mut self, decoder: &mut DecodePointers);
}

/// Helper for serializing collections of fixup objects
pub struct FixupList<T> {
    pub items: Vec<T>,
}

impl<T: Fixup> FixupList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }
    pub fn code_pointers(&self, coder: &mut CodePointers) {
        for item in &self.items {
            item.code_pointers(coder);
        }
    }
    pub fn decode_pointers(&mut self, decoder: &mut DecodePointers) {
        for item in &mut self.items {
            item.decode_pointers(decoder);
        }
    }
}

/// Serializable wrapper for ObjectPtr
#[derive(Serialize, Deserialize)]
struct SerializableObjectPtr {
    id: ObjectId,
}

impl<T> Serialize for ObjectPtr<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializableObjectPtr { id: self.id }.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for ObjectPtr<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = SerializableObjectPtr::deserialize(deserializer)?;
        Ok(ObjectPtr::new(wrapper.id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_decode_pointers() {
        let mut coder = CodePointers::new();
        let mut decoder = DecodePointers::new();
        let mut obj1 = 42u32;
        let mut obj2 = 100u32;
        let id1 = coder.encode_ref(&obj1);
        let id2 = coder.encode_ref(&obj2);
        assert!(id1.is_valid());
        assert!(id2.is_valid());
        assert_ne!(id1, id2);
        decoder.register(id1, &mut obj1 as *mut u32);
        decoder.register(id2, &mut obj2 as *mut u32);
        let val1 = unsafe { *decoder.decode_ref::<u32>(id1) };
        let val2 = unsafe { *decoder.decode_ref::<u32>(id2) };
        assert_eq!(val1, 42);
        assert_eq!(val2, 100);
    }

    #[test]
    fn test_null_pointer() {
        let mut coder = CodePointers::new();
        let id = coder.encode_ptr::<u32>(std::ptr::null());
        assert_eq!(id, ObjectId::NULL);
    }

    #[test]
    fn test_same_object_same_id() {
        let mut coder = CodePointers::new();
        let obj = 42u32;
        let id1 = coder.encode_ref(&obj);
        let id2 = coder.encode_ref(&obj);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_hundred_distinct_objects_get_ids_in_encode_order() {
        let mut coder = CodePointers::new();
        let mut objs: Vec<u32> = (1..=100).collect();
        let ids: Vec<ObjectId> = objs
            .iter()
            .map(|obj| coder.encode_ref::<u32>(obj))
            .collect();

        for (i, id) in ids.iter().enumerate() {
            assert_eq!(id.0, (i + 1) as u32, "ids must be 1..=100 in encode order");
        }
        assert_eq!(coder.object_count(), 100);
        let all = coder.all_ids();
        let expected: Vec<ObjectId> = (1..=100).map(ObjectId::new).collect();
        assert_eq!(all, expected);

        // Decode side: register all objects and verify the round trip.
        let mut decoder = DecodePointers::new();
        for (i, id) in ids.iter().enumerate() {
            decoder.register(*id, &mut objs[i]);
        }
        assert_eq!(decoder.registered_count(), 100);
        assert!(decoder.verify_complete(100).is_ok());
        for (i, id) in ids.iter().enumerate() {
            assert_eq!(
                *unsafe { decoder.decode_const_ref::<u32>(*id) },
                (i + 1) as u32
            );
        }
    }

    #[test]
    fn test_verify_complete_errors_on_mismatched_counts() {
        let mut decoder = DecodePointers::new();
        assert!(
            decoder.verify_complete(0).is_ok(),
            "empty decoder expects 0"
        );
        let mut a = 1u32;
        let mut b = 2u32;
        decoder.register(ObjectId::new(1), &mut a);
        decoder.register(ObjectId::new(2), &mut b);
        assert_eq!(decoder.registered_count(), 2);
        assert!(decoder.verify_complete(2).is_ok());
        assert!(decoder.verify_complete(1).is_err());
        assert!(decoder.verify_complete(3).is_err());
        let err = decoder.verify_complete(1).unwrap_err().to_string();
        assert!(err.contains("expected 1"), "message mentions count: {err}");
    }

    #[test]
    fn test_try_decode_returns_none_for_null_and_unregistered() {
        let mut decoder = DecodePointers::new();
        assert!(decoder.try_decode::<u32>(ObjectId::NULL).is_none());
        assert!(decoder.try_decode::<u32>(ObjectId::INVALID).is_none());
        let mut obj = 5u32;
        let id = ObjectId::new(42);
        decoder.register(id, &mut obj);
        let ptr = decoder.try_decode::<u32>(id).unwrap();
        assert_eq!(unsafe { *ptr }, 5);
    }

    #[test]
    fn test_register_ignores_null_and_invalid_ids() {
        let mut decoder = DecodePointers::new();
        let mut obj = 7u32;
        decoder.register(ObjectId::NULL, &mut obj);
        decoder.register(ObjectId::INVALID, &mut obj);
        assert_eq!(decoder.registered_count(), 0);
    }

    #[test]
    fn test_encode_box_some_and_none() {
        let mut coder = CodePointers::new();
        let boxed: Option<Box<u32>> = Some(Box::new(9));
        assert!(coder.encode_box(&boxed).is_valid());
        let none: Option<Box<u32>> = None;
        assert_eq!(coder.encode_box(&none), ObjectId::NULL);
        assert_eq!(coder.object_count(), 1, "None must not consume an id");
    }
}
