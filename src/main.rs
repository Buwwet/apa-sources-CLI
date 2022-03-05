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

    // Write top header.
    write!(stdout, "{}{}{}{}-- Current APA format type: {}{}{} -- {}{}",
        termion::clear::All,
        termion::cursor::Goto(1,1),
        termion::color::Bg(termion::color::Rgb(120,120,120)),
        termion::color::Fg(termion::color::White),
        termion::style::Italic,
        logic.apa.format,
        termion::style::NoItalic,

        termion::color::Bg(termion::color::Reset),
        termion::color::Fg(termion::color::Reset),
    ).unwrap(); 

    stdout.flush().unwrap();
    
    // This loops forever.
    for key in stdin.keys() {
        match key.unwrap() {
            /* Universal Keys */
            // Quit key
            Key::Ctrl('c') => {
                // Leave raw mode and quit
                stdout.suspend_raw_mode().unwrap();
                std::process::exit(0);
            }
            // Switch editing mode
            Key::Char('\n') => {
                logic.edit_state = !logic.edit_state;
                // Set the cursor position.
                logic.cursor_pos = logic.apa.data.get(&logic.selected).unwrap().1.len();
                
            }
            /* Selecting Field State */
            Key::Down if !logic.edit_state && logic.selected < logic.apa.data.len() - 1  => {
                // Prevent selecting something that doesn't exist ^
                logic.selected += 1;
            }
            Key::Up if !logic.edit_state && logic.selected != 0 => {
                // Prevent underflow ^
                logic.selected -= 1;
            }
            /* Editing State */
            

            Key::Char(c) => {
                
            }

            _ => {}
        };
        
        render(&logic, &mut stdout);
    };
}
