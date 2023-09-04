use crate::serialize::Serializer;

/// Serializes XRPL objects to a [`Serializer`]
pub trait Serialize {
    /// Serialize the object
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error>;
}
