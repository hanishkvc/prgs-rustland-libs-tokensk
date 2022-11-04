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


Usage
#######

Look at the documentation in the source, as well as the sample test app and testlib for more info.

A code sample::

   tstr = TStr::from_str(" a test string")
   print!("INFO:Looking at [{}]", tstr);
   while tstr.remaining_len() > 0 {
        let tok = tstr.nexttok(' ', true);
        print!("\ttok[{}], remaining[{}]\n", tok, tstr.the_str());
   }


