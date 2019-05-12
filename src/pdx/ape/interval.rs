/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * interval.rs
 * 
 * Summary: Simple interval for deciding the minimum and maximum for an object. Basically used to decide which objects may be about to collide.
 * 
 * For more information, see  https://github.com/arctwelve/ape-js-port/tree/master/org/cove/ape
 */
use std::fmt;

#[allow(unused_variables)]
#[derive(Default)]
pub struct interval
{
    pub min:f64,
    pub max:f64,
}

impl interval
{
	pub fn new(min: f64, max: f64) -> interval 
    {
        interval { min:min, max:max }
    }
}

impl fmt::Debug for interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "interval {{ min: {}, max: {} }}", self.min, self.max)
    }
}
