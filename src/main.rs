use std::thread::sleep;
use std::time::Duration;

mod sim;

use sim::Simulation;

fn main() {
    let mut sim = Simulation::new(50, 40);

    sim.activate(25, 25);
    sim.activate(24, 26);
    sim.activate(25, 26);
    sim.activate(25, 27);
    sim.activate(26, 27);

    loop {
        sim.print();
        sim.step();
        sleep(Duration::from_millis(50));
        clearscreen::clear().unwrap();
    }
}
