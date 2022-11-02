//!
//! Scan line to tokens intelligently and return as requested
//! HanishKVC, 2022
//!

use std::str::FromStr;


#[allow(non_snake_case)]
#[derive(Debug)]
struct TString {
    theStr: String,
    spacePrefixs: isize,
    spaceSuffixs: isize,
}

impl TString {

    pub fn from_string(thestr: String) -> TString {
        TString {
            theStr: thestr,
            spacePrefixs: -1,
            spaceSuffixs: -1,
        }
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

impl FromStr for TString {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TString::from_string(s.to_string()))
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_create() {
        let mut str1 = TString { theStr: "A direct string string".to_string(), spacePrefixs: 0, spaceSuffixs: 0 };
        let mut str2 = TString::from_string("  A string 21string ".to_string());
        let mut str3 = TString::from_str(" A str 12string  ").unwrap();
        print!("Created TStrings: {:?}, {:?}, {:?}", str1, str2, str3);
        print!("Str1: {}, {}", str1.space_prefixs(), str1.space_suffixs());
        print!("Str2: {}, {}", str2.space_prefixs(), str2.space_suffixs());
        print!("Str3: {}, {}", str3.space_prefixs(), str3.space_suffixs());
    }

}
