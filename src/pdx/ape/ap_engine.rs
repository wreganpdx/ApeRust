/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510
Final Project
*/

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
/**
 * ApeEngine.rs
 *
 * Summary: This is the core of the Ape Engine
 * Functions to impliment include, adding objects, stepping through physics simulations
 * and painting.
 * ore information, see https://exercism.io/my/tracks/rust
 */
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::circle_particle::CircleParticle;
use crate::owner_collision::OwnerCollision;
use crate::particle::Particle;
use crate::particle_collection::ParticleCollection;
use crate::poly_poly_constraint::PolyPolyConstraint;
use crate::vector::Vector;
extern crate time;

#[derive(Default)]
pub struct ApEngine {
    pub force: Vector,
    pub massless_force: Vector,
    last_step: f64,
    pub delta: f64,
    pub damping: f64,
    constraint_cycles: i64,
    constraint_collision_cycles: i64,
    part_collection: Vec<ParticleCollection>,
    id_count: i64,
    background_color: [f32; 4],
    leftover_elapsed:f64
}

pub trait Paint {
    fn paint(&mut self, args: &RenderArgs, gl: &mut GlGraphics);
}

#[derive(Default)]
pub struct APValues {
    pub force: Vector,
    pub massless_force: Vector,
    pub damping: f64,
    pub time_step: f64,
}

impl Paint for ApEngine {
    fn paint(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        gl.draw(args.viewport(), |_c, gl| {
            clear(self.background_color, gl);
        });

        self.paint_all(args, gl);
    }
}
impl APValues {
    pub fn new(_damping: &f64, m_force: &Vector, force: &Vector, time: &f64) -> APValues {
        let damping: f64 = _damping.clone();
        let massless_force: Vector = m_force.clone();
        let force: Vector = force.clone();
        let time_step: f64 = time.clone();
        APValues {
            force: force,
            massless_force: massless_force,
            damping: damping,
            time_step: time_step,
        }
    }
}

impl ApEngine {
    pub fn get_circle_by_id(&mut self, i: i64) -> &mut CircleParticle {
        for p in self.part_collection.iter_mut() {
            let t = p.get_circle_by_id(&i);
            match t {
                Some(c) => {
                    return c;
                }
                _ => {
                    continue;
                }
            }
        }
        panic!("Couldn't find object!");
    }
    pub fn get_particle_collection_by_id(&mut self, i: i64) -> &mut ParticleCollection {
        let mut part_index = 0;
        let p = loop {
            if self.part_collection[part_index].id == i {
                break &mut self.part_collection[part_index];
            }
            part_index += 1;
            if part_index >= self.part_collection.len() {
                panic!("Couldn't find collection!");
            }
        };
        return p;
    }
    pub fn get_new_id(&mut self) -> i64 {
        self.id_count = self.id_count + 1;
        return self.id_count;
    }
    pub fn paint_all(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        for i in 0..self.part_collection.len() {
            self.part_collection[i].paint(args, gl);
        }
    }
    pub fn get_ap_values(&self) -> APValues {
        return APValues::new(
            &self.damping,
            &self.massless_force,
            &self.force,
            &self.delta,
        );
    }
    pub fn step(&mut self) -> bool {
        let timespec = time::get_time();
        let cur = (timespec.sec as f64 * 1000.0) + (timespec.nsec as f64 / 1000.0 / 1000.0);

        let mut elapsed: f64 = (cur - self.last_step) / 1000.0;
        //add whatever was left from the last engine step
        elapsed += self.leftover_elapsed;
        //check to see if enough time has passed since last engine step
        if elapsed > self.delta 
        {
            //mark what time the engine step is happening at
            self.last_step = cur;
            
            while elapsed > 0.0 
            {
                //do one engine step for each slice of delta in elapsed
                self.step_with_time();
                elapsed -= self.delta;
            }
            //add leftover elapsed for next iteration
            elapsed += self.delta;
            self.leftover_elapsed = elapsed
        } else {
            //return false if no engine step
            return false;
        }
        return true;
    }
    pub fn step_with_time(&mut self) {
        self.integrate();
        for _i in 0..self.constraint_cycles {
            self.satisfy_constraints();
        }
        for _i in 0..self.constraint_collision_cycles {
            self.check_collisions();
            self.satisfy_pending_collisions();
            self.satisfy_constraints();
        }
    }
    pub fn satisfy_pending_collisions(&mut self) {
        let mut collisions: Vec<OwnerCollision> = Vec::new();
        let mut indexs: Vec<usize> = Vec::new();
        for i in 0..self.part_collection.len() {
            let mut pending: Option<OwnerCollision> =
                self.part_collection[i].find_pending_collision();
            while match pending {
                Some(p) => {
                    collisions.push(p);
                    indexs.push(i);
                    true
                }
                None => false,
            } {
                pending = self.part_collection[i].find_pending_collision();
            }
        }
        while collisions.len() > 0 {
            let col = collisions.remove(0).clone();
            let index = indexs.remove(0);
            let mut part = self.part_collection.remove(index);
            let mut i = 0;
            let mut collider = loop {
                let t = self.part_collection[i].get_particle_by_id(&col.collider);
                match t {
                    Some(_c) => {
                        break self.part_collection[i].get_particle_by_id(&col.collider);
                    }
                    None => {}
                }
                i += 1;
                if i >= self.part_collection.len() {
                    break None;
                }
            };
            part.collide_pending_spring(&mut collider, col.clone());
            self.part_collection.insert(index, part);
        }
    }

    pub fn satisfy_constraints(&mut self) {
        let vals = self.get_ap_values();
        for pc in self.part_collection.iter_mut() {
            pc.satisfy_constraints(&vals);
        }
    }
    pub fn check_collisions(&mut self) {
        let values: APValues = self.get_ap_values();
        let length = self.part_collection.len();
        for i in 0..length {
            self.part_collection[i].check_collisions(&values);
        }

        for i in 0..length {
            let mut rem = self.part_collection.remove(i);
            let length2 = self.part_collection.len();
            for j in 0..length2 {
                let mut rem2 = self.part_collection.remove(j);
                rem.check_collisions_vs_collection(&mut rem2, &values);
                self.part_collection.insert(j, rem2);
            }
            self.part_collection.insert(i, rem);
        }
    }
    pub fn integrate(&mut self) {
        let values: APValues = self.get_ap_values();
        for i in 0..self.part_collection.len() {
            self.part_collection[i].integrate(&values);
        }
    }

    pub fn init(&mut self, delta: f64) {
        /*in our case, delta is the ideal amount of time in seconds we want between engine steps*/
        self.delta = delta;

        let timespec = time::get_time();
        let cur = (timespec.sec as f64 * 1000.0) + (timespec.nsec as f64 / 1000.0 / 1000.0);
        self.last_step = cur;
        self.force = Vector::new(0.0, 0.0);
        self.massless_force = Vector::new(0.0, 0.0);
        self.damping = 1.0 - delta;
        self.constraint_cycles = 0;
        self.constraint_collision_cycles = 1;
        println!("APEngine Initialized");
        self.id_count = 0;
        self.background_color = [79.0 / 255.0, 36.0 / 255.0, 59.0 / 255.0, 1.0];
    }
    pub fn get_damping(&self) -> &f64 {
        return &self.damping;
    }

    pub fn new() -> ApEngine {
        let mut ap = ApEngine::default();
        ap.background_color = [0.6, 0.6, 0.6, 1.0];
        return ap;
    }

    pub fn set_background_color(&mut self, col: [f32; 4]) {
        self.background_color = col;
    }

    pub fn add_particle_collection(&mut self, pc: ParticleCollection) {
        self.part_collection.push(pc);
    }

    pub fn set_massless_force(&mut self, v: Vector) {
        self.massless_force = v;
    }
    pub fn set_force(&mut self, v: Vector) {
        self.force = v;
    }
}
