use serialization::ProductPriceFormula;

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

    pub fn data(&self) -> &'data GameData {
        self.data
    }

    pub fn index(&self) -> T
    where
        T: Copy,
    {
        self.target
    }
}

impl<'data, T> std::cmp::Eq for Query<'data, T> where T: Eq {}

impl<'data, T> std::cmp::PartialEq for Query<'data, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.data, other.data) && self.target == other.target
    }
}

impl<'data, T> std::ops::Deref for Query<'data, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

pub struct Product {
    pub name: String,
    pub product_category_index: ProductCategoryIndex,
    pub price_formula: serialization::ProductPriceFormula,
}

typed_index::impl_typed_index!(pub struct ProductIndex(index_types::IndexU32));

pub type ProductVec<T> = typed_index::TypedIndexVec<ProductIndex, T>;

#[derive(Default)]
pub struct ProductData {
    pub name: ProductVec<String>,
    pub product_category_index: ProductVec<ProductCategoryIndex>,
    pub price_formula: ProductVec<serialization::ProductPriceFormula>,
}

impl ProductData {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            name: ProductVec::with_capacity(capacity),
            product_category_index: ProductVec::with_capacity(capacity),
            price_formula: ProductVec::with_capacity(capacity),
        }
    }

    pub fn indices(&self) -> impl Iterator<Item = ProductIndex> + '_ {
        (0..self.name.len()).map(|index| ProductIndex(index.into()))
    }

    pub fn push(&mut self, product: Product) {
        self.name.push(product.name);
        self.product_category_index
            .push(product.product_category_index);
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
        &self.data.product.name[self.target]
    }

    pub fn category(&self) -> Query<'data, ProductCategoryIndex> {
        Query::new(
            self.data,
            self.data.product.product_category_index[self.target],
        )
    }

    pub fn price_formula(&self) -> ProductPriceFormula {
        self.data.product.price_formula[self.target]
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
                    .any(move |entry| entry.product_index == self.target && entry.amount > 0)
            })
            .map(move |(index, _)| Query::new(self.data, index))
    }
}

typed_index::impl_typed_index!(pub struct RecipeIndex(index_types::IndexU32));

pub type RecipeVec<T> = typed_index::TypedIndexVec<RecipeIndex, T>;

#[derive(Default)]
pub struct RecipeData {
    pub name: RecipeVec<String>,
    pub entries: RecipeVec<Vec<RecipeEntry>>,
    pub days: RecipeVec<i64>,
    pub required_module_indices: RecipeVec<Vec<ModuleIndex>>,
}

impl RecipeData {
    pub fn indices(&self) -> impl Iterator<Item = RecipeIndex> + '_ {
        self.name.iter().index().map(|(index, _)| index)
    }
}

impl<'data> Query<'data, RecipeIndex> {
    pub fn name(&self) -> &str {
        &self.data.recipe.name[self.target]
    }

    pub fn entries(&self) -> impl Iterator<Item = Query<'data, RecipeEntry>> {
        self.data.recipe.entries[self.target]
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
        self.data.recipe.days[self.target]
    }

    pub fn required_modules(&self) -> impl Iterator<Item = Query<'data, ModuleIndex>> {
        self.data.recipe.required_module_indices[self.target]
            .iter()
            .map(|&index| Query::new(self.data, index))
    }

    pub fn required_module(&self) -> Option<Query<'data, ModuleIndex>> {
        let mut iter = self.required_modules();
        let first = iter.next();
        if iter.next().is_some() {
            panic!(
                "More than one module for recipe {:?}",
                &self.data.recipe.name[self.target]
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
    pub product_index: ProductIndex,
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
        Query::new(self.data, self.product_index)
    }
}

typed_index::impl_typed_index!(pub struct BuildingIndex(index_types::IndexU32));

pub type BuildingVec<T> = typed_index::TypedIndexVec<BuildingIndex, T>;

#[derive(Default)]
pub struct BuildingData {
    name: BuildingVec<String>,
    base_cost: BuildingVec<i64>,
    available_recipe_indices: BuildingVec<Vec<RecipeIndex>>,
}

impl BuildingData {
    pub fn indices(&self) -> impl Iterator<Item = BuildingIndex> + '_ {
        self.name.iter().index().map(|(index, _)| index)
    }
}

impl<'data> Query<'data, BuildingIndex> {
    pub fn name(&self) -> &str {
        &self.data.building.name[self.target]
    }

    pub fn base_cost(&self) -> i64 {
        self.data.building.base_cost[self.target]
    }

    pub fn available_recipes(&self) -> impl Iterator<Item = Query<'data, RecipeIndex>> {
        self.data.building.available_recipe_indices[self.target]
            .iter()
            .map(|&index| Query::new(self.data, index))
    }

    pub fn recipe_by_name(&self, name: &str) -> Query<'data, RecipeIndex> {
        let mut iter = self
            .available_recipes()
            .filter(|&recipe| recipe.name() == name);
        let Some(first) = iter.next() else {
            panic!("No recipes found");
        };
        if iter.next().is_some() {
            panic!("Multiple recipes found");
        }
        first
    }
}

typed_index::impl_typed_index!(pub struct ModuleIndex(index_types::IndexU32));

pub type ModuleVec<T> = typed_index::TypedIndexVec<ModuleIndex, T>;

#[derive(Default)]
pub struct ModuleData {
    name: ModuleVec<String>,
    base_cost: ModuleVec<i64>,
}

impl ModuleData {
    pub fn indices(&self) -> impl Iterator<Item = ModuleIndex> + '_ {
        self.name.iter().index().map(|(index, _)| index)
    }
}

impl<'data> Query<'data, ModuleIndex> {
    pub fn name(&self) -> &str {
        &self.data.module.name[self.target]
    }

    pub fn base_cost(&self) -> i64 {
        self.data.module.base_cost[self.target]
    }
}

macro_rules! impl_soa {
    (@all_fields
        $T:ident {
            $($f:ident: $t:ty as $r:ty = $m:expr,)*
        }
        index = $X:ident;
        vec = $V:ident;
        data = $D:ident;
    ) => {
        pub struct $T {
            $($f: $t,)*
        }

        #[derive(Default)]
        pub struct $D {
            $($f: $V<$t>,)*
        }

        impl $D {
            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    $($f: $V::with_capacity(capacity),)*
                }
            }

            pub fn push(&mut self, value: $T) {
                $(self.$f.push(value.$f);)*
            }
        }

        impl <'data> Query<'data, $X> {
            $(
                pub fn $f(&self) -> $r {
                    #![allow(clippy::redundant_closure_call)]
                    ($m)(&self.data.product_category.$f[self.target])
                }
            )*
        }
    };
    (
        $T:ident {
            $f0:ident: $t0:ty as $r0:ty = $m0:expr
            $(, $f:ident: $t:ty as $r:ty = $m:expr)* $(,)?
        }
        index = $X:ident;
        vec = $V:ident;
        data = $D:ident;
    ) => {
        typed_index::impl_typed_index!(pub struct $X(index_types::IndexU32));

        pub type $V<T> = typed_index::TypedIndexVec<$X, T>;

        impl $D {
            pub fn indices(&self) -> impl Iterator<Item = $X> + '_ {
                (0..self.$f0.len()).map(|index| $X(index.into()))
            }
        }

        impl FromIterator<$T> for $D {
            fn from_iter<I: IntoIterator<Item = $T>>(iter: I) -> Self {
                iter.into_iter().fold(Self::default(), |mut data, value| {
                    data.push(value);
                    data
                })
            }
        }

        impl_soa!(@all_fields
            $T {
                $f0: $t0 as $r0 = $m0, $($f: $t as $r = $m,)*
            }
            index = $X;
            vec = $V;
            data = $D;
        );
    }
}

impl_soa! {
    ProductCategory {
        name: String as &str = |x| x,
        price_modifier: f64 as f64 = |&x| x,
        growth_modifier: f64 as f64 = |&x| x,
    }
    index = ProductCategoryIndex;
    vec = ProductCategoryVec;
    data = ProductCategoryData;
}

pub struct GameData {
    pub product: ProductData,
    pub recipe: RecipeData,
    pub module: ModuleData,
    pub building: BuildingData,
    pub product_category: ProductCategoryData,
}

impl GameData {
    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let data: crate::serialization::GameData = serde_json::from_str(&contents)?;

        // let product_category = {
        //     let mut values = data.product_categories.into_iter().map(|(guid, value)| {
        //         (guid,
        //             ProductCategory {

        //             }
        //         )
        //     });
        //     values.sort_unstable_by_key(|(_, value)| &value.name);
        //     values.into_iter().fold(Default::default(), |(mut map, mut arr), (guid, value)| {

        //     })
        // };

        // let product = {
        //     let mut values = data.products.into_iter().map(|(guid, value)| {
        //         (
        //             guid,
        //             Product {
        //                 name: value.name,
        //                 product_category_index: value.
        //                 price_formula: todo!(),
        //             },
        //         )
        //     }).collect::<Vec<_>>();
        //     values.sort_unstable_by_key(|value| &value.1.name);

        // }.collect();

        Ok(Self {
            product: Default::default(),
            module: Default::default(),
            recipe: Default::default(),
            building: Default::default(),
            product_category: Default::default(),
        })
    }

    pub fn query<T>(&self, target: T) -> Query<'_, T> {
        Query::new(self, target)
    }

    pub fn products(&self) -> impl Iterator<Item = Query<'_, ProductIndex>> {
        self.product
            .name
            .iter()
            .index()
            .map(|(index, _)| self.query(index))
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
                .any(|output| output.product_index == product_id)
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
        self.module.indices().map(|index| self.query(index))
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

    pub fn recipe_module(&self, recipe: RecipeIndex) -> Query<'_, ModuleIndex> {
        let mut iter = self.query(recipe).required_modules();
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
