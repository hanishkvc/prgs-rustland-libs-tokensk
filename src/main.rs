//!
//! Showcase as well as test TokensK
//! HanishKVC, 2022
//!

use tokensk as toks;

fn test_create() {
    let mut str2 = toks::TString::from_string("  A string 21string ".to_string());
    let mut str3 = <toks::TString as std::str::FromStr>::from_str(" A str 12string  ").unwrap();
    print!("Created TStrings: {:?}, {:?}\n", str2, str3);

    //print!("Str2:{}:p{},s{}\n", str2.the_str(), str2.space_prefixs(), str2.space_suffixs());
    print!("Str2:p{},s{}:{}\n", str2.space_prefixs(), str2.space_suffixs(), str2.the_str());
    print!("Str3:p{},s{}:{}\n", str3.space_prefixs(), str3.space_suffixs(), str3.the_str());
}

fn test_create_raw() {
    let mut str2 = toks::TString::from_string("  A string 21string ".to_string());
    let mut str3 = <toks::TString as std::str::FromStr>::from_str(" A str 12string  ").unwrap();
    print!("Str2:{}:p{},s{}\n", str2.the_str(), str2.space_prefixs_raw(), str2.space_suffixs_raw());
    print!("Str3:{}:p{},s{}\n", str3.the_str(), str3.space_prefixs_raw(), str3.space_suffixs_raw());
    str2.trim();
    str3.trim();
    print!("Str2:{}:p{},s{}\n", str2.the_str(), str2.space_prefixs_raw(), str2.space_suffixs_raw());
    print!("Str3:{}:p{},s{}\n", str3.the_str(), str3.space_prefixs_raw(), str3.space_suffixs_raw());
}

fn main() {
    test_create();
    test_create_raw();
}
