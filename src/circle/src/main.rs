extern crate ApeRust;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use ApeRust::vector::vector;
use ApeRust::APEngine::APEngine;
use ApeRust::rectangle_particle::rectangle_particle;
use ApeRust::circle_particle::circle_particle;
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
use std::f64::consts;

use std::f64;

fn main() 
{

    
    let opengl = OpenGL::V3_2;
    
    let mut window: Window = WindowSettings::new(
            "test window",
            [800, 800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl:GlGraphics = GlGraphics::new(opengl);
    let mut ap:APEngine = APEngine::new();

    ap.init(0.001);

    //setting borders
    let mut left: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut right: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut top: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut bottom: rectangle_particle = rectangle_particle::new(ap.get_new_id());

    left.create_rectangle(5.0,790.0);
    right.create_rectangle(5.0,790.0);
    top.create_rectangle(795.0,5.0);
    bottom.create_rectangle(795.0,5.0);

    left.set_position(&vector::new(5.0, 400.0));
    right.set_position(&vector::new(795.0, 400.0));
    top.set_position(&vector::new(400.0, 795.0));
    bottom.set_position(&vector::new(400.0, 5.0));

    left.set_collidable(true);
    right.set_collidable(true);
    top.set_collidable(true);
    bottom.set_collidable(true);

    left.set_fixed(true);
    right.set_fixed(true);
    top.set_fixed(true);
    bottom.set_fixed(true);

    //obsticles
    let mut left_bar: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut right_bar: rectangle_particle = rectangle_particle::new(ap.get_new_id());

    left_bar.create_rectangle(2.5, 400.0);
    right_bar.create_rectangle(2.5, 400.0);

    left_bar.set_position(&vector::new(275.0, 300.0));
    right_bar.set_position(&vector::new(520.0, 500.0));

    left_bar.set_collidable(true);
    right_bar.set_collidable(true);

    left_bar.set_fixed(true);
    right_bar.set_fixed(true);

    left_bar.set_radian(f64::consts::PI *0.69999);
    right_bar.set_radian(f64::consts::PI *0.39999);

    left_bar.set_friction(0.0);
    right_bar.set_friction(0.0);

    //create circle objects
    let mut p_circle: circle_particle = circle_particle::new(ap.get_new_id());
    let mut p_circle2: circle_particle = circle_particle::new(ap.get_new_id());

    p_circle.init_circle(25.0);
    p_circle2.init_circle(25.0);

    p_circle.set_position(&vector::new(200.0, 100.0));
    p_circle2.set_position(&vector::new(600.0, 100.0));

    p_circle.set_collidable(true);
    p_circle2.set_collidable(true);

    p_circle.set_elasticity(0.7);
    p_circle2.set_elasticity(0.7);

    p_circle.set_friction(0.0);
    p_circle2.set_friction(0.0);
    
    p_circle.set_velocity(&vector::new(0.0, 400.0));
    p_circle2.set_velocity(&vector::new(0.0, 400.0));

    //p_circle.set_friction(0.0);
    //p_circle2.set_friction(0.0);

    let mut list:particle_collection = particle_collection::new(ap.get_new_id());

    list.add_rectangle_particle(left);
    list.add_rectangle_particle(right);
    list.add_rectangle_particle(top);
    list.add_rectangle_particle(bottom);
    list.add_rectangle_particle(left_bar);
    list.add_rectangle_particle(right_bar);
    list.add_circle_particle(p_circle);
    list.add_circle_particle(p_circle2);
    list.set_collide_internal(true);

    ap.add_particle_collection(list);

    ap.set_force(vector::new(0.0,75.0));

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
                ap.paint(&r, &mut gl); 
                break;
            }

            if let Some(r) = e.close_args()
            {
               exit = true;
            }
        }


        if now.elapsed().as_secs() > 30 || exit
        {
            break;
        }
    }
}