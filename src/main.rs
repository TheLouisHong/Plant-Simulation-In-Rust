use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

mod simulation;


trait Command {
    
    fn help(&self) -> &str;
    fn name(&self) -> &str;
    fn execute(&self, input: &str, tx: &Sender<&str>);
}

struct HelpCommand;
impl Command for HelpCommand {

    fn help(&self) -> &str {
        "help"
    }
    fn name(&self) -> &str {
        "help"
    }
    fn execute(&self, _input: &str, tx: &Sender<&str>) {
        println!("Prints this help message.");
    }
}

struct ExitCommand;
impl Command for ExitCommand {

    fn help(&self) -> &str {
        "exits program"
    }
    fn name(&self) -> &str {
        "exit"
    }
    fn execute(&self, _input: &str, tx: &Sender<&str>) {
        println!("Exiting Program...");
        // exit process
        std::process::exit(0);
    }
}

struct PingCommand;
impl Command for PingCommand {

    fn help(&self) -> &str {
        "ping"
    }
    fn name(&self) -> &str {
        "ping"
    }
    fn execute(&self, _input: &str, tx: &Sender<&str>) {
        tx.send("ping").unwrap();
    }
}


fn main() {
    

    let commands : Vec<Box<dyn Command>>= vec![
        Box::new(HelpCommand), Box::new(ExitCommand), Box::new(PingCommand)
    ];

    println!("The plant is simulating on another thread. Ping it with the 'ping' command.");

    // todo: how to make list command?
    // print commands
    println!("Commands:");
    for command in &commands {
        println!("{}: {}", command.name(), command.help());
    }

    // mpsc
    let (tx, rx): (Sender<&str>, Receiver<&str>)= mpsc::channel();

    // run simulation on thread
    let _sim_thread = thread::spawn(move || {
        let mut sim = simulation::Simulation::new();
        loop {
            // receive
            match rx.try_recv() {
                Ok(msg) => {
                    sim.recv(msg);
                },
                Err(_) => {},
            }
            sim.update();
        }
    });

    loop {
        let mut input = String::new();
        println!("> ");
        std::io::stdin().read_line(&mut input).expect("Failed to read line.");
        let input = input.trim();
        let mut args = input.split_whitespace();
        let command = args.next().unwrap();
        let command = commands.iter().find(|c| c.name() == command);
        match command {
            Some(c) => c.execute(&input, &tx),
            None => println!("Command not found."),
        }
    }
}
