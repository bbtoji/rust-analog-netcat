use clap::{Parser, Subcommand};
use netcat::Netcat;

mod netcat;

/// Netcat analog program
#[derive(Parser, Debug)]
#[command(version, about = "Rust Netcat Clone")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start in listening mode
    Listen {
        /// Port to listen on
        #[arg(short, long, default_value_t = 1337)]
        port: u16,
    },

    /// Connect to a remote host
    Connect {
        /// Host to connect to
        #[arg(long)]
        host: String,

        /// Port to connect to
        #[arg(short, long)]
        port: u16,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Listen { port } => {
            println!("Listening on port {}", port);
            let nc = Netcat::new("0.0.0.0".to_string(), port);
            nc.listen();
        }

        Command::Connect { host, port } => {
            println!("Connecting to {} on port {}", host, port);
            let nc = Netcat::new(host, port);
            nc.connect();
        }
    }
}
