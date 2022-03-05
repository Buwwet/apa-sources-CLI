use std::{io::{stdout, Write, Stdout}, str::Bytes};

use apa::Logic;
use termion::{self, raw::{IntoRawMode, RawTerminal}, event::Key, input::TermRead};
use termion::cursor::Goto;

pub fn render(logic: &Logic, stdout: &mut RawTerminal<Stdout>) {

    write!(stdout, "{}{}", termion::cursor::Goto(1,2), logic.edit_state).unwrap();

    // Add each field in the apa data, calculate which is the longest one
    let mut longest_field: usize = 0;
    for (i, apa_data) in logic.apa.data.iter() {
        // Draw field names
        write!(stdout, "{}{}",
            Goto(3, 3 + *i as u16),
            apa_data.0,
        ).unwrap();

        // Draw selector icon
        write!(stdout, "{}{}",
            Goto(1, 3 + *i as u16),
            if logic.selected == *i {">"} else {" "}
        ).unwrap();

        // If this field is bigger than longest field, update it
        if apa_data.0.len() > longest_field {
            longest_field = apa_data.0.len();
        }
    }

    // Draw the field's contents.
    for (i, apa_data) in logic.apa.data.iter() {
        write!(stdout, "{} | {}{}",
            Goto(3 + longest_field as u16, 3 + *i as u16),
            termion::clear::UntilNewline,
            apa_data.1,
        ).unwrap();

        
        
    }

    // Draw the "FINISHED" APA citation.
    write!(stdout, "{} APA reference:\n{}",
        Goto(1, logic.apa.data.len() as u16 + 3),
        logic.apa
    ).unwrap();
    
    // If in edit mode, move cursor at the end.
    if logic.edit_state {
        write!(stdout, "{}{}", 
            termion::cursor::Show,
            Goto(
                6 + logic.cursor_pos as u16 + longest_field as u16,
                3 + logic.selected as u16
            )
        ).unwrap()
    } else {
        write!(stdout, "{}", termion::cursor::Hide).unwrap()
    }

    // Update the terminal.
    stdout.flush().unwrap();
}