use std::{collections::BTreeMap, num::NonZeroUsize};

use log::warn;

pub mod serialization;

#[derive(Copy, Clone)]
pub struct Query<'data, T> {
    data: &'data GameData,
    target: T,
}

impl<'data, T> Query<'data, T> {
    pub fn new(data: &'data GameData, target: T) -> Self {
        Self { data, target }
    }
}

impl<'data, T> std::ops::Deref for Query<'data, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

#[derive(Debug)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
}

#[derive(Debug)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
    pub entries: Vec<RecipeEntry>,
    pub days: i64,
    pub required_modules: Vec<ModuleId>,
}

impl<'d> Query<'d, &'d Recipe> {
    pub fn entries(&self) -> impl Iterator<Item = Query<'d, &'d RecipeEntry>> {
        self.entries
            .iter()
            .map(|entry| Query::new(self.data, entry))
    }

    pub fn inputs(&self) -> impl Iterator<Item = Query<'d, &'d RecipeEntry>> {
        self.entries().filter(|entry| entry.is_input())
    }

    pub fn outputs(&self) -> impl Iterator<Item = Query<'d, &'d RecipeEntry>> {
        self.entries().filter(|entry| entry.is_output())
    }

    pub fn required_modules(&self) -> impl Iterator<Item = Query<'d, &'d Module>> {
        self.required_modules
            .iter()
            .map(|&index| self.data.query(index))
    }

    pub fn required_module(&self) -> Option<Query<'d, &'d Module>> {
        let mut iter = self.required_modules();
        let first = iter.next();
        if iter.next().is_some() {
            panic!("More than one module for recipe {:?}", &self.target.name);
        }
        first
    }
}

const EASY_CHAIN_DAYS: f64 = 15.0;

impl Recipe {
    pub fn easy_chains_days(&self) -> f64 {
        f64::max(
            (self.days as f64 / EASY_CHAIN_DAYS).round() * EASY_CHAIN_DAYS,
            EASY_CHAIN_DAYS,
        )
    }
}

#[derive(Debug)]
pub struct RecipeEntry {
    pub product_id: ProductId,
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
    pub fn product(&self) -> Query<'data, &'data Product> {
        self.data.query(self.target.product_id)
    }
}

#[derive(Debug)]
pub struct Building {
    pub id: BuildingId,
    pub name: String,
    pub base_cost: i64,
    available_recipes: Vec<RecipeId>,
}

impl<'d> Query<'d, &'d Building> {
    pub fn available_recipes(&self) -> impl Iterator<Item = Query<'d, &'d Recipe>> {
        self.target
            .available_recipes
            .iter()
            .map(|&id| self.data.query(id))
    }

    pub fn building_recipe(&self, name: &str) -> Query<'_, &'_ Recipe> {
        let mut iter = self
            .available_recipes()
            .filter(|&recipe| recipe.name == name);
        let Some(first) = iter.next() else {
            panic!(
                "No recipe with name {name:?} for building {:?}",
                &self.target.name
            );
        };
        if iter.next().is_some() {
            panic!(
                "More than one recipe with name {name:?} for building {:?}",
                &self.target.name
            );
        }
        first
    }
}

#[derive(Debug)]
pub struct Module {
    pub id: ModuleId,
    pub name: String,
    pub base_cost: i64,
}

#[derive(Debug)]
pub struct GameData {
    products: Vec<Product>,
    recipes: Vec<Recipe>,
    modules: Vec<Module>,
    buildings: Vec<Building>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
struct Index(NonZeroUsize);

impl Index {
    pub fn new(value: usize) -> Self {
        NonZeroUsize::new(value.wrapping_add(1)).map(Self).unwrap()
    }

    pub fn get(&self) -> usize {
        self.0.get().wrapping_sub(1)
    }
}

impl std::fmt::Debug for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

macro_rules! impl_id {
    ($Id:ident for $Container:ident.$field:ident: $Field:ty) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $Id($crate::Index);

        impl ::std::ops::Index<$Id> for $Container {
            type Output = $Field;

            fn index(&self, index: $Id) -> &Self::Output {
                &self.$field[index.0.get()]
            }
        }
    };
}

impl_id!(ProductId for GameData.products: Product);
impl_id!(RecipeId for GameData.recipes: Recipe);
impl_id!(BuildingId for GameData.buildings: Building);
impl_id!(ModuleId for GameData.modules: Module);

impl GameData {
    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let data: crate::serialization::GameData = serde_json::from_str(&contents)?;

        let (product_guid_to_id, products) = data.products.into_iter().fold(
            (BTreeMap::default(), Vec::default()),
            |(mut guid_to_index, mut values), (guid, product)| {
                let id = ProductId(Index::new(values.len()));
                guid_to_index.insert(guid, id);
                values.push(Product {
                    id,
                    name: product.name,
                });
                (guid_to_index, values)
            },
        );

        let (module_guid_to_id, modules) = data.modules.into_iter().fold(
            (BTreeMap::default(), Vec::default()),
            |(mut guid_to_index, mut values), (guid, module)| {
                let id = ModuleId(Index::new(values.len()));
                guid_to_index.insert(guid, id);
                values.push(Module {
                    id,
                    name: module.name,
                    base_cost: module.base_cost,
                });
                (guid_to_index, values)
            },
        );

        let (recipe_guid_to_id, recipes) = data.recipes.into_iter().fold(
            (BTreeMap::default(), Vec::default()),
            |(mut guid_to_id, mut values), (guid, recipe)| {
                let id = RecipeId(Index::new(values.len()));

                let required_modules = recipe
                    .required_modules
                    .iter()
                    .map(|module_guid| {
                        let module_id = module_guid_to_id.get(module_guid).copied();
                        if module_id.is_none() {
                            warn!(
                                "Recipe {:?} (guid: {}) refers to an unknown Module (guid: {})",
                                &recipe.name, &guid, &module_guid,
                            );
                        }
                        module_id
                    })
                    .collect::<Option<Vec<ModuleId>>>();

                if let Some(required_modules) = required_modules {
                    guid_to_id.insert(guid, id);
                    values.push(Recipe {
                        id,
                        name: recipe.name,
                        entries: recipe
                            .products
                            .iter()
                            .map(|x| RecipeEntry {
                                product_id: product_guid_to_id[&x.product_id],
                                amount: x.amount,
                            })
                            .collect(),
                        days: recipe.days,
                        required_modules,
                    });
                }

                (guid_to_id, values)
            },
        );

        let (_building_guid_to_id, buildings) = data.buildings.into_iter().fold(
            (BTreeMap::default(), Vec::default()),
            |(mut guid_to_id, mut values), (guid, building)| {
                let id = BuildingId(Index::new(values.len()));
                guid_to_id.insert(guid, id);
                values.push(Building {
                    id,
                    name: building.name,
                    base_cost: building.base_cost,
                    available_recipes: building
                        .available_recipes
                        .iter()
                        .map(|guid| recipe_guid_to_id[guid])
                        .collect(),
                });
                (guid_to_id, values)
            },
        );

        Ok(Self {
            products,
            modules,
            recipes,
            buildings,
        })
    }

    pub fn query<I>(&self, index: I) -> Query<'_, &'_ <Self as std::ops::Index<I>>::Output>
    where
        Self: std::ops::Index<I>,
    {
        Query::new(self, &self[index])
    }

    pub fn products(&self) -> impl Iterator<Item = Query<'_, &'_ Product>> {
        self.products
            .iter()
            .map(|product| Query::new(self, product))
    }

    pub fn product(&self, name: &str) -> Query<'_, &'_ Product> {
        let mut iter = self.products().filter(|&product| product.name == name);
        let Some(first) = iter.next() else {
            panic!("No product with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one product with name {name:?}");
        }
        first
    }

    pub fn recipes(&self) -> impl Iterator<Item = Query<'_, &'_ Recipe>> {
        self.recipes.iter().map(|recipe| Query::new(self, recipe))
    }

    pub fn recipes_with_output(
        &self,
        product_id: ProductId,
    ) -> impl Iterator<Item = Query<'_, &'_ Recipe>> {
        self.recipes().filter(move |&recipe| {
            recipe
                .outputs()
                .any(|output| output.product_id == product_id)
        })
    }

    pub fn recipe(&self, name: &str) -> Query<'_, &'_ Recipe> {
        let mut iter = self.recipes().filter(|&recipe| recipe.name == name);
        let Some(first) = iter.next() else {
            panic!("No recipe with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one recipe with name {name:?}");
        }
        first
    }

    pub fn buildings(&self) -> impl Iterator<Item = Query<'_, &'_ Building>> {
        self.buildings
            .iter()
            .map(|building| Query::new(self, building))
    }

    pub fn building(&self, name: &str) -> Query<'_, &'_ Building> {
        let mut iter = self.buildings().filter(|&building| building.name == name);
        let Some(first) = iter.next() else {
            panic!("No building with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one building with name {name:?}");
        }
        first
    }

    pub fn modules(&self) -> impl Iterator<Item = Query<'_, &'_ Module>> {
        self.modules.iter().map(|module| Query::new(self, module))
    }

    pub fn module(&self, name: &str) -> Query<'_, &'_ Module> {
        let mut iter = self.modules().filter(|&module| module.name == name);
        let Some(first) = iter.next() else {
            panic!("No module with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one module with name {name:?}");
        }
        first
    }

    pub fn recipe_module<'data>(&'data self, recipe: &'data Recipe) -> Query<'_, &'_ Module> {
        let mut iter = Query::new(self, recipe).required_modules();
        let Some(first) = iter.next() else {
            panic!("No module for recipe {:?}", &recipe.name);
        };
        if iter.next().is_some() {
            panic!("More than one module for recipe {:?}", &recipe.name);
        }
        first
    }
}
