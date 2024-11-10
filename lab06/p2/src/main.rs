use rusqlite::Connection;
use std::fs;

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[String]);
}

#[derive(Debug)]
struct Bookmark {
    name: String,
    url: String,
}

struct BookmarkCommand {
    conn: Connection,
}

impl BookmarkCommand {
    fn new() -> Result<Self> {
        let conn = Connection::open("bookmarks.db")?;

        let create = r"
        create table if not exists bookmarks (
            name text not null,
            url text not null,
            unique(name, url)
        );
        ";
        conn.execute(create, ())?;
        Ok(BookmarkCommand { conn })
    }

    fn add(&self, name: &str, url: &str) -> Result<()> {
        self.conn.execute(
            "insert into bookmarks (name, url) values (?1, ?2)",
            (name, url),
        )?;
        Ok(())
    }

    fn search(&self, name: &str) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("select * from bookmarks where name like ?1")?;

        let bookmark_iter = stmt.query_map([format!("%{}%", name)], |row| {
            Ok(Bookmark {
                name: row.get("name")?,
                url: row.get("url")?,
            })
        })?;

        for bookmark in bookmark_iter {
            println!("{:?} ", bookmark);
        }
        Ok(())
    }
}

impl Command for BookmarkCommand {
    fn get_name(&self) -> &str {
        "bk"
    }

    fn exec(&mut self, args: &[String]) {
        match args[0].as_str() {
            "add" if args.len() == 3 => {
                if let Err(e) = self.add(&args[1], &args[2]) {
                    println!("Error adding bookmark: {}", e);
                }
            }
            "search" if args.len() == 2 => {
                if let Err(e) = self.search(&args[1]) {
                    println!("Error searching bookmarks: {}", e);
                }
            }
            _ => {
                println!("Usage: bookmark add <name> <url> or bookmark search <query>");
            }
        }
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            commands: Vec::new(),
        }
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
                } else {
                    if command_name.to_ascii_lowercase() == command.get_name() {
                        println!(
                            "Did you mean [{}] instead of [{}]?",
                            command.get_name(),
                            command_name
                        );
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

    match BookmarkCommand::new() {
        Ok(bookmark_command) => terminal.register_command(Box::new(bookmark_command)),
        Err(e) => {
            println!("Failed to initialize bookmark command: {}", e);
            return;
        }
    }

    BookmarkCommand::clear_data(&BookmarkCommand::new().unwrap());

    terminal.run();
}
