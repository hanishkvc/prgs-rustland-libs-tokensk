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
    End,
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
                            return Action::ContinueChain;
                        }
                        x.tok.push(x.ch);
                        return Action::ContinueChain;
                    }
                    Phase::BtwNormal | Phase::BtwString | Phase::BtwBracket(_) => {
                        x.tok.push(x.ch);
                        return Action::ContinueChain;
                    }
                    Phase::EndCleanup(_) => {
                        if x.btrim {
                            x.mphase = Phase::EndCleanup(x.chpos);
                            return Action::NextChar;
                        }
                        return Action::ContinueChain;
                    }
                    Phase::End => {
                        return 
                    }

                    _ => todo!()
                }
                Action::ContinueChain
            },
            CharType::DelimString(chk) => todo!(),
            CharType::DelimBracket(bchk, echk) => todo!(),
        }
    }
}

