use std::{io, os::unix::prelude::AsRawFd};

use termios::*;

pub struct Terminal {
    original: Termios,
    pub raw: Box<Termios>,
}

impl Terminal {
    pub fn setup() -> io::Result<Self> {
        let termios = match enable_raw_mode() {
            Ok(termios) => termios,
            Err(..) => panic!("Error opening raw mode"),
        };

        let raw = Box::new(termios);

        Ok(Terminal {
            original: *raw,
            raw,
        })
    }

    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn cleanup(&self) -> io::Result<()> {
        println!("Cleaning up\r\n");
        tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &self.original)?;

        Ok(())
    }
}

fn enable_raw_mode() -> io::Result<termios::Termios> {
    match Termios::from_fd(io::stdin().as_raw_fd()) {
        Ok(mut termios) => {
            let result = match configure_terminal(&mut termios) {
                Ok(configured_terminal) => configured_terminal,
                Err(..) => panic!("Could not configure terminal\r\n"),
            };

            println!("Enabled raw mode\r\n");
            Ok(result)
        }

        Err(error) => panic!("An error has ocurred: {:?}\r\n", error),
    }
}

fn configure_terminal(termios: &mut termios::Termios) -> io::Result<termios::Termios> {
    println!("Configuring terminal\r\n");
    tcgetattr(io::stdin().as_raw_fd(), termios)?;

    termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    termios.c_oflag &= !(OPOST);
    termios.c_cflag |= CS8;
    termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);

    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;

    tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, termios)?;

    Ok(*termios)
}
