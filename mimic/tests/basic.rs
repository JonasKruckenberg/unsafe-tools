use std::hint::black_box;
use unsafe_tools_mimic::impl_mimic;

#[test]
fn basic() {
    #[derive(Clone)]
    #[allow(unused)]
    pub struct Foo {
        a: u64,
    }
    impl_mimic!(Foo);

    let ty = Foo { a: 42 };
    let mimic = ty.clone().into_mimic();

    assert_eq!(size_of_val(&ty), size_of_val(&mimic));
    assert_eq!(align_of_val(&ty), align_of_val(&mimic));
}

#[test]
fn conversion() {
    #[derive(Debug, PartialEq)]
    pub struct Foo {
        a: u64,
    }
    impl_mimic!(Foo);

    let a = Foo { a: 42 };
    let b = Foo { a: 42 };

    let b = unsafe { Foo::from_mimic(black_box(b.into_mimic())) };

    assert_eq!(a, b);
    assert_eq!(a, b);
}

#[test]
fn with_lifetime() {
    #[derive(Debug, PartialEq)]
    pub struct Foo<'a> {
        a: &'a u64,
    }

    impl_mimic!(Foo<'static>);

    static A_VAL: u64 = 42;
    static B_VAL: u64 = 42;

    let a = Foo { a: &A_VAL };
    let b = Foo { a: &B_VAL };

    let b = unsafe { Foo::from_mimic(black_box(b.into_mimic())) };

    assert_eq!(a, b);
}

#[test]
fn with_generics() {
    #[derive(Debug, PartialEq)]
    pub struct Foo<T, U> {
        a: u64,
        b: T,
        c: U,
    }
    impl_mimic!(Foo<u8, String>);

    let a = Foo {
        a: 42,
        b: 8,
        c: "hello_world".to_string(),
    };
    let b = Foo {
        a: 42,
        b: 8,
        c: "hello_world".to_string(),
    };

    let b = unsafe { Foo::from_mimic(black_box(b.into_mimic())) };

    assert_eq!(a, b);
}
