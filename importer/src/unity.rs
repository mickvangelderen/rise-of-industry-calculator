use std::num::NonZeroI64;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
pub struct MetaDocument {
    #[serde(rename = "guid")]
    pub guid: String,
}

#[derive(Debug, Deserialize)]
pub struct MonoBehaviourMeta {
    #[serde(rename = "m_Script")]
    pub script: Reference,
}

#[derive(Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum ReferenceType {
    Asset = 2,
    Library = 3,
}

#[derive(Debug)]
pub struct FileId(pub Option<NonZeroI64>);

impl<'de> Deserialize<'de> for FileId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        i64::deserialize(deserializer)
            .map(NonZeroI64::new)
            .map(FileId)
    }
}

#[derive(Debug)]
pub struct Reference(pub Option<ReferenceInner>);

impl<'de> Deserialize<'de> for Reference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ReferenceRaw {
            #[serde(rename = "fileID")]
            file_id: FileId,
            #[serde(rename = "guid", default)]
            guid: Option<String>,
            #[serde(rename = "type", default)]
            r#type: Option<ReferenceType>,
        }

        let value = ReferenceRaw::deserialize(deserializer)?;

        match value.file_id.0 {
            Some(file_id) => Ok(Reference(Some(ReferenceInner {
                file_id,
                guid: value
                    .guid
                    .ok_or_else(|| serde::de::Error::missing_field("guid"))?,
                r#type: value
                    .r#type
                    .ok_or_else(|| serde::de::Error::missing_field("type"))?,
            }))),
            None => {
                if value.guid.is_some() {
                    return Err(serde::de::Error::unknown_field("guid", &["file_id"]));
                }
                if value.r#type.is_some() {
                    return Err(serde::de::Error::unknown_field("type", &["file_id"]));
                }
                Ok(Reference(None))
            }
        }
    }
}

#[derive(Debug)]
pub struct ReferenceInner {
    pub file_id: NonZeroI64,
    pub guid: String,
    pub r#type: ReferenceType,
}
