//!
//! Scan line to tokens intelligently and return as requested
//! HanishKVC, 2022
//!

use std::collections::HashMap;
use std::fmt;


pub mod util;
mod nexttoken;


struct Flags {
    /// If spaces should be trimmed
    trim: bool,
    /// Should any escape sequences found during tokenising should be
    /// processed/expanded into the special/non special char represented by them.
    pub escapesequences_expand: bool,
    /// Do block tokens require user specified delim at end
    /// or is block token specific end delimiter good enough
    blocktok_dlimuser_endreqd: bool,
    /// Should the double quote protecting a string should be retained
    /// in the returned string wrt nexttok or not.
    pub stringquotes_retain: bool,
    /// If the 1st/main/toplevel bracketed-content based token can begin standalone,
    /// ie if it can start with begin-bracket-char without needing any textual prefix.
    pub mainbracket_beginstandalone: bool,
    /// If one needs to support bracketed-content based tokens that should have
    /// some textual prefix wrt the 1st/main/toplevel opening bracket.
    /// NOTE: There cant be space between the text prefix and 1st opening bracket
    /// if space is a delimiter.
    pub mainbracket_beginprefixed: bool,
}

impl Flags {

    pub fn new(trim: bool, escapesequences: bool, blocktokdelimited: bool, retainquotes: bool, bracketstandalone: bool, bracketprefixed: bool) -> Flags {
        Flags {
            trim: trim,
            escapesequences_expand: escapesequences,
            blocktok_dlimuser_endreqd: blocktokdelimited,
            stringquotes_retain: retainquotes,
            mainbracket_beginstandalone: bracketstandalone,
            mainbracket_beginprefixed: bracketprefixed,
        }
    }

    pub fn default() -> Flags {
        Flags {
            trim: true,
            escapesequences_expand: true,
            blocktok_dlimuser_endreqd: true,
            stringquotes_retain: true,
            mainbracket_beginprefixed: true,
            mainbracket_beginstandalone: true,
        }
    }

}


#[derive(Debug)]
pub enum TokenType {
    Unknown,
    Normal,
    String,
    BracketStandalone,
    BracketPrefixed,
}


#[allow(non_snake_case)]
#[derive(Debug, Clone)]
///
/// Tokenisable String - created from a passed string slice
/// * using from_str when creating a new instance
/// * using set_str, when updating/reusing a existing instance
///
/// The tokenisation characteristics can be adjusted using some
/// of the members in it and its helper module (nexttoken).
///
/// The following token types are supported
/// * simple tokens, each made up of Non space chars seperated by specified delimiter
/// * block tokens, made up of tokens within them, these could include
///   * quoted string block with spaces/escaped-delimiters/... in between, if required
///     * uses the same string quote char for begin and end of the string block
///   * bracketed content block
///     * contains a seperate begin and end bracket char wrt the block
///     * bracketed content block can contain other bracketed content blocks
///       within them, to what ever depth required.
///     * The logic will try to match opening and its corresponding closing bracket,
///       so that a valid block of text is returned as the token.
///       If there is a string token within a bracketed block, containing the bracket
///       chars, which may not be balanced, they should be escaped.
///     * There are two types of bracketed block tokens
///       * BracketStandalone: those starting with begin-bracket-char at the begining of the token
///       * BracketPrefixed: these need to contain some textual prefix before the begin-bracket-char
///         wrt the 1st/starting bracket in the token. ie the inbetween bracketed blocks within
///         a given bracketed block could be either Standalone or Prefixed type.
///
pub struct TStr<'a> {
    theStr: &'a str,
    /// The amount of space trimmed at the begining of the string
    trimmedPrefixCnt: isize,
    /// The amount of space trimmed at the end of the string
    trimmedSuffixCnt: isize,
    /// Maintain the set of supported escape sequences and the underlying expanded char.
    escSeqMap: HashMap<char, char>,
    /// If you want to use a custom bracket begin char, set it here
    pub charBracketBegin: char,
    /// If you want to use a custom bracket end char, set it here
    pub charBracketEnd: char,
    /// The char used to demarcate/enclose multi word string token
    pub charStringQuote: char,
    /// Explicit trim at end, Will be useful
    /// when a Non space delim is used and there is spaces before the delim
    bTrimAtEnd: bool,
}


/// Creation and setup related methods
impl<'a> TStr<'a> {

    /// Create a new instance of TStr for the given string slice
    pub fn from_str(s: &'a str) -> TStr<'a> {
        TStr {
            theStr: s,
            trimmedPrefixCnt: -1,
            trimmedSuffixCnt: -1,
            escSeqMap: HashMap::new(),
            charBracketBegin: '(',
            charBracketEnd: ')',
            charStringQuote: '"',
            bTrimAtEnd: true,
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
    /// Extract the next token string belonging to this TStr instance,
    /// and inturn its token type info.
    ///
    /// User can specify the delimiter between the tokens.
    /// * space ' ' or comma ',' could be commonly useful delimiters.
    ///
    /// The user specified delimiter will be trimmed out.
    ///
    /// However if block tokens are found, ie tokens which can contain multiple
    /// tokens within them. The delimiter specific to that block kind is required
    /// first, before the user specified delimiter is seen/encountered.
    ///
    /// The block type tokens, include
    /// * quoted string with multiple words in it, with spaces in it, will be
    ///   treated as a single token (double quoted by default)
    /// * bracketed content is treated as a single token ('(' and ')' by default)
    ///   * one can have brackets within brackets.
    ///   * however by default, the starting opening bracket should be prefixed
    ///     with some alphanumeric text. (user can change this behaviour, if reqd)
    ///
    /// If any error identified while scanning for the token, a error message
    /// is returned to the caller, while parallley dropping the token with error,
    /// so that next call to this will potentially retrieve a valid token, if any
    /// still in the string/line. The partial dropped token is also returned, as
    /// part of a tuple, along with the err message. ie (ErrMsg, PartialTok).
    ///
    /// If a escape sequence is found anywhere other than begining of the token,
    /// it will be processed/expanded, if requested.
    ///
    /// If user requests trimming, then any spaces before and after the token
    /// will be trimmed out.
    ///
    pub fn nexttok_ex(&mut self, dlimdef: char, btrim: bool) -> Result<(String, TokenType), (String, String)> {
        let mut ctxt = nexttoken::Ctxt::new(self.theStr, dlimdef, btrim, self.escSeqMap.clone());
        let vchartypes = nexttoken::vchartypes_default_with(self.charStringQuote, self.charBracketBegin, self.charBracketEnd, Some(dlimdef));
        let mut bdone = false;
        for i in 0..ctxt.vchars.len() {
            (ctxt.chpos, ctxt.ch) = ctxt.vchars[i];
            ctxt.ipos = i;
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
                    return Err((format!("TStr:NextTok:{}", act.unwrap_err()), ctxt.tok.to_string()))
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
        // trim spaces that can be at the end, when a non space dlimdef is used
        if btrim && self.bTrimAtEnd{
            ctxt.tok = ctxt.tok.trim().to_string();
        }
        return Ok((ctxt.tok, ctxt.toktype));
    }

    ///
    /// Extract the next token string belonging to this TStr instance.
    ///
    /// NOTE: Look at the doc related to nexttok_ex for more details.
    ///
    pub fn nexttok(&mut self, dlimdef: char, btrim: bool) -> Result<String, (String, String)> {
        let gotr = self.nexttok_ex(dlimdef, btrim);
        if gotr.is_err() {
            return Err(gotr.unwrap_err());
        }
        let gotr = gotr.unwrap();
        return Ok(gotr.0);
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
    /// block is found, it will be treated has a multi-token-based-block-token
    /// on its own.
    ///
    pub fn tokens_vec(&mut self, dlimdef: char, btrim: bool, bcontinue_onerr: bool) -> Result<Vec<String>, String> {
        let mut vtoks = Vec::new();
        while self.remaining_len() > 0 {
            let gottok = self.nexttok(dlimdef, btrim);
            if gottok.is_err() && !bcontinue_onerr {
                return Err(format!("TStr:TokensVec:{:?}", gottok.unwrap_err()));
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
    /// User can specify the delimiter to be used. If a quoted string or
    /// bracket based block is the 1st token, the delimiter will be checked
    /// for beyond the 1st token (ie the delimiter could be inside the
    /// 1st token, if it is a block token).
    ///
    pub fn split_once(&mut self, dlimdef: char) -> Result<(String, String), String> {
        let gottok = self.nexttok(dlimdef, true);
        if gottok.is_err() {
            return Err(format!("TStr:SplitOnce:{:?}",gottok.unwrap_err()));
        }
        return Ok((gottok.unwrap(), self.the_str().to_string()));
    }

    ///
    /// Retrieve upto n tokens.
    /// The nth token will be the remaining part of the string (if any, ie if there
    /// are more than n possible tokens in the string).
    ///
    /// User provided specific delimiter will be used, if found, as one is scanning
    /// through the internal string slice. The retrieved tokens could also represent
    /// block type tokens.
    ///
    pub fn splitn(&mut self, reqcnt: usize, dlimdef: char) -> Result<Vec<String>, String> {
        let mut vres = Vec::new();
        for _i in 1..reqcnt {
            let tok = self.nexttok(dlimdef, true);
            if tok.is_err() {
                return Err(format!("TStr:SplitN:{:?}", tok.unwrap_err()));
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

    ///
    /// Update this TStr instance to contain the string without its enclosing
    /// string delimiter.
    ///
    pub fn peel_string(&mut self, stringdelim: char) -> Result<(), String> {
        self.trim();
        let schar = self.char_first().unwrap();
        let echar = self.char_last().unwrap();
        if (schar != stringdelim) || (echar != stringdelim) {
            return Err(format!("TStr:PeelString:{}:Not used to enclose the string fully", stringdelim));
        }
        self.theStr = &self.theStr[1..self.len()-1];
        Ok(())
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
    fn test_peel() {
        testlib::test_peel_bracket();
        testlib::test_peel_string();
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
