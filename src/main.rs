use std::io::{self, Read};

use terminal::Terminal;

mod terminal;

// TODO: Disable raw mode on exit

fn run_app(terminal: &Terminal) -> io::Result<()> {
    Ok(())
}

fn main() -> io::Result<()> {
    let terminal = match Terminal::setup() {
        Ok(result) => result,
        Err(error) => panic!("An error has ocurred {:?}\r\n", error),
    };

    println!("Running app!\r\n");
    std::process::exit(match run_app(&terminal) {
        Ok(_) => {
            Terminal::cleanup(&terminal)?;
            0
        }

        Err(error) => {
            panic!("An error has ocurred {:?}\r\n", error);
        }
    });
}
