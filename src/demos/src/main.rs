extern crate ApeRust;
use ApeRust::vector::vector;

fn main() 
{
    let mut v:vector = vector::new(2.0,4.0);
    let mut v2:vector = vector::new(0.0,0.0);
    let mut v3:vector = vector::new(0.0,0.0);
    let mut v4:vector = vector::new(0.0,0.0);
    let mut v5:vector = vector::new(0.0,0.0);
    v2.copy(&v);
    v2.plusEquals(&v);
    v3.copy(&v2);
    v4.copy(&v2);
    v2.minusEquals(&v);
    v4 = v4.mult(5.0);
    v4 = v4.mult(6.0);
    v5 = v4.times(&v4);
    v5.divEquals(3.0);
    let c = v5.magnitude();
    let t = v2.distance(&v3);
    v5 = v5.normalize();
    v5 = v5.rotate(&0.52);
    println!("Hello, World! {:?}, {:?}, {}, {}, {:?}, {:?}, {:?}, {:?}", c, t, v2.cross(&v), v2.dot(&v), v2.plus(&v), v2.minus(&v), v3, v5);

}