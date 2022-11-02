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

    pub fn nexttok(&self) -> &str {
        return &self.theStr[0..2];
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
