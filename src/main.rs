//!
//! Showcase as well as test TokensK
//! HanishKVC, 2022
//!


//use tokensk::TStr;
use tokensk::testlib;


fn main() {
    testlib::test_create();
    testlib::test_create_raw();
    testlib::test_nexttoken();
    testlib::test_escseq();
}
