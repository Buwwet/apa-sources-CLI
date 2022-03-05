use std::io::{stdout, Write};

use apa::Logic;
use termion::{self, raw::IntoRawMode};

pub fn render(logic: &Logic) {
    let mut stdout = stdout().into_raw_mode().unwrap();


    print!("{}pos: {}. edit: {}", termion::cursor::Goto(1,1), logic.selected, logic.edit_state);
    
    // Update the terminal.
    stdout.flush().unwrap();
}