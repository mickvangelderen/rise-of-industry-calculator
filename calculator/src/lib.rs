use std::{collections::BTreeMap, num::NonZeroUsize};

use log::warn;

pub mod serialization;

#[derive(Debug)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
}

#[derive(Debug)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
    pub products: Vec<CountedProductId>,
    pub days: i64,
    pub required_modules: Vec<ModuleId>,
}

const EASY_CHAIN_DAYS: f64 = 15.0;

impl Recipe {
    pub fn products<'a>(&'a self, data: &'a GameData) -> impl Iterator<Item = CountedProduct<'a>> {
        self.products.iter().map(|counted_product| CountedProduct {
            product: &data[counted_product.product_id],
            amount: counted_product.amount,
        })
    }

    pub fn required_modules<'a>(&'a self, data: &'a GameData) -> impl Iterator<Item = &'a Module> {
        self.required_modules.iter().copied().map(|id| &data[id])
    }

    pub fn easy_chains_days(&self) -> f64 {
        f64::max(
            (self.days as f64 / EASY_CHAIN_DAYS).round() * EASY_CHAIN_DAYS,
            EASY_CHAIN_DAYS,
        )
    }
}

#[derive(Debug)]
pub struct CountedProductId {
    pub product_id: ProductId,
    pub amount: i64,
}

#[derive(Debug)]
pub struct CountedProduct<'a> {
    pub product: &'a Product,
    pub amount: i64,
}

#[derive(Debug)]
pub struct Building {
    pub id: BuildingId,
    pub name: String,
    pub base_cost: i64,
    available_recipes: Vec<RecipeId>,
}

impl Building {
    pub fn available_recipes<'a>(&'a self, data: &'a GameData) -> impl Iterator<Item = &'a Recipe> {
        self.available_recipes.iter().copied().map(|id| &data[id])
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
                        products: recipe
                            .products
                            .iter()
                            .map(|x| CountedProductId {
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

    pub fn products(&self) -> impl Iterator<Item = &'_ Product> {
        self.products.iter()
    }

    pub fn product(&self, name: &str) -> &Product {
        let mut iter = self.products().filter(|&product| product.name == name);
        let Some(first) = iter.next() else {
            panic!("No product with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one product with name {name:?}");
        }
        first
    }

    pub fn recipes(&self) -> impl Iterator<Item = &'_ Recipe> {
        self.recipes.iter()
    }

    pub fn recipe(&self, name: &str) -> &Recipe {
        let mut iter = self.recipes().filter(|&recipe| recipe.name == name);
        let Some(first) = iter.next() else {
            panic!("No recipe with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one recipe with name {name:?}");
        }
        first
    }

    pub fn building_recipe<'a>(&'a self, building: &'a Building, name: &str) -> &'a Recipe {
        let mut iter = building
            .available_recipes(self)
            .filter(|&recipe| recipe.name == name);
        let Some(first) = iter.next() else {
            panic!(
                "No recipe with name {name:?} for building {:?}",
                &building.name
            );
        };
        if iter.next().is_some() {
            panic!(
                "More than one recipe with name {name:?} for building {:?}",
                &building.name
            );
        }
        first
    }

    pub fn buildings(&self) -> impl Iterator<Item = &'_ Building> {
        self.buildings.iter()
    }

    pub fn building(&self, name: &str) -> &Building {
        let mut iter = self.buildings().filter(|&building| building.name == name);
        let Some(first) = iter.next() else {
            panic!("No building with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one building with name {name:?}");
        }
        first
    }

    pub fn modules(&self) -> impl Iterator<Item = &'_ Module> {
        self.modules.iter()
    }

    pub fn module(&self, name: &str) -> &Module {
        let mut iter = self.modules().filter(|&module| module.name == name);
        let Some(first) = iter.next() else {
            panic!("No module with name {name:?}");
        };
        if iter.next().is_some() {
            panic!("More than one module with name {name:?}");
        }
        first
    }

    pub fn recipe_module<'a>(&'a self, recipe: &'a Recipe) -> &'a Module {
        let mut iter = recipe.required_modules(self);
        let Some(first) = iter.next() else {
            panic!("No module for recipe {:?}", &recipe.name);
        };
        if iter.next().is_some() {
            panic!("More than one module for recipe {:?}", &recipe.name);
        }
        first
    }
}
