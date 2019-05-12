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

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


use crate::vector::vector;
use crate::particle_collection::particle_collection;
extern crate time;



#[derive(Default)]
pub struct APEngine
{
	pub force:vector,
	pub massless_force:vector,
	time_step:f64,
	last_step:f64,
	pub delta:f64,
	num_groups:i64,
	pub damping:f64,
	constraint_cycles:i64,
	constraint_collision_cycles:i64,
	part_collection:Vec<particle_collection>
}

pub trait Paint
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics);
}

#[derive(Default)]
pub struct APValues
{
	pub force:vector,
	pub massless_force:vector,
	pub damping:f64,
	pub time_step:f64,
}

impl Paint for APEngine
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		use graphics::*;
		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });
		self.paint_all(args, gl);
	}
}
impl APValues
{
	
	pub fn new(_damping:&f64, m_force:&vector, force:&vector, time:&f64) -> APValues
	{
		let damping:f64 = _damping.clone();
		let massless_force:vector = m_force.clone();
		let force:vector = force.clone();
		let time_step:f64 = time.clone();
		APValues { force:force, massless_force:massless_force, damping:damping, time_step:time_step}
	}
}
/*
lazy_static! 
{
	pub static ref Engine: APEngine =
	{
		let mut ap:APEngine = APEngine::new();
		ap
	};
}
*/

impl APEngine
{
	/**
		 * The main step function of the engine. This method should be called
		 * continously to advance the simulation. The faster this method is 
		 * called, the faster the simulation will run. Usually you would call
		 * this in your main program loop. 
		 */			
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
			//print!(" *Delta: {} * ", self.delta);
			//print!(" *Stepping Elapsed: {} * ", elapsed);
			println!("--");
			self.time_step = elapsed;
			self.last_step = cur;
		}
		else
		{
			//sprint!("Elapsed: {}", elapsed);
			return false;
		}
		self.integrate();
		for i in 0..self.constraint_cycles
		{
			self.satisfy_constraints();
		}

		for i in 0..self.constraint_collision_cycles
		{
			self.satisfy_constraints();
			self.check_collisions();
		}
		return true;
	}

	pub fn satisfy_constraints(&mut self)
	{
		for pc in self.part_collection.iter_mut() {
			//pc.satisfy_constraints(&self);
		}
	}
	pub fn check_collisions(&mut self)
	{
	//	println!("Check collisions");
		let values:APValues = self.get_ap_values();
		for i in 0..self.part_collection.len()
		{
		//	println!("Check LIST");
			self.part_collection[i].check_collisions(&values);
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
		self.force = vector::new(0.0,0.0);
		self.massless_force = vector::new(0.0,0.0);
		self.damping = 1.0;
		self.constraint_cycles = 0;
		self.constraint_collision_cycles = 1;
		println!("Ape Engine Initialized");

	}

	pub fn get_damping(&self) -> &f64
	{
		return &self.damping
	}

	pub fn new() -> APEngine
	{
		return APEngine::default();
	}

	pub fn add_particle_collection(&mut self, pc:particle_collection)
	{
		self.part_collection.push(pc);
	}
}


/*
package org.cove.ape {
	
	import flash.display.DisplayObjectContainer;
	
	/**
	 * The main engine class. 
	 * 
	 */
	public final class APEngine {
		
		/**@private */
		internal static var force:Vector;
		/**@private */
		internal static var masslessForce:Vector;
			
		private static var groups:Array;
		private static var numGroups:int;
		internal static var timeStep:Number;
		
		private static var _damping:Number;
		private static var _container:DisplayObjectContainer;
		
		private static var _constraintCycles:int;
		private static var _constraintCollisionCycles:int;
		
	
		/**
		 * Initializes the engine. You must call this method prior to adding
		 * any particles or constraints.
		 * 
		 * @param dt The delta time value for the engine. This parameter can be used -- in 
		 * conjunction with speed at which <code>APEngine.step()</code> is called -- to change the speed
		 * of the simulation. Typical values are 1/3 or 1/4. Lower values result in slower,
		 * but more accurate simulations, and higher ones result in faster, less accurate ones.
		 * Note that this only applies to the forces added to particles. If you do not add any
		 * forces, the <code>dt</code> value won't matter.
		 */
		public static function init(dt:Number = 0.25):void {
			timeStep = dt * dt;
			
			numGroups = 0;
			groups = new Array();
		
			force = new Vector(0,0);
			masslessForce = new Vector(0,0);
			
			damping = 1;
			
			_constraintCycles = 0;
			_constraintCollisionCycles = 1;
		}


		/**
		 * The global damping. Values should be between 0 and 1. Higher numbers
		 * result in less damping. A value of 1 is no damping. A value of 0 will
		 * not allow any particles to move. The default is 1.
		 * 
		 * <p>
		 * Damping will slow down your simulation and make it more stable. If you find
		 * that your sim is "blowing up', try applying more damping. 
		 * </p>
		 * 
		 * @param d The damping value. Values should be >=0 and <=1.
		 */
		public static function get damping():Number {
			return _damping;
		}
		
		
		/**
		 * @private
		 */
		public static function set damping(d:Number):void {
			_damping = d;
		}


		/**
		 * Determines the number of times in a single <code>APEngine.step()</code> cycle that 
		 * the constraints have their positions corrected. Increasing this number can result in
		 * stiffer, more stable configurations of constraints, especially when they are in large
		 * complex arrangements. The trade off is that the higher you set this number the more 
		 * performance will suffer.
		 *
		 * <p>
		 * This setting differs from the <code>constraintCollisionCycles</code> property in that it
		 * only resolves constraints during a <code>APEngine.step()</code>. The default value
		 * is 0. Because this property doesn't correct for collisions, you should only use it when
		 * the collisions of an arrangement of particles and constraints are not an issue. If you 
		 * do set this value higher than the default of 0, then  <code>constraintCollisionCycles</code>
		 * should at least be 1, in order to check collisions one time during the 
		 * <code>APEngine.step()</code> cycle.
		 * </p>
		 * 
		 */
		public static function get constraintCycles():int {
			return _constraintCycles;
		}
		
		
		/**
		 * @private
		 */
		public static function set constraintCycles(numCycles:int):void {
			_constraintCycles = numCycles;
		}	
		
		
		/**
		 * 
		 * Determines the number of times in a single <code>APEngine.step()</code> cycle that
		 * the constraints and particles have their positions corrected. This can greatly increase
		 * stability and prevent breakthroughs, especially with large complex arrangements of 
		 * constraints and particles. The larger this number, the more stable the simulation,
		 * at an expense of performance.
		 *
		 * <p> 
		 * This setting differs from the <code>constraintCycles</code> property in that it
		 * resolves both constraints and collisions during a <code>APEngine.step()</code>. 
		 * The default value is 1.
		 * </p>
		 */
		public static function get constraintCollisionCycles():int {
			return _constraintCollisionCycles;
		}
		
		
		/**
		 * @private
		 */
		public static function set constraintCollisionCycles(numCycles:int):void {
			_constraintCollisionCycles = numCycles;
		}			
		
		
		/**
		 * The default container used by the default painting methods of the particles and
		 * constraints. If you wish to use to the built in painting methods you must set 
		 * this first.
		 *
		 * @param s An instance of the Sprite class that will be used as the default container.
		 */
		public static function get container():DisplayObjectContainer {
			return _container;
		}
			
		
		/**
		 * @private
		 */
		public static function set container(d:DisplayObjectContainer):void {
			_container = d;
		}
		
	
		/**
		 * Adds a force to all particles in the system. The mass of the particle is taken into 
		 * account when using this method, so it is useful for adding forces that simulate effects
		 * like wind. Particles with larger masses will not be affected as greatly as those with
		 * smaller masses. Note that the size (not to be confused with mass) of the particle has
		 * no effect on its physical behavior.
		 * 
		 * @param f A Vector represeting the force added.
		 */ 
		public static function addForce(v:Vector):void {
			force.plusEquals(v);
		}
		
		
		/**
		 * Adds a 'massless' force to all particles in the system. The mass of the particle is 
		 * not taken into account when using this method, so it is useful for adding forces that
		 * simulate effects like gravity. Particles with larger masses will be affected the same
		 * as those with smaller masses. Note that the size (not to be confused with mass) of 
		 * the particle has no effect on its physical behavior.
		 * 
		 * @param f A Vector represeting the force added.
		 */ 	
		public static function addMasslessForce(v:Vector):void {
			masslessForce.plusEquals(v);
		}
			
			
		/**
		 * 
		 */
		public static function addGroup(g:Group):void {
			groups.push(g);
			g.isParented = true;
			numGroups++;
			g.init();
		}
		
		
		/**
		 * @private
		 */
		public static function removeGroup(g:Group):void {
			
			var gpos:int = groups.indexOf(g);
			if (gpos == -1) return;
			
			groups.splice(gpos, 1);
			g.isParented = false;
			numGroups--;
			g.cleanup();
		}
		
		
		/**
		 * The main step function of the engine. This method should be called
		 * continously to advance the simulation. The faster this method is 
		 * called, the faster the simulation will run. Usually you would call
		 * this in your main program loop. 
		 */			
		public static function step():void {
			integrate();
			for (var j:int = 0; j < _constraintCycles; j++) {
				satisfyConstraints();
			}
			for (var i:int = 0; i < _constraintCollisionCycles; i++) {
				satisfyConstraints();
				checkCollisions();
			}
		}


		/**
		 * Calling this method will in turn call each particle and constraint's paint method.
		 * Generally you would call this method after stepping the engine in the main program
		 * cycle.
		 */			
		public static function paint():void {
			for (var j:int = 0; j < numGroups; j++) {
				var g:Group = groups[j];
				g.paint();
			}
		}
				

		private static function integrate():void {	
			for (var j:int = 0; j < numGroups; j++) {
				var g:Group = groups[j];
				g.integrate(timeStep);
			}
		}
	
		
		private static function satisfyConstraints():void {
			for (var j:int = 0; j < numGroups; j++) {
				var g:Group = groups[j];
				g.satisfyConstraints();
			}
		}


		private static function checkCollisions():void {
			for (var j:int = 0; j < numGroups; j++) {
				var g:Group = groups[j];
				g.checkCollisions();
			}
		}	
	}	
}
*/