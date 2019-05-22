extern crate ape_rust;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use ape_rust::vector::Vector;
use ape_rust::ap_engine::ApEngine;
//use ape_rust::polygon_particle::PolygonParticle;
use ape_rust::rectangle_particle::RectangleParticle;
use ape_rust::circle_particle::CircleParticle;
//use std::{thread, time};
use std::time::Instant;
//use std::time::Duration;
use ape_rust::particle::Particle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::ap_engine::Paint;
//use crate object_helper::create_rectangle; 
mod car_create;
mod capsule_create;
mod surfaces_create;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
//use std::f64::consts;
//use std::f64;

fn main() 
{
    let opengl = OpenGL::V3_2;
    
    let mut window: Window = WindowSettings::new(
            "testcd.",
            [660, 350]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl:GlGraphics = GlGraphics::new(opengl);
    let mut ap:ApEngine = ApEngine::new();

    ap.init(0.01);

    //boundries
    let mut left: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut right: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut top: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut bottom: RectangleParticle = RectangleParticle::new(ap.get_new_id());

    left.create_rectangle(5.0,800.0);
    right.create_rectangle(5.0,800.0);
    top.create_rectangle(790.0,5.0);
    bottom.create_rectangle(800.0,5.0);

    left.set_position(&Vector::new(5.0, 400.0));
    right.set_position(&Vector::new(797.5, 405.0));
    top.set_position(&Vector::new(400.0, 802.5));
    bottom.set_position(&Vector::new(400.0, 2.5));

    left.set_collidable(true);
    right.set_collidable(true);
    top.set_collidable(true);
    bottom.set_collidable(true);

    left.set_fixed(true);
    right.set_fixed(true);
    top.set_fixed(true);
    bottom.set_fixed(true);

    //objects
    let mut rect: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut circ: CircleParticle = CircleParticle::new(ap.get_new_id());
    let mut p_circle: CircleParticle = CircleParticle::new(ap.get_new_id());
    let mut wheel: CircleParticle = CircleParticle::new(ap.get_new_id());

    
    p_circle.init_circle(25.0);
    wheel.init_circle(25.0);
    wheel.init_wheel(2.0);

    p_circle.set_position(&Vector::new(200.0, 600.0));

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

    circ.set_position(&Vector::new(600.0,415.0));
    rect.set_position(&Vector::new(225.0,415.0));
    wheel.set_position(&Vector::new(400.0,215.0));

    rect.set_velocity(&Vector::new(20.0,70.0));
    circ.set_velocity(&Vector::new(20.0,-4.000));
    wheel.set_velocity(&Vector::new(-200.0,30.0));

    let mut list:ParticleCollection = ParticleCollection::new(ap.get_new_id());

    list.add_rectangle_particle(rect);
    list.add_circle_particle(circ);
    list.add_rectangle_particle(left);
    list.add_rectangle_particle(right);
    list.add_rectangle_particle(top);
    list.add_rectangle_particle(bottom);
    list.add_circle_particle(wheel);
    list.add_circle_particle(p_circle);

    list.set_collide_internal(true);

    let mut cap = ParticleCollection::new(ap.get_new_id());
    capsule_create::capsule_create(&mut cap, (ap.get_new_id(),ap.get_new_id(),ap.get_new_id()));

    let mut surf = ParticleCollection::new(ap.get_new_id());
    surfaces_create::surfaces_create(&mut surf, (
        ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),  
        ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),  
        ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),  
        ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),  
        ));

    //let mut car = ParticleCollection::new(ap.get_new_id());

    ap.set_force(Vector::new(0.0,200.0));

    let mut car = ParticleCollection::new(ap.get_new_id());
    let wheel_id_1 = ap.get_new_id();
    let wheel_id_2 = ap.get_new_id();
    car_create::car_create(&mut car, (wheel_id_1.clone(), wheel_id_2.clone(), ap.get_new_id()));

    ap.add_particle_collection(car);
    ap.add_particle_collection(list);
    ap.add_particle_collection(cap);
    ap.add_particle_collection(surf);
    
    let mut _step:bool = false;
    _step = ap.step();
    _step = ap.step();
    //let mut i:i32 = 0;
    let mut events = Events::new(EventSettings::new());
    let now = Instant::now();
    let mut now_render = Instant::now();
    let mut exit = false;
    let mut steps = 0;
    let mut engine_steps = 0;
    let mut frames_rendered = 0;

    loop
    {
        for i in 0..100
        {
            _step = ap.step();
        }
        
        if now_render.elapsed().as_millis() * 3 > 100
        {
            while let Some(e) = events.next(&mut window) 
            {
                if let Some(_r) = e.render_args() 
                {
                    ap.paint(&_r, &mut gl); 
                    now_render = Instant::now();
                    frames_rendered += 1;
                    break;
                }

                if let Some(_r) = e.close_args()
                {
                    exit = true;
                }
               // if let Some(Input::Button::press_args)
                if let Some(Button::Keyboard(key)) = e.press_args()
                {
                    
                    match key 
                    {
                        Key::A => {
                            println!("A");
                            speed_up_wheel(wheel_id_1, 0.2, &mut ap);
                            speed_up_wheel(wheel_id_2, 0.2, &mut ap);
                            },
                        Key::D => {
                            println!("D");
                            speed_up_wheel(wheel_id_1, -0.2, &mut ap);
                            speed_up_wheel(wheel_id_2, -0.2, &mut ap);
                            },
                        _ => {println!("KEY: {:?}", key);}
                    }
                    
                }
            }
        }
        if _step
        {
           engine_steps += 1;
        }

        if now.elapsed().as_secs() > 60 || exit
        {
            
            break;
        }
        steps = steps + 1;
    }
    println!("Engine steps: {}, Frames Rendered: {}, Total Steps: {}, Seconds: {}", engine_steps, frames_rendered, steps, now.elapsed().as_secs());
}

pub fn speed_up_wheel(i:i64, s:f64, ap:&mut ApEngine)
{
    let p = &mut ap.get_circle_by_id(i);
    p.set_ang_velocity(s);
}
