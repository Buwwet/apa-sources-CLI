use std::collections::HashMap;
use std::fmt;
pub enum ApaFormatType {
    Website
}

impl fmt::Display for ApaFormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Website => write!(f, "website")
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
                    data.insert(0, ("authors".to_string(), "Barack Obama".to_string()));
                    data.insert(1, ("date".to_string(), "".to_string()));
                    data.insert(2, ("title".to_string(), "How to code".to_string()));
                    data.insert(3, ("publisher".to_string(), "".to_string()));
                    data.insert(4, ("URL".to_string(), "wikipedia.com".to_string())); 
                }
        };

        ApaFormat {
            format,
            data
        }
    }
}

// Base logic of the program
pub struct Logic {
    pub edit_state: bool,
    pub selected: usize,
    pub cursor_pos: usize,
    pub apa: ApaFormat,
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            edit_state: false,
            selected: 0,
            cursor_pos: 0,
            // TODO: create a menu to choose.
            apa: ApaFormat::new(ApaFormatType::Website),
        }
    }
}