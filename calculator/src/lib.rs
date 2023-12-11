use serialization::ProductPriceFormula;

pub mod serialization;

pub trait Field {
    type Borrow;

    fn borrow(&self) -> Self::Borrow;
}

impl<'data, T> Field for Query<'data, &'data Vec<T>>
where
    T: 'data,
{
    type Borrow = Query<'data, std::slice::Iter<'data, T>>;

    fn borrow(&self) -> Self::Borrow {
        self.data.query(self.target.iter())
    }
}

macro_rules! impl_field_ {
    (@item) => {};
    (@item query copy $T:ty; $($tail:tt)*) => {
        impl<'data> Field for Query<'data, &'data $T> {
            type Borrow = Query<'data, $T>;

            fn borrow(&self) -> Self::Borrow {
                self.data.query(*self.target)
            }
        }
        impl_field_!(@item $($tail)*);
    };
    (@item copy $T:ty; $($tail:tt)*) => {
        impl<'data> Field for Query<'data, &'data $T> {
            type Borrow = $T;

            fn borrow(&self) -> Self::Borrow {
                *self.target
            }
        }
        impl_field_!(@item $($tail)*);
    };
    (@item deref $T:ty; $($tail:tt)*) => {
        impl<'data> Field for Query<'data, &'data $T> {
            type Borrow = &'data <$T as std::ops::Deref>::Target;

            fn borrow(&self) -> Self::Borrow {
                self.target
            }
        }
        impl_field_!(@item $($tail)*);
    };
}

macro_rules! impl_field {
    ($($body:tt)+) => {
        impl_field_!(@item $($body)*);
    };
}

impl_field! {
    copy i64;
    copy f64;
    copy ProductPriceFormula;
    deref String;
}

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

impl<'data, T, I> Iterator for Query<'data, I>
where
    T: 'data + Copy,
    I: Iterator<Item = &'data T>,
{
    type Item = Query<'data, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.target.next().map(|&item| self.data.query(item))
    }
}

macro_rules! impl_soa {
    (@all_fields
        $T:ident {
            $($f:ident: $t:ty,)*
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

        impl<'data> Query<'data, $X> {
            $(
                pub fn $f(&self) -> <Query<'data, &'data $t> as Field>::Borrow {
                    let data: &$D = self.data.as_ref();
                    self.data.query(&data.$f[self.target]).borrow()
                }
            )*
        }
    };
    (
        $T:ident {
            $f0:ident: $t0:ty
            $(, $f:ident: $t:ty)* $(,)?
        }
        index = $X:ident;
        vec = $V:ident;
        data = $D:ident;
    ) => {
        typed_index::impl_typed_index!(pub struct $X(index_types::IndexU32));

        impl_field! {
            query copy $X;
        }

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
                $f0: $t0, $($f: $t,)*
            }
            index = $X;
            vec = $V;
            data = $D;
        );
    }
}

impl_soa! {
    Product {
        name: String,
        product_category: ProductCategoryIndex,
        price_formula: ProductPriceFormula,
    }
    index = ProductIndex;
    vec = ProductVec;
    data = ProductData;
}

impl<'data> Query<'data, ProductIndex> {
    pub fn producing_recipes(&self) -> impl Iterator<Item = Query<'data, RecipeIndex>> {
        let product_index = self.target;
        self.data.recipes().filter(move |&value| {
            value
                .outputs()
                .any(|entry| entry.is_output() && entry.product_index == product_index)
        })
    }
}

impl_soa! {
    Recipe {
        name: String,
        entries: Vec<RecipeEntry>,
        days: i64,
        required_modules: Vec<ModuleIndex>,
    }
    index = RecipeIndex;
    vec = RecipeVec;
    data = RecipeData;
}

impl<'data> Query<'data, RecipeIndex> {
    pub fn inputs(&self) -> impl Iterator<Item = Query<'data, RecipeEntry>> {
        self.entries().filter(|entry| entry.is_input())
    }

    pub fn outputs(&self) -> impl Iterator<Item = Query<'data, RecipeEntry>> {
        self.entries().filter(|entry| entry.is_output())
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

impl_soa! {
    Building {
        name: String,
        base_cost: i64,
        available_recipes: Vec<RecipeIndex>,
    }
    index = BuildingIndex;
    vec = BuildingVec;
    data = BuildingData;
}

impl<'data> Query<'data, BuildingIndex> {
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

impl_soa!(
    Module {
        name: String,
        base_cost: i64,
    }
    index = ModuleIndex;
    vec = ModuleVec;
    data = ModuleData;
);

impl_soa! {
    ProductCategory {
        name: String,
        price_modifier: f64,
        growth_modifier: f64,
    }
    index = ProductCategoryIndex;
    vec = ProductCategoryVec;
    data = ProductCategoryData;
}

macro_rules! impl_soa_group {
    ($T:ident { $($f:ident ($fp:ident): $t:ty[$X:ty]),* $(,)? }) => {
        pub struct $T {
            $( pub $f: $t, )*
        }

        $(
            impl AsRef<$t> for $T {
                fn as_ref(&self) -> &$t {
                    &self.$f
                }
            }
        )*

        impl $T {
            $(
                pub fn $fp(&self) -> impl Iterator<Item = Query<'_, $X>> {
                    self.$f.indices().map(|index| self.query(index))
                }
            )*
        }
    };
}

impl_soa_group! {
    GameData {
        product (products): ProductData[ProductIndex],
        recipe (recipes): RecipeData[RecipeIndex],
        module (modules): ModuleData[ModuleIndex],
        building (buildings): BuildingData[BuildingIndex],
        product_category (product_categories): ProductCategoryData[ProductCategoryIndex],
    }
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
        //                 product_category: value.
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
