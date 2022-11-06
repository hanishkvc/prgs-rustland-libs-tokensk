//!
//! Scan line to tokens intelligently and return as requested
//! HanishKVC, 2022
//!

use std::collections::HashMap;
use std::fmt;


pub mod util;
mod nexttoken;


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
/// The following token types are supported
/// * simple tokens, each made up of Non space chars seperated by specified delimiter
/// * block tokens, made up of tokens within them, these could include
///   * quoted string block with spaces/escaped-delimiters/... in between, if required
///     * uses the same char for begin and end of the block
///   * bracketed content block
///     * contains a seperate begin and end bracket char wrt the block
///     * bracked content block can contain other bracketed content blocks
///       within them, to what ever depth required.
///     * The logic will try to match opening and its corresponding closing bracket,
///       so that a valid block of text is returned as the token.
///       If there is a string token within a bracketed block, containing the bracket
///       chars, they should be escaped.
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
    /// If you want to use a custom bracket begin char, set it here
    pub charBracketBegin: char,
    /// If you want to use a custom bracket end char, set it here
    pub charBracketEnd: char,
    /// The char used to demarcate/enclose multi word string token
    pub charStringQuote: char,
}


/// Creation and setup related methods
impl<'a> TStr<'a> {

    /// Create a new instance of TStr for the given string slice
    pub fn from_str(s: &'a str) -> TStr<'a> {
        TStr {
            theStr: s,
            trimmedPrefixCnt: -1,
            trimmedSuffixCnt: -1,
            bIncludeStringQuotes: true,
            bExpandEscapeSequences: true,
            escSeqMap: HashMap::new(),
            bMainBracketStandalone: false,
            charBracketBegin: '(',
            charBracketEnd: ')',
            charStringQuote: '"',
        }
    }

    /// Create a new instance of TStr from the given string slice, additionally
    /// * if btrim, trim the string and
    /// * if bescseq, setup the library provided default escape sequences
    pub fn from_str_ex(s: &'a str, btrim: bool, bescseq: bool) -> TStr<'a> {
        let mut tstr = Self::from_str(s);
        if btrim {
            tstr.trim();
        }
        if bescseq {
            tstr.escseq_defaults();
        }
        return tstr;
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

    /// Setup a set of predefined / common / useful escape sequences.
    /// Sets up the currently configured StringQuote and Bracket chars,
    /// as part of the escape sequencing, so that user can escape them
    /// if required as part of string literals, etal.
    pub fn escseq_defaults(&mut self) {
        self.escSeqMap.insert('n', '\n');
        self.escSeqMap.insert('t', '\t');
        self.escSeqMap.insert('r', '\r');
        self.escSeqMap.insert(self.charStringQuote, self.charStringQuote);
        self.escSeqMap.insert(self.charBracketBegin, self.charBracketBegin);
        self.escSeqMap.insert(self.charBracketEnd, self.charBracketEnd);
    }

}


/// Add support for std::fmt::Display trait
impl<'a> fmt::Display for TStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.the_str()))
    }
}


/// the base set of methods around the string slice
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


/// Tokenisation related methods
impl<'a> TStr<'a> {

    ///
    /// Drop text till specified NextTokPos
    ///
    pub fn drop_adjust(&mut self, nexttokpos: usize) {
        if nexttokpos >= self.theStr.len() {
            self.theStr = &"";
        } else {
            self.theStr = &self.theStr[nexttokpos..];
        }
    }

    ///
    /// Get the next token from the current string
    ///
    /// User can specify the delimiter between the tokens.
    /// * space ' ' or comma ',' could be commonly useful delimiters.
    ///
    /// However if block tokens are found, ie tokens which can contain multiple
    /// tokens within them. The delimiter specific to that block kind is used,
    /// instead of what is specified by user. At the same time, if a user specified
    /// delimiter follows the block type token, it will be trimmed out. So will
    /// any whitespace following the block type token.
    ///
    /// The block type tokens, which override the provided delimiter include
    /// * quoted string is treated as a single token (double quoted by default)
    /// * bracketed content is treated as a single token ('(' and ')' by default)
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
    /// If user requests trimming, then any spaces before the token will be
    /// trimmed out. So also will any spaces
    /// * following a block type token.
    /// * at the end of a non block type token, if a non space delimiter is used.
    ///
    /// NOTE: If space is the delimiter, then any spaces following a token will
    /// be trimmed out, when the subsequent/next token is requested.
    ///
    pub fn nexttok(&mut self, dlimdef: char, btrim: bool) -> Result<String, String> {
        let mut ctxt = nexttoken::Ctxt::new(self.theStr, dlimdef, btrim, self.escSeqMap.clone());
        let vchartypes = nexttoken::vchartypes_default_with(Some(dlimdef));
        let mut bdone = false;
        for i in 0..ctxt.vchars.len() {
            (ctxt.chpos, ctxt.ch) = ctxt.vchars[i];
            for vct in &vchartypes {
                let act = vct.process_char(&mut ctxt);
                if act.is_err() {
                    let nexti = i + 1;
                    let nexttokpos;
                    if nexti < ctxt.vchars.len() {
                        nexttokpos = ctxt.vchars[nexti].0;
                    } else {
                        nexttokpos = self.theStr.len();
                    }
                    self.drop_adjust(nexttokpos);
                    return Err(format!("NextTok:{}", act.unwrap_err()))
                }
                match act.unwrap() {
                    nexttoken::Action::NextChar => break,
                    nexttoken::Action::ContinueChain => continue,
                    nexttoken::Action::DoneBreak => bdone=true,
                }
            }
            if bdone {
                break;
            }
        }
        if !bdone {
            ctxt.nextpos = self.len();
        }
        self.drop_adjust(ctxt.nextpos);
        // trim spaces that can be at the end, wrt non block token,
        // when a non space dlimdef is used
        if btrim {
            ctxt.tok = ctxt.tok.trim().to_string();
        }
        return Ok(ctxt.tok);
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


/// Helper methods, matching similar functionality of strings in general
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

    ///
    /// Retrieve upto n tokens.
    /// The nth token will be the remaining part of the string (if any, ie if there
    /// are more than n possible tokens in the string).
    ///
    /// User provided specific delimiter will be used, if found, as one is scanning
    /// through the internal string slice. However if any block type tokens are found,
    /// they will be retrieved as part of the n tokens, even, if the delimiter following
    /// it is different from the one provided.
    ///
    pub fn splitn(&mut self, reqcnt: usize, dlimdef: char) -> Result<Vec<String>, String> {
        let mut vres = Vec::new();
        for _i in 1..reqcnt {
            let tok = self.nexttok(dlimdef, true);
            if tok.is_err() {
                return Err(format!("TStr:SplitN:{}", tok.unwrap_err()));
            }
            vres.push(tok.unwrap());
            if self.remaining_len() == 0 {
                break;
            }
        }
        if self.remaining_len() > 0 {
            vres.push(self.the_str().to_string())
        }
        Ok(vres)
    }

    /// Return the 1st (0th index) character in the internal string slice
    pub fn char_first(&self) -> Option<char> {
        self.theStr.chars().nth(0)
    }

    /// Return the last character in the internal string slice
    pub fn char_last(&self) -> Option<char> {
        self.theStr.chars().last()
    }

    /// Return the nth character in the internal string slice
    pub fn char_nth(&self, pos: usize) -> Option<char> {
        self.theStr.chars().nth(pos)
    }

}


impl<'a> TStr<'a> {

    ///
    /// Assumes that a bracketed block is what is currently stored in this TStr instance,
    /// inturn
    /// * return any prefix text associated with the bracket
    /// * update this TStr instance to contain the string representing
    ///   the contents of the bracket
    ///
    /// User can specify the type of the begining bracket
    ///
    pub fn peel_bracket(&mut self, bracket_begin: char) -> Result<String, String> {
        self.trim();
        let prefixplus = self.theStr.split_once(bracket_begin);
        if prefixplus.is_none() {
            return Err(format!("TStr:PeelBracket:{}:from:{}:failed", bracket_begin, self.theStr));
        }
        let prefixplus = prefixplus.unwrap();
        let smembers = &prefixplus.1[..prefixplus.1.len()-1];
        self.theStr = smembers;
        return Ok(prefixplus.0.to_string());
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

    #[test]
    fn test_peel_bracket() {
        testlib::test_peel_bracket();
    }

    #[test]
    fn test_first_nth_last() {
        testlib::test_first_nth_last();
    }

    #[test]
    fn test_splitn() {
        testlib::test_splitn();
    }

}
