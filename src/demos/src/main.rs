extern crate ape_rust;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use ape_rust::vector::Vector;
use ape_rust::ap_engine::ApEngine;
//use ape_rust::polygon_particle::PolygonParticle;
use ape_rust::rectangle_particle::RectangleParticle;
//use ape_rust::circle_particle::CircleParticle;
//use std::{thread, time};
use std::time::Instant;
//use std::time::Duration;
use ape_rust::particle::Particle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::ap_engine::Paint;
//use crate object_helper::create_rectangle; 
mod object_helper;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
//use std::f64::consts::PI;

//use std::f64;

fn main() 
{

    
    let opengl = OpenGL::V3_2;
    
    let mut window: Window = WindowSettings::new(
            "testcd.",
            [800, 800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl:GlGraphics = GlGraphics::new(opengl);
    let mut ap:ApEngine = ApEngine::new();

    ap.init(0.001);

    //boundries
    let mut left: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut right: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut top: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut bottom: RectangleParticle = RectangleParticle::new(ap.get_new_id());

    left.create_rectangle(5.0,790.0);
    right.create_rectangle(5.0,790.0);
    top.create_rectangle(795.0,5.0);
    bottom.create_rectangle(795.0,5.0);

    left.set_position(&Vector::new(5.0, 400.0));
    right.set_position(&Vector::new(795.0, 400.0));
    top.set_position(&Vector::new(400.0, 795.0));
    bottom.set_position(&Vector::new(400.0, 5.0));

    left.set_collidable(true);
    right.set_collidable(true);
    top.set_collidable(true);
    bottom.set_collidable(true);

    left.set_fixed(true);
    right.set_fixed(true);
    top.set_fixed(true);
    bottom.set_fixed(true);

    //objects
    let mut p: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut p2: RectangleParticle = RectangleParticle::new(ap.get_new_id());

    p.create_rectangle(40.0,40.0);
    p2.create_rectangle(40.0,15.0);

    //p.set_radian(PI /2.0);
    //p2.set_radian(PI /2.0);

    p.set_collidable(true);
    p2.set_collidable(true);

    p.set_elasticity(1.0);
    p2.set_elasticity(1.0);

    p.set_position(&Vector::new(200.0,415.0));
    p2.set_position(&Vector::new(600.0,415.0));

    p.set_velocity(&Vector::new(100.0,0.0));
    p2.set_velocity(&Vector::new(-100.0,0.000));

    //let mut p_circle: CircleParticle = CircleParticle::new(ap.get_new_id());
    //p_circle.init_circle(25.0);
    //p_circle.set_position(&Vector::new(200.0, 600.0));

    let mut p3 = ParticleCollection::new(ap.get_new_id());

    p3.init_composite(Vector::new(400.0, 415.0));

    object_helper::create_rectangle(&mut p3, 
    (ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id()));

    p3.set_collide_internal(false);

    let mut list:ParticleCollection = ParticleCollection::new(ap.get_new_id());

    list.add_rectangle_particle(p);
    list.add_rectangle_particle(p2);
    list.add_rectangle_particle(left);
    list.add_rectangle_particle(right);
    list.add_rectangle_particle(top);
    list.add_rectangle_particle(bottom);

    //list.add_circle_particle(p_circle);

    list.set_collide_internal(true);

    ap.add_particle_collection(list);
   // ap.add_particle_collection(p3);

   //ap.set_force(Vector::new(0.0,20.0));
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
    let mut _step:bool = false;
    _step = ap.step();
    _step = ap.step();
    //let mut i:i32 = 0;
    let mut events = Events::new(EventSettings::new());
    let now = Instant::now();
    let mut exit = false;
    for _i in 0..100000
    {
        _step = ap.step();
        
        while let Some(e) = events.next(&mut window) 
        {
            if let Some(_r) = e.render_args() 
            {
               // print!("Rendering");
                ap.paint(&_r, &mut gl); //.render(&r);
                break;
            }

            if let Some(_r) = e.close_args()
            {
               exit = true;
            }
        }
        if _step
        {
           // println!(" p: {:?}", p.get_position());
            //let ten_millis = time::Duration::from_millis(1);
            //thread::sleep(ten_millis);
           // print!("Sleeping: {}", i);
        }
        else
        {
           // print!("Step: {} ", i);
        }


        if now.elapsed().as_secs() > 15 || exit
        {
            break;
        }
    }
}