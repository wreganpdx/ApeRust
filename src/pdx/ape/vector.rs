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
use std::f64;
#[allow(unused_variables)]
#[derive(Default)]
pub struct Vector
{
    pub x:f64,
    pub y:f64,
}

impl Vector
{

    pub fn get_x(&mut self)-> f64
    {
        return self.x.clone();
    }
    pub fn get_y(&mut self)-> f64
    {
        return self.y.clone();
    }
	pub fn new(x: f64, y: f64) -> Vector 
    {
        Vector { x: x, y: y }
    }

    pub fn clone(&self) -> Vector 
    {
        return Vector::new(self.x.clone(), self.y.clone());
    }

    pub fn set_to(&mut self, px: f64, py: f64)
    {
		self.x = px;
		self.y = py;
	}
    pub fn copy(&mut self, v:&Vector)
    {
        self.x = v.x;
        self.y = v.y;
	}

    pub fn dot(&self, v:&Vector)->f64 
    {
		return self.x * v.x + self.y * v.y;
	}

    pub fn cross(&self, v:&Vector)-> f64
    {
		return self.x * v.y - self.y * v.x;
	}

    pub fn plus(&self, v:&Vector)-> Vector
    {
		return Vector::new(self.x + v.x, self.y + v.y);
	}

    		
	pub fn plus_equals(&mut self, v:&Vector)
    {
        self.x += v.x;
        self.y += v.y;
	}

    pub fn minus(&self, v:&Vector) ->Vector 
    {
		return Vector::new(self.x - v.x, self.y - v.y);  
	}

    pub fn minus_equals(&mut self, v:&Vector)
    {
        self.x -= v.x;
        self.y -= v.y;
	}

    pub fn mult(& self, s:f64)->Vector 
    {
        return Vector::new(self.x *s, self.y *s);
	}

    pub fn mult_equals(&mut self, s:f64)
    {
        self.x *= s;
        self.y *= s;
	}

    pub fn times(&self, v:&Vector) ->Vector 
    {
		return Vector::new(self.x * v.x, self.y * v.y);  
	}

    pub fn times_equals(&mut self, v:&Vector)
    {
		self.x = self.x * v.x;
        self.y = self.y * v.y;  
	}

    pub fn div_equals(&mut self, mut s:f64)
    {
        if s == 0.0
        { s = 0.0001;}
        self.x /= s;
        self.y /= s;
	}

    pub fn divided_by(&self, mut s:f64) ->Vector 
    {
        if s == 0.0
        { s = 0.0001;}
		return Vector::new(self.x / s, self.y /s);  
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
        if a < 0.0
        {
            a *= -1.0;
        }
        while a > 0.2
        {
            c = w/b;
            b = (b+c) * 0.5;
            a = b - c;
        }

		return b;
	}

    pub fn mag_or_one(&self)->f64 
    {
        let mag = self.magnitude();
        if mag < 1.0
        {
            return 1.0
        }
        return mag;
	}

    pub fn length(&mut self)->f64 
    {
        return f64::sqrt(self.x * self.x + self.y * self.y);
    }

    pub fn distance(&self, v:&Vector)->f64 
    {
        let delta:Vector = self.minus(&v);
        let mut mag:f64 = delta.magnitude();
        if mag == 0.0
        {
            mag = 0.0001;
        }
        return mag;
    }
    pub fn normalize(&mut self)->Vector 
    {
        let mut m:f64 = self.magnitude();
        if m == 0.0
        {
            m = 0.0001;
        }
        return self.mult(1.0/m);
	}

    pub fn normalize_self(&mut self) 
    {
        let mut m:f64 = self.magnitude();
        if m == 0.0
        {
            m = 0.0001;
        }
        self.mult_equals(1.0/m);
	}
	pub fn rotate(&self, r:&f64) ->Vector 
    {
        let c = r.cos();
        let s = r.sin();
        return Vector::new(self.x*c-self.y*s,self.x*s+self.y*c);
	}

    pub fn swap(&self)-> Vector
    {
        return Vector::new(self.y.clone(), self.x.clone());
    }

    pub fn swap_with_neg_y(&self)-> Vector
    {
        return Vector::new(-self.y.clone(), self.x.clone());
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector {{ x: {}, y: {} }}", self.x, self.y)
    }
}
