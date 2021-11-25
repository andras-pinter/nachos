use std::io::{BufRead, BufReader};
use std::process::{exit, Stdio, Command};

const WORKDIR: &str = "guacamole-server";

macro_rules! run {
    ($command:tt, $($arg:tt),+) => {
        {
            let mut proc = Command::new($command);
            $(
                let proc = proc.arg($arg);
            )+
            let mut proc = proc.current_dir(WORKDIR)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to spawn command");
            let out = BufReader::new(proc.stdout.take().unwrap());
            out.lines().for_each(|line| if let Ok(line) = line {
                println!("{}", line);
            });

            proc.wait()
        }
    };
}

fn main() {
    println!("cargo:rustc-link-search={}", "../dist");
    println!("cargo:rerun-if-changed={}", WORKDIR);

    println!("Configuring guacamole-server");
    if let Err(e) = run!("autoreconf", "-fi") {
        eprintln!("Error: {}", e);
        exit(1)
    }
}
