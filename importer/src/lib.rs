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

// TODO: Address that this is serialized as { fileID: 0 }, probably when it is unset. Does not
// affect the files we want to read though so...
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
    ProductCategory(ProductCategoryMonoBehaviour),
    ProductCategoryModifierInfo(ProductCategoryModifierInfoMonoBehaviour),
    GathererHub(GathererHubMonoBehaviour),
    Factory(FactoryMonoBehaviour),
    Harvester(HarvesterMonoBehaviour),
    Building(BuildingMonoBehaviour),
}

// Field: DisconnectedHarvester
const FIELD_GUID: &str = "37877dc8090b6c86ab7ebdd152757ce2";

// DisconnectedHarvester: Harvester
const DISCONNECTED_HARVESTER: &str = "729682943bcacee6c3bfcfa694f8d28f";

// Harvester: Module
const HARVESTER_GUID: &str = "9e91acce255b80b153adaea3a62e14f1";

// Module: BuildingBehaviour
const MODULE_GUID: &str = "e2441e19b10b3db8d7730d6b7c90f92c";

// Farm: GathererHub
const FARM_GUID: &str = "48f60db05a30e6f6a2c4f58e376db169";

// GathererHub: RecipeUser
const GATHERER_HUB_GUID: &str = "2cafc42823a354fcf7c0170bea0bcb7d";

// Factory: RecipeUser
const FACTORY_GUID: &str = "38614187f7f363776435354b6ad3dd66";

// RecipeUser: BuildingBehaviour
const RECIPE_USER_GUID: &str = "046ea3fff0e361ee25fd49004ac10ed9";

// Recipe
const RECIPE_GUID: &str = "86eee4258519014ad55f04d4a92d2556";

// ProductDefinition
const PRODUCT_DEFINITION_GUID: &str = "23940808cf3b3e11ddbcefa65cb07256";

// Building
const BUILDING_GUID: &str = "6219336138908849fca2c4c8fb8c7e83";

impl<'de> Deserialize<'de> for MonoBehaviour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;

        let meta = MonoBehaviourMeta::deserialize(&value).map_err(serde::de::Error::custom)?;

        Ok(match meta.script.guid.as_str() {
            RECIPE_GUID => MonoBehaviour::Known(KnownMonoBehaviour::Recipe(
                Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            PRODUCT_DEFINITION_GUID => MonoBehaviour::Known(KnownMonoBehaviour::ProductDefinition(
                Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            FARM_GUID | GATHERER_HUB_GUID => MonoBehaviour::Known(KnownMonoBehaviour::GathererHub(
                Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            FACTORY_GUID => MonoBehaviour::Known(KnownMonoBehaviour::Factory(
                Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            FIELD_GUID | DISCONNECTED_HARVESTER | HARVESTER_GUID => {
                MonoBehaviour::Known(KnownMonoBehaviour::Harvester(
                    Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
                ))
            }
            BUILDING_GUID => MonoBehaviour::Known(KnownMonoBehaviour::Building(
                Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            PRODUCT_CATEGORY_GUID => MonoBehaviour::Known(KnownMonoBehaviour::ProductCategory(
                Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            PRODUCT_CATEGORY_MODIFIER_INFO_GUID => {
                MonoBehaviour::Known(KnownMonoBehaviour::ProductCategoryModifierInfo(
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

    /// Time for recipe to complete in days.
    #[serde(rename = "_gameDays")]
    pub days: i64,

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
    pub amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct ProductDefinitionMonoBehaviour {
    #[serde(rename = "productName")]
    pub name: String,

    #[serde(rename = "_category")]
    pub category: ScriptReference,
}

#[derive(Debug, Deserialize)]
pub struct GathererHubMonoBehaviour {
    #[serde(rename = "availableRecipes")]
    pub available_recipes: Vec<ScriptReference>,
}

#[derive(Debug, Deserialize)]
pub struct HarvesterMonoBehaviour {
    #[serde(rename = "consumption")]
    pub consumption: i64,
}

#[derive(Debug, Deserialize)]
pub struct BuildingMonoBehaviour {
    #[serde(rename = "buildingName")]
    pub name: String,

    #[serde(rename = "baseCost")]
    pub base_cost: i64,
}

#[derive(Debug, Deserialize)]
pub struct FactoryMonoBehaviour {
    #[serde(rename = "availableRecipes")]
    pub available_recipes: Vec<ScriptReference>,
}

const PRODUCT_CATEGORY_GUID: &str = "d6fc0e4aff0acdc78b884c7ca29c6687";

#[derive(Debug, Deserialize)]
pub struct ProductCategoryMonoBehaviour {
    #[serde(rename = "categoryName")]
    pub name: String,
}

const PRODUCT_CATEGORY_MODIFIER_INFO_GUID: &str = "558262efc8fda9116a369bbbd4ee5aa7";

#[derive(Debug, Deserialize)]
pub struct ProductCategoryModifierInfoMonoBehaviour {
    #[serde(rename = "defaultPriceModifier")]
    pub default_price_modifier: f64,

    #[serde(rename = "defaultGrowthModifier")]
    pub default_growth_modifier: f64,

    pub modifiers: Vec<ProductCategoryModifierInfoEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ProductCategoryModifierInfoEntry {
    #[serde(rename = "priceModifier")]
    pub price_modifier: f64,

    #[serde(rename = "growthModifier")]
    pub growth_modifier: f64,

    #[serde(rename = "category")]
    pub category: ScriptReference,
}

/// This operation is necessary to allow `serde_yaml` to parse Unity's asset and prefab files. There
/// may be better solutions but this worked for me.
pub fn rewrite_yaml_tags(value: &str) -> Cow<'_, str> {
    let regex = regex::RegexBuilder::new("^---.*$")
        .multi_line(true)
        .build()
        .unwrap();
    regex.replace_all(value, "---")
}
