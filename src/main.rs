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
    print!("Created TStrings: {:?}, {:?}\n", str2, str3);
    //print!("Str1: {}, {}", str1.space_prefixs(), str1.space_suffixs());
    print!("Str2: {}, {}\n", str2.space_prefixs(), str2.space_suffixs());
    print!("Str3: {}, {}\n", str3.space_prefixs(), str3.space_suffixs());
}

fn test_print_su(instr: &str, inusize: usize) {
    print!("{}, {}", instr, inusize);
    //std::io::stdout().write_fmt("{}, {}", instr, inusize);
}

fn test_print_iu(inisize: isize, inusize: usize) {
    print!("{}, {}", inisize, inusize);
}

fn test_print_uu(inusize1: usize, inusize2: usize) {
    print!("{}, {}", inusize1, inusize2);
}


fn test_create_raw() {
    let mut str2 = toks::TString::from_string("  A string 21string ".to_string());
    let mut str3 = <toks::TString as std::str::FromStr>::from_str(" A str 12string  ").unwrap();
    print!("Str2: {}, {}, {}\n", str2.the_str(), str2.space_prefixs_raw(), str2.space_suffixs_raw());
    print!("Str3: {}, {}, {}\n", str3.the_str(), str3.space_prefixs_raw(), str3.space_suffixs_raw());
    str2.trim();
    str3.trim();
    print!("Str2: {}, {}, {}\n", str2.the_str(), str2.space_prefixs_raw(), str2.space_suffixs_raw());
    print!("Str3: {}, {}, {}\n", str3.the_str(), str3.space_prefixs_raw(), str3.space_suffixs_raw());
    // the no go zone
    test_print_su(str2.the_str(), str2.space_suffixs());
    test_print_iu(str2.space_prefixs_raw(), str2.space_suffixs());
    test_print_uu(str2.space_prefixs(), str2.space_suffixs());

    print!("Str2: {}, {}\n", str2.the_str(), str2.space_suffixs());
    print!("Str2: {}, {}\n", str2.space_suffixs(), str2.the_str());
    print!("Str2: {}, {}\n", str2.space_prefixs_raw(), str2.space_suffixs());
    print!("Str2: {}, {}\n", str2.space_prefixs(), str2.space_suffixs());
}

fn main() {
    test_create();
    test_create_raw();
}