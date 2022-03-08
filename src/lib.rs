use std::fmt;
use std::{collections::HashMap, slice::Iter};

use termion::style;

#[derive(Clone, Copy)]
pub enum ApaFormatType {
    None,
    Website,
    Newspaper,
    Dictionary,
}
impl ApaFormatType {
    // Used for the generation of the format list.
    pub fn list() -> [ApaFormatType; 3] {
        [ApaFormatType::Website, ApaFormatType::Newspaper, ApaFormatType::Dictionary]
    }
    pub fn link(&self) -> &'static str {
        // Provide the link with more information about the format.
        match self {
            Self::Website => "https://www.scribbr.com/apa-examples/website/",
            Self::Newspaper => "https://www.scribbr.com/apa-examples/website/",
            Self::Dictionary =>  "https://www.scribbr.com/apa-examples/website/",
            Self::None => "",
        }
    }
}

impl fmt::Display for ApaFormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // display the name for the format list.
            Self::Website => write!(f, "webpage"),
            Self::Newspaper => write!(f, "newspaper article"),
            Self::Dictionary => write!(f, "dictionary entry"),
            Self::None => write!(f, "none"),
        }
    }
}

pub struct ApaFormat {
    // An apa format contains its type and a hashmap with fields representing
    // its parts.
    pub format: ApaFormatType,
    pub data: HashMap<usize, (String, String)>,
}
impl ApaFormat {
    pub fn new(format: ApaFormatType) -> ApaFormat {
        // Creates an empty version of the apa format.
        let mut data = HashMap::new();

        match format {
            // Each format has a different amount and types of fields.
            ApaFormatType::Website => {
                data.insert(0, ("authors".to_string(), "".to_string()));
                data.insert(1, ("date".to_string(), "".to_string()));
                data.insert(2, ("title".to_string(), "".to_string()));
                data.insert(3, ("website".to_string(), "".to_string()));
                data.insert(4, ("URL".to_string(), "".to_string()));
            }
            ApaFormatType::Newspaper => {
                data.insert(0, ("authors".to_string(), "".to_string()));
                data.insert(1, ("date".to_string(), "".to_string()));
                data.insert(2, ("title".to_string(), "".to_string()));
                data.insert(3, ("newspaper".to_string(), "".to_string()));
                data.insert(4, ("URL".to_string(), "".to_string()));
            }
            ApaFormatType::Dictionary => {
                data.insert(0, ("authors".to_string(), "".to_string()));
                data.insert(1, ("date".to_string(), "".to_string()));
                data.insert(2, ("word".to_string(), "".to_string()));
                data.insert(3, ("editors".to_string(), "".to_string()));
                data.insert(4, ("dictionary".to_string(), "".to_string()));
                data.insert(5, ("publisher".to_string(), "".to_string()));
                data.insert(6, ("URL".to_string(), "".to_string()));
            }
            ApaFormatType::None => {}
        };

        ApaFormat { format, data }
    }
}
// Fit everything into the format.
impl fmt::Display for ApaFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.format {
            ApaFormatType::None => {
                write!(f, "none")
            }
            // Defines how each apa format is structured and 
            ApaFormatType::Website => {
                // Get all of the fields.
                let authors = self.data.get(&0).unwrap();
                let date = self.data.get(&1).unwrap();
                let title = self.data.get(&2).unwrap();
                let publisher = self.data.get(&3).unwrap();
                let URL = self.data.get(&4).unwrap();

                write!(
                    f,
                    "{}. ({}). /{}/. {}. {}",
                    if &authors.1 != "" { &authors.1 } else {"Author's Last Name, Initial(s)"},
                    // If date is not found, add n.d
                    if &date.1 != "" { &date.1 } else { "n.d." },
                    if &title.1 != "" { &title.1 } else { "Title of work"},
                    if &publisher.1 != "" { &publisher.1 } else { "Website" },
                    URL.1,
                )
            }
            ApaFormatType::Newspaper => {
                // Get all of the fields.
                let authors = self.data.get(&0).unwrap();
                let date = self.data.get(&1).unwrap();
                let title = self.data.get(&2).unwrap();
                let newspaper = self.data.get(&3).unwrap();
                let URL = self.data.get(&4).unwrap();

                write!(
                    f,
                    "{}. ({}). {}. /{}/. {}",
                    // If text is not present, fill with tooltip.
                    if &authors.1 != "" {&authors.1} else { "Author's Last Name, Initial(s)" },
                    if &date.1 != "" { &date.1 } else { "n.d." },
                    if &title.1 != "" { &title.1 } else { "Title of article" },

                    if &newspaper.1 != "" { &newspaper.1 } else { "Newspaper" },

                    URL.1
                )
            }
            ApaFormatType::Dictionary => {
                // Get all of the fields.
                let authors = self.data.get(&0).unwrap();
                let date = self.data.get(&1).unwrap();
                let word = self.data.get(&2).unwrap();
                let editors = self.data.get(&3).unwrap();
                let dictionary = self.data.get(&4).unwrap();
                let publisher = self.data.get(&5).unwrap();
                let URL = self.data.get(&6).unwrap();

                write!(f,
                    "{}. ({}). {}. In {} (Ed.). /{}/. {}. {}",
                    // If text is not present, fill with tooltip.
                    if &authors.1 != "" {&authors.1} else { "Author's Last Name, Initial(s)" },
                    if &date.1 != "" {&date.1} else { "n.d." },
                    if &word.1 != "" {&word.1} else { "Word" },
                    if &editors.1 != "" {&editors.1} else { "Initial(s). Last Name" },
                    if &dictionary.1 != "" {&dictionary.1} else { "Dictionary" },
                    if &publisher.1 != "" {&publisher.1} else { "Publisher" },
                    URL.1
                )
            }
        }
    }
}

// Base logic of the program
// Used by main to change the logic state for the renderer in a loop.
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
            apa: ApaFormat::new(ApaFormatType::None),
        }
    }
}
