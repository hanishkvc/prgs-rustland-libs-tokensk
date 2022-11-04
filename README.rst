##########
tokensk
##########

Author: hanishkvc
Version: v20221104IST1511

Overview
###########

A simple tokeniser library written in rust, which allows one to extract the individual tokens
from a given string.

A token is any of the following

* a adjucent grouping of bunch of non space characters (if space is specified as the delimiter)

  * Atest123String 2ndToken ThirdTOken

* a bunch of characters including spaces, seperated by the specified non-space delimiter

  * a test    123  string, 2nd token, third TOken

* a string enclosed within double quotes

  * " a test   123      string " "2nd token" "what the 3rd token"
  * " a test   123      string " , 2nd    token,     "what the 3rd token"

* a bunch of bracketed content

  * 1sttoken ( 2nd token what is this    , MeAPrefix(a bunch   of) bracketed content ) 3rdToken


The logic is encapsulated into a new custom type called TStr.

One can control tokeniser behaviour by configuring certain properties/members of TStr instance,
and or deciding which helper method to call wrt extracting contents of the string.


This includes

* whether enclosing double quotes are retained or not wrt string tokens

* whether escape sequences if any in the given string are expanded/processed or not.

* whether bracketed blocks additionally include a textual prefix wrt their 1st opening bracket.

  * meprefix( what else (what (no (no (nooo   not again) ) ) whats happening) )

It provides methods for trimming the string, getting 1 token at a time or all tokens in 1 shot,
getting 1st or last char, split once wrt a given delimiter, peel a bracket wrt its prefix name
and members, ...


Usage
#######

Look at the documentation in the source, as well as the sample test app and testlib for more info.

A code sample::

   #
   # Get tokens one at a time
   #
   tstr = TStr::from_str(" a test string");
   print!("INFO:Looking at [{}]", tstr);
   while tstr.remaining_len() > 0 {
        let tok = tstr.nexttok(' ', true);
        print!("\ttok[{}], remaining[{}]\n", tok, tstr.the_str());
   }
   #
   # Get tokens in one shot
   #
   tstr = TStr::from_str(" a test string");
   print!("\t1Short:[{}]", tstr.tokens_vec(' ', true, true).unwrap());
   #
   # Handle escape sequences
   #
   tstr = TStr::from_str(" a test\tstring with\\t escape sequences");
   tstr.escseq_defaults();
   tstr = TStr::from_str_ex(" a test\tstring with\\t escape sequences", true, true);
   #
   # Peel bracketted content
   #
   tstr = TStr::from_str("testme( a test, 123, msg in a bracket)");
   sprefix = tstr.peel_bracket('(');
   scontents = tstr.the_str();
   vContentTokens = tstr.tokens_vec(',', true, true).unwrap();
   #
   # Get first, nth and last chars
   #
   let tstr = TStr::from_str("0123456789 Test extracting chars ॐ"");
   print!("TEST:FirstNthLast:{},{},{}",tstr.char_first().unwrap(), tstr.char_nth(8).unwrap(), tstr.char_last().unwrap());


