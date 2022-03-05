mod lib;
mod renderer;


use apa::Logic;
use renderer::render;

use std::{time, thread, io::stdout, fmt::write, io::Write};
use termion::{input::TermRead, event::Key, raw::IntoRawMode};







fn main() {
    // Define container that houses all of the variables
    let mut logic = Logic::new();

    // Logic Loop

    // Terminal
    let stdin = std::io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    // This loops forever.
    for key in stdin.keys() {
        match key.unwrap() {
            // Quit key
            Key::Ctrl('c') => {
                // Leave raw mode and quit
                stdout.suspend_raw_mode().unwrap();
                std::process::exit(0);
            }

            // Switch editing mode
            Key::Insert => {
                logic.edit_state = true;
            }
            _ => {}
        };
        
        render(&logic, &mut stdout);
    };

}
