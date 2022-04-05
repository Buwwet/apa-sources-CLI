mod lib;
mod renderer;


use apa::{Logic, ApaFormatType, ApaFormat, save_to_x11_clipboard};
use renderer::render;
use unicode_segmentation::UnicodeSegmentation;

use std::{time, thread, io::stdout, fmt::write, io::Write, process::{Command, Stdio}};
use termion::{input::TermRead, event::Key, raw::IntoRawMode, cursor::DetectCursorPos, terminal_size};


fn main() {
    // Define container that houses all of the variables
    let mut logic = Logic::new();

    // Terminal
    let stdin = std::io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Dynamically placed cursor. Affects the Goto of all printing.
    let mut cursor_pos = stdout.cursor_pos().unwrap();

    // Check if there's enough space for the program below.
    const PRINT_SIZE: u16 = 14;
    if cursor_pos.1 + PRINT_SIZE >= terminal_size().unwrap().1 {
        // There is not enough space, so we scroll up.
        write!(stdout, "{}", termion::scroll::Up(PRINT_SIZE)).unwrap();
        cursor_pos.1 -= PRINT_SIZE;
    }
    
    // Write the top header.
    write!(stdout, "{}{}{}{}{}-- APA 7 CLI: choose the format --{} (←) left | (→) right | (Enter) choose{}{}",
        termion::cursor::Goto(1, cursor_pos.1),
        termion::clear::CurrentLine,
        termion::color::Fg(termion::color::AnsiValue(7)),
        termion::style::Bold,
        termion::style::Invert,

        termion::cursor::Goto(1, 1 + cursor_pos.1),

        termion::color::Bg(termion::color::Reset),
        termion::style::Reset,
    ).unwrap(); 
    stdout.flush().unwrap();
    
    // Render once to have a bit of content to show.
    render(&logic, &mut stdout, cursor_pos);
    // This loops forever and houses the logic of the program.
    for key in stdin.keys() {
        match key.as_ref().unwrap() {
        /* Universal Keys */
            // Quit key
            Key::Ctrl('c') => {
                // Leave raw mode and quit
                stdout.suspend_raw_mode().unwrap();

                // Quick fix to exit the program with the cursor at the bottom.
                logic.edit_state = false;
                render(&logic, &mut stdout, cursor_pos);

                // Show cursor
                writeln!(stdout, "{}", termion::cursor::Show).unwrap();
                stdout.flush().unwrap();

                // Copy the apa to the clipboard x11;
                save_to_x11_clipboard(logic.apa);

                std::process::exit(0);
            }
            _ => {}
        }

    /* APA editing mode */
        if !logic.selecting_format {
        match key.as_ref().unwrap() {
            // Switch editing mode
            Key::Char('\n') | Key::Char('\t') => {
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
            // Editing the text present using the cursor position and char.
            Key::Backspace if logic.edit_state && logic.cursor_pos != 0 => {
                // Prevent deleting nothing ^
                // Delete the last char from the string
                let apa_field = &mut logic.apa.data.get_mut(&logic.selected).unwrap();
                
                // Firstly, divide the apa field to graphene_clusters
                let mut graphene_fields = UnicodeSegmentation::graphemes(apa_field.1.as_str(), true).collect::<Vec<&str>>();
                // Remove the character found before the cursor.
                graphene_fields.remove(logic.cursor_pos - 1);
                
                // Update the apa field with the remaining clusters.
                apa_field.1 = graphene_fields.iter()
                    .map(|cluster| {cluster.to_string()})
                    .collect();


                // Update the character position.
                logic.cursor_pos -= 1;

            }
            Key::Char(c) if logic.edit_state => {
                // Append the character to the end of the field
                let apa_field = &mut logic.apa.data.get_mut(&logic.selected).unwrap();

                // Divde the apa format to graphene fields
                let mut graphene_fields = UnicodeSegmentation::graphemes(apa_field.1.as_str(), true).collect::<Vec<&str>>();
                
                // Define the character we will insert into a &str
                let character: &str = &c.to_string();
                // Insert the character where the cursor is located.
                graphene_fields.insert(logic.cursor_pos, character);
                
                let new_cursor_pos = graphene_fields.len();

                // Update the field with the graphene_fields turned into a String from a Vec<&str>.
                apa_field.1 = graphene_fields.iter()
                    .map(|cluster| {cluster.to_string()})
                    .collect();

                // Update the character position.
                logic.cursor_pos = new_cursor_pos;
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
                        termion::cursor::Goto(1, cursor_pos.1),
                        termion::clear::AfterCursor,
                        termion::color::Fg(termion::color::AnsiValue(7)),
                        termion::style::Bold,
                        termion::style::Invert,

                        termion::style::Italic,
                        logic.apa.format,
                        termion::style::NoItalic,

                        termion::cursor::Goto(1, 1 + cursor_pos.1),

                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                        termion::style::Reset,
                    ).unwrap(); 
                }
                _ => {}
            }
        }
        
        render(&logic, &mut stdout, cursor_pos);
    };
}
