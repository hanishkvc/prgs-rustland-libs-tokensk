//!
//! Some utility/helper functions
//! HanishKVC, 2022
//!


///
/// Remove extra space (ie beyond a single space) outside double quoted text in a line.
/// Any whitespace inbetween a begin and end double quote, will be retained.
///
/// Outside double quoted text, \ is not treated has a escape sequence marker.
/// Inside double quoted text, \ is treated has a escape sequence marker, and the char next to it,
/// will be treated has a normal char (wrt this logic) and not treated has special, even if it is " or \.
///
pub fn remove_extra_whitespaces(ins: &str) -> String {
    let mut outs = String::new();
    let mut besc = false;
    let mut binquotes = false;
    let mut bwhitespace = false;
    let incv: Vec<char> = ins.chars().collect();
    for i in 0..incv.len() {
        let c = incv[i];

        if c.is_whitespace() {
            if binquotes {
                outs.push(c);
            } else {
                if !bwhitespace {
                    bwhitespace = true;
                    outs.push(' ');
                }
            }
            continue;
        }
        bwhitespace = false;
        outs.push(c);

        if besc {
            besc = false;
            continue;
        }

        if c == '"' {
            if binquotes {
                binquotes = false;
            } else {
                binquotes = true;
            }
            continue;
        }

        if c == '\\' {
            if binquotes {
                besc = true;
            }
            continue;
        }
    }
    outs
}
