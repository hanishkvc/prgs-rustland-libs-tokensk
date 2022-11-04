//!
//! Scan line to tokens intelligently and return as requested
//! HanishKVC, 2022
//!

use std::collections::HashMap;
use std::fmt;


pub mod util;

#[allow(non_snake_case)]
#[derive(Debug)]
///
/// Tokenisable String - created from a passed string slice
/// * using from_str when creating a new instance
/// * using set_str, when updating/reusing a existing instance
///
/// The tokenisation characteristics can be adjusted using some
/// of the members in it.
///
pub struct TStr<'a> {
    theStr: &'a str,
    /// The amount of space trimmed at the begining of the string
    trimmedPrefixCnt: isize,
    /// The amount of space trimmed at the end of the string
    trimmedSuffixCnt: isize,
    /// Should the double quote protecting a string should be retained
    /// in the returned string wrt nexttok or not.
    pub bIncludeStringQuotes: bool,
    /// Should any escape sequences found during tokenising should be
    /// processed/expanded into the special/non special char represented by them.
    pub bExpandEscapeSequences: bool,
    /// Maintain the set of supported escape sequences and the underlying expanded char.
    escSeqMap: HashMap<char, char>,
    /// If a bracket based token should have some textual prefix wrt the 1st opening bracket
    pub bMainBracketStandalone: bool,
}

impl<'a> TStr<'a> {

    /// create a new instance of TStr for the given string slice
    pub fn from_str(s: &'a str) -> TStr<'a> {
        TStr {
            theStr: s,
            trimmedPrefixCnt: -1,
            trimmedSuffixCnt: -1,
            bIncludeStringQuotes: true,
            bExpandEscapeSequences: true,
            escSeqMap: HashMap::new(),
            bMainBracketStandalone: false,
        }
    }

    /// Allow an existing TStr to be used wrt a new string/line
    pub fn set_str(&mut self, s: &'a str) {
        self.theStr = s;
        self.trimmedPrefixCnt = -1;
        self.trimmedSuffixCnt = -1;
    }

    /// Clear any existing supported escape sequences
    pub fn escseq_clear(&mut self) {
        self.escSeqMap.clear();
    }

    /// Add a new supported escape sequence
    pub fn escseq_set(&mut self, find: char, replace: char) {
        self.escSeqMap.insert(find, replace);
    }

    /// Setup a set of predefined / common escape sequences
    pub fn escseq_defaults(&mut self) {
        self.escSeqMap.insert('n', '\n');
        self.escSeqMap.insert('t', '\t');
        self.escSeqMap.insert('r', '\r');
        self.escSeqMap.insert('"', '"');
    }

}

impl<'a> fmt::Display for TStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("TStr[{}]", self.the_str()))
    }
}

impl<'a> TStr<'a> {

    /// retrieve the internal string slice, as it stands currently
    pub fn the_str(&self) -> &str {
        &self.theStr
    }

    pub fn trimmed_prefix_cnt_raw(&self) -> isize {
        self.trimmedPrefixCnt
    }

    pub fn trimmed_suffix_cnt_raw(&self) -> isize {
        self.trimmedSuffixCnt
    }

    ///
    /// THis trims any space at begin or end of the line/string maintained internally.
    /// It also updates the number of spaces that was found at the begin and end of the string,
    /// when trimming it.
    ///
    pub fn trim(&mut self) {
        let olen = self.theStr.len();
        let nstr = self.theStr.trim_start();
        let nlen = nstr.len();
        self.trimmedPrefixCnt = (olen - nlen) as isize;
        self.theStr = nstr.trim();
        self.trimmedSuffixCnt = (nlen - self.theStr.len()) as isize;
    }

    /// Returns the number of spaces if any at the beginning of the string, which was trimmed.
    /// If trim has not been called before, it will be automatically called.
    pub fn trimmed_prefix_cnt(&mut self) -> usize {
        if self.trimmedPrefixCnt == -1 {
            self.trim();
            return self.trimmedPrefixCnt as usize;
        }
        return self.trimmedPrefixCnt as usize;
    }

    /// Returns the number of spaces if any at the end of the string, which was trimmed.
    /// If trim has not been called before, it will be automatically called.
    pub fn trimmed_suffix_cnt(&mut self) -> usize {
        if self.trimmedSuffixCnt == -1 {
            self.trim();
            return self.trimmedSuffixCnt as usize;
        }
        return self.trimmedSuffixCnt as usize;
    }

}

impl<'a> TStr<'a> {

    ///
    /// drop text till and including specified LastTokPos
    ///
    pub fn drop_adjust(&mut self, mut lasttokpos: usize) {
        lasttokpos += 1;
        if lasttokpos >= self.theStr.len() {
            self.theStr = &"";
        } else {
            self.theStr = &self.theStr[lasttokpos..];
        }
    }

    ///
    /// Get the next token from the current string
    ///
    /// User can specify the delimiter between the tokens.
    /// * space ' ' or comma ',' could be commonly useful delimiters.
    ///
    /// However if certain type of tokens are found, the delimiter specific
    /// to that kind is used, instead of what is specified by user.
    ///
    /// The specific types of tokens, which override the provided delimiter
    /// include the following
    /// * double quoted string is treated as a single token
    /// * () bracketed content is treated as a single token
    ///   * one can have brackets within brackets.
    ///   * however the starting opening bracket should be prefixed with some
    ///     alphanumeric text.
    ///     This is a specific semantic, wrt how fuzzerk works currently.
    ///
    /// If any error identified while scanning for the token, a error message
    /// is returned to the caller, while parallley dropping the token with error,
    /// so that next call to this will potentially retrieve a valid token, if any
    /// still in the string/line.
    ///
    /// If a escape sequence is found anywhere other than begining of the token,
    /// it will be processed/expanded, if requested.
    ///
    pub fn nexttok(&mut self, dlimdef: char, btrim: bool) -> Result<String, String> {
        let vchars:Vec<(usize, char)> = self.theStr.char_indices().collect();
        let mut cend = dlimdef;
        let mut bbegin = true;
        let mut bescape = false;
        let mut bcheckstart;
        let mut tok = String::new();
        let mut chpos= 0;
        let mut ch;
        let mut bracketcnt = 0;
        for i in 0..vchars.len() {
            (chpos, ch) = vchars[i];
            //log_d(format!("DBUG:NextTok:Char[Pos]:[{}][{}][{}]\n", ch, ch as usize, chpos));
            // Handle escape sequence, if we are in one
            if bescape {
                //log_d(format!("DBUG:NextTok:In EscSeq:{}\n", ch));
                if self.bExpandEscapeSequences {
                    let replace = self.escSeqMap.get(&ch);
                    if replace.is_none() {
                        self.drop_adjust(chpos);
                        return Err(format!("Tok:NextTok:Unknown escseq [{}]", ch));
                    }
                    //log_d(format!("DBUG:NextTok:EscSeq:{}:=:{:?}:\n", ch, replace));
                    tok.push(*replace.unwrap())
                } else {
                    tok.push(ch);
                }
                bescape = false;
                continue;
            }
            // Handle space char,
            // also taking care of trimming it at the beginning, if requested
            if ch == ' ' {
                if bbegin {
                    if !btrim {
                        tok.push(ch);
                    }
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
            // Help with handling double quoted strings
            if ch == '"' {
                if self.bIncludeStringQuotes || (ch != cend) {
                    tok.push(ch);
                }
                if cend == ch {
                    break;
                }
                if bcheckstart {
                    cend = ch;
                    continue;
                }
                continue;
            }
            // Identify starting of a escape sequence
            if ch == '\\' {
                if bcheckstart {
                    self.drop_adjust(chpos);
                    return Err(format!("Tok:NextTok:EscChar at start"));
                }
                bescape = true;
                continue;
            }
            // Help handle a bracketed block, by identifying its boundries
            if ch == '(' {
                if bcheckstart && !self.bMainBracketStandalone {
                    self.drop_adjust(chpos);
                    return Err(format!("Tok:NextTok:( at start, without any prefix text"));
                }
                if self.bMainBracketStandalone && !bcheckstart && (bracketcnt == 0) {
                    self.drop_adjust(chpos);
                    return Err(format!("Tok:NextTok:1st/Main ( didnt start at begin"));
                }
                if cend == dlimdef {
                    cend = ')';
                }
                bracketcnt += 1;
                tok.push(ch);
                continue;
            }
            if ch == ')' {
                if bcheckstart {
                    self.drop_adjust(chpos);
                    return Err(format!("Tok:NextTok:) at start"));
                }
                tok.push(ch);
                bracketcnt -= 1;
                if cend == ')' {
                    if bracketcnt <= 0 {
                        break;
                    }
                }
                continue;
            }
            // Handle the delimiter char specified/passed
            if ch == dlimdef {
                if ch == cend {
                    break;
                }
            }
            // Handle other chars, as well as any of above, which has been
            // allowed to fall through to here.
            tok.push(ch);
        }
        self.drop_adjust(chpos);
        return Ok(tok);
    }

    ///
    /// Return remaining text len wrt the current line, which is not yet tokenised/extracted
    ///
    pub fn remaining_len(&self) -> usize {
        self.theStr.len()
    }

    ///
    /// Return the length of the string still left inside, yet to be tokenising/...
    ///
    pub fn len(&self) -> usize {
        self.theStr.len()
    }

}

impl<'a> TStr<'a> {

    ///
    /// Get a vector of all the tokens in the current string/line
    /// One can control
    /// * whether spaces at either end of the token is trimmed or not
    /// * whether to abort or continue on encountering errors when tokenising
    ///
    /// User can specify a specific delimiter, which will be used to identify
    /// the tokens. However additionally if a double quoted string or bracketed
    /// block is found, it will be treated has a token on its own.
    ///
    pub fn tokens_vec(&mut self, dlimdef: char, btrim: bool, bcontinue_onerr: bool) -> Result<Vec<String>, String> {
        let mut vtoks = Vec::new();
        while self.remaining_len() > 0 {
            let gottok = self.nexttok(dlimdef, btrim);
            if gottok.is_err() && !bcontinue_onerr {
                return Err(format!("TokensVec:{}", gottok.unwrap_err()));
            }
            if gottok.is_ok() {
                vtoks.push(gottok.unwrap());
            }
        }
        Ok(vtoks)
    }

}

impl<'a> TStr<'a> {

    ///
    /// Retrieve the 1st available token, and remaining string.
    ///
    /// User can specify a specific delimiter, which will be used provided its
    /// valid, wrt the 1st token actually found. Else what ever valid 1st token
    /// is found will be retrieved. Look at nexttok doc for info.
    ///
    pub fn split_once(&mut self, dlimdef: char) -> Result<(String, String), String> {
        let gottok = self.nexttok(dlimdef, true);
        if gottok.is_err() {
            return Err(gottok.unwrap_err());
        }
        return Ok((gottok.unwrap(), self.the_str().to_string()));
    }

    /// Return the 1st (0th index) character in the internal string slice
    pub fn char_first(&self) -> Option<char> {
        self.theStr.chars().nth(0)
    }

    /// Return the last character in the internal string slice
    pub fn char_last(&self) -> Option<char> {
        self.theStr.chars().last()
    }

}



pub mod testlib;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        testlib::test_create();
        testlib::test_create_raw();
    }

    #[test]
    fn test_nexttoken() {
        testlib::test_nexttoken();
    }

}
