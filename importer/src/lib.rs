#![allow(unused)]

// Import assets exported from the game files with https://assetripper.github.io/AssetRipper/articles/Downloads.html.

use std::{
    borrow::Cow,
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    fs::FileType,
    path::{Path, PathBuf},
};

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MetaDocument {
    #[serde(rename = "guid")]
    pub guid: String,
}

#[derive(Debug, Deserialize)]
pub struct MonoBehaviourMeta {
    #[serde(rename = "m_Script")]
    pub script: ScriptReference,
}

#[derive(Debug)]
pub enum Document {
    Known(KnownDocument),
    Unknown(serde_yaml::Value),
}

#[derive(Debug, Deserialize)]
pub enum KnownDocument {
    MonoBehaviour(MonoBehaviour),
}

impl<'de> Deserialize<'de> for Document {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct MonoBehaviourDocument {
            #[serde(rename = "MonoBehaviour")]
            mono_behaviour: serde_yaml::Value,
        }

        Ok(
            if let Ok(document) = MonoBehaviourDocument::deserialize(&value) {
                Document::Known(KnownDocument::MonoBehaviour(
                    MonoBehaviour::deserialize(document.mono_behaviour)
                        .map_err(serde::de::Error::custom)?,
                ))
            } else {
                Document::Unknown(value)
            },
        )
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct ScriptReference {
    // #[serde(rename = "")]
    // file_id: u64,
    #[serde(rename = "guid")]
    pub guid: String,
    // #[serde(rename = "type")]
    // ty: u64,
}

#[derive(Debug)]
pub enum MonoBehaviour {
    Known(KnownMonoBehaviour),
    Unknown(serde_yaml::Value),
}

#[derive(Debug)]
pub enum KnownMonoBehaviour {
    Recipe(RecipeMonoBehaviour),
    ProductDefinition(ProductDefinitionMonoBehaviour),
    GathererHub(GathererHubMonoBehaviour),
    Building(BuildingMonoBehaviour),
}

impl<'de> Deserialize<'de> for MonoBehaviour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;

        let meta = MonoBehaviourMeta::deserialize(&value).map_err(serde::de::Error::custom)?;

        Ok(match meta.script.guid.as_str() {
            "86eee4258519014ad55f04d4a92d2556" => MonoBehaviour::Known(KnownMonoBehaviour::Recipe(
                Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "23940808cf3b3e11ddbcefa65cb07256" => {
                MonoBehaviour::Known(KnownMonoBehaviour::ProductDefinition(
                    Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
                ))
            }
            "2cafc42823a354fcf7c0170bea0bcb7d" => {
                MonoBehaviour::Known(KnownMonoBehaviour::GathererHub(
                    Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
                ))
            }
            "6219336138908849fca2c4c8fb8c7e83" => {
                MonoBehaviour::Known(KnownMonoBehaviour::Building(
                    Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
                ))
            }
            _ => MonoBehaviour::Unknown(value),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct RecipeMonoBehaviour {
    #[serde(rename = "Title")]
    pub name: String,

    #[serde(rename = "ingredients")]
    pub ingredients: ProductList,

    #[serde(rename = "result")]
    pub result: ProductList,

    #[serde(rename = "requiredModules")]
    pub required_modules: Vec<ScriptReference>,
}

#[derive(Debug, Deserialize)]
pub struct ProductList {
    #[serde(rename = "entries")]
    pub entries: Vec<Ingredient>,
}

#[derive(Debug, Deserialize)]
pub struct Ingredient {
    #[serde(rename = "_definition")]
    pub definition: ScriptReference,

    #[serde(rename = "amount")]
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct ProductDefinitionMonoBehaviour {
    #[serde(rename = "productName")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct GathererHubMonoBehaviour {
    #[serde(rename = "availableRecipes")]
    pub available_recipes: Vec<ScriptReference>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct BuildingMonoBehaviour {
    #[serde(rename = "buildingName")]
    pub name: String,

    #[serde(rename = "baseCost")]
    pub base_cost: i64,
}

pub fn rewrite_yaml_tags(value: &str) -> Cow<'_, str> {
    let regex = regex::RegexBuilder::new("^---.*$")
        .multi_line(true)
        .build()
        .unwrap();
    regex.replace_all(value, "---")
}
