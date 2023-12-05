use typed_index::{
    impl_typed_index, IndexU16, IndexableIterator, TypedIndexBoxedSlice, TypedIndexVec,
};

impl_typed_index!(pub struct ProductIndex(IndexU16));

type ProductVec<T> = TypedIndexVec<ProductIndex, T>;

type ProductBoxedSlice<T> = TypedIndexBoxedSlice<ProductIndex, T>;

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
    let mut iter = TypedIndexVec::new(vec!["a", "b", "c"]).into_iter().index();
    assert_eq!(iter.next(), Some((0, "a")));
    assert_eq!(iter.next_back(), Some((2, "c")));
    assert_eq!(iter.next(), Some((1, "b")));
}

#[test]
fn iter_map_and_everything() {
    struct Product {
        name: String,
        price: f64,
    }

    #[derive(Default)]
    struct Products {
        names: ProductBoxedSlice<String>,
        prices: ProductBoxedSlice<f64>,
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

    let products = {
        let (names, prices) = products.into_iter().fold(
            <(ProductVec<String>, ProductVec<f64>)>::default(),
            |(mut names, mut prices), product| {
                names.push(product.name);
                prices.push(product.price);
                (names, prices)
            },
        );
        Products {
            names: names.into_boxed_slice(),
            prices: prices.into_boxed_slice(),
        }
    };
}
