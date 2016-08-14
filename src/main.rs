extern crate argparse;
use std::str::FromStr;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::time::Duration;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::channel;
struct Pomodoro {
    message: String,
    time: u64,
}
fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    let seconds_left = seconds - hours * 3600 - minutes * 60;
    format!("{}:{}:{}", hours, minutes, seconds_left)
}
impl Pomodoro {
    fn run(&self) {
        let (tx, rx) = channel();
        let (tk, rk) = channel();
        let seconds = self.time * 60;
        println!("{}", self.message.clone());
        let _ = thread::spawn(move|| {
            for i in (1..seconds+1).rev() {
                if let Ok(_) = rx.try_recv() {
                    if let Err(_) = rx.recv() {
                        panic!("Message stack corrupted!");
                    }
                }
                let mut stdout = io::stdout();
                let _ = write!(stdout, "{}[K{}\r", 27 as char, format_time(i));
                io::stdout().flush().ok().expect("Could not flush stdout");
                thread::sleep(Duration::from_secs(1));
            };
            println!("Press Enter to continue...");
            let _ = tk.send(());
        });
        let mut paused = false;
        loop {
            // Read a single byte and discard
            let stdin = io::stdin();
            let _ = stdin.lock().read_line(&mut String::new());
            print!("{0}[1A", 27 as char);
            let _ = tx.send(());
            paused = !paused;
            if let Ok(_) = rk.try_recv() {
                break;
            }
        }
    }
}
fn main() {
    let work_pomodoro = Pomodoro {
        message: String::from_str("Back To Work").unwrap(),
        time: 25
    };
    let sb_pomodoro = Pomodoro {
        message: String::from_str("Break Start").unwrap(),
        time: 4
    };
    let lb_pomodoro = Pomodoro {
        message: String::from_str("Take a Long Break").unwrap(),
        time: 15
    };
    loop {
        work_pomodoro.run();
        sb_pomodoro.run();

        work_pomodoro.run();
        sb_pomodoro.run();

        work_pomodoro.run();
        sb_pomodoro.run();

        work_pomodoro.run();
        lb_pomodoro.run();
    };
    // loop {
    //     timer(25);
    //     timer(3);
    //     timer(25);
    //     timer(3);
    //     timer(25);
    //     timer(3);
    //     timer(25);
    //     timer(15);
    // }
}
/*
extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
let mut verbose = false;
let mut name = "World".to_string();
{  // this block limits scope of borrows by ap.refer() method
let mut ap = ArgumentParser::new();
ap.set_description("Greet somebody.");
ap.refer(&mut verbose)
.add_option(&["-v", "--verbose"], StoreTrue,
"Be verbose");
ap.refer(&mut name)
.add_option(&["--name"], Store,
"Name for the greeting");
ap.parse_args_or_exit();
    }

    if verbose {
        println!("name is {}", name);
    }
    println!("Hello {}!", name);
}
*/
