#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::vec_init_then_push)]

fn main() {
    let a = SomeStruct::new();
    let b = OtherStruct::new();

    a.some_function();
    b.some_function();

    some_object_function(a);

    let c = SomeStruct::new();

    let b = b.object();
    let c = c.object();

    let mut vec = Vec::new();

    vec.push(b);
    vec.push(c);

    for val in &vec {
        val.some_function();
    }
}

pub trait SomeTrait {
    // trait which requires "new" to be implemented,
    // "new" takes nothing and returns a new instance for the implemented structure.
    fn new() -> Self
    // "SomeTrait" allows to for object traits to be created by adding the "Self: Size" boundary
    where
        Self: Sized;
}

pub trait SomeObject: SomeTrait {
    // an object trait, which implements the trait "SomeTrait".
    // object traits can be used the same as structs.
    // object trait methods must all take "self" as an argument to remain as object trait.
    // reminder: "self" is the structure we implement upon, so the "object" function transforms to become the object trait.
    fn object(self) -> Box<dyn SomeObject>;
    fn some_function(&self) {}
}

pub struct SomeStruct;
// a struct "SomeStruct", which holds nothing

impl SomeObject for SomeStruct {
    // implementation of "SomeObject" for "SomeStruct"
    fn object(self) -> Box<dyn SomeObject> {
        Box::new(SomeStruct)
    }
    fn some_function(&self) {
        println!("Some Struct")
    }
}

impl SomeTrait for SomeStruct {
    // "SomeTrait" needs to be implemented for "SomeStruct" as "SomeObject" requires it
    fn new() -> Self {
        SomeStruct
    }
}

pub fn some_object_function(object: impl SomeObject) {
    // function which takes the object trait "SomeObject" (static dispatched)
    drop(object); // _ = object;
}

pub struct OtherStruct;

impl SomeObject for OtherStruct {
    fn object(self) -> Box<dyn SomeObject> {
        Box::new(OtherStruct)
    }
    fn some_function(&self) {
        println!("Other Object");
    }
}

impl SomeTrait for OtherStruct {
    fn new() -> Self {
        OtherStruct
    }
}
