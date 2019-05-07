extern crate ApeRust;
use ApeRust::vector::vector;
use ApeRust::APEngine::APEngine;

fn main() 
{
    let mut v:vector = vector::new(2.0,4.0);
    let mut v2:vector = vector::new(3.0,4.0);
    let mut v3:vector = vector::new(5.0,6.0);
    let mut v4:vector = vector::new(7.0,8.0);
    let mut v5:vector = vector::new(9.0,10.0);
    let mut ap:APEngine = APEngine::new();

    ap.init(0.25);
    
    println!("v = {:?}, v2 = {:?}, v3 = {:?} , v4 = {:?} , v5 = {:?}  ", v, v2, v3, v4, v5);
    v.set_to(4.0, 5.0);
    println!("v set to 4, 5 = {:?} ", v);
    v2.copy(&v);
    println!("v2 copy v = {:?}", v2);
    v2.plusEquals(&v);
    println!("v2 plus Equals v {:?}", v2);
    v2.minusEquals(&v);
    println!("v2 minus Equals v {:?}", v2);
    v4 = v4.mult(5.0);
    println!("v4 = v4 mult 5.0 {:?}", v4);
    v5 = v4.times(&v4);
    println!("v5 = v4 times v4 {:?}", v5);
    v5.divEquals(3.0);
    println!("v5 divEquals 3.0 {:?}", v5);
    let c = v5.magnitude();
    println!("c equals magnitude of v5 {}", c);
    let t = v2.distance(&v3);
    println!("t distance v2, v3 {}", t);
    v5 = v5.normalize();
    v5 = v5.rotate(&0.52);
    println!("Hello, World! {:?}, {:?}, {}, {}, {:?}, {:?}, {:?}, {:?}", c, t, v2.cross(&v), v2.dot(&v), v2.plus(&v), v2.minus(&v), v3, v5);
  

}