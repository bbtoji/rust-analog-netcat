use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::{Command, Output};
use std::thread;
pub struct Netcat {
    ip: String,
    port: u16,
}

impl Netcat {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }

    fn change_directory(path: &str) -> io::Result<String> {
        let dir = Path::new(path);
        if !dir.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("[*] Directory not founded: {}", dir.display()),
            ));
        }

        env::set_current_dir(dir)?;
        let current_dir = env::current_dir()?;
        Ok(format!("{}\n", current_dir.display().to_string()))
    }

    fn execute(cmd: &str) -> Result<Output, String> {
        let cmd_parts: Vec<&str> = cmd.split_whitespace().collect();
        if cmd_parts.is_empty() {
            return Err("[!] Empty command!".into());
        }
        let result = match cmd_parts[0] {
            "ls" => Command::new("ls").args(&cmd_parts[1..]).output(),
            "pwd" => Command::new("pwd").args(&cmd_parts[1..]).output(),
            _ => Command::new("sh").arg("-c").arg(cmd).output(),
        };
        result.map_err(|e| format!("[!] Error when execute command {e}"))
    }

    fn handler(mut stream: TcpStream) {
        println!("Connection on {}", stream.peer_addr().unwrap());
        loop {
            let _ = stream.write_all("[ bbtoji@rustcat ] >> ".as_bytes());
            let mut buffer = [0u8; 4096];
            let size = match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };

            let command = String::from_utf8_lossy(&buffer[..size]);
            let command = command.trim();

            if command == "exit" {
                break;
            }

            println!(
                "[*] Execute command, which recieved from client: {}",
                command
            );

            if let Some(cmd) = command.split_whitespace().next() {
                if cmd == "cd" {
                    let cmd_parts: Vec<&str> = command.split_whitespace().collect();
                    if cmd_parts.len() < 2 {
                        let _ = stream.write_all("[!] Usage cd <directory>".as_bytes());
                        continue;
                    }
                    match Netcat::change_directory(cmd_parts[1]) {
                        Ok(dir) => {
                            let _ = stream.write_all(dir.as_bytes());
                        }
                        Err(e) => {
                            let _ = stream.write_all(
                                format!("[!] Error changing directory: {e}\n").as_bytes(),
                            );
                        }
                    }
                    continue;
                }
            }

            match Netcat::execute(&command) {
                Ok(output) => {
                    let mut response = String::new();
                    if !output.stdout.is_empty() {
                        response += &String::from_utf8_lossy(&output.stdout);
                    } else if !output.stderr.is_empty() {
                        response += &String::from_utf8_lossy(&output.stderr);
                    }
                    let _ = stream.write_all(response.as_bytes());
                }
                Err(e) => {
                    let _ = stream.write_all(format!("{e}\n").as_bytes());
                    continue;
                }
            };
        }
    }

    pub fn listen(&self) {
        let listener =
            TcpListener::bind((self.ip.as_str(), self.port)).expect("[!] Error listening port...");
        println!("[*] Server listening on {}:{}", self.ip, self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        Netcat::handler(stream);
                    });
                }
                Err(e) => eprintln!("Error connection: {}", e),
            }
        }
    }

    pub fn connect(&self) {
        println!("Connection for {}:{}...", self.ip, self.port);

        let mut stream = match TcpStream::connect((self.ip.as_str(), self.port)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error connection: {}", e);
                return;
            }
        };

        loop {
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "exit" {
                let _ = stream.write_all(input.as_bytes());
                break;
            }

            let _ = stream.write_all(input.as_bytes());

            let mut buffer = [0u8; 4096];
            let size = stream.read(&mut buffer).unwrap_or(0);
            if size > 0 {
                println!("{}", String::from_utf8_lossy(&buffer[..size]));
            }
        }
    }
}
