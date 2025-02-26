#![allow(clippy::vec_init_then_push)]

fn main() {
    let a = SomeStruct::new();

    let b = a.object();

    some_object(b);

    let c = SomeStruct::new().object();
    let mut vec = Vec::new();
    vec.push(c);

    let d = OtherStruct::new();
    let e = d.object();

    vec.push(e);
    // error here, "opaque type" problem
}

pub trait SomeTrait {
    // trait which requires "new" to be implemented,
    // "new" takes nothing and returns a new instance for the implemented structure.
    fn new() -> Self;
}

pub trait SomeObject: SomeTrait {
    // an object trait, which implements the trait "SomeTrait".
    // object traits can be used the same as structs.
    // object trait methods must all take "self" as an argument to remain as object trait.
    // reminder: "self" is the structure we implement upon, so the "object" function transforms to become the object trait.
    fn object(self) -> impl SomeObject;
}

pub struct SomeStruct;
// a struct "SomeStruct", which holds nothing

impl SomeObject for SomeStruct {
    // implementation of "SomeObject" for "SomeStruct"
    fn object(self) -> impl SomeObject {
        SomeStruct
    }
}

impl SomeTrait for SomeStruct {
    // "SomeTrait" needs to be implemented for "SomeStruct" as "SomeObject" requires it
    fn new() -> Self {
        SomeStruct
    }
}

pub fn some_object(object: impl SomeObject) {
    // function which takes the object trait "SomeObject" (static dispatched)
    drop(object); // _ = object;
}

pub struct OtherStruct;

impl SomeObject for OtherStruct {
    fn object(self) -> impl SomeObject {
        OtherStruct
    }
}

impl SomeTrait for OtherStruct {
    fn new() -> Self {
        OtherStruct
    }
}
