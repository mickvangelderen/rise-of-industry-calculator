use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type ProductId = String;
type RecipeId = String;
type BuildingModuleId = String;
type BuildingId = String;
type ProductCategoryId = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProductPriceFormula {
    /// (ingredientsValue + ((upkeep / 30) * recipeDays)) / recipeOutput
    Factories,
    /// ingredientsValue * 2.8
    FarmProduce,
    /// ((ingredientsValue * 3) + ((upkeep / 30) * recipeDays)) / (recipeOutput * 3)
    Farms,
    /// upkeep / ((3 * recipeOutput) * (30 / recipeDays))
    Gatherers,
    /// ((((ingredientsValue * 3) + ((upkeep / 30) * recipeDays)) / (recipeOutput * 3)) * (recipeOutput - productOutput)) / recipeOutput
    Livestock,
    /// 75 * recipeDays * 3.25
    RawResources,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub category: ProductCategoryId,
    pub price_formula: ProductPriceFormula,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub entries: Vec<RecipeEntry>,
    pub days: i64,
    pub required_modules: Vec<BuildingModuleId>,
    pub tier: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeEntry {
    pub product_id: ProductId,
    pub amount: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildingModule {
    pub name: String,
    pub base_cost: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    pub base_cost: i64,
    pub available_recipes: Vec<RecipeId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCategory {
    pub name: String,
    pub price_modifier: f64,
    pub growth_modifier: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameData {
    pub products: BTreeMap<ProductId, Product>,
    pub recipes: BTreeMap<RecipeId, Recipe>,
    pub modules: BTreeMap<BuildingModuleId, BuildingModule>,
    pub buildings: BTreeMap<BuildingId, Building>,
    pub product_categories: BTreeMap<ProductCategoryId, ProductCategory>,
}
