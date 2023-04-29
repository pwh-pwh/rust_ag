use std::fmt::{Debug, Display};
use std::marker::PhantomData;

#[derive(Default,Debug,Eq,PartialEq)]
struct Id<T> {
    inner: u64,
    _tag: PhantomData<T>
}

#[derive(Default,Debug,Eq,PartialEq)]
struct User {
    id: Id<Self>
}
#[derive(Default,Debug,Eq,PartialEq)]
struct Person {
    id: Id<Self>
}




fn main() {
    let u = User::default();
    let person = Person::default();
    //assert_eq!(u.id,person.id);
}