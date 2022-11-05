//!
//! NextToken
//! HanishKVC, 2022
//!


enum Phase {
    Begin,
    CheckStart,
    BtwNormal,
    BtwString,
    /// Maintain current open brackets count
    BtwBracket(usize),
    /// Maintain current tok's end position in source string
    EndCleanup(usize),
}

pub(crate) struct Ctxt {
    pub vchars: Vec<(usize, char)>,
    dlimdef: char,
    cend: char,
    mphase: Phase,
    bescape: bool,
    tok: String,
    pub chpos: usize,
    pub ch: char,
    btrim: bool,
}

impl Ctxt {

    pub fn new(thestr: &str, dlimdef: char, btrim: bool) -> Ctxt {
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
        }
    }

}


enum Action {
    NextChar,
    ContinueChain,
    DoneBreak,
}

enum CharType {
    DelimSpace(char),
    DelimNormal(char),
    DelimString(char),
    DelimBracket(char, char),
    Normal,
}

impl CharType {

    pub fn process_char(&self, x: &mut Ctxt) -> Action {
        match *self {
            CharType::DelimSpace(chk) => {
                if x.ch != chk {
                    return Action::ContinueChain;
                }
                match x.mphase {
                    Phase::Begin => {
                        if x.btrim {
                            return Action::NextChar;
                        }
                        x.tok.push(x.ch);
                        return Action::NextChar;
                    }
                    Phase::BtwNormal => {
                        if chk == x.cend {
                            return Action::DoneBreak;
                        }
                        x.tok.push(x.ch);
                        return Action::NextChar;
                    }
                    Phase::BtwString | Phase::BtwBracket(_) => {
                        // NOTE: For now not worrying about delim within string or bracket token needing to be escaped.
                        x.tok.push(x.ch);
                        return Action::NextChar;
                    }
                    Phase::EndCleanup(_) => {
                        if x.btrim {
                            x.mphase = Phase::EndCleanup(x.chpos);
                            return Action::NextChar;
                        }
                        return Action::DoneBreak;
                    }
                    _ => todo!()
                }
            },
            CharType::DelimNormal(chk) => {
                if x.ch != chk {
                    return Action::ContinueChain;
                }
                todo!()
            }
            CharType::DelimString(chk) => {
                if x.ch != chk {
                    return Action::ContinueChain;
                }
                todo!()
            },
            CharType::DelimBracket(bchk, echk) => {
                if (x.ch != bchk) && (x.ch != echk) {
                    return Action::ContinueChain;
                }
                todo!()
            },
            CharType::Normal => {
                x.tok.push(x.ch);
                return Action::NextChar;
            },
        }
    }

}

