use std::collections::HashMap;

use serialization::ProductPriceFormula;

mod fields {
/*

entity Product
    attribute ProductName: String
    attribute ProductPrice: f64

entity Recipe index RecipeIndex
    Recipe attribute RecipeName: String
    Recipe attribute RecipeEntries: Vec<RecipeEntry>

trait Entity {
    type Index
}

trait Attribute {
    type Entity: Entity;
    type Value;
}

arguments(Owned, Ref, Mut)

group Data
    Product
    Recipe
    ProductName
    RecipeEntries


*/
    use std::ops::Index;

    use typed_index_collections::TypedIndexVec;

    pub trait Field {
        type Index;
        type Type;
    }

    pub trait FieldIndex<F>: Index<F, Output = FieldVec<F>>
    where
        F: Field,
    {
    }

    impl<G, F> FieldIndex<F> for G
    where
        G: Index<F, Output = FieldVec<F>>,
        F: Field,
    {
    }

    pub type FieldVec<F> = TypedIndexVec<<F as Field>::Index, <F as Field>::Type>;

    macro_rules! index {
        ($X:ident) => {
            typed_index_collections::impl_typed_index!(pub struct $X(index_types::IndexU32));
        }
    }

    macro_rules! field {
        ($F:ident[$X:ty]: $T:ty) => {
            struct $F;

            impl Field for $F {
                type Index = $X;
                type Type = $T;
            }

            impl Index<$F> for FieldVec<$F> {
                type Output = Self;

                fn index(&self, _: $F) -> &Self::Output {
                    self
                }
            }
        };
    }

    // macro_rules! group {
    //     ($D:ident { $($f:ident: $F:ty),* $(,)? }) => {
    //         struct $D {
    //             $(
    //                 $f: FieldVec<$F>,
    //             )*
    //         }

    //         $(
    //             impl Index<$F> for $D {
    //                 type Output = FieldVec<$F>;

    //                 fn index(&self, _: $F) -> &Self::Output {
    //                     &self.$f
    //                 }
    //             }
    //         )*
    //     };
    // }

    macro_rules! group {
        (@stop $D:ident
            ($($f:tt: $F:ty,)*)
        ) => {
            struct $D (
                $(
                    FieldVec<$F>,
                )*
            );

            $(
                impl Index<$F> for $D {
                    type Output = FieldVec<$F>;

                    fn index(&self, _: $F) -> &Self::Output {
                        &self.$f
                    }
                }
            )*
        };
        (@zip $D:ident
            ($($fields:tt)*)
            ($($is:tt)*)
            ()
        ) => {
            group!(@stop $D ($($fields)*));
        };
        (@zip $D:ident
            ($($fields:tt)*)
            ($i:tt, $($is:tt)*)
            ($F:ty, $($Fs:tt)*)
        ) => {
            group!(@zip $D
                ($($fields)* $i: $F,)
                ($($is)*)
                ($($Fs)*)
            );
        };
        ($D:ident($($Fs:ty),* $(,)?)) => {
            group!(@zip $D
                ()
                (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,)
                ($($Fs,)*)
            );
        };
    }

    index!(ProductIndex);
    field!(ProductName[ProductIndex]: String);

    pub struct RecipeEntry {
        pub product: ProductIndex,
        pub amount: i64,
    }

    index!(RecipeIndex);
    field!(RecipeName[RecipeIndex]: String);
    field!(RecipeEntries[RecipeIndex]: Vec<RecipeEntry>);
    field!(RecipePrice[RecipeIndex]: f64);

    group!(Data (
        ProductName,
        RecipeName,
        RecipeEntries,
    ));

    #[test]
    fn test() {
        fn recipe_prices(data: &(impl FieldIndex<RecipeEntries> + FieldIndex<ProductName>)) -> FieldVec<RecipePrice>
        {
            data[RecipeEntries]
                .iter()
                .map(|entries| {
                    entries.len() as f64
                        + entries
                            .iter()
                            .map(|entry| data[ProductName][entry.product].len())
                            .sum::<usize>() as f64
                })
                .collect()
        }

        let data = Data(
            vec!["Product 1".to_string(), "Product 2".to_string()]
                .into_iter()
                .collect(),
            vec!["Recipe 1".to_string(), "Recipe 2".to_string()]
                .into_iter()
                .collect(),
            vec![
                vec![RecipeEntry {
                    product: ProductIndex(0.into()),
                    amount: 1,
                }],
                vec![],
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(&data[ProductName][ProductIndex(0.into())], "Product 1");

        group!(PriceInputData(ProductName, RecipeEntries));

        recipe_prices(&data);

        let data = PriceInputData(data.0, data.2);
        recipe_prices(&data);
    }
}

pub mod serialization;

pub trait Field<T> {
    type Borrow;

    fn borrow(self, value: T) -> Self::Borrow;
}

macro_rules! impl_field_ {
    (@item) => {};
    (@item query copy $T:ty; $($tail:tt)*) => {
        impl<'data> Field<&'data $T> for &'data GameData {
            type Borrow = Query<'data, $T>;

            fn borrow(self, value: &'data $T) -> Self::Borrow {
                self.query(*value)
            }
        }
        impl_field_!(@item $($tail)*);
    };
    (@item copy $T:ty; $($tail:tt)*) => {
        impl<'data> Field<&'data $T> for &'data GameData {
            type Borrow = $T;

            fn borrow(self, value: &'data $T) -> Self::Borrow {
                *value
            }
        }
        impl_field_!(@item $($tail)*);
    };
    (@item deref $T:ty; $($tail:tt)*) => {
        impl<'data> Field<&'data $T> for &'data GameData {
            type Borrow = &'data <$T as std::ops::Deref>::Target;

            fn borrow(self, value: &'data $T) -> Self::Borrow {
                value
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

impl<'data, T> Field<&'data Vec<T>> for &'data GameData {
    type Borrow = Query<'data, std::slice::Iter<'data, T>>;

    fn borrow(self, value: &'data Vec<T>) -> Self::Borrow {
        self.query(value.iter())
    }
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
        }

        impl<'data> Query<'data, $X> {
            $(
                pub fn $f(&self) -> <&'data GameData as Field<&'data $t>>::Borrow {
                    let data: &$D = self.data.as_ref();
                    self.data.borrow(&data.$f[self.target])
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
        typed_index_collections::impl_typed_index!(pub struct $X(index_types::IndexU32));

        impl_field! {
            query copy $X;
        }

        pub type $V<T> = typed_index_collections::TypedIndexVec<$X, T>;

        impl $D {
            pub fn indices(&self) -> impl Iterator<Item = $X> + '_ {
                (0..self.$f0.len()).map(|index| $X(index.into()))
            }

            pub fn push(&mut self, value: $T) -> $X {
                let index = self.$f0.len();
                self.$f0.push(value.$f0);
                $(self.$f.push(value.$f);)*
                $X(index.into())
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

impl From<serialization::ProductCategory> for ProductCategory {
    fn from(value: serialization::ProductCategory) -> Self {
        Self {
            name: value.name,
            price_modifier: value.price_modifier,
            growth_modifier: value.growth_modifier,
        }
    }
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

        fn sort_split_guids<T, X>(
            data: impl IntoIterator<Item = (String, T)>,
            sort: impl Fn(&T, &T) -> std::cmp::Ordering,
            x: impl Fn(usize) -> X,
        ) -> (HashMap<String, X>, impl Iterator<Item = T>) {
            let mut pairs = data
                .into_iter()
                .map(|(guid, value)| (Some(guid), value))
                .collect::<Vec<_>>();
            pairs.sort_unstable_by(|(_, a), (_, b)| sort(a, b));
            let guid_to_index = pairs
                .iter_mut()
                .enumerate()
                .map(|(index, (guid, _))| (guid.take().unwrap(), x(index)))
                .collect::<HashMap<_, _>>();
            (guid_to_index, pairs.into_iter().map(|(_, value)| value))
        }

        let (guid_to_product_category, product_categories) = sort_split_guids(
            data.product_categories,
            |a, b| a.name.cmp(&b.name),
            |index| ProductCategoryIndex(index.into()),
        );

        let (guid_to_product, products) = sort_split_guids(
            data.products,
            |a, b| a.name.cmp(&b.name),
            |index| ProductIndex(index.into()),
        );

        let (guid_to_module, modules) = sort_split_guids(
            data.modules,
            |a, b| a.name.cmp(&b.name),
            |index| ModuleIndex(index.into()),
        );

        let (guid_to_recipe, recipes) = sort_split_guids(
            data.recipes,
            |a, b| a.name.cmp(&b.name),
            |index| RecipeIndex(index.into()),
        );

        let (_guid_to_building, buildings) = sort_split_guids(
            data.buildings,
            |a, b| a.name.cmp(&b.name),
            |index| BuildingIndex(index.into()),
        );

        Ok(Self {
            product: products
                .map(|value| Product {
                    name: value.name,
                    product_category: guid_to_product_category[&value.category],
                    price_formula: value.price_formula,
                })
                .collect(),
            module: modules
                .map(|value| Module {
                    name: value.name,
                    base_cost: value.base_cost,
                })
                .collect(),
            recipe: recipes
                .map(|value| Recipe {
                    name: value.name,
                    entries: value
                        .entries
                        .into_iter()
                        .map(|value| RecipeEntry {
                            product_index: guid_to_product[&value.product_id],
                            amount: value.amount,
                        })
                        .collect(),
                    days: value.days,
                    required_modules: value
                        .required_modules
                        .into_iter()
                        .map(|value| guid_to_module[&value])
                        .collect(),
                })
                .collect(),
            building: buildings
                .map(|value| Building {
                    name: value.name,
                    base_cost: value.base_cost,
                    available_recipes: value
                        .available_recipes
                        .into_iter()
                        .map(|value| guid_to_recipe[&value])
                        .collect(),
                })
                .collect(),
            product_category: product_categories
                .map(|value| ProductCategory {
                    name: value.name,
                    price_modifier: value.price_modifier,
                    growth_modifier: value.growth_modifier,
                })
                .collect(),
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
