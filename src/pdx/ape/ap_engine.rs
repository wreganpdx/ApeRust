/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * ApeEngine.rs
 * 
 * Summary: This is the core of the Ape Engine
 * Functions to impliment include, adding objects, stepping through physics simulations
 * and painting.
 * ore information, see https://exercism.io/my/tracks/rust
 */

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

//use piston::window::WindowSettings;
//use piston::event_loop::*;
use piston::input::*;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
//use opengl_graphics::OpenGL;

use crate::circle_particle::CircleParticle;
use crate::vector::Vector;
use crate::particle_collection::ParticleCollection;
extern crate time;



#[derive(Default)]
pub struct ApEngine
{
	pub force:Vector,
	pub massless_force:Vector,
	time_step:f64,
	last_step:f64,
	pub delta:f64,
	num_groups:i64,
	pub damping:f64,
	constraint_cycles:i64,
	constraint_collision_cycles:i64,
	part_collection:Vec<ParticleCollection>,
	id_count:i64,
	background_color:[f32; 4]
}

pub trait Paint
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics);
}

#[derive(Default)]
pub struct APValues
{
	pub force:Vector,
	pub massless_force:Vector,
	pub damping:f64,
	pub time_step:f64,
}

impl Paint for ApEngine
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		use graphics::*;
		
		gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(self.background_color, gl);
        });

		//let rect = rectangle::rectangle_by_corners(0.0, 0.0, args.width/2.0, args.height/2.0);

		self.paint_all(args, gl);
	}
}
impl APValues
{
	
	pub fn new(_damping:&f64, m_force:&Vector, force:&Vector, time:&f64) -> APValues
	{
		let damping:f64 = _damping.clone();
		let massless_force:Vector = m_force.clone();
		let force:Vector = force.clone();
		let time_step:f64 = time.clone();
		APValues { force:force, massless_force:massless_force, damping:damping, time_step:time_step}
	}
}


impl ApEngine
{
	/**
		 * The main step function of the engine. This method should be called
		 * continously to advance the simulation. The faster this method is 
		 * called, the faster the simulation will run. Usually you would call
		 * this in your main program loop. 
		 */		

	pub fn get_circle_by_id(&mut self, i:i64)->&mut CircleParticle
	{
		for p in self.part_collection.iter_mut()
		{
			let t = p.get_circle_by_id(&i);
			match t
			{
				Some(c) => 
				{
					return c;
				}
				_=> 
				{
					continue;
				}
			}
		}
		panic!("Couldn't find object!");
	}
	pub fn get_new_id(&mut self)->i64
	{
		self.id_count = self.id_count + 1;
		return self.id_count;
	}	
	pub fn paint_all(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		for i in 0..self.part_collection.len()
		{
			self.part_collection[i].paint(args, gl);
		}
	}
	pub fn get_ap_values(&self)->APValues
	{
		return APValues::new(&self.damping, &self.massless_force , &self.force, &self.time_step)
	}
	pub fn step(&mut self)->bool
	{
		let timespec = time::get_time();
		let cur = (timespec.sec as f64 * 1000.0) + (timespec.nsec as f64 / 1000.0 / 1000.0);

		let elapsed:f64 = (cur - self.last_step)/1000.0;

		if elapsed > self.delta
		{
			//print!("Elapsed: {}", elapsed);
			//print!(" *Delta: {} * ", self.delta);
			//print!(" *Stepping Elapsed: {} * ", elapsed);
			//println!("--");
			//print!("Elapsed: {}", elapsed);
			self.time_step = elapsed;
			self.last_step = cur;
		}
		else
		{
			//print!("Time {:?}", timespec);
			//print!("Return Elapsed: {}", elapsed);
			return false;
		}
		self.integrate();
		for _i in 0..self.constraint_cycles
		{
			self.satisfy_constraints();
		}

		for _i in 0..self.constraint_collision_cycles
		{
			self.satisfy_constraints();
			self.check_collisions();
		}
		return true;
	}

	pub fn satisfy_constraints(&mut self)
	{
		let vals = self.get_ap_values();
		for pc in self.part_collection.iter_mut() {
			pc.satisfy_constraints(&vals);
		}
	}
	pub fn check_collisions(&mut self)
	{
	//	println!("Check collisions");
		let values:APValues = self.get_ap_values();
		let length = self.part_collection.len();
		for i in 0..length
		{
			self.part_collection[i].check_collisions(&values);
		}
		
		for i in 0..length
		{
			let mut rem = self.part_collection.remove(i);
			let length2 = self.part_collection.len();
			for j in 0..length2
			{
				let mut rem2 = self.part_collection.remove(j);
				rem.check_collisions_vs_collection(&mut rem2, &values);
				self.part_collection.insert(j, rem2);
			}
			self.part_collection.insert(i, rem);
		}
	}
	pub fn integrate(&mut self)
	{	
		let values:APValues = self.get_ap_values();
		for i in 0..self.part_collection.len()
		{
			self.part_collection[i].integrate(&values);
		}
	}

	pub fn init(&mut self,delta:f64)
	{
		self.delta = delta;
		self.time_step = 0.0;
		let timespec = time::get_time();
		let cur = (timespec.sec as f64 * 1000.0) + (timespec.nsec as f64 / 1000.0 / 1000.0);
		self.last_step = cur;
		self.num_groups = 0;
		//self.groups = new Array();	
		self.force = Vector::new(0.0,0.0);
		self.massless_force = Vector::new(0.0,0.0);
		self.damping = 1.0 - delta;
		self.constraint_cycles = 0;
		self.constraint_collision_cycles = 1;
		println!("Ape Engine Initialized");
		self.id_count = 0;

	}

	pub fn get_damping(&self) -> &f64
	{
		return &self.damping
	}

	pub fn new() -> ApEngine
	{
		let mut ap = ApEngine::default();
		ap.background_color = [0.6, 0.6, 0.6, 1.0];
		return ap;
	}

	pub fn set_background_color(&mut self, col:[f32;4])
	{
		self.background_color = col;
	}

	pub fn add_particle_collection(&mut self, pc:ParticleCollection)
	{
		self.part_collection.push(pc);
	}

	pub fn set_massless_force(&mut self, v:Vector)
	{
		self.massless_force = v;
	}
	pub fn set_force(&mut self, v:Vector)
	{
		self.force = v;
	}
}