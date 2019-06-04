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

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::f64::consts::PI;

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
    let mut ap:ApEngine = ApEngine::new();

    ap.init(0.01);

    //setting borders
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

    
    left.set_elasticity(0.49);
    right.set_elasticity(0.49);
    top.set_elasticity(0.49);
    bottom.set_elasticity(0.49);

    //obsticles
    let mut left_bar: RectangleParticle = RectangleParticle::new(ap.get_new_id());
    let mut right_bar: RectangleParticle = RectangleParticle::new(ap.get_new_id());

    left_bar.set_elasticity(0.49);
    right_bar.set_elasticity(0.49);

    left_bar.create_rectangle(2.5, 400.0);
    right_bar.create_rectangle(2.5, 400.0);

    left_bar.set_position(&Vector::new(275.0, 300.0));
    right_bar.set_position(&Vector::new(520.0, 500.0));

    left_bar.set_friction(0.0);
    right_bar.set_friction(0.0);

    left_bar.set_collidable(true);
    right_bar.set_collidable(true);

    left_bar.set_fixed(true);
    right_bar.set_fixed(true);

    left_bar.set_radian(PI *0.69999);
    right_bar.set_radian(PI *0.39999);

    left_bar.set_friction(0.0);
    right_bar.set_friction(0.0);

    //create circle objects
    let mut p_circle: CircleParticle = CircleParticle::new(ap.get_new_id());
    let mut p_circle2: CircleParticle = CircleParticle::new(ap.get_new_id());

    p_circle.init_circle(25.0);
    p_circle2.init_circle(25.0);

    let mut p_wheel: CircleParticle = CircleParticle::new(ap.get_new_id());
    p_wheel.init_circle(25.0);

    p_wheel.init_wheel(2.0);

    p_circle.set_position(&Vector::new(200.0, 120.0));
    p_circle2.set_position(&Vector::new(300.0, 80.0));

    p_wheel.set_position(&Vector::new(600.0, 140.0));

    p_circle.set_friction(0.0);
    p_circle2.set_friction(0.0);

    p_wheel.set_friction(0.0);

    p_circle.set_collidable(true);
    p_circle2.set_collidable(true);
    p_wheel.set_collidable(true);

    p_circle.set_elasticity(0.49);
    p_circle2.set_elasticity(0.49);
    p_wheel.set_elasticity(0.49);

    
    p_circle.set_velocity(&Vector::new(0.0, 4.0));
    p_circle2.set_velocity(&Vector::new(0.0, 4.0));
    p_wheel.set_velocity(&Vector::new(0.0, 4.0));

    let mut list:ParticleCollection = ParticleCollection::new(ap.get_new_id());

    list.add_rectangle_particle(left);
    list.add_rectangle_particle(right);
    list.add_rectangle_particle(top);
    list.add_rectangle_particle(bottom);
    list.add_rectangle_particle(left_bar);
    list.add_rectangle_particle(right_bar);
    list.add_circle_particle(p_circle);
    list.add_circle_particle(p_circle2);
    list.add_circle_particle(p_wheel);
    list.set_collide_internal(true);

    ap.add_particle_collection(list);


    ap.set_force(Vector::new(0.0,3.0));

      let mut _step: bool = false;
    _step = ap.step();
    _step = ap.step();
    let mut events = Events::new(EventSettings::new());
    let now = Instant::now();
    let mut now_render = Instant::now();
    let mut exit = false;
    let mut steps = 0;
    let mut engine_steps = 0;
    let mut frames_rendered = 0;

    loop {
        _step = ap.step();

        if now_render.elapsed().as_millis() * 3 > 100 {
            while let Some(e) = events.next(&mut window) {
                if let Some(_r) = e.render_args() {
                    ap.paint(&_r, &mut gl);
                    now_render = Instant::now();
                    frames_rendered += 1;
                    break;
                }

                if let Some(_r) = e.close_args() {
                    exit = true;
                }
                if let Some(Button::Keyboard(key)) = e.release_args() {
                    match key {
                        Key::A => {
                            println!("Release A");
                            do_something(&mut ap);
                            do_something(&mut ap);
                        }
                        Key::D => {
                            println!("Release D");
                            do_something(&mut ap);
                            do_something(&mut ap);
                        }
                        _ => {
                            println!("Release KEY: {:?}", key);
                        }
                    }
                }
                if let Some(Button::Keyboard(key)) = e.press_args() {
                    let wheel_speed = 0.3;
                    match key {
                        Key::A => {
                            println!("Press A");
                            do_something(&mut ap);
                            do_something(&mut ap);
                        }
                        Key::D => {
                            println!("Press D");
                            do_something(&mut ap);
                            do_something(&mut ap);
                        }
                        _ => {
                            println!("Press KEY: {:?}", key);
                        }
                    }
                }
            }
        }
        if _step {
            engine_steps += 1;
            do_something_2(&mut ap);
        }

        if now.elapsed().as_secs() > 600 || exit
        //stops after 10 minutes or clicking exiting.
        {
            break;
        }
        steps = steps + 1;
    }
    println!(
        "Engine steps: {}, Frames Rendered: {}, Total Steps: {}, Seconds: {}",
        engine_steps,
        frames_rendered,
        steps,
        now.elapsed().as_secs()
    );
}

pub fn do_something(ap: &mut ApEngine) {
}
pub fn do_something_2(_ap: &mut ApEngine) {
}
