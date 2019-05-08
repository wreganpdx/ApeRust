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
#[derive(Default)]
pub struct vector
{
    pub x:f64,
    pub y:f64,
}

impl vector
{
	pub fn new(x: f64, y: f64) -> vector 
    {
        vector { x: x, y: y }
    }

    pub fn set_to(&mut self, px: f64, py: f64)
    {
		self.x = px;
		self.y = py;
	}
    pub fn copy(&mut self, v:&vector)
    {
        self.x = v.x;
        self.y = v.y;
	}

    pub fn dot(&self, v:&vector)->f64 
    {
		return self.x * v.x + self.y * v.y;
	}

    pub fn cross(&self, v:&vector)-> f64
    {
		return self.x * v.y - self.y * v.x;
	}

    pub fn plus(&self, v:&vector)-> vector
    {
		return vector::new(self.x + v.x, self.y + v.y);
	}

    		
	pub fn plusEquals(&mut self, v:&vector)->&vector 
    {
        self.x += v.x;
        self.y += v.y;
        return self as &vector;
	}

    pub fn minus(&self, v:&vector) ->vector 
    {
		return vector::new(self.x - v.x, self.y - v.y);  
	}

    pub fn minusEquals(&mut self, v:&vector)->&vector 
    {
        self.x -= v.x;
        self.y -= v.y;
        return self as &vector;
	}

    pub fn mult(& self, s:f64)->vector 
    {
        return vector::new(self.x *s, self.y *s);
	}

    pub fn multEquals(&mut self, s:f64)->&vector 
    {
        self.x *= s;
        self.y *= s;
        return self as &vector;
	}

    pub fn times(&self, v:&vector) ->vector 
    {
		return vector::new(self.x * v.x, self.y * v.y);  
	}

    pub fn divEquals(&mut self, mut s:f64)->&vector 
    {
        if s == 0.0
        { s = 0.0001;}
        self.x /= s;
        self.y /= s;
        return self as &vector;
	}

    pub fn magnitude(&self)->f64 
    {
        let w:f64 = self.x * self.x + self.y * self.y;
        if w == 0.0
        {
            return 0.0;
        }
        let mut b:f64 = w * 0.25;
        let mut c:f64 = w/b;
        b = (b+c) * 0.5;
        let mut a:f64 = b-c;
        if (a < 0.0)
        {
            a *= -1.0;
        }
        while (a > 0.2)
        {
            c = w/b;
            b = (b+c) * 0.5;
            a = b - c;
        }

		return b;
	}

    pub fn distance(&self, v:&vector)->f64 
    {
        let delta:vector = self.minus(&v);
        let mut mag:f64 = delta.magnitude();
        if (mag == 0.0)
        {
            mag = 0.0001;
        }
        return mag;
    }
    pub fn normalize(&mut self)->vector 
    {
        let mut m:f64 = self.magnitude();
        if m == 0.0
        {
            m = 0.0001;
        }
        return self.mult(1.0/m);
	}
	pub fn rotate(&self, r:&f64) ->vector 
    {
        let c = r.cos();
        let s = r.sin();
        return vector::new(self.x*c-self.y*s,self.x*s+self.y*c);
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

	}
}
*/
