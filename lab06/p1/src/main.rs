use std::fs;

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[String]); 
} 

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, args: &[String]) {
        println!("[{}]: pong!", self.get_name());
    }
}

struct CountCommand;

impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[String]) {
        println!("[{}]: counted {} args", self.get_name(), args.len());
    }
}

struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, args: &[String]) {
        self.count += 1;
        println!("[{}]: called {} times", self.get_name(), self.count);
    }
}

struct HelloCommand;

impl Command for HelloCommand {
    fn get_name(&self) -> &str {
        "hello"
    }

    fn exec(&mut self, args: &[String]) {
        if args.len() == 0 {
            println!("Hello, World !");
        }
        else{
            print!("Hello, ");
        for arg in args {
            print!("{} ", arg);
        }
        println!("!");
    }
    }
}

struct StopCommand;

impl Command for StopCommand {
    fn get_name(&self) -> &str {
        "stop"
    }

    fn exec(&mut self, args: &[String]) {
        println!("Stopping...");
        std::process::exit(0);
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal{
    fn new() -> Self {
        Terminal { commands: Vec::new() }
    }

    fn register_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn run(&mut self) {
        let filename = "src/commands.txt";
        let input = match fs::read_to_string(filename) {
            Ok(contents) => contents,
            Err(e) => {
                println!("Error reading file '{}': {}", filename, e);
                return;
            }
        };

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            let command_name = parts[0];
            let args = &parts[1..];
            
            let mut found = false;

            for command in &mut self.commands {
                if command.get_name() == command_name {
                    let mut args_vec = Vec::new();
                    for arg in args {
                        args_vec.push(arg.to_string());
                    }
                    command.exec(&args_vec);
                    found = true;
                    break;
                }
                else{
                    if command_name.to_ascii_lowercase() == command.get_name()
                    {
                        println!("Did you mean [{}] instead of [{}]?", command.get_name(), command_name);
                        found = true;
                    }
                    }
                }
            

            if !found {
                println!("Unknown command: [{}]", command_name);
                }
            }
        }
    }

fn main() {
  
    let mut terminal = Terminal::new();

    terminal.register_command(Box::new(PingCommand)); 
    terminal.register_command(Box::new(CountCommand));
    terminal.register_command(Box::new(TimesCommand{count: 0}));
    terminal.register_command(Box::new(HelloCommand));
    terminal.register_command(Box::new(StopCommand));

    terminal.run();

}
