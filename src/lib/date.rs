// Return "consultado en" or "retrived" depending on language.

use chrono::prelude::*;

use crate::Lang;

pub fn retrive_current_date(lang: &Lang) -> String {

    let local_date = Local::now();
   
    let month = parse_month(&lang, local_date.month());
    let day = local_date.day();
    let year = local_date.year();

    // Build up the string.
    let format = match lang {
        Lang::English => {
            format!("Retrived {} {}, {}, from", month, day, year)
        }
        Lang::Spanish => {
            format!("Consultado el {} de {}, {}, de", day, month, year)
        }
    };
    return format;
}

pub fn parse_month(lang: &Lang, month_number: u32) -> String {
    // Return the name of the month depending on the language.
    let month: &str = match lang {
        Lang::English => {
            match month_number {
                1 => "January",
                2 => "Febuary",
                3 => "March",
                4 => "April",
                5 => "May",
                6 => "June",
                7 => "July",
                8 => "August",
                9 => "September",
                10 => "October",
                11 => "November",
                12 => "December",
                _ => {"month number out of range"}
            }
        },
        Lang::Spanish => {
            match month_number {
                1 => "enero",
                2 => "febrero",
                3 => "marzo",
                4 => "abirl",
                5 => "mayo",
                6 => "junio",
                7 => "julio",
                8 => "agosto",
                9 => "septiembre",
                10 => "octubre",
                11 => "noviembre",
                12 => "diciembre",
                _ => {"month number out of range"}
        }
        }
    };

    return month.to_string();
}