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
use std::f64::consts;

use std::f64;

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

    ap.init(0.001);



    let mut p_circle: circle_particle = circle_particle::new(ap.get_new_id());
    let mut p_circle2: circle_particle = circle_particle::new(ap.get_new_id());


    p_circle.init_circle(25.0);
    p_circle2.init_circle(25.0);

    p_circle.set_position(&vector::new(200.0, 400.0));
    p_circle2.set_position(&vector::new(600.0, 400.0));

    p_circle.set_collidable(true);
    p_circle2.set_collidable(true);



    p_circle.set_velocity(&vector::new(1.0,0.0));
    p_circle2.set_velocity(&vector::new(-1.0,0.0));

    let mut list:particle_collection = particle_collection::new();

    list.add_circle_particle(p_circle);
    list.add_circle_particle(p_circle2);

    list.set_collide_internal(true);

    ap.add_particle_collection(list);

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