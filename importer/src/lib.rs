mod unity;
pub use unity::*;

use std::borrow::Cow;

use serde::Deserialize;

#[derive(Debug)]
pub enum MonoBehaviour {
    Known(KnownMonoBehaviour),
    Unknown(serde_yaml::Value),
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

// Recipe
const RECIPE_GUID: &str = "86eee4258519014ad55f04d4a92d2556";

#[derive(Debug, Deserialize)]
pub struct RecipeMonoBehaviour {
    #[serde(rename = "Title")]
    pub name: String,

    #[serde(rename = "ingredients")]
    pub ingredients: RecipeEntries,

    #[serde(rename = "result")]
    pub result: RecipeEntries,

    /// Time for recipe to complete in days.
    #[serde(rename = "_gameDays")]
    pub days: i64,

    #[serde(rename = "requiredModules")]
    pub required_modules: Vec<Reference>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeEntries {
    #[serde(rename = "entries")]
    pub entries: Vec<RecipeEntry>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeEntry {
    #[serde(rename = "_definition")]
    pub definition: Reference,

    #[serde(rename = "amount")]
    pub amount: i64,
}

// ProductDefinition
const PRODUCT_DEFINITION_GUID: &str = "23940808cf3b3e11ddbcefa65cb07256";

#[derive(Debug, Deserialize)]
pub struct ProductDefinitionMonoBehaviour {
    #[serde(rename = "productName")]
    pub name: String,

    #[serde(rename = "_category")]
    pub category: Reference,

    #[serde(rename = "categoryProvider")]
    pub category_provider: Reference,
}

// Farm: GathererHub
const FARM_GUID: &str = "48f60db05a30e6f6a2c4f58e376db169";

// GathererHub: RecipeUser
const GATHERER_HUB_GUID: &str = "2cafc42823a354fcf7c0170bea0bcb7d";

#[derive(Debug, Deserialize)]
pub struct GathererHubMonoBehaviour {
    #[serde(rename = "availableRecipes")]
    pub available_recipes: Vec<Reference>,
}

// Field: DisconnectedHarvester
const FIELD_GUID: &str = "37877dc8090b6c86ab7ebdd152757ce2";

// DisconnectedHarvester: Harvester
const DISCONNECTED_HARVESTER: &str = "729682943bcacee6c3bfcfa694f8d28f";

// Harvester: Module
const HARVESTER_GUID: &str = "9e91acce255b80b153adaea3a62e14f1";

#[derive(Debug, Deserialize)]
pub struct HarvesterMonoBehaviour {
    #[serde(rename = "consumption")]
    pub consumption: i64,
}

// Building
const BUILDING_GUID: &str = "6219336138908849fca2c4c8fb8c7e83";

#[derive(Debug, Deserialize)]
pub struct BuildingMonoBehaviour {
    #[serde(rename = "buildingName")]
    pub name: String,

    #[serde(rename = "baseCost")]
    pub base_cost: i64,
}

// Factory: RecipeUser
const FACTORY_GUID: &str = "38614187f7f363776435354b6ad3dd66";

#[derive(Debug, Deserialize)]
pub struct FactoryMonoBehaviour {
    #[serde(rename = "availableRecipes")]
    pub available_recipes: Vec<Reference>,
}

const PRODUCT_CATEGORY_GUID: &str = "d6fc0e4aff0acdc78b884c7ca29c6687";

#[derive(Debug, Deserialize)]
pub struct ProductCategoryMonoBehaviour {
    #[serde(rename = "categoryName")]
    pub name: String,
}

const PRODUCT_DEFINITION_DATA_CATEGORY_PROVIDER_GUID: &str = "de5bbb24ddada981b140c3919e5585d1";

#[derive(Debug, Deserialize)]
pub struct ProductDefinitionDataCategoryProviderMonoBehaviour {
    #[serde(rename = "tierToCategory")]
    pub tier_to_category: Vec<Reference>,
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
    pub category: Reference,
}

const TECH_TREE_RECIPE_UNLOCK_GUID: &str = "a6a9c74a16da41b8da0c11bafb544c69";

#[derive(Debug, Deserialize)]
pub struct TechTreeRecipeUnlockMonoBehaviour {
    #[serde(rename = "tier")]
    pub tier: i64,

    #[serde(rename = "recipes")]
    pub recipes: Vec<Reference>,
}

const GAME_DATA_MANIFEST_GUID: &str = "40dc454a91d591f3af0f535c9bb857cb";

#[derive(Debug, Deserialize)]
pub struct GameDataManifestMonoBehaviour {
    #[serde(rename = "assets")]
    pub assets: Vec<Reference>,
}

macro_rules! impl_known_mono_behaviour {
    ($($guids:pat => $V:ident($T:ty)),* $(,)?) => {
        #[derive(Debug)]
        pub enum KnownMonoBehaviour {
            $(
                $V($T)
            ),*
        }

        impl<'de> Deserialize<'de> for MonoBehaviour {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde_yaml::Value::deserialize(deserializer)?;

                let meta = MonoBehaviourMeta::deserialize(&value).map_err(serde::de::Error::custom)?;

                let Some(script) = meta.script.0 else {
                    return Ok(MonoBehaviour::Unknown(value));
                };

                Ok(match script.guid.as_str() {
                    $(
                        $guids => MonoBehaviour::Known(KnownMonoBehaviour::$V(
                            Deserialize::deserialize(value).map_err(serde::de::Error::custom)?,
                        )),
                    )*
                    _ => MonoBehaviour::Unknown(value),
                })
            }
        }

    }
}

impl_known_mono_behaviour! {
    RECIPE_GUID => Recipe(RecipeMonoBehaviour),
    PRODUCT_DEFINITION_GUID => ProductDefinition(ProductDefinitionMonoBehaviour),
    PRODUCT_CATEGORY_GUID => ProductCategory(ProductCategoryMonoBehaviour),
    PRODUCT_CATEGORY_MODIFIER_INFO_GUID => ProductCategoryModifierInfo(ProductCategoryModifierInfoMonoBehaviour),
    FARM_GUID | GATHERER_HUB_GUID => GathererHub(GathererHubMonoBehaviour),
    FACTORY_GUID => Factory(FactoryMonoBehaviour),
    FIELD_GUID | DISCONNECTED_HARVESTER | HARVESTER_GUID => Harvester(HarvesterMonoBehaviour),
    BUILDING_GUID => Building(BuildingMonoBehaviour),
    TECH_TREE_RECIPE_UNLOCK_GUID => TechTreeRecipeUnlock(TechTreeRecipeUnlockMonoBehaviour),
    PRODUCT_DEFINITION_DATA_CATEGORY_PROVIDER_GUID => ProductDefinitionDataCategoryProvider(ProductDefinitionDataCategoryProviderMonoBehaviour),
    GAME_DATA_MANIFEST_GUID => GameDataManifest(GameDataManifestMonoBehaviour),
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
