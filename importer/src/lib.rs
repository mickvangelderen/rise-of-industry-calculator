#![allow(unused)]

// Import assets exported from the game files with https://assetripper.github.io/AssetRipper/articles/Downloads.html.

use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf}, collections::BTreeMap, fs::FileType,
};

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};

struct Script<'a> {
    path: &'a str,
    guid: &'a str,
}

const RECIPE_SCRIPT: Script<'static> = Script {
    path: "ExportedProject/Assets/Scripts/Assembly-CSharp/ProjectAutomata/Recipe.cs",
    guid: "86eee4258519014ad55f04d4a92d2556",
};

const PRODUCT_DEFINITION_GUID: &str = "23940808cf3b3e11ddbcefa65cb07256";
const PRODUCT_DEFINITION_ROOTS: &[&str] = &[
    "ExportedProject/Assets/MonoBehavior",
    "ExportedProject/Assets/Resources/gamedata",
];

#[derive(Debug, Deserialize)]
struct MonoBehaviourDocument {
    #[serde(rename = "MonoBehaviour")]
    mono_behavior: serde_yaml::Value,
}

#[derive(Debug, Deserialize)]
struct MonoBehaviourMetaData {
    #[serde(rename = "m_Script")]
    script: ScriptReference,
}

#[derive(Debug, Deserialize)]
struct ScriptReference {
    // #[serde(rename = "")]
    // file_id: u64,
    #[serde(rename = "guid")]
    guid: String,
    // #[serde(rename = "type")]
    // ty: u64,
}

#[derive(Debug, Deserialize)]
struct RecipeMonoBehaviour {
    #[serde(rename = "Title")]
    name: String,

    #[serde(rename = "ingredients")]
    ingredients: ProductList,

    #[serde(rename = "result")]
    result: ProductList,
}

#[derive(Debug, Deserialize)]
struct ProductList {
    #[serde(rename = "entries")]
    entries: Vec<Ingredient>,
}

#[derive(Debug, Deserialize)]
struct Ingredient {
    #[serde(rename = "_definition")]
    definition: ScriptReference,

    #[serde(rename = "amount")]
    amount: u64,
}

#[derive(Debug, Deserialize)]
struct ProductMonoBehavior {
    #[serde(rename = "productName")]
    title: String,

    #[serde(rename = "price")]
    ingredients: ProductList,

    #[serde(rename = "result")]
    result: ProductList,
}

const PRODUCT_GUID: &str = "140d8ecb3bb64d6ce76eb1e625f1a7a8";
