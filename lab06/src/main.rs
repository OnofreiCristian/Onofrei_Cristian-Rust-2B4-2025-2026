use std::fs;

trait Commands {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]);
}

struct Ping;

struct Count;

struct Times {
    count: u32,
}

impl Commands for Ping {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("pong!");
    }
}

impl Commands for Count {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("counted {} arguments", args.len());
    }
}

impl Commands for Times {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, args: &[&str]) {
        self.count += 1;
        println!("times has been called {} times", self.count);
    }
}

struct Hello;

impl Commands for Hello {
    fn get_name(&self) -> &str {
        "hello"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("Hello World!");
    }
}

struct Terminal {
    commands: Vec<Box<dyn Commands>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, cmd: Box<dyn Commands>) {
        self.commands.push(cmd);
    }

    fn run(&mut self, file: &str) {
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", file, e);
                return;
            }
        };

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            let name = parts[0];
            let args = &parts[1..];

            if name == "stop" {
                println!("Stopping terminal.");
                break;
            }

            let mut found_command = false;

            for command in self.commands.iter_mut() {
                if command.get_name() == name {
                    command.exec(&args);

                    found_command = true;
                    break;
                }
            }

            if !found_command {
                println!("Error! Couldn't find a command with that name.");
            }
        }
    }
}

fn pb1() {
    let mut terminal = Terminal::new();

    // Înregistrăm comenzile
    terminal.register(Box::new(Ping {}));
    terminal.register(Box::new(Count {}));
    terminal.register(Box::new(Times { count: 0 }));
    terminal.register(Box::new(Hello {}));

    let filepath = "C:\\Users\\Cris\\Desktop\\Facultate\\An2\\Rust\\lab06\\src\\commands.txt";

    terminal.run(filepath);
}

fn main() {
    pb1();
}
