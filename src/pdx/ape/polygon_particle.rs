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
use crate::vector::vector;
use crate::interval::interval;
use crate::collision::collision;
use crate::particle::particle;

#[allow(unused_variables)]
#[derive(Default)]
struct polygon_particle
{
    radian:f64,
    density:f64,
    original_vertices:Vec<vector>,
    vertices:Vec<vector>,
    numVertices:i64,
    axes:Vec<vector>,
    curr:vector,
    prev:vector,
    temp:vector,
    samp:vector,
    mass:f64,
    friction:f64,
    elasticity:f64,
    rest_loops:i64,
    rest_count:i64,
    center:vector,
    pinned:bool,
    smashable:bool,
    max_exit_velocity:f64,
    at_rest:bool,
    left_max:f64,
    right_max:f64,
    multi_sampe:i64,
    coll:collision,
    pin:vector,
}

impl polygon_particle
{
    fn new()->polygon_particle
    {
        return polygon_particle::default();
    }
}
impl particle for polygon_particle
{
    
    fn get_mass(&self)-> f64
    {
        return self.mass;
    }
    fn set_mass(&mut self, m:f64)
    {
        self.mass = m;
    }

	fn get_elasticity(&self)-> f64
    {
        return self.elasticity;
    }
	fn set_elasticity(&mut self, e:f64)
    {
        self.elasticity = e;
    }

	fn get_curr(&self)-> vector
    {
        return self.curr;
    }
	fn set_curr(&mut self, c:vector)
    {
        self.curr.copy(c);
    }

	fn get_position(&self)-> vector
    {
        return vector::new().copy(self);
    }
	fn set_position(&mut self, c:vector)
    {
        self.curr.copy(c);
        self.prev.copy(c);
    }

	fn get_prev(&self)-> vector
    {
        return self.prev;
    }
	fn set_prev(&mut self, p:vector)
    {
         self.prev = p;
    }

	fn get_px(&self)-> f64
    {
        return self.curr.x;
    }
	fn set_px(&mut self, x:f64)
    {
        self.curr.x = x;
        self.prev.x = x;
    }

	fn get_py(&self)-> f64
    {
        return self.curr.y;
    }
	fn set_py(&mut self, y:f64)
    {
        self.curr.y = y;
        self.prev.y = y;
    }

	fn get_curr_x(&self)-> f64
    {
        return self.curr.x;
    }
	fn set_curr_x(&mut self, x:f64)
    {
        self.curr.x = x;
    }

	fn get_curr_y(&mut self)-> f64
    {
        return self.curr.y;
    }
	fn set_curr_y(&mut self, y:f64)
    {
        self.curr.y = y;
    }
    

	fn get_samp(&self)-> vector
    {

    }
	fn set_samp(&mut self, s:vector)
    {

    }

	fn get_interval(&self)-> interval
    {

    }
	fn set_interval(&mut self, i:interval)
    {

    }

	fn get_temp(&self)-> vector
    {

    }
	fn set_temp(&mut self, t:vector)
    {

    }

	fn get_forces(&self)-> vector
    {

    }
	fn set_forces(&mut self, f:vector)
    {

    }

	fn get_collision(&self)-> collision
    {

    }
	fn set_collision(&mut self, f:collision)
    {

    }

	//fn get_parent()-> particle_collection;
	//fn set_parent(&mut self, pc:particle_collection);

	fn get_kfr(&self)-> f64
    {

    }
	fn set_kfr(&mut self, kfr:f64)
    {

    }

	fn get_inv_mass(&self)-> f64
    {

    }
	fn set_inv_mass(&mut self, im:f64)
    {

    }

	fn get_friction(&self)-> f64
    {

    }
	fn set_friction(&mut self, f:f64)
    {

    }

	fn get_fixed(&self)-> bool
    {

    }
	fn set_fixed(&mut self, f:bool)
    {

    }

	fn get_collidable(&self)-> bool
    {

    }
	fn set_collidable(&mut self, f:bool)
    {

    }

	fn get_pinned(&self)-> bool
    {

    }
	fn set_pinned(&mut self, f:bool)
    {

    }

	//fn get_pinned_to()-> particle;
	//fn set_pinned_to(&mut self, p:particle);

	fn get_pin(&self)-> vector
    {

    }
	fn set_pin(&mut self, p:vector)
    {

    }

	fn get_center(&self)-> vector
    {

    }
	fn set_center(&mut self, c:vector)
    {

    }

	fn get_multi_sample(&self)-> i64
    {

    }
	fn set_multi_sample(&mut self, i:i64)
    {

    }

	fn get_smashable(&self)-> bool
    {

    }
	fn set_smashable(&mut self, i:bool)
    {

    }

	fn get_max_exit_velocity(&self)-> f64
    {

    }
	fn set_max_exit_velocity(&mut self, ev:f64)
    {

    }

	fn get_velocity(&self)-> vector
    {

    }
	fn set_velocity(&mut self, i:vector)
    {

    }

	fn get_at_rest(&self)-> bool
    {

    }
	fn set_at_rest(&mut self, i:bool)
    {

    }

	fn get_rest_loops(&self)-> i64
    {

    }
	fn set_rest_loops(&mut self, rl:i64)
    {

    }

	fn get_rest_count(&self)-> i64
    {

    }
	fn set_rest_count(&mut self, rc:i64)
    {

    }

	fn get_left_max_x(&self)-> i64
    {

    }
	fn set_left_max_x(&mut self, lm:i64)
    {

    }

	fn get_right_max_x(&self)-> i64
    {

    }
	fn set_right_max_x(&mut self, rm:i64)
    {

    }

	fn add_force(&mut self, f:vector)
    {

    }

	fn add_massless_force(&mut self, f:vector)
    {

    }

	fn update(dt2:f64)
    {

    }

	fn get_components(cn:vector)->collision
    {

    }

	//fn resolve_collision(mtd:vector, vel:vector, n:vector, d:f64, o:int, p:particle);

	fn resolve_velocities(dv:vector, dw:f64, normal:vector)
    {

    }

	fn get_inv_inertia(&self)->f64
    {

    }

	fn get_ang_velocity(&self)->f64
    {

    }

	fn set_ang_velocity(&mut self, a:f64)
    {

    }

	fn get_radian(&self)->f64
    {

    }

	fn get_left_most_x_value(&self)->f64
    {

    }

	fn get_right_most_x_value(&self)->f64
    {

    }
}
/*
package org.cove.ape {
	
	import flash.display.Graphics;
	import flash.geom.Matrix;
	
	/**
	 * An n-sided polygon shaped particle. 
	 */ 
	public class PolygonParticle extends AbstractParticle {

		
		public function PolygonParticle(x:Number, 
				y:Number,
				width:Number, 
				height:Number,
				numVertices:int,
				rotation:Number = 0,
				fixedPosition:Boolean = false,
				mass:Number = 1, 
				elasticity:Number = 0.15,
				friction:Number = 0.1) {
				
				super(x, y, fixedPosition, mass, elasticity, friction);
				
				_numVertices = numVertices;
				createVertices(width, height);
				radian = rotation;
				
				//this.density = density;
		}
		
		internal function createVertices(width:Number, height:Number):void{
			_vertices = new Array();
			_originalVertices = new Array();
			
			var a:Number = Math.PI/numVertices;
			var da:Number = MathUtil.TWO_PI/numVertices;
			
			for(var i:int = 0; i < numVertices; i++){
				a+= da;
				_originalVertices.push(new Vector(Math.cos(a) * width, Math.sin(a) * height));
			}			
		}
		
		
		public override function get radian():Number {
			return _radian;
		}
		
		/**
		 * @private
		 */		
		public function set radian(t:Number):void {
			t = t % (MathUtil.TWO_PI);
			_radian = t;
			orientVertices(t);
			setAxes();
		}
		
		public function get angle():Number {
			return radian * MathUtil.ONE_EIGHTY_OVER_PI;
		}

		/**
		 * @private
		 */		
		public function set angle(a:Number):void {
			radian = a * MathUtil.PI_OVER_ONE_EIGHTY;
		}
		
		/**
		 * Sets up the visual representation of this PolygonParticle. This method is called 
		 * automatically when an instance of this PolygonParticle's parent Group is added to 
		 * the APEngine, when  this PolygonParticle's Composite is added to a Group, or the 
		 * PolygonParticle is added to a Composite or Group.
		 */				
		public override function init():void {
			cleanup();
			if (displayObject != null) {
				initDisplay();
			} else {
			
				sprite.graphics.clear();
				sprite.graphics.lineStyle(lineThickness, lineColor, lineAlpha);
				sprite.graphics.beginFill(fillColor, fillAlpha);
				sprite.graphics.moveTo(_originalVertices[0].x, _originalVertices[0].y);
				for(var i:int = 1; i < _originalVertices.length; i++){
					sprite.graphics.lineTo(_originalVertices[i].x, _originalVertices[i].y);
				}
				sprite.graphics.lineTo(_originalVertices[0].x, _originalVertices[0].y);
				sprite.graphics.endFill();
			}
			paint();
		}
		
		public override function paint():void {
			sprite.x = curr.x;
			sprite.y = curr.y;
			sprite.rotation = angle;
		}
		
		public function clearSprite():void{
			sprite.parent.removeChild(sprite);
		}
		
		internal function get vertices():Array{
			return _vertices;
		}
		
		internal function get numVertices():int{
			return _numVertices;
		}
		
		internal function set density(d:Number):void{
			_density = d;
			mass = calculateMass();
		}
		
		internal function get density():Number{
			return _density;
		}
		
		internal function calculateMass():Number{
			if(numVertices < 2){
				return 5 * density;
			}
			
			var m:Number = 0;
			var j:int = numVertices - 1;
			for (var i:int = 0; i < numVertices; i++){
				var P0:Vector = vertices[j];
				var P1:Vector = vertices[i];
				m += Math.abs(P0.cross(P1));
				j = i;
			}
			if(numVertices <= 2){
				m = 10;
			}
			m *= density * .5;
			return m;
		}
		
		internal function orientVertices(r:Number){
			for(var i:int = 0; i < _originalVertices.length; i++){
				_vertices[i] = _originalVertices[i].rotate(r);
			}
		}
		
		/**
		 * @private
		 */	
		internal function getProjection(axis:Vector):Interval {
			
			var c:Number = curr.dot(axis);
			
			var rad:Number = _vertices[0].dot(axis);
			var negRad:Number = rad;
			var posRad:Number = rad;
			
			for (var i:int = 1; i < _vertices.length; i++){
				rad = _vertices[i].dot(axis);
				if(rad < negRad){
					negRad = rad;
				}else if(rad > posRad){
					posRad = rad;
				}
			}
			
			interval.min = c + negRad;
			interval.max = c + posRad;
			
			return interval;
		}
		
		internal function getAxes():Array{
			return _axes;
		}
		
		internal function setAxes():void{
			_axes = new Array();
			var j:int = _numVertices - 1;
			for(var i:int = 0; i < _numVertices; i++){
				var e0:Vector = _vertices[j];
				var e1:Vector = _vertices[i];
				var e:Vector = e1.minus(e0);
				var currAxis:Vector = (new Vector(-e.y, e.x)).normalize();
				_axes.push(currAxis);
				j=i;
			}
		}
		
		internal function getClosestVertex(v:Vector):Vector{
			var d:Vector = v.minus(curr);
			var maxDist:Number = 0;
			var index:int = -1;
			
			for(var i:int = 0; i<_vertices.length; i++){
				var dist:Number = d.dot(_vertices[i]);
				if(dist > maxDist){
					maxDist = dist;
					index = i;
				}
			}
			return _vertices[index].plus(curr);
		}
		
		public override function leftMostXValue():Number{
			if(!isNaN(lmx) && fixedPosition) return lmx;
			
			var vx:Number = _vertices[0].x;
			lmx = vx;
			for(var i:int = 1; i < _vertices.length; i++){
				vx = _vertices[i].x;
				if( vx < lmx){
					lmx = vx;
				}
			}
			lmx += curr.x;
			return lmx;
		}
		
		public override function rightMostXValue():Number{
			if(!isNaN(rmx) && fixedPosition) return rmx;
			
			var vx:Number = _vertices[0].x;
			rmx = vx;
			for(var i:int = 1; i < _vertices.length; i++){
				vx = _vertices[i].x;
				if( vx > rmx){
					rmx = vx;
				}
			}
			rmx += curr.x;
			return rmx;
		}
	}
}

*/