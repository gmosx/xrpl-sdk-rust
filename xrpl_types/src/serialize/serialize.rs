use crate::serialize::Serializer;

pub trait Serialize {
    /// Serialize the object. Notice that fields must be serialized in the order given by
    /// <https://xrpl.org/serialization.html#canonical-field-order>. Field codes can
    /// be found at <https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json>
    /// and <https://github.com/XRPLF/rippled/blob/72e6005f562a8f0818bc94803d222ac9345e1e40/src/ripple/protocol/impl/SField.cpp#L72-L266>.
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error>;
}
