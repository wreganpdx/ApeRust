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
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min: min, max: max }
    }
}

impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interval {{ min: {}, max: {} }}", self.min, self.max)
    }
}
