extern crate rustl;

use rustl::foo;

fn main() {
    let some_struct = rustl::display::AStructInYoRust{zeroth: 0};
    println!("{}", some_struct);
    foo::foo();
    rustl::lol(10);
}
