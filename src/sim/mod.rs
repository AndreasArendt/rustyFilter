use crate::types::point::Point2D;

pub struct SimState {
    pub robot : Point2D<f64>,
    pub target : Point2D<f64>, 
    pub distance_m : f64,
    pub time_s : f64,
    pub dt_s : f64,
}

impl SimState {
    pub fn new(x : f64, y: f64) -> Self {
        let mut simstate = SimState {
            robot : Point2D::new(0.0, 0.0),
            target : Point2D::new(x, y),
            distance_m : 0.0,
            time_s : 0.0,
            dt_s : 0.01,
        };

        simstate.distance_m = simstate.robot.euclidean_distance(&simstate.target);
        simstate
    }
}

pub fn sim_step(state: &mut SimState) -> &mut SimState {
    state.time_s += state.dt_s;

    // robot movement on a circle
    state.robot.x = f64::cos(std::f64::consts::TAU * state.time_s);
    state.robot.y = f64::sin(std::f64::consts::TAU * state.time_s);

    state.distance_m = state.robot.euclidean_distance(&state.target);

    state
}