use std::{io::{stdout, Write, Stdout}, str::Bytes, ops::Range};

use crate::lib::{Logic, ApaFormatType, LogicState};


use termion::{self, raw::{IntoRawMode, RawTerminal}, event::Key, input::TermRead, color::Fg, terminal_size, cursor::DetectCursorPos};
use termion::cursor::Goto;
use termion::style;
use termion::color;

pub fn render(logic: &Logic, stdout: &mut RawTerminal<Stdout>, root_pos : (u16, u16)) {

    // Dynamically placed cursor. Affects the Goto of all printing.
    let mut cursor_pos = stdout.cursor_pos().unwrap();

    // Check if there's enough space for the program below.
    const PRINT_SIZE: u16 = 8;
    if cursor_pos.1 + PRINT_SIZE >= terminal_size().unwrap().1 {
        // There is not enough space, so we scroll up.
        write!(stdout, "{}", termion::scroll::Up(PRINT_SIZE)).unwrap();
        cursor_pos.1 -= PRINT_SIZE;
    }

    // Select format
    match logic.state { 
        LogicState::SelectingFormat => {
            let format_list = ApaFormatType::list();

        
        // Write the top header.
        write!(stdout, "{}{}{}{}-- APA 7 CLI: choose the format --{} (←) left | (→) right | [LANG: {:?}] {}{}",
            termion::cursor::Goto(1, root_pos.1),
            termion::color::Fg(termion::color::AnsiValue(7)),
            termion::style::Bold,
            termion::style::Invert,

            termion::cursor::Goto(1, 1 + root_pos.1),

            logic.apa.lang,

            termion::color::Bg(termion::color::Reset),
            termion::style::Reset,
        ).unwrap(); 
        

        write!(stdout, "{}Select a format: ",
            Goto(1, 2 + root_pos.1)
        ).unwrap();

        // For each format present, print it.
        for (i, format) in format_list.iter().enumerate() {
            write!(stdout, "{}{}{}, ",
                if i == logic.selected {format!("{}", style::Underline)} else {"".to_string()},
                format,
                style::NoUnderline,
            ).unwrap();
        }

    },
    // Add each field in the apa data, calculate which is the longest one
    LogicState::EditState => {

        // Write top header.
        write!(stdout, "{}{}{}{}-- Current APA 7 format type: {}{}{} --{} (d) full delete |  (Tab) switch state | (Enter) down | (down arrow) down. {}{}{}",
            termion::cursor::Goto(1, root_pos.1),
            termion::color::Fg(termion::color::AnsiValue(7)),
            termion::style::Bold,
            termion::style::Invert,

            termion::style::Italic,
            logic.apa.format,
            termion::style::NoItalic,

            termion::cursor::Goto(1, 1 + root_pos.1),

            termion::color::Bg(termion::color::Reset),
            termion::color::Fg(termion::color::Reset),
            termion::style::Reset,
        ).unwrap(); 

        let mut longest_field_name: usize = 0;
        // Draw each field fields
        for (i, apa_data) in logic.apa.data.iter() {
            // Draw field names
            write!(stdout, "{}{}{}{}",
                Goto(3, 2 + *i as u16 + root_pos.1),

                // Add color if selected

                //if logic.selected == *i && logic.edit_state { format!("{}", Fg(color::Yellow) ) } else { "".to_string() },
                //if logic.selected == *i && logic.edit_state { format!("{}", style::Invert) } else { "".to_string() },
                //if logic.selected == *i && logic.edit_state { format!("{}", style::Blink) } else { "".to_string() },

                apa_data.0,
                style::Reset,
                Fg(color::Reset)
                
            ).unwrap();

            // Draw selector icon
            write!(stdout, "{}{}",
                Goto(1, 2 + *i as u16 + root_pos.1),
                if logic.selected == *i {">"} else {" "}
            ).unwrap();

            // If this field is bigger than longest field, update it
            if apa_data.0.len() > longest_field_name {
                longest_field_name = apa_data.0.len();
            }
        }

        // Draw the field's contents.
        let mut longest_field: usize = 0;
        for (i, apa_data) in logic.apa.data.iter() {
            
            write!(stdout, "{} │{}{} {}{}",
                Goto(3 + longest_field_name as u16, 2 + *i as u16 + root_pos.1),

                // Lightup the beam of the selected field                
                Fg(color::Reset),
                style::Reset,

                termion::clear::UntilNewline,
                apa_data.1,
            ).unwrap();

            // Update the longest field if this one is longer.
            if apa_data.1.len() > longest_field {
                longest_field = apa_data.1.len();
            }
        }

        // Draw the bar that separates the fields and the link.
        for i in 1..longest_field + longest_field_name + 6 {
            // Go to position
            write!(stdout, "{}", 
                Goto(i as u16, logic.apa.data.len() as u16 + 2 + root_pos.1)
            ).unwrap();

            //write sytle
            match i {
                1 => {
                    // Add style
                    write!(stdout, "{}",
                        Fg(color::Reset),
                    ).unwrap();
                }
                30 => {
                    // Remove style
                    write!(stdout, "{}",
                        Fg(color::Reset),
                    ).unwrap();
                }
                _ => {}
            }

            // write letter
            match i {
                _ if i == longest_field_name + 4 => {
                    write!(stdout, "┴",
                    ).unwrap();
                }
                _ => {
                    write!(stdout, "─",
                    ).unwrap();
                }
            }
        }

        // Print link with more info on this type of apa format.
        write!(stdout, "{}More Info: {}{}{}{}{}{}",
            Goto(1, logic.apa.data.len() as u16 + 3 + root_pos.1),
            
            color::Fg(color::LightBlue),
            style::Underline,
            style::Italic,
            logic.apa.format.link(),
            color::Fg(color::Reset),
            style::Reset,
        ).unwrap();

        // Draw the "FINISHED" APA citation.
        write!(stdout, "{}{}APA reference:{}    {}{}",
            Goto(1, logic.apa.data.len() as u16 + 4 + root_pos.1),
            termion::clear::UntilNewline,

            Goto(1, logic.apa.data.len() as u16 + 5 + root_pos.1),
            logic.apa,
            termion::clear::AfterCursor,
        ).unwrap();
        
        // If in edit mode, move cursor at the end.
        if logic.edit_state {
            write!(stdout, "{}{}", 
                termion::cursor::Show,
                Goto(
                    6 + logic.cursor_pos as u16 + longest_field_name as u16,
                    2 + logic.selected as u16 + root_pos.1
                )
            ).unwrap()
        } else {
            write!(stdout, "{}", termion::cursor::Hide).unwrap()
        }
    },
    LogicState::Result => {
        // Simple result screen.

        // Update the header.
        write!(stdout, "{}{}{}{}{}-- Current APA 7 format type: {}{}{} --{} Press (Control + C) to exit.{}{}{}",
            termion::cursor::Goto(1, root_pos.1),
            termion::color::Fg(termion::color::AnsiValue(7)),
            termion::clear::AfterCursor,
            termion::style::Bold,
            termion::style::Invert,

            termion::style::Italic,
            logic.apa.format,
            termion::style::NoItalic,

            termion::cursor::Goto(1, 1 + root_pos.1),

            termion::color::Bg(termion::color::Reset),
            termion::color::Fg(termion::color::Reset),
            termion::style::Reset,
        ).unwrap(); 

        // Write the apa reference.
        write!(stdout, "{}{}Finished APA reference:{}    {}{}",
            Goto(1, 2 + root_pos.1),
            termion::clear::UntilNewline,

            Goto(1, 3 + root_pos.1),
            logic.apa,
            termion::clear::AfterCursor,
        ).unwrap();
    }

}

    // Update the terminal.
    stdout.flush().unwrap();
}