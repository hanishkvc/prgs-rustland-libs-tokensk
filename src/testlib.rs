//!
//! Test code for this library
//! HanishKVC, 2022
//!

//!
//! This is common code which is inturn used by
//! * the tests module within the library source, as well as
//! * the main.rs test app, to stop rust from stupidly complaining
//!   about unused / dead code wrt the library's methods etal
//!

use std::collections::HashMap;

use crate::{TStr, Flags, TStrX, Delimiters};

const MTAG: &str = "TEST:TestLib";

pub fn test_create() {
    let mtag = format!("{}:TestCreate", MTAG);
    let mut str1 = TStr {
        theStr: "A direct string string",
        trimmedPrefixCnt: 0,
        trimmedSuffixCnt: 0,
        escSeqMap: HashMap::new(),
        flags: Flags::default(),
        delims: Delimiters::default(),
    };
    let thestring = "  A string 21string ".to_string();
    let mut str2 = TStr::from_str(&thestring, false);
    let mut str3 = TStr::from_str(" A str 12string  ", false);
    print!("{}:Created TStrs: {:?}, {:?}, {:?}\n", mtag, str1, str2, str3);
    //print!("Str1:{}:p{},s{}\n", str1.the_str(), str1.space_prefixs(), str1.space_suffixs());
    print!("{}:Str1:p{},s{}:{}\n", mtag, str1.trimmed_prefix_cnt(), str1.trimmed_suffix_cnt(), str1.the_str());
    print!("{}:Str2:p{},s{}:{}\n", mtag, str2.trimmed_prefix_cnt(), str2.trimmed_suffix_cnt(), str2.the_str());
    print!("{}:Str3:p{},s{}:{}\n", mtag, str3.trimmed_prefix_cnt(), str3.trimmed_suffix_cnt(), str3.the_str());
}

pub fn test_create_raw() {
    let mtag = format!("{}:TestCreateRaw", MTAG);
    let thestring = "  A string 21string ".to_string();
    let mut str2 = TStr::from_str(&thestring, false);
    let mut str3 = TStr::from_str(" A str 12string  ", false);
    print!("{}:Str2:{}:p{},s{}\n", mtag, str2.the_str(), str2.trimmed_prefix_cnt_raw(), str2.trimmed_suffix_cnt_raw());
    print!("{}:Str3:{}:p{},s{}\n", mtag, str3.the_str(), str3.trimmed_prefix_cnt_raw(), str3.trimmed_suffix_cnt_raw());
    str2.trim();
    str3.trim();
    print!("{}:Str2:{}:p{},s{}\n", mtag, str2.the_str(), str2.trimmed_prefix_cnt_raw(), str2.trimmed_suffix_cnt_raw());
    print!("{}:Str3:{}:p{},s{}\n", mtag, str3.the_str(), str3.trimmed_prefix_cnt_raw(), str3.trimmed_suffix_cnt_raw());
}


pub fn test_nexttoken_ex(testlines: Vec<&str>, dlimdef: char) {
    let mtag = format!("{}:TestNextToken", MTAG);
    print!("\n\n\n\n{}: **** Lets test nexttoken [{}] ****\n\n", mtag, dlimdef);
    let mut tline = TStr::from_str("", true);
    for line in testlines {
        tline.set_str(line, false);
        print!("{}:Line:[{}]\n", mtag, line);
        while tline.remaining_len() > 0 {
            let gottok = tline.nexttok(dlimdef, true);
            if gottok.is_err() {
                print!("ERRR:{}:{:?}\n", mtag, gottok.unwrap_err());
            } else {
                let gottok = gottok.unwrap();
                print!("\ttok[{}]; rem[{}]\n", gottok, tline.the_str());
            }
        }
        // use set_str and tokens_vec to rescan into a vector
        tline.set_str(line, false);
        let vtoks = tline.tokens_vec(dlimdef, true, true);
        if vtoks.is_err() {
            print!("ERRR:{}:FullSet:{}\n", mtag, vtoks.unwrap_err());
        } else {
            print!("\tFullSet:{:#?}\n", vtoks.unwrap());
        }
    }
}

pub fn test_nexttoken() {
    let testlines = vec![
        "what now",
        "   hello\n wold ",
        "  123 hello\\n    0x123",
        "  test( \"hello  world\", 123, what(0x123))",
        "\" lests chec\tk brackets within string what(yes, notnow,) \"",
        "\" lests chec\\tk brackets within string what(yes, notnow,) ending quote missing",
        "  test( \"hello  world\", 123, what((0x123)), extra bracket at begin",
        "  test( \"hello  world\", 123, what((0x123)))), extra bracket at end",
    ];
    test_nexttoken_ex(testlines.clone(), ' ');
    test_nexttoken_ex(testlines, ',');
    let testlines = vec![
        "line with spaces and, commas,yes   ,commas",
        " test(what,now with space,also)   ,,bit more text "
    ];
    test_nexttoken_ex(testlines.clone(), ' ');
    test_nexttoken_ex(testlines.clone(), ',');
}

pub fn test_peel_bracket() {
    let mut tstr = TStr::from_str("testbracket( 123, abc, \" msg inside bracket\")", false);
    let prefix = tstr.peel_bracket('(').unwrap();
    print!("TEST:PeelBracket:prefix[{}], contents[{}]\n", prefix, tstr.the_str());
}

pub fn test_peel_string() {
    let tstrx = TStrX::new();
    let delim = '"';
    let tstr = tstrx.from_str("   \"A string with double quote at one end  ", false);
    let mut rstr = tstr.clone();
    let gotr = rstr.peel_string(delim);
    if gotr.is_err() {
        print!("TEST:PeelString:[{}]:StringDelim:{}:Expected Failure:{}\n", tstr, delim, gotr.unwrap_err());
    } else {
        print!("DBUG:PeelString:[{}]:StringDelim:{}:Unexpected Success:{}\n", tstr, delim, rstr);
    }
    let tstr = tstrx.from_str("              \"A string with double quote at both end\"  ", false);
    let mut rstr = tstr.clone();
    let gotr = rstr.peel_string(delim);
    if gotr.is_err() {
        print!("TEST:PeelString:[{}]:StringDelim:{}:Unexpected Failure:{}\n", tstr, delim, gotr.unwrap_err());
    } else {
        print!("DBUG:PeelString:[{}]:StringDelim:{}:Expected Success:{}\n", tstr, delim, rstr);
    }
}

pub fn test_first_nth_last() {
    let tstr = TStr::from_str("0123456789 Test extracting chars ॐ", false);
    print!("TEST:FirstNthLast:{},{},{}\n",tstr.char_first().unwrap(), tstr.char_nth(8).unwrap(), tstr.char_last().unwrap());
}

pub fn test_splitn_ex(instr: &str, splitn: usize, dlimdef: char) {
    let mut tstr = TStr::from_str(instr, false);
    let vstrs = tstr.splitn(splitn, dlimdef).unwrap();
    print!("TEST:SplitN:{}:{}-[{}]:[{:?}]\n", instr, splitn, dlimdef, vstrs);
}

pub fn test_splitn() {
    test_splitn_ex("one two three four five", 3, ' ');
    test_splitn_ex("oneXXtwoXthree four five", 3, 'X');
    test_splitn_ex("oneXXtwoXthree four five", 1, 'X');
    test_splitn_ex("one two three four five", 3, 'X');
}

pub fn test_escseq() {
    let mut tstrx = TStrX::new();
    let sstr = r"test \v escseqs \\v also \t and \\t. Ok done";
    let mut tstr = tstrx.from_str(sstr, false);
    print!("TEST:EscSeq:Default:tstr[{:#?}]\n", tstr);
    let vtoks = tstr.tokens_vec(';', true, false);
    if vtoks.is_err() {
        print!("TEST:EscSeq:Default:WillFail:{}\n", vtoks.unwrap_err());
    } else {
        let vtoks = vtoks.unwrap();
        print!("TEST:EscSeq:Default:[{:#?}]\n", vtoks);
    }

    let estr = r"test W escseqs \\v also \n and \\t. Ok done";
    print!("TEST:EscSeq:In[{}], Out[{}]\n", sstr, estr);
    tstrx.escseqs_set('v', 'W');
    tstrx.escseqs_set('t', '\n');
    let mut tstr = tstrx.from_str(sstr, true);
    print!("TEST:EscSeq:Updated:tstr[{:#?}]\n", tstr);
    let vtoks = tstr.tokens_vec(';', true, false).unwrap();
    print!("TEST:EscSeq:Updated:Debug:[{:#?}]\n", vtoks);
    print!("TEST:EscSeq:Updated:String:[{}]\n", vtoks[0]);
}

pub fn test_tstrx() {
    let sstr1 = "    test,   me  ";
    let sstr2 = "    test,   [me, hello world], [[save nature], [save earth]]  ";
    let mut tstrx = TStrX::new();
    tstrx.flags.trim = false;
    tstrx.flags.mainbracket_beginprefixed = false;
    tstrx.escseqs_set('v', 'W');
    tstrx.delims.bracket = ('[',']');

    let mut tstr = tstrx.from_str(sstr1, true);
    print!("TEST:TStrX:Trimmed:[{:?}]\n", tstr.tokens_vec(',', true, false).unwrap());

    let mut tstr = tstrx.from_str(sstr2, false);
    print!("TEST:TStrX:UnTrimd:[{:?}]\n", tstr.tokens_vec(',', false, false).unwrap());

}

pub fn test_string_subparts() {
    let sstr1 = r#""skey1":"svalue2","skey2":"svalue2", what else, "nothing new" another, "again again", the end"#;
    let mut tstrx = TStrX::new();
    tstrx.flags.string_canbe_asubpart = true;
    tstrx.flags.blocktok_dlimuser_endreqd = false;
    let mut tstr = tstrx.from_str(sstr1, true);
    let toks = tstr.tokens_vec(',', true, false).unwrap();
    println!("TEST:StringSubParts:>>{}<<:>>{:?}<<", sstr1, toks);
}

pub fn test_multibrackets() {
    let sstr1 = r#""skey1":"svalue2","skey2":"svalue2", what else, ["nothing new" another], {"again again", the end}, "skey3":[{what now}, {again}]  , no more"#;
    let mut tstr = TStr::from_str(sstr1, true);
    tstr.delims.bracket = ('{','}');
    tstr.delims.obracket = Some(('[',']'));
    tstr.flags.string_canbe_asubpart = true;
    tstr.flags.blocktok_dlimuser_endreqd = false;
    let toks = tstr.tokens_vec(',', true, false).unwrap();
    println!("TEST:MultiBrackets:>>{}<<:>>{:#?}<<", sstr1, toks);
}
