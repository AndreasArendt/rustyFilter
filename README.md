# Rusty Filter
This is a sample particle filter implementation where a robot is simulated that drives in a circle and receives distance measurements of a point in its world. 
The goal of the robot is to locate that point with a particle filter.
This project also uses my plotting/visualization crate [rplt](https://github.com/AndreasArendt/rust-plot)

# Example
Estimated point cloud and true location of emitter at `(x: 0, y: 18)` and robot final location after 1000 steps at `(x: 0, y: 0)`
![alt text](/doc/example.png)