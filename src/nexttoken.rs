//!
//! NextToken
//! HanishKVC, 2022
//!

use std::collections::HashMap;


enum Phase {
    Begin,
    BtwNormal,
    BtwString,
    /// Maintain current open brackets count
    BtwBracket(usize),
    /// Peek ahead to see and handle any trimmable spaces
    EndCleanup,
}

struct Flags {
    /// If spaces should be trimmed
    trim: bool,
    /// Do block tokens require user specified delim at end
    /// or is block token specific end delimiter good enough
    blocktok_dlimuser_endreqd: bool,
    /// Should the double quote protecting a string should be retained
    /// in the returned string wrt nexttok or not.
    pub stringquotes_retain: bool,
    /// Should any escape sequences found during tokenising should be
    /// processed/expanded into the special/non special char represented by them.
    pub escapesequences_expand: bool,
    /// If a bracket based token should have some textual prefix wrt
    /// the 1st opening bracket or not
    pub mainbracket_beginstandalone: bool,
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
    /// The current char's byte position
    pub chpos: usize,
    /// The current char
    pub ch: char,
    /// The byte position to start searching for next token
    pub nextpos: usize,
    /// The map of enabled escape sequences
    esmap: HashMap<char, char>,
    f: Flags,
}

impl Ctxt {

    pub fn new(thestr: &str, dlim: char, btrim: bool, esmap: HashMap<char, char>) -> Ctxt {
        Ctxt {
            vchars: thestr.char_indices().collect(),
            _dlimuser: dlim,
            cend: dlim,
            mphase: Phase::Begin,
            bescape: false,
            tok: String::new(),
            chpos: 0,
            ch: ' ',
            nextpos: 0,
            esmap: esmap,
            f: Flags {
                trim: btrim,
                blocktok_dlimuser_endreqd: true,
                stringquotes_retain: true,
                escapesequences_expand: true,
                mainbracket_beginstandalone: false,
            }
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
                        return Err(format!("CharType:ProcessChar:Unknown escseq [{}]", x.ch));
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
                        // NOTE: For now not worrying about delim within string or bracket token needing to be escaped.
                        x.tok.push(x.ch);
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
                        // NOTE: For now not worrying about delim within string or bracket token needing to be escaped.
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
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
                        x.mphase = Phase::BtwString;
                        if x.f.stringquotes_retain {
                            x.tok.push(x.ch);
                        }
                        return Ok(Action::NextChar);
                    }
                    Phase::BtwString => {
                        if x.f.blocktok_dlimuser_endreqd {
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
                                return Err(format!("CharType:ProcessChar:Opening bracket [{}] at begining of token???", bchk));
                            }
                            x.mphase = Phase::BtwBracket(1);
                            x.tok.push(x.ch);
                            return Ok(Action::NextChar);
                        }
                        Phase::BtwNormal => {
                            if x.f.mainbracket_beginstandalone {
                                return Err(format!("CharType:ProcessChar:Opening bracket [{}] not at begining of token???", bchk));
                            }
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
                        Phase::EndCleanup => {
                            x.nextpos = x.chpos;
                            return Ok(Action::DoneBreak);
                        }
                    }
                } else if x.ch == echk {
                    match x.mphase {
                        Phase::Begin => {
                            return Err(format!("CharType:ProcessChar:Closing bracket [{}] at begining of token???", echk));
                        }
                        Phase::BtwNormal => {
                            return Err(format!("CharType:ProcessChar:Closing bracket [{}] at middle of normal token???", echk));
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
                                    x.mphase = Phase::BtwNormal;
                                } else {
                                    x.mphase = Phase::EndCleanup;
                                }
                                return Ok(Action::NextChar);
                            }
                            x.mphase = Phase::BtwBracket(cnt);
                            return Ok(Action::NextChar);
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
                        x.mphase = Phase::BtwNormal;
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
pub fn vchartypes_default_with(delimstring: char, bracketbegin: char, bracketend: char, delim: Option<char>) -> Vec<CharType> {
    let mut vct = Vec::new();
    vct.push(CharType::EscSeq('\\'));
    if delim.is_some() {
        let delim = delim.unwrap();
        if delim !=  ' ' {
            vct.push(CharType::DelimNormal(delim));
        }
    }
    vct.push(CharType::DelimSpace(' '));
    vct.push(CharType::DelimString(delimstring));
    vct.push(CharType::DelimBracket(bracketbegin, bracketend));
    vct.push(CharType::Normal);
    return vct;
}
