use std::collections::HashMap;
pub enum ApaFormatType {
    Website
}

pub struct ApaFormat {
    pub format: ApaFormatType,
    pub data: HashMap<String, String>,
}

impl ApaFormat {
    pub fn new(format: ApaFormatType) -> ApaFormat {
        // Creates an empty version of the apa format.
        let mut data = HashMap::new();
        
        match format {
            ApaFormatType::Website => {
                    data.insert("authors".to_string(), "".to_string());
                    data.insert("date".to_string(), "".to_string());
                    data.insert("title".to_string(), "".to_string());
                    data.insert("publisher".to_string(), "".to_string());
                    data.insert("URL".to_string(), "".to_string()); 
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
    pub apa: ApaFormat,
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            edit_state: false,
            selected: 0,
            // TODO: create a menu to choose.
            apa: ApaFormat::new(ApaFormatType::Website),
        }
    }
}