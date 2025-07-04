pub mod particlefilter;
use rplt::create_figure;
use rplt::figure::{Layout, LineStyle, MarkerStyle};

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

        let pos = pf.mean_position();

        let x_pos = pf.particles.iter().map(|p| p.pos.x).collect();
        let y_pos = pf.particles.iter().map(|p| p.pos.y).collect();

        let mut p = create_figure(Layout{rows:1, columns:1});    
        p.subplot(0,0).plot(&x_pos, &y_pos, Some(LineStyle::Marker(MarkerStyle::Dot)));
        p.subplot(0,0).plot(&vec!(sim_state.robot.x), &vec!(sim_state.robot.y), Some(LineStyle::Marker(MarkerStyle::Cross)));
        p.subplot(0,0).plot(&vec!(sim_state.target.x), &vec!(sim_state.target.y), Some(LineStyle::Marker(MarkerStyle::Cross)));
        let _ = p.show();

        println!("Estimate ({x},{y})", x=pos.x, y=pos.y);
    }    
}