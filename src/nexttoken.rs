//!
//! NextToken - Allows tokenisation using char types and tokenisation phase based flow.
//! HanishKVC, 2022
//!

use std::collections::HashMap;

use crate::{TokenType, Flags, Delimiters};


enum Phase {
    Begin,
    BtwNormal,
    BtwString,
    /// Maintain current open brackets count, as well as the open bracket char
    /// Allow more than one bracket type to be supported.
    BtwBracket(char, usize),
    /// Seek delimiter, by dropping any spaces
    EndSeekDelim,
    /// Peek ahead to see and handle any trimmable spaces
    EndCleanup,
}


pub struct Ctxt {
    /// The characters of the string, to extract token from
    pub vchars: Vec<(usize, char)>,
    /// The initial delimiter specified by user
    _dlimuser: char,
    /// The currently active end delimiter
    cend: char,
    /// The phase of tokenisation
    mphase: Phase,
    /// If we are in escape mode
    pub bescape: bool,
    /// The token being constructed
    pub tok: String,
    /// The current char's index/position
    pub ipos: usize,
    /// The current char's byte position
    pub chpos: usize,
    /// The current char
    pub ch: char,
    /// The byte position to start searching for next token
    pub nextpos: usize,
    /// The map of enabled escape sequences
    esmap: HashMap<char, char>,
    /// Helps control the behaviour of tokenising
    f: Flags,
    /// Possible Token type
    pub toktype: TokenType,
}

impl Ctxt {

    pub fn new(thestr: &str, dlim: char, esmap: HashMap<char, char>, flags: Flags) -> Ctxt {
        Ctxt {
            vchars: thestr.char_indices().collect(),
            _dlimuser: dlim,
            cend: dlim,
            mphase: Phase::Begin,
            bescape: false,
            tok: String::new(),
            ipos: 0,
            chpos: 0,
            ch: ' ',
            nextpos: 0,
            esmap: esmap,
            f: flags,
            toktype: TokenType::Unknown,
        }
    }

}


#[derive(Debug)]
pub enum Action {
    NextChar,
    ContinueChain,
    DoneBreak,
}


#[derive(Debug)]
pub enum CharType {
    /// This char will be used to allow special chars like newline, etc
    /// to be represented within/as-part-of a token.
    /// It also allows a delimiter to be part of some token as a normal
    /// char, without its delimiting characteristics.
    /// One needs to prefix the special char or the delim char with this
    /// char, to allow this.
    EscSeq(char),
    /// The char specified here will be trimmed, if present at begin or end
    /// of a token, if trimming is requested.
    /// Additionally it can also act as a delimiter to terminate/demarcate
    /// a token, if cend in Ctxt set to this char.
    DelimSpace(char),
    /// A delimiter of tokens in general
    DelimNormal(char),
    /// Identify a block of chars including spaces (or other normal demarcaters),
    /// which will be treated has a single token, which is demarcated by this
    /// same char at both ends.
    DelimString(char),
    /// Identify a block of chars including spaces (or other normal demarcaters),
    /// which will be treated has a single token, which is demarcated by this
    /// set of chars at either end.
    /// It allows one such block to contain additional such blocks within it,
    /// and so on for what ever depth required.
    DelimBracket(char, char),
    /// Represents all the other chars, which inturn will be treated as normal chars.
    Normal,
}

impl CharType {

    ///
    /// If Multiple DelimBracket sets are provided, then the 1st begining bracket type
    /// matched (ie outermost bracket type) will be used for bracket block delimiting /
    /// demarcating wrt that token, including any included sub bracket blocks
    /// (ie brackets within brackets) within that block token.
    ///
    /// DevNote: nextpos contains the position of last char to consume wrt current tok,
    /// till the logic executes a Action::DoneBreak from within Phase::EndCleanup,
    /// at which point it points to the begining of the next token roughly.
    ///
    pub fn process_char(&self, x: &mut Ctxt) -> Result<Action, String> {
        //print!("DBUG:NextToken:ProcessChar:{}:{}\n", x.ipos, x.ch);
        match *self {
            CharType::EscSeq(chk) => {
                if !x.bescape {
                    if x.ch != chk {
                        return Ok(Action::ContinueChain);
                    }
                    match x.mphase {
                        Phase::BtwNormal | Phase::BtwString | Phase::BtwBracket(_,_) => {
                            if x.f.escapesequences_expand {
                                x.bescape = true;
                            } else {
                                x.tok.push(x.ch);
                            }
                            return Ok(Action::NextChar);
                        }
                        _ => {
                            return Ok(Action::ContinueChain);
                        }
                    }
                }
                x.bescape = false;
                if !x.f.escapesequences_expand {
                    x.tok.push(x.ch);
                } else {
                    let replace = x.esmap.get(&x.ch);
                    if replace.is_none() {
                        return Err(format!("CharType:ProcessChar:Unknown escseq [{}] @ {}", x.ch, x.ipos));
                    }
                    x.tok.push(*replace.unwrap());
                }
                return Ok(Action::NextChar);
            }
            CharType::DelimSpace(chk) => {
                if x.ch != chk {
                    return Ok(Action::ContinueChain);
                }
                match x.mphase {
                    Phase::Begin => {
                        if x.f.trim {
                            return Ok(Action::NextChar);
                        }
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                    Phase::BtwNormal => {
                        if chk == x.cend {
                            x.nextpos = x.chpos;
                            x.mphase = Phase::EndCleanup;
                            return Ok(Action::NextChar);
                        }
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                    Phase::BtwString | Phase::BtwBracket(_,_) => {
                        // NOTE: For now not worrying about delim space within string or bracket token needing to be escaped.
                        // However from a overall flow perspective, the delim needs to be escaped, to ensure proper functioning
                        // of the overall logic.
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                    Phase::EndSeekDelim => {
                        x.nextpos = x.chpos;
                        if chk == x.cend {
                            x.mphase = Phase::EndCleanup;
                            return Ok(Action::NextChar);
                        }
                        if !x.f.trim {
                            x.tok.push(x.ch);
                        }
                        return Ok(Action::NextChar);
                    }
                    Phase::EndCleanup => {
                        x.nextpos = x.chpos;
                        if x.f.trim {
                            return Ok(Action::NextChar);
                        }
                        return Ok(Action::DoneBreak);
                    }
                }
            },
            CharType::DelimNormal(chk) => {
                if x.ch != chk {
                    return Ok(Action::ContinueChain);
                }
                match x.mphase {
                    Phase::BtwString | Phase::BtwBracket(_,_) => {
                        // NOTE: For now not worrying about delim normal within string or bracket token needing to be escaped.
                        // However from a overall flow perspective, the delim needs to be escaped, to ensure proper functioning
                        // of the overall logic.
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                    Phase::EndSeekDelim => {
                        x.nextpos = x.chpos;
                        if chk == x.cend {
                            x.mphase = Phase::EndCleanup;
                            return Ok(Action::NextChar);
                        }
                        return Err(format!("DBUG:CharType:DelimNormal:ProcessChar:EndSeekingDeLim:Non delim char [{}] @ {}", x.ch, x.ipos));
                    }
                    Phase::EndCleanup => {
                        x.nextpos = x.chpos;
                        return Ok(Action::DoneBreak);
                    }
                    _ => {
                        if chk == x.cend {
                            x.nextpos = x.chpos;
                            x.mphase = Phase::EndCleanup;
                            return Ok(Action::NextChar);
                        }
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                }
            }
            CharType::DelimString(chk) => {
                if x.ch != chk {
                    return Ok(Action::ContinueChain);
                }
                match x.mphase {
                    Phase::Begin => {
                        x.toktype = TokenType::String;
                        x.mphase = Phase::BtwString;
                        if x.f.stringquotes_retain {
                            x.tok.push(x.ch);
                        }
                        return Ok(Action::NextChar);
                    }
                    Phase::BtwNormal => {
                        if x.f.string_canbe_asubpart {
                            x.toktype = TokenType::String; // Maybe add a StringPlus type
                            x.mphase = Phase::BtwString;
                        }
                        // In this case dont bother about StringQuotesRetain flag,
                        // bcas its definitely in the middle of some other token
                        // So retain the quote.
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                    Phase::BtwString => {
                        if x.f.blocktok_dlimuser_endreqd {
                            x.mphase = Phase::EndSeekDelim;
                        } else if x.f.string_canbe_asubpart {
                            x.mphase = Phase::BtwNormal;
                        } else {
                            x.mphase = Phase::EndCleanup;
                        }
                        x.nextpos = x.chpos;
                        if x.f.stringquotes_retain {
                            x.tok.push(x.ch);
                        }
                        return Ok(Action::NextChar);
                    }
                    Phase::EndSeekDelim => {
                        x.nextpos = x.chpos;
                        return Err(format!("DBUG:CharType:DelimString:ProcessChar:EndSeekingDeLim:Non delim char [{}] @ {}", x.ch, x.ipos));
                    }
                    Phase::EndCleanup => {
                        x.nextpos = x.chpos;
                        return Ok(Action::DoneBreak);
                    }
                    _ => {
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                }
            },
            CharType::DelimBracket(bchk, echk) => {
                if x.ch == bchk {
                    match x.mphase {
                        Phase::Begin => {
                            if !x.f.mainbracket_beginstandalone {
                                return Err(format!("CharType:ProcessChar:Opening bracket [{}] @ {} at begining of token???", bchk, x.ipos));
                            }
                            x.toktype = TokenType::BracketStandalone;
                            x.mphase = Phase::BtwBracket(bchk, 1);
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::BtwNormal => {
                            if !x.f.mainbracket_beginprefixed {
                                return Err(format!("CharType:ProcessChar:Opening bracket [{}] @ {} not at begining of token???", bchk, x.ipos));
                            }
                            x.toktype = TokenType::BracketPrefixed;
                            x.mphase = Phase::BtwBracket(bchk, 1);
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::BtwString => {
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::BtwBracket(curb, cnt) => {
                            if curb == bchk {
                                x.mphase = Phase::BtwBracket(curb, cnt+1);
                            }
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::EndSeekDelim => {
                            x.nextpos = x.chpos;
                            return Err(format!("DBUG:CharType:DelimBracket:ProcessChar:EndSeekingDeLim:Non delim char [{}] @ {}", x.ch, x.ipos));
                        }
                        Phase::EndCleanup => {
                            x.nextpos = x.chpos;
                            return Ok(Action::DoneBreak);
                        }
                    }
                } else if x.ch == echk {
                    match x.mphase {
                        Phase::Begin => {
                            return Err(format!("CharType:ProcessChar:Closing bracket [{}] @ {} at begining of token???", echk, x.ipos));
                        }
                        Phase::BtwNormal => {
                            return Err(format!("CharType:ProcessChar:Closing bracket [{}] @ {} at middle of normal token???", echk, x.ipos));
                        }
                        Phase::BtwString => {
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::BtwBracket(curb, cnt) => {
                            if curb != bchk {
                                x.tok.push(x.ch);
                                return Ok(Action::NextChar);
                            }
                            let cnt = cnt - 1;
                            x.tok.push(x.ch);
                            if cnt == 0 {
                                x.nextpos = x.chpos;
                                if x.f.blocktok_dlimuser_endreqd {
                                    x.mphase = Phase::EndSeekDelim;
                                } else {
                                    x.mphase = Phase::EndCleanup;
                                }
                                return Ok(Action::NextChar);
                            }
                            x.mphase = Phase::BtwBracket(curb, cnt);
                            return Ok(Action::NextChar);
                        }
                        Phase::EndSeekDelim => {
                            x.nextpos = x.chpos;
                            return Err(format!("DBUG:CharType:DelimBracket:ProcessChar:EndSeekingDeLim:Non delim char [{}] @ {}", x.ch, x.ipos));
                        }
                        Phase::EndCleanup => {
                            x.nextpos = x.chpos;
                            return Ok(Action::DoneBreak);
                        }
                    }
                } else {
                    return Ok(Action::ContinueChain);
                }
            },
            CharType::Normal => {
                match x.mphase {
                    Phase::Begin => {
                        x.toktype = TokenType::Normal;
                        x.mphase = Phase::BtwNormal;
                    }
                    Phase::EndSeekDelim => {
                        x.nextpos = x.chpos;
                        return Err(format!("DBUG:CharType:Normal:ProcessChar:EndSeekingDeLim:Non delim char [{}] @ {}", x.ch, x.ipos));
                    }
                    Phase::EndCleanup => {
                        x.nextpos = x.chpos;
                        return Ok(Action::DoneBreak);
                    }
                    _ => ()
                }
                x.tok.push(x.ch);
                return Ok(Action::NextChar);
            },
        }
    }

}


/// The vector of chartypes, which will be used by nexttok, to process
/// the chars in the string it is given, to identify the next token
pub struct VCharTypes {
    pub vct: Vec<CharType>,
}

impl VCharTypes {

    ///
    /// Create the vector of chartypes, from the given chars
    ///
    /// normaldelim: the normal delimiter, if seperate from the space delimiter
    ///
    pub fn from_chars(delimescseq: char, delimspace: char, delimstring: char, delimbracket: (char,char), normaldelim: Option<char>, odelimbracket: Option<(char,char)>) -> VCharTypes {
        let mut vct = Vec::new();
        vct.push(CharType::EscSeq(delimescseq));
        if normaldelim.is_some() {
            let delim = normaldelim.unwrap();
            if delim !=  delimspace {
                vct.push(CharType::DelimNormal(delim));
            }
        }
        vct.push(CharType::DelimSpace(delimspace));
        vct.push(CharType::DelimString(delimstring));
        vct.push(CharType::DelimBracket(delimbracket.0, delimbracket.1));
        if odelimbracket.is_some() {
            let obracket = odelimbracket.unwrap();
            vct.push(CharType::DelimBracket(obracket.0, obracket.1));
        }
        vct.push(CharType::Normal);
        VCharTypes {
            vct: vct
        }
    }

    pub fn from_delimiters(delims: &Delimiters, normaldelim: Option<char>) -> VCharTypes {
        return Self::from_chars(delims.escseq, delims.space, delims.string, delims.bracket, normaldelim, delims.obracket);
    }

}
