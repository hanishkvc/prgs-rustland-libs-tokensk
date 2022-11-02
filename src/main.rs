//!
//! Showcase as well as test TokensK
//! HanishKVC, 2022
//!


use tokensk::TStr;


fn test_create() {
    //let mut str2 = TStr::from_str(&"  A string 21string ".to_string());
    let thestring = "  A string 21string ".to_string();
    let mut str2 = TStr::from_str(&thestring);
    let mut str3 = TStr::from_str(" A str 12string  ");
    print!("Created TStrings: {:?}, {:?}\n", str2, str3);
    //print!("Str2:{}:p{},s{}\n", str2.the_str(), str2.space_prefixs(), str2.space_suffixs());
    print!("Str2:p{},s{}:{}\n", str2.space_prefixs(), str2.space_suffixs(), str2.the_str());
    print!("Str3:p{},s{}:{}\n", str3.space_prefixs(), str3.space_suffixs(), str3.the_str());
}

fn test_create_raw() {
    let thestring = "  A string 21string ".to_string();
    let mut str2 = TStr::from_str(&thestring);
    let mut str3 = TStr::from_str(" A str 12string  ");
    print!("Str2:{}:p{},s{}\n", str2.the_str(), str2.space_prefixs_raw(), str2.space_suffixs_raw());
    print!("Str3:{}:p{},s{}\n", str3.the_str(), str3.space_prefixs_raw(), str3.space_suffixs_raw());
    str2.trim();
    str3.trim();
    print!("Str2:{}:p{},s{}\n", str2.the_str(), str2.space_prefixs_raw(), str2.space_suffixs_raw());
    print!("Str3:{}:p{},s{}\n", str3.the_str(), str3.space_prefixs_raw(), str3.space_suffixs_raw());
}

fn test_nexttoken() {
    let testlines = vec![
        "what now",
        "   hello wold ",
        "  123 hello    0x123",
        "  test( \"hello  world\", 123, what(0x123))",
        "\" lests check brackets within string what(yes, notnow,) \"",
        "\" lests check brackets within string what(yes, notnow,) ending quote missing",
        "  test( \"hello  world\", 123, what((0x123)), extra bracket at begin",
        "  test( \"hello  world\", 123, what((0x123)))), extra bracket at end",
        ];
    for line in testlines {
        let mut tline = TStr::from_str(line);
        print!("INFO:Test:NextTok:Line:[{}]\n", line);
        while tline.remaining_len() > 0 {
            let gottok = tline.nexttok(true).expect("ERRR:TestNextTok");
            print!("\ttok[{}]; rem[{}]\n", gottok, tline.the_str());
        }
    }
}

fn main() {
    test_create();
    test_create_raw();
    test_nexttoken();
}
