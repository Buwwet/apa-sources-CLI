use std::{io::{stdout, Write, Stdout}, str::Bytes, ops::Range};

use apa::{Logic, ApaFormatType};
use termion::{self, raw::{IntoRawMode, RawTerminal}, event::Key, input::TermRead, color::Fg};
use termion::cursor::Goto;
use termion::style;
use termion::color;

pub fn render(logic: &Logic, stdout: &mut RawTerminal<Stdout>, root_pos : (u16, u16)) {
    // Select format
    if logic.selecting_format {
        let format_list = ApaFormatType::list();

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

    }

    // Add each field in the apa data, calculate which is the longest one
    if !logic.selecting_format {
        let mut longest_field: usize = 0;
        for (i, apa_data) in logic.apa.data.iter() {
            // Draw field names
            write!(stdout, "{}{}{}{}{}{}{}",
                Goto(3, 2 + *i as u16 + root_pos.1),

                if logic.selected == *i && logic.edit_state { format!("{}", Fg(color::Yellow) ) } else { "".to_string() },
                if logic.selected == *i && logic.edit_state { format!("{}", style::Invert) } else { "".to_string() },
                if logic.selected == *i && logic.edit_state { format!("{}", style::Blink) } else { "".to_string() },

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
            if apa_data.0.len() > longest_field {
                longest_field = apa_data.0.len();
            }
        }

        // Draw the field's contents.
        const DISTANCE_COLOR: [u8; 3] = [1, 14, 4];
        for (i, apa_data) in logic.apa.data.iter() {
            
            // Simulate lighting based on the distance to the distance from the cursor for the |
            let mut lighting_value: usize = (logic.selected as i32 - *i as i32).abs() as usize;
            if lighting_value >= DISTANCE_COLOR.len() { lighting_value = DISTANCE_COLOR.len() - 1 }


            write!(stdout, "{} {}{}│{}{} {}{}",
                Goto(3 + longest_field as u16, 2 + *i as u16 + root_pos.1),

                style::Bold,
                Fg(color::AnsiValue(DISTANCE_COLOR[lighting_value])),
                Fg(color::Reset),
                style::NoBold,

                termion::clear::UntilNewline,
                apa_data.1,
            ).unwrap();
        }

        // Draw the bar that separates the fields and link.
        for i in 1..30 {
            // Go to position
            write!(stdout, "{}", 
                Goto(i as u16, logic.apa.data.len() as u16 + 2 + root_pos.1)
            ).unwrap();

            //write sytle
            match i {
                1 => {
                    // Add style
                    write!(stdout, "{}",
                        Fg(color::LightCyan),
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
                _ if i == longest_field + 4 => {
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
                    6 + logic.cursor_pos as u16 + longest_field as u16,
                    2 + logic.selected as u16 + root_pos.1
                )
            ).unwrap()
        } else {
            write!(stdout, "{}", termion::cursor::Hide).unwrap()
        }
    }

    // Update the terminal.
    stdout.flush().unwrap();
}