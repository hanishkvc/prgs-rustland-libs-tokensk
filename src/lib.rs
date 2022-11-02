//!
//! Scan line to tokens intelligently and return as requested
//! HanishKVC, 2022
//!


#[allow(non_snake_case)]
#[derive(Debug)]
pub struct TString<'a, 'b> {
    theStr: String,
    spacePrefixs: isize,
    spaceSuffixs: isize,
    aStr: &'a str,
    bStr: &'b str,
}

impl<'a, 'b> TString<'a, 'b> {

    pub fn from_string(thestr: String, astr: &'a str, bstr: &'b str) -> TString<'a, 'b> {
        TString {
            theStr: thestr,
            spacePrefixs: -1,
            spaceSuffixs: -1,
            aStr: astr,
            bStr: bstr,
        }
    }

    /*
    pub fn from_string_limited(thestr: String) -> TString<'a> {
        TString { theStr: thestr.to_string(), spacePrefixs: -1, spaceSuffixs: -1, aStr: &thestr }
    }
    */

    pub fn the_str(&self) -> &str {
        &self.theStr
    }

    pub fn the_astr(&self) -> &str {
        self.aStr
    }

    pub fn the_bstr(&self) -> &str {
        self.bStr
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
        self.theStr = nstr.trim().to_string();
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

/*
impl<'a> FromStr for TString<'a> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TString::from_string_limited(s.to_string()))
    }
}
*/

impl<'a, 'b> TString<'a, 'b> {

    pub fn nexttok(&self) -> &str {
        return &self.theStr[0..2];
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mut str1 = TString { theStr: "A direct string string".to_string(), spacePrefixs: 0, spaceSuffixs: 0, aStr: "test", bStr: "me" };
        let mut str2 = TString::from_string("  A string 21string ".to_string(), "hello", "world");
        let mut str3 = TString::from_string(" A str 12string  ".to_string(), "save", "nature");
        print!("Created TStrings: {:?}, {:?}, {:?}\n", str1, str2, str3);
        print!("Str1: {}, {}\n", str1.space_prefixs(), str1.space_suffixs());
        print!("Str2: {}, {}\n", str2.space_prefixs(), str2.space_suffixs());
        print!("Str3: {}, {}\n", str3.space_prefixs(), str3.space_suffixs());
    }

}
