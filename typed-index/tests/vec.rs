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

macro_rules! soa {
    (struct $T:ident { $($f:ident: $t:ty),* $(,)? } with index $I:ident and soa $D:ident) => {
        struct $T {
            $($f: $t),*
        }

        struct $D {
            $($f: TypedIndexVec<$I, $t>),*
        }
    };
}

/*
let (a, b, c) = (d.a[i], &d.b[i], &mut d.c[i])
*/

// f: i           => i, d.f[i]
// f: ref i       => f, &d.f[i]
// f: ref mut i   => f, &mut d.f[i]
// f              =>
// ref f          =>
// ref mut f      =>

// macro_rules! soa_let {
//     ({ $($body:tt)* } = $d:ident.*[$i:expr]) => {
//         let ($($f),*) = {
//             let i = $i;
//             ($($d.$f[i]),*)
//         };
//     }
// }

#[test]
fn iter_map_and_everything() {
    soa! {
        struct Product {
            name: String,
            price: f64,
        }
        with index ProductIndex
        and soa ProductData
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
            ProductIndexIter(0..self.name.len())
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
            name: names,
            price: prices,
        }
    };

    for index in &products {
        // let ProductData {
        //     price: ref original_price,
        //     ..
        // } = products;

        // soa_let!({
        //     price,
        //     name
        // } = products.*[index]);

        let (original_price, price, name) = (
            products.price[index],
            &mut products.price[index],
            &products.name[index],
        );

        *price = original_price * 2.0 + name.len() as f64;
    }
}
