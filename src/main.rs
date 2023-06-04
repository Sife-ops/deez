use macro_deez_derive::Sugon;

trait MyTrait {
    fn answer() -> i32 {
        42
    }
}

#[derive(Sugon)]
struct Foo;

fn main() {
    println!("Hello, world!");
    println!("{}", Foo::answer())
}
