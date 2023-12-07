use std::ops::Index;

use index_types::IndexU16;
use typed_index::{
    impl_typed_index, TypedIndexBoxedSlice, TypedIndexSlice, TypedIndexSliceMut, TypedIndexVec,
};

impl_typed_index!(pub struct ProductIndex(IndexU16));

type ProductVec<T> = TypedIndexVec<ProductIndex, T>;
type ProductBoxedSlice<T> = TypedIndexBoxedSlice<ProductIndex, T>;
type ProductSlice<'a, T> = TypedIndexSlice<'a, ProductIndex, T>;
type ProductSliceMut<'a, T> = TypedIndexSliceMut<'a, ProductIndex, T>;

#[test]
fn into_iter_late_index() {
    let mut iter = ProductVec::new(vec![(), ()]).into_iter();
    assert_eq!(iter.next(), Some(()));
    let mut iter = iter.index();
    assert_eq!(iter.next(), Some((ProductIndex(1.into()), ())));
    assert_eq!(iter.next(), None);
}

#[test]
fn into_iter_next_back() {
    let mut iter = ProductVec::new(vec!["a", "b", "c"]).into_iter().index();
    assert_eq!(iter.next(), Some((ProductIndex(0.into()), "a")));
    assert_eq!(iter.next_back(), Some((ProductIndex(2.into()), "c")));
    assert_eq!(iter.next(), Some((ProductIndex(1.into()), "b")));
}

#[test]
fn iter_map_and_everything() {
    struct Product {
        name: String,
        price: f64,
    }

    #[derive(Default)]
    struct ProductData {
        names: ProductBoxedSlice<String>,
        prices: ProductBoxedSlice<f64>,
    }

    struct ProductIndexIter(std::ops::Range<usize>);

    impl Iterator for ProductIndexIter {
        type Item = ProductIndex;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(ProductIndex::from)
        }
    }

    impl<'data> IntoIterator for &'data ProductData {
        type Item = ProductIndex;

        type IntoIter = ProductIndexIter;

        fn into_iter(self) -> Self::IntoIter {
            ProductIndexIter(0..self.names.len())
        }
    }

    impl ProductData {
        fn proxy_mut(&mut self, index: ProductIndex) -> ProductProxyMut<'_, '_> {
            ProductProxyMut {
                names: self.names.as_mut_slice(),
                prices: self.prices.as_mut_slice(),
                index,
            }
        }
    }

    struct ProductProxyMut<'name, 'price> {
        names: ProductSliceMut<'name, String>,
        prices: ProductSliceMut<'price, f64>,
        index: ProductIndex,
    }

    impl<'name, 'price> ProductProxyMut<'name, 'price> {
        fn index(&self) -> ProductIndex {
            self.index
        }

        fn name<'a: 'name>(&'a self) -> &'name String {
            &self.names[self.index]
        }

        fn name_mut<'a: 'name>(&'a mut self) -> &'name String {
            &mut self.names[self.index]
        }

        fn price<'a: 'price>(&'a self) -> &'price f64 {
            &self.prices[self.index]
        }

        fn price_mut<'a: 'price>(&'a mut self) -> &'price mut f64 {
            &mut self.prices[self.index]
        }
    }

    let products = ProductVec::new(vec![
        Product {
            name: "Orange".to_string(),
            price: 2.3,
        },
        Product {
            name: "Apple".to_string(),
            price: 5.1,
        },
    ]);

    let mut products = {
        let (names, prices) = products.into_iter().fold(
            <(ProductVec<String>, ProductVec<f64>)>::default(),
            |(mut names, mut prices), product| {
                names.push(product.name);
                prices.push(product.price);
                (names, prices)
            },
        );
        ProductData {
            names: names.into_boxed_slice(),
            prices: prices.into_boxed_slice(),
        }
    };

    for index in &products {
        let ProductData {
            ref mut prices,
            ref names,
        } = products;
        prices[index] = prices[index] * 2.0 + names[index].len() as f64;
    }

    for index in &products {
        let mut product = products.proxy_mut(index);
        *product.price_mut() = *product.price() * 2.0 + product.name().len() as f64;
    }
}
