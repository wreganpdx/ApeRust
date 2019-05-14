extern crate ApeRust;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use ApeRust::vector::vector;
use ApeRust::APEngine::APEngine;
use ApeRust::polygon_particle::polygon_particle;
use ApeRust::rectangle_particle::rectangle_particle;
use ApeRust::circle_particle::circle_particle;
use std::{thread, time};
use std::time::{Duration, Instant};
use ApeRust::particle::particle;
use ApeRust::particle_collection::particle_collection;
use ApeRust::APEngine::Paint;
//use crate object_helper::create_rectangle; 
mod object_helper;

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
    let mut ap:APEngine = APEngine::new();

    ap.init(0.01);

    //boundries
    let mut left: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut right: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut top: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut bottom: rectangle_particle = rectangle_particle::new(ap.get_new_id());

    left.create_rectangle(5.0,800.0);

    right.create_rectangle(5.0,800.0);
    top.create_rectangle(790.0,5.0);
    bottom.create_rectangle(800.0,5.0);

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
    let mut p: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut p2: rectangle_particle = rectangle_particle::new(ap.get_new_id());

    let mut p_circle: circle_particle = circle_particle::new(ap.get_new_id());

    //p.set_radian(0.2);

    p_circle.init_circle(6.0);

    p_circle.set_position(&vector::new(200.0, 600.0));

    p.create_rectangle(40.0,5.0);
    p2.create_rectangle(40.0,5.0);

    //p.set_radian(0.2);

    p.set_collidable(true);
    p2.set_collidable(true);

    p.set_elasticity(1.0);
    p2.set_elasticity(1.0);

    p.set_position(&vector::new(200.0,415.0));
    p2.set_position(&vector::new(600.0,415.0));

    p.set_velocity(&vector::new(1.0,0.0));
    p2.set_velocity(&vector::new(-1.0,0.000));

    let mut p3 = particle_collection::new();

    p3.init_composite(vector::new(400.0, 415.0));

    object_helper::create_rectangle(&mut p3, 
    (ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id()));

    p3.set_collide_internal(false);

    let mut list:particle_collection = particle_collection::new();

    list.add_rectangle_particle(p);
    list.add_rectangle_particle(p2);
    list.add_rectangle_particle(left);
    list.add_rectangle_particle(right);
   // list.add_rectangle_particle(top);
   // list.add_rectangle_particle(bottom);

    list.add_circle_particle(p_circle);

    list.set_collide_internal(true);

    ap.add_particle_collection(list);
   // ap.add_particle_collection(p3);

   //ap.set_force(vector::new(0.0,20.0));
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
    let mut exit = false;
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

            if let Some(r) = e.close_args()
            {
               exit = true;
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


        if now.elapsed().as_secs() > 15 || exit
        {
            break;
        }
    }
}