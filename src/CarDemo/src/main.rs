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
mod car_create;
mod capsule_create;
mod surfaces_create;

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
            [650, 350]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl:GlGraphics = GlGraphics::new(opengl);
    let mut ap:ApEngine = ApEngine::new();

    ap.init(0.01);

  

    let col_a = [51.0/255.0, 68.0/255.0, 51.0/255.0, 1.0];
    let col_b = [51.0/255.0, 102.0/255.0, 170.0/255.0, 1.0];
    let col_c = [170.0/255.0, 187.0/255.0, 187.0/255.0, 1.0];
    let col_d = [102.0/255.0, 153.0/255.0, 170.0/255.0, 1.0];
    let col_e = [119.0/255.0, 136.0/255.0, 119.0/255.0, 1.0];

    ap.set_background_color(col_a.clone());

    let mut cap = ParticleCollection::new(ap.get_new_id());
    capsule_create::capsule_create(&mut cap, (ap.get_new_id(),ap.get_new_id(),ap.get_new_id()),col_c.clone());

    let mut surf = ParticleCollection::new(ap.get_new_id());
    surfaces_create::surfaces_create(&mut surf, (
        ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),  
        ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),  
        ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),  
        ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),ap.get_new_id(),  
        ),
        col_d.clone(), col_b.clone());


    ap.set_massless_force(Vector::new(0.0,7.0));

    let mut car = ParticleCollection::new(ap.get_new_id());
    let wheel_id_1 = ap.get_new_id();
    let wheel_id_2 = ap.get_new_id();
    car_create::car_create(&mut car, (wheel_id_1.clone(), wheel_id_2.clone(), ap.get_new_id(), ap.get_new_id()), col_c.clone(), col_e.clone());

    ap.add_particle_collection(car);
    ap.add_particle_collection(cap);
    ap.add_particle_collection(surf);
    
    let mut _step:bool = false;
    _step = ap.step();
    _step = ap.step();
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
                if let Some(Button::Keyboard(key)) = e.release_args()
                {
                    
                    match key 
                    {
                        Key::A => {
                            println!("Release A");
                            speed_up_wheel(wheel_id_1, 0.0, &mut ap);
                            speed_up_wheel(wheel_id_2, 0.0, &mut ap);
                            },
                        Key::D => {
                            println!("Release D");
                            speed_up_wheel(wheel_id_1, 0.0, &mut ap);
                            speed_up_wheel(wheel_id_2, 0.0, &mut ap);
                            },
                        _ => {println!("Release KEY: {:?}", key);}
                    }
                    
                }
                if let Some(Button::Keyboard(key)) = e.press_args()
                {
                    
                    match key 
                    {
                        Key::A => {
                            println!("Press A");
                            speed_up_wheel(wheel_id_1, -0.4, &mut ap);
                            speed_up_wheel(wheel_id_2, -0.4, &mut ap);
                            },
                        Key::D => {
                            println!("Press D");
                            speed_up_wheel(wheel_id_1, 0.4, &mut ap);
                            speed_up_wheel(wheel_id_2, 0.4, &mut ap);
                            },
                        _ => {println!("Press KEY: {:?}", key);}
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
