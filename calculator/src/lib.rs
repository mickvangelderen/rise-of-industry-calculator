use std::{collections::BTreeMap, num::NonZeroUsize};

use log::error;

pub mod serialization;

#[derive(Copy, Clone)]
pub struct Query<'data, T> {
    data: &'data GameData,
    index: T,
}

impl<'data, T> Query<'data, T> {
    pub fn new(data: &'data GameData, target: T) -> Self {
        Self {
            data,
            index: target,
        }
    }
}

impl<'data, T> std::ops::Deref for Query<'data, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

struct Product {
    name: String,
    category: ProductCategoryIndex,
    price_formula: serialization::ProductPriceFormula,
}

typed_index::impl_typed_index!(pub struct ProductIndex(index_types::IndexU32));

type ProductVec<T> = typed_index::TypedIndexVec<ProductIndex, T>;

#[derive(Default)]
pub struct ProductData {
    pub name: ProductVec<String>,
    pub category: ProductVec<ProductCategoryIndex>,
    pub price_formula: ProductVec<serialization::ProductPriceFormula>,
}

impl ProductData {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            name: ProductVec::with_capacity(capacity),
            category: ProductVec::with_capacity(capacity),
            price_formula: ProductVec::with_capacity(capacity),
        }
    }

    pub fn indices(&self) -> impl Iterator<Item = ProductIndex> + '_ {
        (0..self.name.len()).map(|index| ProductIndex(index.into()))
    }

    pub fn push(&mut self, product: Product) {
        self.name.push(product.name);
        self.category.push(product.category);
        self.price_formula.push(product.price_formula);
    }
}

impl FromIterator<Product> for ProductData {
    fn from_iter<T: IntoIterator<Item = Product>>(iter: T) -> Self {
        let iter = iter.into_iter();

        iter.into_iter().fold(Self::default(), |mut data, entity| {
            data.push(entity);
            data
        })
    }
}

impl<'data> Query<'data, ProductIndex> {
    pub fn name(&self) -> &str {
        &self.data.product.name[self.index]
    }

    pub fn category(&self) -> Query<'data, ProductCategoryIndex> {
        Query::new(self.data, self.data.product.category[self.index])
    }

    pub fn producing_recipes<'a: 'data>(
        &'a self,
    ) -> impl Iterator<Item = Query<'data, RecipeIndex>> {
        self.data
            .recipe
            .entries
            .iter()
            .index()
            .filter(move |&(_, entries)| {
                entries
                    .iter()
                    .any(move |entry| entry.product_id == self.index && entry.amount > 0)
            })
            .map(move |(index, _)| Query::new(self.data, index))
    }
}

typed_index::impl_typed_index!(pub struct RecipeIndex(index_types::IndexU32));

type RecipeVec<T> = typed_index::TypedIndexVec<RecipeIndex, T>;

#[derive(Default)]
pub struct RecipeData {
    pub name: RecipeVec<String>,
    pub entries: RecipeVec<Vec<RecipeEntry>>,
    pub days: RecipeVec<i64>,
    pub required_modules: RecipeVec<Vec<ModuleIndex>>,
}

impl RecipeData {
    fn indices(&self) -> impl Iterator<Item = RecipeIndex> + '_ {
        self.name.iter().index().map(|(index, _)| index)
    }
}

impl<'data> Query<'data, RecipeIndex> {
    pub fn name(&self) -> &str {
        &self.data.recipe.name[self.index]
    }

    pub fn entries(&self) -> impl Iterator<Item = Query<'data, RecipeEntry>> {
        self.data.recipe.entries[self.index]
            .iter()
            .map(|&entry| Query::new(self.data, entry))
    }

    pub fn inputs(&self) -> impl Iterator<Item = Query<'data, RecipeEntry>> {
        self.entries().filter(|entry| entry.is_input())
    }

    pub fn outputs(&self) -> impl Iterator<Item = Query<'data, RecipeEntry>> {
        self.entries().filter(|entry| entry.is_output())
    }

    pub fn days(&self) -> i64 {
        self.data.recipe.days[self.index]
    }

    pub fn required_modules(&self) -> impl Iterator<Item = Query<'data, ModuleIndex>> {
        self.data.recipe.required_modules[self.index]
            .iter()
            .map(|&index| Query::new(self.data, index))
    }

    pub fn required_module(&self) -> Option<Query<'data, ModuleIndex>> {
        let mut iter = self.required_modules();
        let first = iter.next();
        if iter.next().is_some() {
            panic!(
                "More than one module for recipe {:?}",
                &self.data.recipe.name[self.index]
            );
        }
        first
    }

    pub fn easy_chains_days(&self) -> f64 {
        const EASY_CHAIN_DAYS: f64 = 15.0;
        f64::max(
            (self.days() as f64 / EASY_CHAIN_DAYS).round() * EASY_CHAIN_DAYS,
            EASY_CHAIN_DAYS,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecipeEntry {
    pub product_id: ProductIndex,
    pub amount: i64,
}

impl RecipeEntry {
    pub fn is_input(&self) -> bool {
        self.amount < 0
    }

    pub fn is_output(&self) -> bool {
        self.amount > 0
    }
}

impl<'data> Query<'data, RecipeEntry> {
    pub fn product(&self) -> Query<'data, ProductIndex> {
        Query::new(self.data, self.product_id)
    }
}

typed_index::impl_typed_index!(pub struct BuildingIndex(index_types::IndexU32));

type BuildingVec<T> = typed_index::TypedIndexVec<BuildingIndex, T>;

#[derive(Default)]
pub struct BuildingData {
    name: BuildingVec<String>,
    base_cost: BuildingVec<i64>,
    available_recipes: BuildingVec<Vec<RecipeIndex>>,
}

impl BuildingData {
    fn indices(&self) -> impl Iterator<Item = BuildingIndex> + '_ {
        self.name.iter().index().map(|(index, _)| index)
    }
}

impl<'data> Query<'data, BuildingIndex> {
    pub fn name(&self) -> &str {
        &self.data.building.name[self.index]
    }

    pub fn base_cost(&self) -> i64 {
        self.data.building.base_cost[self.index]
    }

    pub fn available_recipes(&self) -> impl Iterator<Item = Query<'data, RecipeIndex>> {
        self.data.building.available_recipes[self.index]
            .iter()
            .map(|&index| Query::new(self.data, index))
    }
}

typed_index::impl_typed_index!(pub struct ModuleIndex(index_types::IndexU32));

type ModuleVec<T> = typed_index::TypedIndexVec<ModuleIndex, T>;

#[derive(Default)]
pub struct ModuleData {
    name: ModuleVec<String>,
    base_cost: ModuleVec<i64>,
}

impl ModuleData {
    fn indices(&self) -> impl Iterator<Item = ModuleIndex> + '_ {
        self.name.iter().index().map(|(index, _)| index)
    }
}

typed_index::impl_typed_index!(pub struct ProductCategoryIndex(index_types::IndexU32));

type ProductCategoryVec<T> = typed_index::TypedIndexVec<ProductCategoryIndex, T>;

impl<'data> Query<'data, ModuleIndex> {
    pub fn name(&self) -> &str {
        &self.data.module.name[self.index]
    }
}

#[derive(Default)]
pub struct ProductCategoryData {
    pub name: ProductCategoryVec<String>,
    pub price_modifier: ProductCategoryVec<f64>,
    pub growth_modifier: ProductCategoryVec<f64>,
}

pub struct GameData {
    product: ProductData,
    recipe: RecipeData,
    module: ModuleData,
    building: BuildingData,
    product_category: ProductCategoryData,
}

impl GameData {
    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let data: crate::serialization::GameData = serde_json::from_str(&contents)?;

        Ok(Self {
            product: Default::default(),
            module: Default::default(),
            recipe: Default::default(),
            building: Default::default(),
            product_category: Default::default(),
        })
    }

    pub fn query<I>(&self, index: I) -> Query<'_, &'_ <Self as std::ops::Index<I>>::Output>
    where
        Self: std::ops::Index<I>,
    {
        Query::new(self, &self[index])
    }

    pub fn products(&self) -> impl Iterator<Item = Query<'_, ProductIndex>> {
        self.product
            .name
            .iter()
            .index()
            .map(|(index, _)| Query::new(self, index))
    }

    pub fn product_by_name(&self, name: &str) -> Query<'_, ProductIndex> {
        let mut iter = self.products().filter(|&product| product.name() == name);
        let Some(first) = iter.next() else {
            panic!("No product with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one product with name {name:?}");
        }
        first
    }

    pub fn recipes(&self) -> impl Iterator<Item = Query<'_, RecipeIndex>> {
        self.recipe.indices().map(|recipe| Query::new(self, recipe))
    }

    pub fn recipes_with_output(
        &self,
        product_id: ProductIndex,
    ) -> impl Iterator<Item = Query<'_, RecipeIndex>> {
        self.recipes().filter(move |&recipe| {
            recipe
                .outputs()
                .any(|output| output.product_id == product_id)
        })
    }

    pub fn recipe_by_name(&self, name: &str) -> Query<'_, RecipeIndex> {
        let mut iter = self.recipes().filter(|&recipe| recipe.name() == name);
        let Some(first) = iter.next() else {
            panic!("No recipe with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one recipe with name {name:?}");
        }
        first
    }

    pub fn buildings(&self) -> impl Iterator<Item = Query<'_, BuildingIndex>> {
        self.building
            .indices()
            .map(|building| Query::new(self, building))
    }

    pub fn building_by_name(&self, name: &str) -> Query<'_, BuildingIndex> {
        let mut iter = self.buildings().filter(|&building| building.name() == name);
        let Some(first) = iter.next() else {
            panic!("No building with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one building with name {name:?}");
        }
        first
    }

    pub fn modules(&self) -> impl Iterator<Item = Query<'_, ModuleIndex>> {
        self.module.indices().map(|module| Query::new(self, module))
    }

    pub fn module_by_name(&self, name: &str) -> Query<'_, ModuleIndex> {
        let mut iter = self.modules().filter(|module| module.name() == name);
        let Some(first) = iter.next() else {
            panic!("No module with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one module with name {name:?}");
        }
        first
    }

    pub fn recipe_module<'data>(&'data self, recipe: RecipeIndex) -> Query<'_, ModuleIndex> {
        let mut iter = Query::new(self, recipe).required_modules();
        let Some(first) = iter.next() else {
            panic!("No module for recipe {:?}", &self.recipe.name[recipe]);
        };
        if iter.next().is_some() {
            panic!(
                "More than one module for recipe {:?}",
                &self.recipe.name[recipe]
            );
        }
        first
    }
}
