use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::hash;
use crate::hashing_serializer::{self, HashingSerializer};

#[derive(Clone, Copy, Deserialize)]
pub struct EntityId([u8; 32]);

impl From<String> for EntityId {
    fn from(value: String) -> Self {
        Self(hash(&value).hash)
    }
}

/// A generic identifier type that can be used for different entity types
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Id<T: ?Sized> {
    /// The raw identifier value
    pub value: [u8; 32],
    /// Phantom data to associate the ID with a specific type
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T: ?Sized> Id<T> {
    /// Create a new ID with the given value
    pub fn new(value: [u8; 32]) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// Generate a random ID
    pub fn random() -> Self {
        let mut value = [0u8; 32];
        // Simple random generation for demonstration
        for i in 0..32 {
            value[i] = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u8)
                .wrapping_add(i as u8);
        }
        Self::new(value)
    }

    /// Convert the ID to a u64 for easier handling (uses only first 8 bytes)
    pub fn to_u64(&self) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            if i < self.value.len() {
                result |= (self.value[i] as u64) << (i * 8);
            }
        }
        result
    }

    /// Create an ID from a u64 (fills only first 8 bytes)
    pub fn from_u64(value: u64) -> Self {
        let mut bytes = [0u8; 32];
        for i in 0..8 {
            bytes[i] = ((value >> (i * 8)) & 0xFF) as u8;
        }
        Self::new(bytes)
    }

    /// Convert the ID to a pair of u128 values for handling larger values
    /// Returns (high_bits, low_bits)
    pub fn to_u128_pair(&self) -> (u128, u128) {
        let mut high = 0u128;
        let mut low = 0u128;

        // First 16 bytes go to low
        for i in 0..16 {
            low |= (self.value[i] as u128) << (i * 8);
        }

        // Next 16 bytes go to high
        for i in 0..16 {
            high |= (self.value[i + 16] as u128) << (i * 8);
        }

        (high, low)
    }

    /// Create an ID from a pair of u128 values
    pub fn from_u128_pair(high: u128, low: u128) -> Self {
        let mut bytes = [0u8; 32];

        // Low bits fill first 16 bytes
        for i in 0..16 {
            bytes[i] = ((low >> (i * 8)) & 0xFF) as u8;
        }

        // High bits fill next 16 bytes
        for i in 0..16 {
            bytes[i + 16] = ((high >> (i * 8)) & 0xFF) as u8;
        }

        Self::new(bytes)
    }
}

impl<T: ?Sized> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: ?Sized> Eq for Id<T> {}

impl<T: ?Sized> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: ?Sized> Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (high, low) = self.to_u128_pair();
        write!(
            f,
            "Id<{}>({:016x}{:016x})",
            std::any::type_name::<T>(),
            high,
            low
        )
    }
}

impl<T: ?Sized> Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (high, low) = self.to_u128_pair();
        write!(f, "{:016x}{:016x}", high, low)
    }
}

pub struct ContentHash {
    value: [u8; 32],
}

impl ContentHash {}

pub trait Entity {
    fn entity_id() -> Id<dyn Entity> where Self: Sized;
}

pub trait Component: Send + Sync + 'static {
    fn component_id() -> &'static str;
}

// impl<'de, T> Component for T where T: Serialize + Deserialize<'de> {}
