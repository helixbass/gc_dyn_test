use gc::{Finalize, Gc, GcCell, Trace};
use std::mem;

trait Trait: Trace + Finalize {
    fn print(&self);
}

#[derive(Trace, Finalize)]
struct Foo {
    a: String,
    _dyn_wrapper: GcCell<Option<Gc<Box<dyn Trait>>>>,
}

impl Foo {
    pub fn new() -> Gc<Box<Self>> {
        let dyn_wrapper: Gc<Box<dyn Trait>> = Gc::new(Box::new(Self {
            a: "Hello".to_owned(),
            _dyn_wrapper: Default::default(),
        }));
        let downcasted: Gc<Box<Foo>> = unsafe { mem::transmute(dyn_wrapper.clone()) };
        *downcasted._dyn_wrapper.borrow_mut() = Some(dyn_wrapper);
        downcasted
    }

    pub fn as_dyn_trait(&self) -> Gc<Box<dyn Trait>> {
        self._dyn_wrapper.borrow().clone().unwrap()
    }

    pub fn print_from_foo(&self) {
        println!("{}", self.a);
    }
}

impl Trait for Foo {
    fn print(&self) {
        println!("printing from Foo");
    }
}

fn main() {
    let foo = Foo::new();
    foo.as_dyn_trait().print();
    foo.print_from_foo();
}
