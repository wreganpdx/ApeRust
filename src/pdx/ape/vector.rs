/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * Vector.rs
 * 
 * Summary: Simple two dimensional object for doing math on x and y coords
 * 
 * For more information, see  https://github.com/arctwelve/ape-js-port/tree/master/org/cove/ape
 */
use std::fmt;

#[allow(unused_variables)]

pub struct vector
{
    x:f64,
    y:f64,
}

impl vector
{
	pub fn new(x: f64, y: f64) -> vector {
        vector { x: x, y: y }
    }
}

impl fmt::Debug for vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector {{ x: {}, y: {} }}", self.x, self.y)
    }
}

/*
	import flash.filters.ColorMatrixFilter;
	import flash.geom.Matrix;
	
	public class Vector {
		
		public var x:Number;
		public var y:Number;
	
	
		public function Vector(px:Number = 0, py:Number = 0) {
			x = px;
			y = py;
		}
		
		
		public function setTo(px:Number, py:Number):void {
			x = px;
			y = py;
		}
		
		
		public function copy(v:Vector):void {
			x = v.x;
			y = v.y;
		}
	
	
		public function dot(v:Vector):Number {
			return x * v.x + y * v.y;
		}
		
		
		public function cross(v:Vector):Number {
			return x * v.y - y * v.x;
		}
		
	
		public function plus(v:Vector):Vector {
			return new Vector(x + v.x, y + v.y); 
		}
	
		
		public function plusEquals(v:Vector):Vector {
			x += v.x;
			y += v.y;
			return this;
		}
		
		
		public function minus(v:Vector):Vector {
			return new Vector(x - v.x, y - v.y);    
		}
	
	
		public function minusEquals(v:Vector):Vector {
			x -= v.x;
			y -= v.y;
			return this;
		}
	
	
		public function mult(s:Number):Vector {
			return new Vector(x * s, y * s);
		}
	
	
		public function multEquals(s:Number):Vector {
			x *= s;
			y *= s;
			return this;
		}
	
	
		public function times(v:Vector):Vector {
			return new Vector(x * v.x, y * v.y);
		}
		
		
		public function divEquals(s:Number):Vector {
			if (s == 0) s = 0.0001;
			x /= s;
			y /= s;
			return this;
		}
		
		
		public function magnitude():Number {
			//return Math.sqrt(x * x + y * y);
			var w:Number = (x * x + y * y);
			if(w == 0) return 0;
			var b:Number = w * 0.25;
			var c:Number = 0;
			var a:Number = 0;
			do {
				c = w / b;
				b = (b + c) * 0.5;
				a = b - c;
				if (a < 0)  a = -a;
			} while (a > .2);
			return b;
		}

		
		public function distance(v:Vector):Number {
			var delta:Vector = this.minus(v);
			var mag:Number = delta.magnitude();
			if (mag == 0) mag = 0.0001;
			return mag;
		}

	
		public function normalize():Vector {
			 var m:Number = magnitude();
			 if (m == 0) m = 0.0001;
			 return mult(1 / m);
		}
		
				
		public function toString():String {
			return (x + " : " + y);
		}
		
		public function applyMatrix(m:Matrix):Vector {
			var v:Vector = new Vector();
			v.x = x * m.a + y * m.c;
			v.y = x * m.b + y * m.d;
			return v;			
		}
		
		public function multMatrix(m:Matrix):Vector {
			var v:Vector = new Vector();
			v.x = x * m.a + y * m.b;
			v.y = x * m.c + y * m.d;
			return v;
		}
		
		public function multEqualsMatrix(m:Matrix):Vector{
			x = x * m.a + y * m.b;
			y = x * m.c + y * m.d;
			return this;
		}
		
		public function rotate(r:Number):Vector{
			var c=Math.cos(r);
			var s=Math.sin(r);
			return new Vector(x*c-y*s,x*s+y*c);
		}
	}
}
*/
