macro_rules! multi_index {
    (@comma ($(,)?)
        ($data:ident, $index:expr)
        ($(($($bind:tt)*) ($($take:tt)*) ($field:ident))*)
    ) => {
        let ($($($bind)* $field),*) = {
            let index = $index;
            ($($($take)* ($data).$field[index]),*)
        };
    };

    (@comma (, $($tail:tt)+)
        ($($final:tt)*)
        ($($acc:tt)*)
    ) => {
        $crate::multi_index!(@item_bind ($($tail)*)
            ($($final)*)
            ($($acc)*)
        );
    };

    (@item_bind (mut $($tail:tt)*)
        ($($final:tt)*)
        ($($acc:tt)*)
    ) => {
        $crate::multi_index!(@item_rest ($($tail)*)
            ($($final)*)
            ($($acc)* (mut))
        );
    };

    (@item_bind ($($tail:tt)*)
        ($($final:tt)*)
        ($($acc:tt)*)
    ) => {
        $crate::multi_index!(@item_rest ($($tail)*)
            ($($final)*)
            ($($acc)* ())
        );
    };

    (@item_rest (&mut $field:ident $($tail:tt)*)
        ($($final:tt)*)
        ($($acc:tt)*)
    ) => {
        $crate::multi_index!(@comma ($($tail)*)
            ($($final)*)
            ($($acc)* (&mut) ($field))
        );
    };

    (@item_rest (& $field:ident $($tail:tt)*)
        ($($final:tt)*)
        ($($acc:tt)*)
    ) => {
        $crate::multi_index!(@comma ($($tail)*)
            ($($final)*)
            ($($acc)* (&) ($field))
        );
    };

    (@item_rest ($field:ident $($tail:tt)*)
        ($($final:tt)*)
        ($($acc:tt)*)
    ) => {
        $crate::multi_index!(@comma ($($tail)*)
            ($($final)*)
            ($($acc)* () ($field))
        );
    };

    (let { $($body:tt)+ } = ($data:expr)[$index:expr]) => {
        $crate::multi_index!(@item_bind ($($body)*)
            ($data, $index)
            ()
        );
    };

    (let { $($body:tt)+ } = $data:ident[$index:expr]) => {
        $crate::multi_index!(@item_bind ($($body)*)
            ($data, $index)
            ()
        );
    };
}

pub(crate) use multi_index;

#[test]
fn test() {
    struct Data {
        a: Vec<usize>,
        b: Vec<usize>,
        c: Vec<usize>,
        d: Vec<usize>,
    }

    let mut data = Data {
        a: (0..3).collect(),
        b: (0..3).collect(),
        c: (0..3).collect(),
        d: (0..3).collect(),
    };

    // let mut container = ((), data);

    multi_index!(let { a, mut b } = data[1]);
    assert_eq!((a, b), (data.a[1], data.b[1]));
    b = 1;
    _ = b;

    multi_index!(let { &a, mut &b } = data[1]);
    assert_eq!((a, b), (&data.a[1], &data.b[1]));
    b = &1;
    _ = b;

    multi_index!(let { &mut a, mut &mut b } = data[1]);
    assert_eq!((a as *mut _, b as *mut _), (&mut data.a[1] as *mut _, &mut data.b[1] as *mut _));
    b = &mut data.b[1];
    _ = b;
}
