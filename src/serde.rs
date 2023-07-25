use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::ImString;

impl <'a> Serialize for ImString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ImString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let str = alloc::string::String::deserialize(deserializer)?;
        Ok(ImString::from(str))
    }
}