use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type ProductId = String;
type RecipeId = String;
type ModuleId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    // pub id: ProductId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    // pub id: RecipeId,
    pub name: String,
    pub products: Vec<CountedProductId>,
    pub required_modules: Vec<ModuleId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountedProductId {
    pub product_id: ProductId,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    // pub id: ModuleId,
    pub name: String,
    pub available_recipes: Vec<RecipeId>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameData {
    pub products: BTreeMap<ProductId, Product>,
    pub recipes: BTreeMap<RecipeId, Recipe>,
    pub modules: BTreeMap<ModuleId, Module>,
}
