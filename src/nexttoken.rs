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
    cend: char,
    mphase: Phase,
    bescape: bool,
    tok: String,
    pub chpos: usize,
    pub ch: char,
}

impl Ctxt {

    pub fn new(thestr: &str, dlimdef: char) -> Ctxt {
        Ctxt {
            vchars: thestr.char_indices().collect(),
            cend: dlimdef,
            mphase: Phase::Begin,
            bescape: false,
            tok: String::new(),
            chpos: 0,
            ch: ' ',
        }
    }

}


enum Action {
    Add(char),
    NextChar,
    Continue,
}

enum Delimiter {
    Space(char),
    String(char),
    Bracket(char, char)
}

impl Delimiter {

    pub fn process_char(&self, x: &mut Ctxt) -> Action {
        match *self {
            Delimiter::Space(chk) => {
                if x.ch == chk {

                }
                Action::NextChar
            },
            Delimiter::String(chk) => todo!(),
            Delimiter::Bracket(bchk, echk) => todo!(),
        }
    }
}

