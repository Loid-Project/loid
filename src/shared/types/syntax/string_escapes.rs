use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum StringEscape { // TODO: add a lot more
        // spacing
        Newline => "\\n",   // \n
        Tab     =>"\\t", //     \t


        //quoting
        Quote => "\\\"", //  \"

        // other
        Slash   => "\\", // \
        Fstring => "\\(" // \(

    }
}
