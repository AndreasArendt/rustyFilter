use num_traits::Float;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2D { x, y }
    }

    // Note the added T: Debug bound here
    pub fn euclidean_distance(&self, other: &Self) -> T    
    {        
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}
