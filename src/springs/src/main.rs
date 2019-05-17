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
            [1000, 1000]
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
    let mut rect: rectangle_particle = rectangle_particle::new(ap.get_new_id());
    let mut circ: circle_particle = circle_particle::new(ap.get_new_id());

    let mut p_circle: circle_particle = circle_particle::new(ap.get_new_id());

    let mut wheel: circle_particle = circle_particle::new(ap.get_new_id());

    
    p_circle.init_circle(25.0);
    wheel.init_circle(25.0);
    wheel.init_wheel(2.0);

    p_circle.set_position(&vector::new(200.0, 600.0));

    rect.create_rectangle(40.0,40.0);
    circ.init_circle(20.0);

    //p.set_radian(f64::consts::PI /2.0);

    //p2.set_radian(f64::consts::PI /2.0);

    rect.set_collidable(true);
    circ.set_collidable(true);
    wheel.set_collidable(true);
    p_circle.set_collidable(true);

    rect.set_elasticity(0.9);
    circ.set_elasticity(0.9);
    wheel.set_elasticity(0.9);

    circ.set_position(&vector::new(600.0,415.0));
    rect.set_position(&vector::new(225.0,415.0));
    wheel.set_position(&vector::new(400.0,215.0));

    rect.set_velocity(&vector::new(20.0,70.0));
    circ.set_velocity(&vector::new(20.0,-4.000));
    wheel.set_velocity(&vector::new(-200.0,30.0));

    let mut p3 = particle_collection::new();

    p3.init_composite(vector::new(400.0, 415.0));

    object_helper::create_rectangle(&mut p3, 
    (ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id()));

    p3.set_collide_internal(false);

    let mut list:particle_collection = particle_collection::new();

    list.add_rectangle_particle(rect);
    list.add_circle_particle(circ);
    list.add_rectangle_particle(left);
    list.add_rectangle_particle(right);
    list.add_rectangle_particle(top);
    list.add_rectangle_particle(bottom);
    list.add_circle_particle(wheel);
    list.add_circle_particle(p_circle);

    list.set_collide_internal(true);



    ap.set_force(vector::new(0.0,20.0));

    for i in 0..10
    {
        let mut w = circle_particle::new(ap.get_new_id());
        w.init_circle(25.0);
        w.init_wheel(2.0);
        w.set_elasticity(0.4);
        w.set_collidable(true);
        w.set_position(&vector::new((i as f64) * -60.0 + 600.0,215.0 + (i as f64) * -30.0));
        w.set_velocity(&vector::new(-200.0,30.0));
        list.add_circle_particle(w);
    }

    ap.add_particle_collection(list);
    ap.add_particle_collection(p3);
    
    let mut step:bool = false;
    step = ap.step();
    step = ap.step();
    let mut i:i32 = 0;
    let mut events = Events::new(EventSettings::new());
    let now = Instant::now();
    let mut nowRender = Instant::now();
    let mut exit = false;
    let mut steps = 0;
    let mut EngineSteps = 0;
    let mut FramesRendered = 0;
    while (true)
    {
        step = ap.step();
        if nowRender.elapsed().as_millis() * 3 > 100
        {
            while let Some(e) = events.next(&mut window) 
            {
                if let Some(r) = e.render_args() 
                {
                    ap.paint(&r, &mut gl); 
                    nowRender = Instant::now();
                    FramesRendered += 1;
                    break;
                }

                if let Some(r) = e.close_args()
                {
                    exit = true;
                }
            }
        }
        if step
        {
           EngineSteps += 1;
        }

        if now.elapsed().as_secs() > 60 || exit
        {
            
            break;
        }
        steps = steps + 1;
    }
    println!("Engine steps: {}, Frames Rendered: {}, Total Steps: {}, Seconds: {}", EngineSteps, FramesRendered, steps, now.elapsed().as_secs());
}