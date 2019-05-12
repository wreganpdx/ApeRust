extern crate ApeRust;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use ApeRust::vector::vector;
use ApeRust::APEngine::APEngine;
use ApeRust::polygon_particle::polygon_particle;
use std::{thread, time};
use std::time::{Duration, Instant};
use ApeRust::particle::particle;
use ApeRust::particle_collection::particle_collection;
use ApeRust::APEngine::Paint;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

fn main() 
{

    
    let opengl = OpenGL::V3_2;
    
    let mut window: Window = WindowSettings::new(
            "testcd.",
            [850, 850]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl:GlGraphics = GlGraphics::new(opengl);
    let mut v:vector = vector::new(2.0,4.0);
    let mut v2:vector = vector::new(3.0,4.0);
    let v3:vector = vector::new(5.0,6.0);
    let mut v4:vector = vector::new(7.0,8.0);
    let mut v5:vector = vector::new(9.0,10.0);
    let mut ap:APEngine = APEngine::new();

    ap.init(0.01);

    //boundries
    let mut left: polygon_particle = polygon_particle::new();
    let mut right: polygon_particle = polygon_particle::new();
    let mut top: polygon_particle = polygon_particle::new();
    let mut bottom: polygon_particle = polygon_particle::new();

    left.create_vertices_from_rect(5.0,800.0);

    right.create_vertices_from_rect(5.0,800.0);
    top.create_vertices_from_rect(790.0,5.0);
    bottom.create_vertices_from_rect(800.0,5.0);

    left.set_position(&vector::new(5.0, 400.0));

    right.set_position(&vector::new(797.5, 405.0));
    top.set_position(&vector::new(400.0, 802.5));
    bottom.set_position(&vector::new(400.0, 2.5));

    top.set_collidable(true);
    bottom.set_collidable(true);
    left.set_collidable(true);
    right.set_collidable(true);

    top.set_fixed(true);
    bottom.set_fixed(true);
    left.set_fixed(true);
    right.set_fixed(true);


    //objects
    let mut p: polygon_particle = polygon_particle::new();
    let mut p2: polygon_particle = polygon_particle::new();

    //p.set_radian(0.2);

    p.create_vertices_from_rect(40.0,5.0);
    p2.create_vertices_from_rect(40.0,5.0);

    
  //  p.set_radian(0.2);

    p.set_collidable(true);
    p2.set_collidable(true);

    p.set_elasticity(1.0);
    p2.set_elasticity(1.0);

    p.set_position(&vector::new(200.0,415.0));
    p2.set_position(&vector::new(600.0,415.0));

    p.set_velocity(&vector::new(1.0,0.0));
    p2.set_velocity(&vector::new(-1.0,0.000));


    let mut list:particle_collection = particle_collection::new();

    list.add_poly_particle(p);
    list.add_poly_particle(p2);
    list.add_poly_particle(left);
    list.add_poly_particle(right);
    list.add_poly_particle(top);
    list.add_poly_particle(bottom);

    list.set_collide_internal(true);

    ap.add_particle_collection(list);
    /*
    println!("v = {:?}, v2 = {:?}, v3 = {:?} , v4 = {:?} , v5 = {:?}  ", v, v2, v3, v4, v5);
    v.set_to(4.0, 5.0);
    println!("v set to 4, 5 = {:?} ", v);
    v2.copy(&v);
    println!("v2 copy v = {:?}", v2);
    v2.plus_equals(&v);
    println!("v2 plus Equals v {:?}", v2);
    v2.minus_equals(&v);
    println!("v2 minus Equals v {:?}", v2);
    v4 = v4.mult(5.0);
    println!("v4 = v4 mult 5.0 {:?}", v4);
    v5 = v4.times(&v4);
    println!("v5 = v4 times v4 {:?}", v5);
    v5.div_equals(3.0);
    println!("v5 divEquals 3.0 {:?}", v5);
    let c = v5.magnitude();
    println!("c equals magnitude of v5 {}", c);
    let t = v2.distance(&v3);
    println!("t distance v2, v3 {}", t);
    v5 = v5.normalize();
    v5 = v5.rotate(&0.52);
    println!("Hello, World! {:?}, {:?}, {}, {}, {:?}, {:?}, {:?}, {:?}", c, t, v2.cross(&v), v2.dot(&v), v2.plus(&v), v2.minus(&v), v3, v5);
*/
    let mut step:bool = false;
    step = ap.step();
    step = ap.step();
    let mut i:i32 = 0;
    let mut events = Events::new(EventSettings::new());
    let now = Instant::now();
    for i in 0..100000
    {
        step = ap.step();
        
        while let Some(e) = events.next(&mut window) 
        {
            if let Some(r) = e.render_args() 
            {
               // print!("Rendering");
                ap.paint(&r, &mut gl); //.render(&r);
                break;
            }
        }
        if !step
        {
           // println!(" p: {:?}", p.get_position());
            //let ten_millis = time::Duration::from_millis(1);
            //thread::sleep(ten_millis);
           // print!("Sleeping: {}", i);
        }
        else
        {
            print!("Step: {} ", i);
        }

        if now.elapsed().as_secs() > 15
        {
            break;
        }
    }
}