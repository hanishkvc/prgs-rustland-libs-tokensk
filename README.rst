##########
 tokensk
##########

Author: hanishkvc
Version: v20221104IST1511

Overview
##########

A simple tokeniser library written in rust, which allows one to extract the individual tokens
from a given string.

A token is any of the following

* a adjucent grouping of bunch of non space characters (if space is specified as the delimiter)

  * Atest123String 2ndToken ThirdTOken

* a bunch of characters including spaces, seperated by the specified non-space delimiter

  * a test    123  string, 2nd token, third TOken

* a string enclosed within quotes (double-quote by default)

  * " a test   123      string " "2nd token" "what the 3rd token"
  * " a test   123      string " , 2nd    token,     "what the 3rd token"

  * dont forget to escape the string boundry quote char, if it is present within the string
    somewhere.

* a bunch of bracketed content ('(' and ')' by default)

  * 1sttoken ( 2nd token what is this    , MeAPrefix(a bunch   of) bracketed content ) 3rdToken

  * a bracketed content can contain embedded bracketed contents within them.

  * dont forget to escape the bracket boundry chars, if they are present as part of a string
    literal within.


The logic is encapsulated into a new custom type called TStr.

One can control tokeniser behaviour by configuring certain properties/members of TStr instance,
and or deciding which helper method to call wrt extracting contents of the string.

These includes

* whether enclosing/boundry/marker quotes are retained or not wrt string tokens

* whether escape sequences if any found by TStr are expanded/processed or not.

* whether bracketed blocks additionally include a textual prefix wrt their 1st opening bracket.

  * meprefix( what else (what (no (no (nooo   not again) ) ) whats happening) )

* set the string and bracket boundry marker chars.

It provides methods for trimming the string, getting 1 token at a time or all tokens in 1 shot,
getting 1st or Nth or last char, split once or n-times wrt a given delimiter, peel a bracket
wrt its prefix name and members, ...

If one wants to share the same tokenisation characteristics across multiple TStr instances,
then instead of creating TStr directly, one can first create a TStrX instance, and setup the
required charactersistics. Then create TStr instances from that configured TStrX instance


Usage
#######

Look at the documentation in the source, as well as the sample test app and testlib for more info.

.. code-block:: rust

   #
   # Get tokens one at a time
   #
   tstr = TStr::from_str(" a test string", true);
   print!("INFO:Looking at [{}]", tstr);
   while tstr.remaining_len() > 0 {
        let tok = tstr.nexttok(' ', true);
        print!("\ttok[{}], remaining[{}]\n", tok, tstr.the_str());
   }
   #
   # Get tokens in one shot
   #
   tstr = TStr::from_str(" a test string", true);
   print!("\t1Short:[{}]", tstr.tokens_vec(' ', true, true).unwrap());
   #
   # Handle escape sequences
   #
   tstr = TStr::from_str(r" a test\tstring with\\t escape sequences", true);
   tstr = TStr::from_str_ex(r" a test\tstring with\\t escape sequences", true, Delimiters::default(), TStrX::escseqs_default(), Flags::default());
   #
   # Peel bracketted content
   #
   tstr = TStr::from_str("testme( a test, 123, msg in a bracket)", true);
   sprefix = tstr.peel_bracket('(');
   scontents = tstr.the_str();
   vContentTokens = tstr.tokens_vec(',', true, true).unwrap();
   #
   # Get first, nth and last chars
   #
   let tstr = TStr::from_str("0123456789 Test extracting chars ॐ", false);
   print!("TEST:FirstNthLast:{},{},{}",tstr.char_first().unwrap(), tstr.char_nth(8).unwrap(), tstr.char_last().unwrap());
   #
   # Use TStrX to share tokenisation characteristics wrt multiple TStr's if reqd
   #
   let tstrbuilder = TStrX::new()
   tstrbuilder.flags.mainbracket_beginstandalonge=false
   let tstr = tstrbuilder.from_str("    a    test string", true);
   print!("{}", tstr.nexttok(' ', true));
   print!("{}", tstr.nexttok(' ', true));
   let tstr = tstrbuilder.from_str("    a    test string", false);
   print!("{}", tstr.nexttok(' ', false));
   print!("{}", tstr.nexttok(' ', false));

