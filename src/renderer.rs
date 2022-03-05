use std::{io::{stdout, Write, Stdout}, str::Bytes};

use apa::Logic;
use termion::{self, raw::{IntoRawMode, RawTerminal}, event::Key, input::TermRead};

pub fn render(logic: &Logic, stdout: &mut RawTerminal<Stdout>) {

    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}{}", termion::cursor::Goto(1,1), 2).unwrap();

    
    // Update the terminal.
    stdout.flush().unwrap();
}