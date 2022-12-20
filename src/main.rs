use gc::{Finalize, Gc, Trace};

trait Trait: Trace + Finalize {
    fn print(&self);
}

#[derive(Trace, Finalize)]
struct Foo;

impl Trait for Foo {
    fn print(&self) {
        println!("printing from Foo");
    }
}

fn main() {
    let dyn_trait: Gc<Box<dyn Trait>> = Gc::new(Box::new(Foo));
    dyn_trait.print();
}
