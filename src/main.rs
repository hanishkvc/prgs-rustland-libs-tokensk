//!
//! Showcase as well as test TokensK
//! HanishKVC, 2022
//!

use tokensk as toks;

fn test_create() {
    //let mut str1 = toks::TString { theStr: "A direct string string".to_string(), spacePrefixs: 0, spaceSuffixs: 0 };
    let mut str2 = toks::TString::from_string("  A string 21string ".to_string());
    let mut str3 = <toks::TString as std::str::FromStr>::from_str(" A str 12string  ").unwrap();
    //print!("Created TStrings: {:?}, {:?}, {:?}", str1, str2, str3);
    print!("Created TStrings: {:?}, {:?}", str2, str3);
    //print!("Str1: {}, {}", str1.space_prefixs(), str1.space_suffixs());
    print!("Str2: {}, {}", str2.space_prefixs(), str2.space_suffixs());
    print!("Str3: {}, {}", str3.space_prefixs(), str3.space_suffixs());
}

fn main() {
    test_create();
}