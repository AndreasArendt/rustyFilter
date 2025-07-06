use crate::point::Point2D;

pub struct SimState {
    pub robot: Point2D<f64>,
    pub target: Point2D<f64>,
    pub distance_m: f64,
    pub time_s: f64,
    pub dt_s: f64,
}

impl SimState {
    pub fn new(x: f64, y: f64) -> Self {
        let mut simstate = SimState {
            robot: Point2D::new(0.0, 0.0),
            target: Point2D::new(x, y),
            distance_m: 0.0,
            time_s: 0.0,
            dt_s: 0.01,
        };

        simstate.distance_m = simstate.robot.euclidean_distance(&simstate.target);
        simstate
    }

    pub fn step(&mut self) {
        self.time_s += self.dt_s;

        // robot movement on a circle
        self.robot.x = f64::cos(std::f64::consts::TAU * self.time_s);
        self.robot.y = f64::sin(std::f64::consts::TAU * self.time_s);

        self.distance_m = self.robot.euclidean_distance(&self.target);
    }
}
