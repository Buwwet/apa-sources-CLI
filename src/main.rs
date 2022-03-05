mod lib;
mod renderer;


use apa::Logic;
use renderer::render;

use std::{time, thread};
use termion::{input::TermRead, event::Key};






fn main() {
    let mut logic = Logic::new();



    // Logic Loop
    loop {
        // Get all inputs

        
        let stdin = std::io::stdin();

        /* 
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Ctrl('c') => {
                    std::process::exit(0);
                }
                _ => {}
            };
        };
        */

        
        
        render(&logic);
        thread::sleep(time::Duration::from_millis(1))
    }
}
