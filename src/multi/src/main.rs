extern crate ape_rust;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use ape_rust::vector::Vector;
use ape_rust::ap_engine::ApEngine;
use ape_rust::rectangle_particle::RectangleParticle;
use ape_rust::circle_particle::CircleParticle;
use std::time::Instant;
use ape_rust::particle::Particle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::ap_engine::Paint;
mod object_helper;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::f64::consts::PI;
#[derive()]

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

    circ.set_radian(PI - PI);
    //p2.set_radian(PI - PI);

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

    rect.set_velocity(&Vector::new(25.0,70.0));
    circ.set_velocity(&Vector::new(10.0,-4.000));
    wheel.set_velocity(&Vector::new(-100.0,-4.0));

    let mut p3 = ParticleCollection::new(ap.get_new_id());
    p3.init_composite(Vector::new(400.0, 415.0));

    object_helper::create_rectangle(&mut p3, 
    (ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id(),
    ap.get_new_id(), ap.get_new_id()));

    p3.set_collide_internal(false);

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

    ap.add_particle_collection(list);
   // ap.add_particle_collection(p3);

    ap.set_force(Vector::new(0.0,20.0));
    
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
        _step = ap.step();
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
            }
        }
        if _step
        {
           engine_steps += 1;
        }

        if now.elapsed().as_secs() > 30 || exit
        {
            
            break;
        }
        steps = steps + 1;
    }
    println!("Engine steps: {}, Frames Rendered: {}, Total Steps: {}, Seconds: {}", engine_steps, frames_rendered, steps, now.elapsed().as_secs());
}