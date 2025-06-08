pub mod particlefilter;
use crate::particlefilter::ParticleFilter;

mod types;

pub mod sim;
use crate::sim::{sim_step, SimState};

fn main() {
    
    let mut sim_state = SimState::new(18.0, 0.0); // assuming you have a constructor
    let mut pf = ParticleFilter::new();
    
    pf.init(sim_state.distance_m);
    
    for _ in 0..1000 {
        sim_step(&mut sim_state);        

        pf.sample(0.1);
        pf.correct(sim_state.distance_m, &sim_state.robot, 2.0);
        pf.resample();

        //println!("Target ({x},{y})", x=sim_state.target.x, y=sim_state.target.y);
        //println!("Robot ({x},{y})", x=sim_state.robot.x, y=sim_state.robot.y);
        let pos = pf.mean_position();
        println!("Estimate ({x},{y})", x=pos.x, y=pos.y);
    }    
}