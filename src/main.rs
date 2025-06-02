pub mod particlefilter;
use crate::particlefilter::ParticleFilter;

fn main() {

    // Create an instance of ParticleFilter
    let mut pf = ParticleFilter::new();

    // Call the sample method to demonstrate functionality
    pf.sample();
}
