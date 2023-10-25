use crate::deserialize::Deserializer;

/// Deserializes XRPL objects from a [`Deserializer`]
pub trait Deserialize {
    /// Deserialize the object
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error>
    where
        Self: Sized;
}