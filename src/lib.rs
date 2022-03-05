use std::{collections::HashMap, slice::Iter};
use std::fmt;

use termion::style;

#[derive(Clone, Copy)]
pub enum ApaFormatType {
    None,
    Website
}
impl ApaFormatType {
    pub fn list() -> [ApaFormatType; 2] {
        [ApaFormatType::Website, ApaFormatType::None]
    }
    pub fn link(&self) -> &'static str {
        // Provide the link with more information.
        match self {
            Self::Website => "https://www.scribbr.com/apa-examples/website/",
            Self::None => "",
        }
    }
}

impl fmt::Display for ApaFormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Website => write!(f, "webpage"),
            Self::None => write!(f, "none"),
        }
    }
}

pub struct ApaFormat {
    pub format: ApaFormatType,
    pub data: HashMap<usize, (String, String)>,
}
impl ApaFormat {
    pub fn new(format: ApaFormatType) -> ApaFormat {
        // Creates an empty version of the apa format.
        let mut data = HashMap::new();
        
        match format {
            ApaFormatType::Website => {
                //TODO: remove answered fields
                    data.insert(0, ("authors".to_string(), "".to_string()));
                    data.insert(1, ("date".to_string(), "".to_string()));
                    data.insert(2, ("title".to_string(), "".to_string()));
                    data.insert(3, ("website".to_string(), "".to_string()));
                    data.insert(4, ("URL".to_string(), "".to_string())); 
                }
            ApaFormatType::None => {}
        };

        ApaFormat {
            format,
            data
        }
    }
}
// Fit everything into the format.
impl fmt::Display for ApaFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self.format {
            ApaFormatType::None => {
                write!(f, "none")
            }
            ApaFormatType::Website => {
                // Get all of the fields.
                let authors = self.data.get(&0).unwrap();
                let date = self.data.get(&1).unwrap();
                let title = self.data.get(&2).unwrap();
                let publisher = self.data.get(&3).unwrap();
                let URL = self.data.get(&4).unwrap();

                write!(f, "{}. ({}). {}{}{}. {}. {}",
                    authors.1,
                    // If date is not found, add n.d
                    if &date.1 != "" {&date.1} else {"n.d."},

                    style::Italic,
                    title.1,
                    style::NoItalic,

                    publisher.1,
                    URL.1,
                )
            }
        }
    }
}


// Base logic of the program
pub struct Logic {
    /* APA format selector */
    pub selecting_format: bool,

    /* APA editor */
    pub edit_state: bool,
    pub selected: usize,
    pub cursor_pos: usize,

    pub apa: ApaFormat,
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            selecting_format: true,
            edit_state: false,
            selected: 0,
            cursor_pos: 0,
            // TODO: create a menu to choose.
            apa: ApaFormat::new(ApaFormatType::None),
        }
    }
}