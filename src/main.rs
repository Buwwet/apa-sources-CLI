mod lib;
mod renderer;


use apa::{Logic, ApaFormatType, ApaFormat};
use renderer::render;

use std::{time, thread, io::stdout, fmt::write, io::Write};
use termion::{input::TermRead, event::Key, raw::IntoRawMode};


fn main() {
    // Define container that houses all of the variables
    let mut logic = Logic::new();


    // Terminal
    let stdin = std::io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Write the top header.
    write!(stdout, "{}{}{}{}{}-- APA 7 CLI: choose the format --{} (←) left | (→) right | (Enter) choose{}{}",
        termion::cursor::Goto(1,1),
        termion::clear::AfterCursor,
        termion::color::Fg(termion::color::AnsiValue(7)),
        termion::style::Bold,
        termion::style::Invert,

        termion::cursor::Goto(1,2),

        termion::color::Bg(termion::color::Reset),
        termion::style::Reset,
    ).unwrap(); 

    stdout.flush().unwrap();
    
    // Render once to have a bit of content to show.
    render(&logic, &mut stdout);
    // This loops forever.
    for key in stdin.keys() {
        match key.as_ref().unwrap() {
            /* Universal Keys */
            // Quit key
            Key::Ctrl('c') => {
                // Leave raw mode and quit
                stdout.suspend_raw_mode().unwrap();

                // Quick fix to exit the program with the cursor at the bottom.
                logic.edit_state = false;
                render(&logic, &mut stdout);

                // Show cursor and print the finished apa format.
                writeln!(stdout, "{}", termion::cursor::Show).unwrap();
                stdout.flush().unwrap();

                std::process::exit(0);
            }
            _ => {}
        }

        /* APA editing mode */
        if !logic.selecting_format {
        match key.as_ref().unwrap() {
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
            Key::Char('d') if !logic.edit_state => {
                // Completely wipe the selected field.
                let apa_field = &mut logic.apa.data.get_mut(&logic.selected).unwrap();
                apa_field.1 = "".to_string();
            }

            /* Editing State */
            // Movement keys
            Key::Left if logic.edit_state && logic.cursor_pos != 0 => {
                logic.cursor_pos -= 1;
            }
            Key::Right if logic.edit_state => {
                // Get the field's length and check if it's bigger.
                let apa_field = logic.apa.data.get(&logic.selected).unwrap();
                if logic.cursor_pos < apa_field.1.len() {
                    logic.cursor_pos += 1;
                }
            }
            Key::Down if logic.edit_state && logic.selected < logic.apa.data.len() - 1  => {
                // Prevent selecting something that doesn't exist ^
                logic.selected += 1;

                // If current position is too large, switch to field's length
                let new_field_length = logic.apa.data.get(&logic.selected).unwrap().1.len();
                if logic.cursor_pos > new_field_length {
                    logic.cursor_pos = new_field_length;
                }
            }
            Key::Up if logic.edit_state && logic.selected != 0 => {
                // Prevent underflow ^
                logic.selected -= 1;

                // If current position is too large, switch to field's length
                let new_field_length = logic.apa.data.get(&logic.selected).unwrap().1.len();
                if logic.cursor_pos > new_field_length {
                    logic.cursor_pos = new_field_length;
                }
            }

            Key::Backspace if logic.edit_state && logic.cursor_pos != 0 => {
                // Prevent deleting nothing ^
                // Delete the last char from the string
                let apa_field = &mut logic.apa.data.get_mut(&logic.selected).unwrap();
                apa_field.1.remove(logic.cursor_pos - 1);

                // Update the character position.
                logic.cursor_pos -= 1;

            }
            Key::Char(c) if logic.edit_state => {
                // Append the character to the end of the field
                let apa_field = &mut logic.apa.data.get_mut(&logic.selected).unwrap();
                apa_field.1.insert(logic.cursor_pos, *c);

                // Update the character position.
                logic.cursor_pos += 1;
            }
            
            _ => {}
        };
        }

        /* APA selecting mode */
        if logic.selecting_format 
        {
            let format_num: usize = ApaFormatType::list().len();

            match key.as_ref().unwrap() {
                /* Selection Keys */
                Key::Left if logic.selected != 0 => {
                    logic.selected -= 1;
                }
                Key::Right if logic.selected < format_num - 1 => {
                    logic.selected += 1;
                }
                // Select the format and switch to editing mode
                Key::Char('\n') => {
                    logic.apa = ApaFormat::new(ApaFormatType::list()[logic.selected]);
                    logic.selected = 0;
                    logic.selecting_format = false;
                    // Write top header.
                    write!(stdout, "{}{}{}{}{}-- Current APA 7 format type: {}{}{} --{} (d) full delete | (Return) edit{}{}{}",
                        termion::cursor::Goto(1,1),
                        termion::clear::AfterCursor,
                        termion::color::Fg(termion::color::AnsiValue(7)),
                        termion::style::Bold,
                        termion::style::Invert,

                        termion::style::Italic,
                        logic.apa.format,
                        termion::style::NoItalic,

                        termion::cursor::Goto(1,2),

                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                        termion::style::Reset,
                    ).unwrap(); 

                    // Clear out the junk
                }
                _ => {}
            }
        }
        
        render(&logic, &mut stdout);
    };
}
