pub struct Simulation {
    pub time: f64,
    pub dt: f64,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            time: 0.0,
            dt: 0.1,
        }
    }

    pub fn recv(&mut self, input: &str) {
        //println!("Received: {}", input);
        print!("[SimRPC][{:.1}]: ", self.time);
        match input {
            "ping" => {
                println!("Pong!");
            }
            _ => {
                println!("Unknown RPC");
            }
        }
    }

    pub fn update(&mut self) {
        // update time
        self.time += self.dt;
    }

}
