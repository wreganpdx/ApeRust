extern crate ApeRust;
use ApeRust::vector::vector;

fn main() 
{
    let mut v:vector = vector::new(2.0,4.0);
    println!("Hello, World! {:?}", v);
}