//!
//! NextToken - Allows tokenisation using char types and tokenisation phase based flow.
//! HanishKVC, 2022
//!

use std::collections::HashMap;

use crate::{TokenType, Flags};


enum Phase {
    Begin,
    BtwNormal,
    BtwString,
    /// Maintain current open brackets count
    BtwBracket(usize),
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
    EscSeq(char),
    DelimSpace(char),
    DelimNormal(char),
    DelimString(char),
    DelimBracket(char, char),
    Normal,
}

impl CharType {

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
                        Phase::BtwNormal | Phase::BtwString | Phase::BtwBracket(_) => {
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
                    Phase::BtwString | Phase::BtwBracket(_) => {
                        // NOTE: For now not worrying about delim space within string or bracket token needing to be escaped.
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
                    Phase::BtwString | Phase::BtwBracket(_) => {
                        // NOTE: For now not worrying about delim normal within string or bracket token needing to be escaped.
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
                            x.mphase = Phase::BtwBracket(1);
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::BtwNormal => {
                            if !x.f.mainbracket_beginprefixed {
                                return Err(format!("CharType:ProcessChar:Opening bracket [{}] @ {} not at begining of token???", bchk, x.ipos));
                            }
                            x.toktype = TokenType::BracketPrefixed;
                            x.mphase = Phase::BtwBracket(1);
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::BtwString => {
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::BtwBracket(cnt) => {
                            x.mphase = Phase::BtwBracket(cnt+1);
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
                        Phase::BtwBracket(cnt) => {
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
                            x.mphase = Phase::BtwBracket(cnt);
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


///
/// Create the default vector of chartypes, which will be used by nexttok
/// to process the chars to identify the next token.
///
pub fn vchartypes_with(delimspace: char, delimstring: char, bracketbegin: char, bracketend: char, delim: Option<char>) -> Vec<CharType> {
    let mut vct = Vec::new();
    vct.push(CharType::EscSeq('\\'));
    if delim.is_some() {
        let delim = delim.unwrap();
        if delim !=  delimspace {
            vct.push(CharType::DelimNormal(delim));
        }
    }
    vct.push(CharType::DelimSpace(delimspace));
    vct.push(CharType::DelimString(delimstring));
    vct.push(CharType::DelimBracket(bracketbegin, bracketend));
    vct.push(CharType::Normal);
    return vct;
}
