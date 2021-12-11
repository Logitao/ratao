use std::{
    io::{self, BufRead, Read},
    process::exit,
};

use terminal::Terminal;

mod input;
mod terminal;

enum Modes {
    Command,
    Insert,
    Normal,
}

fn run_app(terminal: &Terminal) -> io::Result<()> {
    let mut buffer = String::new();
    let mut command_buffer = String::new();
    let mut mode = Modes::Insert;

    loop {
        for result_bytes in io::stdin().bytes() {
            match result_bytes {
                Ok(byte) => match mode {
                    Modes::Insert => match byte {
                        17 => {
                            terminal.clear_screen();
                            print!("Exiting!\r\n----------\r\n");
                            print!("\r\n----------\r\n");
                            exit(0)
                        }

                        58 => {
                            mode = Modes::Command;

                            terminal.clear_screen();
                            print!("Entering command mode!\r\n----------\r\n");
                            print!("\r\n----------\r\n");
                        }
                        _ => {
                            terminal.clear_screen();
                            buffer.push(byte as char);
                            print!("{}\r\n", buffer);
                        }
                    },
                    Modes::Command => match byte {
                        13 => {
                            mode = Modes::Insert;

                            terminal.clear_screen();

                            print!("Entering normal mode!\r\n----------\r\n");
                            print!("\r\n----------\r\n");

                            command_buffer.clear();
                        }
                        _ => {
                            terminal.clear_screen();

                            print!("Entering normal mode!\r\n----------\r\n");
                            print!("\r\n----------\r\n");
                            command_buffer.push(byte as char);
                            print!("{}\r\n", command_buffer);
                        }
                    },
                    Modes::Normal => todo!(),
                },
                Err(_) => print!("error reading byte!\r\n"),
            }
        }
    }
    // print!("{}\r\n", buffer);
}

fn main() -> io::Result<()> {
    let terminal = match Terminal::setup() {
        Ok(result) => result,
        Err(error) => panic!("An error has ocurred {:?}\r\n", error),
    };

    print!("Running app!\r\n----------\r\n");
    std::process::exit(match run_app(&terminal) {
        Ok(_) => {
            Terminal::cleanup(&terminal)?;
            0
        }

        Err(_) => 1,
    });
}
