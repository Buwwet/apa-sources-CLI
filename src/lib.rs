use std::{fmt, thread, time::Duration};
use std::{collections::HashMap, slice::Iter};

use termion::style;
use x11_clipboard::Clipboard;

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

    //TODO
    pub placeholders: HashMap<usize, (String, String)>,
}
impl ApaFormat {
    pub fn new(format: ApaFormatType) -> ApaFormat {
        // Creates an empty version of the apa format.
        let mut data: HashMap<usize, (String, String)> = HashMap::new();
        let mut placeholders: HashMap<usize, (String, String)> = HashMap::new();

        match format {
            // Each format has a different amount and types of fields.
            ApaFormatType::Website => {
                const fields: &'static [&'static str] = &["authors","date", "title", "website", "URL"];
                const field_placeholders: &'static [&'static str] = &["Author's Last Name, Initial(s)","date", "Title the article", "Website", "URL"];
                data.fill_with_fields(fields, None);
                placeholders.fill_with_fields(fields, Some(field_placeholders));
            }
            ApaFormatType::Newspaper => {
                const fields: &'static [&'static str] = &["authors","date", "title", "newspaper", "URL"];
                const field_placeholders: &'static [&'static str] = &["Author's Last Name, Initial(s)","date", "Title the article", "Newspaper", "URL"];
                data.fill_with_fields(fields, None);
                placeholders.fill_with_fields(fields, Some(field_placeholders));
            }
            ApaFormatType::Dictionary => {
                const fields: &'static [&'static str] = &["authors", "date", "word", "editors", "dictionary", "publisher", "URL"];
                const field_placeholders: &'static [&'static str] = &["Author's Last Name, Initial(s)","date", "Word", "Editors's Initial(s). Last Name", "Dictionary", "Publisher", "URL"];
                data.fill_with_fields(fields, None);
                placeholders.fill_with_fields(fields, Some(field_placeholders));
            }
            ApaFormatType::None => {}
        };

        ApaFormat { format, data, placeholders }
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
                // Here's the format.
                let reference = "authors. (date). <i>title</i>. website. URL".to_string();

                // We replace the field's names in the string of the data.
                // We then add the placeholders if it wasn't modified.
                let reference = replace_string_contents(reference, &self.data, &self.placeholders);

                write!(f, "{}", reference)
            },
            ApaFormatType::Newspaper => {
                // Here's the format.
                let reference = "authors. (date). <i>title</i>. newspaper. URL".to_string();
                // We replace the field's names in the string of the data.
                // We then add the placeholders if it wasn't modified.
                let reference = replace_string_contents(reference, &self.data, &self.placeholders);

                write!(f, "{}", reference)
            }
            ApaFormatType::Dictionary => {
                // Here's the format.
                let reference = "authors. (date). word. In editors (Ed.). <i>dictionary</i>. publisher. URL".to_string();
                // We replace the field's names in the string of the data.
                // We then add the placeholders if it wasn't modified.
                let reference = replace_string_contents(reference, &self.data, &self.placeholders);

                write!(f, "{}", reference)
            }
        }
    }
}

// Replace the contents of a string using a Hashmap.
pub fn replace_string_contents(string: String, data: &HashMap<usize, (String, String)>, placeholders: &HashMap<usize, (String, String)>) -> String {
    let mut output = string;

    for (i, field) in data {
        // Check that the field is valid.
        if field.1 != "" {
            // Find and replace the field names to the field contents.
            output = output.replace(&field.0, &field.1);
        } else {
            // Insert placeholder if field data is empty.
            output = output.replace(&placeholders[i].0, &placeholders[i].1);
        }
    }

    return output;

}

// Base logic of the program
// Used by main to change the logic state for the renderer in a loop.
pub struct Logic {
    pub state: LogicState,
    /* APA editor */
    pub edit_state: bool,
    pub selected: usize,
    pub cursor_pos: usize,

    pub apa: ApaFormat,
}

#[derive(PartialEq, Eq)]
pub enum LogicState {
    /* APA format selector */
    SelectingFormat,
    /* APA editor */
    EditState,
    /* APA copying state (process must be alive to save to clipboard.) */
    Result
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            state: LogicState::SelectingFormat,
            edit_state: true,
            selected: 0,
            cursor_pos: 0,
            apa: ApaFormat::new(ApaFormatType::None),
        }
    }
}

pub fn save_to_x11_clipboard(clipboard: &Clipboard, format_apa: &ApaFormat) {
    // Create clipboard
    //TODO: MAYBE IT DOESN'T WORK BECAUSE THE CLIPBOARD GOES OUT OF SCOPE INSTANTLY ANYWAYS:
    // FIX BY CREATING THE CLIPBOARD IN MAIN.

    clipboard.store(
        //Where?
        clipboard.getter.atoms.clipboard, 
        // Determine format.
        clipboard.getter.get_atom("text/html").unwrap(), 
        format!("<meta http-equiv=\"content-type\" content=\"text; charset=utf-8\">{}", format_apa),
    ).unwrap();

    //thread::sleep(Duration::from_millis(10000));
}

pub trait ApaFiller {
    /// Fills the HashMap with fields and values.
    fn fill_with_fields(&mut self, fields: &[&str], content: Option<&[&str]>);
}

impl ApaFiller for HashMap<usize, (String, String)> {
    fn fill_with_fields(&mut self, fields: &[&str], content: Option<&[&str]>) {
        
        // Check if there should be any content
        match content {
            Some(content) => {
                // Fill the hashmap with custom content.
                for (i, data) in fields.iter().zip(content).enumerate() {
                    self.insert(i, (data.0.to_string(), data.1.to_string()));
                }
            }
            None => {
                // Fill the fields with nothing in their content.
                for (i, field) in fields.iter().enumerate() {
                    self.insert(i, (field.to_string(), "".to_string()));
                }
            }
        }
    }
}