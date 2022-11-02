//!
//! Scan line to tokens intelligently and return as requested
//! HanishKVC, 2022
//!


#[allow(non_snake_case)]
#[derive(Debug)]
pub struct TStr<'a> {
    theStr: &'a str,
    spacePrefixs: isize,
    spaceSuffixs: isize,
}

impl<'a> TStr<'a> {

    pub fn the_str(&self) -> &str {
        &self.theStr
    }

    pub fn space_prefixs_raw(&self) -> isize {
        self.spacePrefixs
    }

    pub fn space_suffixs_raw(&self) -> isize {
        self.spaceSuffixs
    }

    pub fn trim(&mut self) {
        let olen = self.theStr.len();
        let nstr = self.theStr.trim_start();
        let nlen = nstr.len();
        self.spacePrefixs = (olen - nlen) as isize;
        self.theStr = nstr.trim();
        self.spaceSuffixs = (nlen - self.theStr.len()) as isize;
    }

    pub fn space_prefixs(&mut self) -> usize {
        if self.spacePrefixs == -1 {
            self.trim();
            return self.spacePrefixs as usize;
        }
        return self.spacePrefixs as usize;
    }

    pub fn space_suffixs(&mut self) -> usize {
        if self.spaceSuffixs == -1 {
            self.trim();
            return self.spaceSuffixs as usize;
        }
        return self.spaceSuffixs as usize;
    }

}

impl<'a> TStr<'a> {

    pub fn from_str(s: &'a str) -> TStr<'a> {
        TStr {
            theStr: s,
            spacePrefixs: -1,
            spaceSuffixs: -1,
        }
    }

}

impl<'a> TStr<'a> {

    pub fn nexttok(&mut self) -> Result<String, String> {
        let vchars:Vec<(usize, char)> = self.theStr.char_indices().collect();
        let mut cend = ' ';
        let mut bbegin = true;
        let mut bescape = false;
        let mut bcheckstart;
        let mut tok = String::new();
        let mut chpos= 0;
        let mut ch;
        let mut bracketcnt = 0;
        for i in 0..vchars.len() {
            (chpos, ch) = vchars[i];
            if bescape {
                tok.push(ch);
                bescape = false;
                continue;
            }
            if ch == ' ' {
                if bbegin {
                    tok.push(ch);
                    continue;
                }
                if cend != ' ' {
                    tok.push(ch);
                    continue;
                }
                break;
            }
            if bbegin {
                bbegin = false;
                bcheckstart = true;
            } else {
                bcheckstart = false;
            }
            if ch == '"' {
                if cend == ch {
                    break;
                }
                if bcheckstart {
                    cend = ch;
                    continue;
                }
            }
            if ch == '\\' {
                if bcheckstart {
                    return Err(format!("Tok:NextTok:EscChar at start"));
                }
                bescape = true;
                continue;
            }
            if ch == '(' {
                if bcheckstart {
                    return Err(format!("Tok:NextTok:( at start"));
                }
                if cend == ' ' {
                    cend = ')';
                }
                bracketcnt += 1;
                tok.push(ch);
                continue;
            }
            if ch == ')' {
                if bcheckstart {
                    return Err(format!("Tok:NextTok:( at start"));
                }
                tok.push(ch);
                if cend == ')' {
                    bracketcnt -= 1;
                    if bracketcnt <= 0 {
                        break;
                    }
                }
                continue;
            }
            tok.push(ch);
        }
        chpos += 1;
        if chpos >= self.theStr.len() {
            self.theStr = &"";
        } else {
            self.theStr = &self.theStr[chpos..];
        }
        return Ok(tok);
    }

    pub fn remaining_len(&self) -> usize {
        self.theStr.len()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mut str1 = TStr { theStr: "A direct string string", spacePrefixs: 0, spaceSuffixs: 0 };
        let mut str2 = TStr::from_str("  A string 21string ");
        let mut str3 = TStr::from_str(" A str 12string  ");
        print!("Created TStrings: {:?}, {:?}, {:?}\n", str1, str2, str3);
        print!("Str1: {}, {}\n", str1.space_prefixs(), str1.space_suffixs());
        print!("Str2: {}, {}\n", str2.space_prefixs(), str2.space_suffixs());
        print!("Str3: {}, {}\n", str3.space_prefixs(), str3.space_suffixs());
    }

}
