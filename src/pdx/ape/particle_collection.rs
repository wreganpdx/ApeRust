
/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * AbstractCollection.rs
 * 
 * Summary: Should allow grouping of objects for collision detection
 * ore information, see  https://github.com/arctwelve/ape-js-port/tree/master/org/cove/ape
 */

use crate::vector::vector;
use crate::interval::interval;
use crate::collision::collision;
use crate::particle::particle;
use crate::polygon_particle::polygon_particle;
use crate::circle_particle::circle_particle;
use crate::APEngine::APEngine;
use crate::APEngine::APValues;
use crate::collision_detector;
use crate::APEngine::Paint;
use std::any::Any;
use std::default::Default;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

#[allow(unused_variables)]
#[derive(Default)]
#[allow(dead_code)]
pub struct particle_collection
{
	pub collide_internal:bool,
    poly_particles:Vec<polygon_particle>,
	circle_particles:Vec<circle_particle>
}

impl Paint for particle_collection
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		for poly in self.poly_particles.iter_mut()
		{
			poly.paint(args, gl);
		}
	}
}

#[allow(dead_code)]
impl particle_collection
{
	pub fn set_collide_internal(&mut self, c:bool)
	{
		self.collide_internal = c;
	}
	pub fn new() -> particle_collection 
    {
        particle_collection::default()
    }

	fn get_poly_particles(&self)->&Vec<polygon_particle>
	{
		return &self.poly_particles;
	}

	pub fn add_poly_particle(&mut self, p:polygon_particle)
	{
		self.poly_particles.push(p);
	}

	fn get_circle_particles(&self)->&Vec<circle_particle>
	{
		return &self.circle_particles;
	}

	pub fn add_circle_particle(&mut self, p:circle_particle)
	{
		self.circle_particles.push(p);
	}

	pub fn integrate(&mut self, ap:&APValues) 
	{
		for poly in self.poly_particles.iter_mut()
		{
			poly.update(ap);	
		}
/*
		for circ in self.circle_particles 
		{
			circ.update(ap.delta, ap);	
		}
		*/
	}
	pub fn satisfy_constraints(&mut self, ap:&APEngine)
	{
		//
	}	
	pub fn check_collisions(&mut self, ap:&APValues)
	{
			//println!("Check collisions ?");
			if self.collide_internal
			{
				//println!("Check collisions Internal");
				self.check_internal_collisions(ap);
			} 
			/*
			//var len:int = collisionList.length;
			for (var i:int = 0; i < collisionList.length; i++) {
				var g:Group = collisionList[i];
				checkCollisionVsGroup(g);
			}
			*/
	}
	pub fn check_internal_collisions(&mut self, ap:&APValues)
	{
		
		let length:usize = self.poly_particles.len();
		//println!("Check LIST - internal - ");
		for i in 0..length
		{
			//println!("Check LIST - internal - {}", i);
			let mut p = self.poly_particles.remove(i);
			if !p.get_collidable()
			{
				//println!("Check LIST -no collision 1");
				self.poly_particles.insert(i, p);
				continue;
			}
			for j in (0..length-1)
			{
				let mut p2 = self.poly_particles.remove(j);
				if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed())
				{
					//println!("Check LIST -no collision 2");
					self.poly_particles.insert(j, p2);
					continue;
				}
				
				p.set_samp(p.get_curr());
				p2.set_samp(p2.get_curr());
				let p_size = p.get_axes_len();
				let p2_size = p.get_axes_len();
				collision_detector::test_polygon_vs_polygon(&mut p,&mut p2, p_size, p2_size);
				self.poly_particles.insert(j, p2);
			}
			self.poly_particles.insert(i, p);
		}
	}

	pub fn check_collisions_vs_collection(&mut self, col:particle_collection, ap:&APEngine)
	{
			/*
			internal function checkInternalCollisions():void {
		 
			// every particle in this AbstractCollection
			//var plen:int = _particles.length;
			for (var j:int = 0; j < _particles.length; j++) {
				
				var pa:AbstractParticle = _particles[j];
				if (pa == null || ! pa.collidable) continue;
				
				// ...vs every other particle in this AbstractCollection
				for (var i:int = j + 1; i < _particles.length; i++) {
					var pb:AbstractParticle = _particles[i];
					if (pb.collidable) CollisionDetector.test(pa, pb);
				}
				
				// ...vs every other constraint in this AbstractCollection
				//var clen:int = _constraints.length;
				for (var n:int = 0; n < _constraints.length; n++) {
					if(_constraints[n] is SpringConstraint){
						var c:SpringConstraint = _constraints[n];
						if (c.collidable && ! c.isConnectedTo(pa)) {
							c.scp.updatePosition();
							CollisionDetector.test(pa, c.scp);
						}
					}
				}
			}*/
	}
}

/*
package org.cove.ape {
	
	import flash.display.Sprite;
	import flash.utils.getQualifiedClassName;
	import flash.events.Event;
	import flash.events.EventDispatcher;
	import flash.events.IEventDispatcher;
	
	
	/**
	 * The abstract base class for all grouping classes. 
	 * 
	 * <p>
	 * You should not instantiate this class directly -- instead use one of the subclasses.
	 * </p>
	 */	
	public class AbstractCollection implements IEventDispatcher{
		
	
		private var _sprite:Sprite;
		private var _particles:Array;
		private var _constraints:Array;
		private var _isParented:Boolean;
		
		public var dispatcher:EventDispatcher;
		
		
		public function AbstractCollection() {
			if (getQualifiedClassName(this) == "org.cove.ape::AbstractCollection") {
				throw new ArgumentError("AbstractCollection can't be instantiated directly");
			}
			_isParented = false;
			_particles = new Array();
			_constraints = new Array();
			dispatcher = new EventDispatcher(this);
		}
		
		
		/**
		 * The Array of all AbstractParticle instances added to the AbstractCollection
		 */
		public function get particles():Array {
			return _particles;
		}
		
		
		/**
		 * The Array of all AbstractConstraint instances added to the AbstractCollection
		 */	
		public function get constraints():Array {
			return _constraints;	
		}

		
		/**
		 * Adds an AbstractParticle to the AbstractCollection.
		 * 
		 * @param p The particle to be added.
		 */
		public function addParticle(p:AbstractParticle):void {
			particles.push(p);
			p.parent = this;
			if (isParented) p.init();
		}
		
		
		/**
		 * Removes an AbstractParticle from the AbstractCollection.
		 * 
		 * @param p The particle to be removed.
		 */
		public function removeParticle(p:AbstractParticle):Boolean {
			var ppos:int = particles.indexOf(p);
			if (ppos == -1) return false;
			particles.splice(ppos, 1);
			p.cleanup();
			return true;
		}
		
		
		/**
		 * Adds a constraint to the Collection.
		 * 
		 * @param c The constraint to be added.
		 */
		public function addConstraint(c:AbstractConstraint):void {
			constraints.push(c);
			if (isParented) c.init();
		}


		/**
		 * Removes a constraint from the Collection.
		 * 
		 * @param c The constraint to be removed.
		 */
		public function removeConstraint(c:AbstractConstraint):Boolean {
			var cpos:int = constraints.indexOf(c);
			if (cpos == -1) return false;
			constraints.splice(cpos, 1);
			c.cleanup();
			return true;
		}
		
		
		/**
		 * Initializes every member of this AbstractCollection by in turn calling 
		 * each members <code>init()</code> method.
		 */
		public function init():void {
			
			for (var i:int = 0; i < particles.length; i++) {
				particles[i].init();	
			}
			for (i = 0; i < constraints.length; i++) {
				constraints[i].init();
			}
		}
		
				
		/**
		 * paints every member of this AbstractCollection by calling each members
		 * <code>paint()</code> method.
		 */
		public function paint():void {
			
			var p:AbstractParticle;
			for (var i:int = 0; i < _particles.length; i++) {
				p = _particles[i];
				//if ((! p.fixed) || p.alwaysRepaint) p.paint();
				p.paint();
			}
			
			for (i = 0; i < _constraints.length; i++) {
				var sc:AbstractConstraint = _constraints[i];
				if (sc.alwaysRepaint) sc.paint();
			}
		}
		
		
		/**
		 * Calls the <code>cleanup()</code> method of every member of this AbstractCollection.
		 * The cleanup() method is called automatically when an AbstractCollection is removed
		 * from its parent.
		 */
		public function cleanup():void {
			
			for (var i:int = 0; i < particles.length; i++) {
				particles[i].cleanup();	
			}
			for (i = 0; i < constraints.length; i++) {
				constraints[i].cleanup();
			}
		}
				
		
		/**
		 * Provides a Sprite to use as a container for drawing or adding children. When the
		 * sprite is requested for the first time it is automatically added to the global
		 * container in the APEngine class.
		 */	
		public function get sprite():Sprite {
			
			if (_sprite != null) return _sprite;
			
			if (APEngine.container == null) {
				throw new Error("The container property of the APEngine class has not been set");
			}
			
			_sprite = new Sprite();
			APEngine.container.addChild(_sprite);
			return _sprite;
		}
		
	
		/**
		 * Returns an array of every particle and constraint added to the AbstractCollection.
		 */
		public function getAll():Array {
			return particles.concat(constraints);
		}	
		
		
		/**
		 * @private
		 */
		internal function get isParented():Boolean {
			return _isParented;
		}	


		/**
		 * @private
		 */		
		internal function set isParented(b:Boolean):void {
			_isParented = b;
		}	
		
								
		/**
		 * @private
		 */
		internal function integrate(dt2:Number):void {
			for (var i:int = 0; i < _particles.length; i++) {
				var p:AbstractParticle = _particles[i];
				p.update(dt2);	
			}
		}		
		
			
		/**
		 * @private
		 */
		internal function satisfyConstraints():void {
			//var len:int = _constraints.length;
			for (var i:int = 0; i < _constraints.length; i++) {
				var c:AbstractConstraint = _constraints[i];
				c.resolve();	
			}
		}			
		

		/**
		 * @private
		 */	
		 internal function checkInternalCollisions():void {
		 
			// every particle in this AbstractCollection
			//var plen:int = _particles.length;
			for (var j:int = 0; j < _particles.length; j++) {
				
				var pa:AbstractParticle = _particles[j];
				if (pa == null || ! pa.collidable) continue;
				
				// ...vs every other particle in this AbstractCollection
				for (var i:int = j + 1; i < _particles.length; i++) {
					var pb:AbstractParticle = _particles[i];
					if (pb.collidable) CollisionDetector.test(pa, pb);
				}
				
				// ...vs every other constraint in this AbstractCollection
				//var clen:int = _constraints.length;
				for (var n:int = 0; n < _constraints.length; n++) {
					if(_constraints[n] is SpringConstraint){
						var c:SpringConstraint = _constraints[n];
						if (c.collidable && ! c.isConnectedTo(pa)) {
							c.scp.updatePosition();
							CollisionDetector.test(pa, c.scp);
						}
					}
				}
			}
		}
	
	
		/**
		 * @private
		 */	
		internal function checkCollisionsVsCollection(ac:AbstractCollection):void {
			
			// every particle in this collection...
			//var plen:int = _particles.length;
			for (var j:int = 0; j < _particles.length; j++) {
				
				var pga:AbstractParticle = _particles[j];
				if (pga == null || ! pga.collidable) continue;
				
				// ...vs every particle in the other collection
				//var acplen:int = ac.particles.length;
				for (var x:int = 0; x < ac.particles.length; x++) {
					var pgb:AbstractParticle = ac.particles[x];
					if (pgb.collidable) CollisionDetector.test(pga, pgb);
				}
				// ...vs every constraint in the other collection
				//var acclen:int = ac.constraints.length;
				for (x = 0; x < ac.constraints.length; x++) {
					if(ac.constraints[x] is SpringConstraint){
						var cgb:SpringConstraint = ac.constraints[x];
						if (cgb.collidable && ! cgb.isConnectedTo(pga)) {
							cgb.scp.updatePosition();
							CollisionDetector.test(pga, cgb.scp);
						}
					}
				}
			}
			
			// every constraint in this collection...
			//var clen:int = _constraints.length;
			for (j = 0; j < _constraints.length; j++) {
				if(_constraints[j] is SpringConstraint){
					var cga:SpringConstraint = _constraints[j];
					if (cga == null || ! cga.collidable) continue;
					
					// ...vs every particle in the other collection
					//acplen = ac.particles.length;
					for (var n:int = 0; n < ac.particles.length; n++) {
						pgb = ac.particles[n];
						if (pgb.collidable && ! cga.isConnectedTo(pgb)) {
							cga.scp.updatePosition();
							CollisionDetector.test(pgb, cga.scp);
						}
					}
				}
			}
		}
		
		/**
		 * Event dispatcher functions
		 */
		 
		public function addEventListener(type:String, listener:Function, useCapture:Boolean = false, priority:int = 0, useWeakReference:Boolean = false):void{
			dispatcher.addEventListener(type, listener, useCapture, priority);
		}
           
		public function dispatchEvent(evt:Event):Boolean{
			return dispatcher.dispatchEvent(evt);
		}
    
		public function hasEventListener(type:String):Boolean{
			return dispatcher.hasEventListener(type);
		}
		
		public function removeEventListener(type:String, listener:Function, useCapture:Boolean = false):void{
			dispatcher.removeEventListener(type, listener, useCapture);
		}
		
		public function willTrigger(type:String):Boolean {
			return dispatcher.willTrigger(type);
		}
	}
}'
*/