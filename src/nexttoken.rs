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
    /// Maintain current tok's end position in source string
    EndCleanup,
}

pub struct Ctxt {
    /// The characters of the string, to extract token from
    pub vchars: Vec<(usize, char)>,
    /// The initial delimiter specified by user
    dlimdef: char,
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
    /// If spaces should be trimmed
    btrim: bool,
    /// The byte position till which the source string should be trimmed
    /// after token has been extracted.
    pub endpos: usize,
    esmap: HashMap<char, char>,
}

impl Ctxt {

    pub fn new(thestr: &str, dlimdef: char, btrim: bool, esmap: HashMap<char, char>) -> Ctxt {
        Ctxt {
            vchars: thestr.char_indices().collect(),
            dlimdef: dlimdef,
            cend: dlimdef,
            mphase: Phase::Begin,
            bescape: false,
            tok: String::new(),
            chpos: 0,
            ch: ' ',
            btrim: btrim,
            endpos: 0,
            esmap: esmap,
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

    pub fn process_char(&self, x: &mut Ctxt) -> Result<Action, String> {
        match *self {
            CharType::EscSeq(chk) => {
                if !x.bescape {
                    if x.ch != chk {
                        return Ok(Action::ContinueChain);
                    }
                    match x.mphase {
                        Phase::BtwNormal | Phase::BtwString | Phase::BtwBracket(_) => {
                            x.bescape = true;
                            return Ok(Action::NextChar);
                        }
                        _ => {
                            return Ok(Action::ContinueChain);
                        }
                    }
                }
                let replace = x.esmap.get(&x.ch);
                if replace.is_none() {
                    return Err(format!("CharType:ProcessChar:Unknown escseq [{}]", x.ch));
                }
                x.tok.push(*replace.unwrap());
                x.bescape = false;
                return Ok(Action::NextChar);
            }
            CharType::DelimSpace(chk) => {
                if x.ch != chk {
                    return Ok(Action::ContinueChain);
                }
                match x.mphase {
                    Phase::Begin => {
                        if x.btrim {
                            return Ok(Action::NextChar);
                        }
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                    Phase::BtwNormal => {
                        if chk == x.cend {
                            x.endpos = x.chpos;
                            return Ok(Action::DoneBreak);
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
                        x.endpos = x.chpos;
                        if x.btrim {
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
                        x.endpos = x.chpos;
                        return Ok(Action::DoneBreak);
                    }
                    _ => {
                        if chk == x.cend {
                            x.endpos = x.chpos;
                            return Ok(Action::DoneBreak);
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
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                    Phase::BtwString => {
                        x.mphase = Phase::EndCleanup;
                        x.endpos = x.chpos;
                        x.tok.push(x.ch);
                        return Ok(Action::NextChar);
                    }
                    Phase::EndCleanup => {
                        x.endpos = x.chpos;
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
                            return Err(format!("CharType:ProcessChar:Opening bracket [{}] at begining of token???", bchk));
                        }
                        Phase::BtwNormal => {
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
                            x.endpos = x.chpos;
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
                                x.endpos = x.chpos;
                                x.mphase = Phase::EndCleanup;
                                return Ok(Action::NextChar);
                            }
                            x.mphase = Phase::BtwBracket(cnt);
                            return Ok(Action::NextChar);
                        }
                        Phase::EndCleanup => {
                            x.endpos = x.chpos;
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
                        x.endpos = x.chpos;
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

pub fn default_vcharprocs() -> Vec<CharType> {
    let mut vcp = Vec::new();
    vcp.push(CharType::EscSeq('\\'));
    vcp.push(CharType::DelimSpace(' '));
    vcp.push(CharType::DelimString('"'));
    vcp.push(CharType::DelimBracket('(', ')'));
    vcp.push(CharType::Normal);
    return vcp;
}
