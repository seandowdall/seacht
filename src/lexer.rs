use std::str::Chars;
/*

The <'a> syntax in rust defines a lifetime parameter. Lifetimes are how
rust rensures memory safety without a garbage collector.
They tell the compiler how long references should be valid.

The usage in this case of the lexer means, "the lexer can only live as long as the sources string."

Breakdown of <'a>

<> angle brackets are used for generic parameters.
it could be <T> for type parameters like: struct Wrapper<T> {value: T}
so <> is just saying "hey this struct takes some kind of parameter"

'a is the name of a lifetime.
It's just a label and technically any value could be used 'b -> but 'a is most common.

Why does <'a> exist?
Because rust does not have a garbage collector and needs to track how long references live.
So by using <'a> we are telling rust "This struct is generic over a lifetime 'a and any
references it contains will live at least as long as 'a"


*/

struct Lexer<'a> {
    // Source Text
    source: &'a str,

    // The remaining characters
    chars: Chars<'a>,
}

// Why to lifetimes passed as params?
// impl<'a> declares a lifetime parameter for the implementation block.
// Lecer<'a> uses the same lifetime parameter for the struct.

// Think of it like this:
// The first <'a> in impl<'a> says "this implementation block is working with a lifetime parameter named 'a'"
// The second <'a> in Lexer<'a> says "we're implementing methods for the Lexer struct that uses this lifetime parameter"
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        /* This is the same as saying:

          ... -> Lexer {
          Lexer {
              source,
              chars: source.chars(),
          }

          Self is just a shorthand for the current type: "Lexer<'a>" in this case;
          Using self is preferred, it's shorter and cleaner.
          And if you ever rename the struct you don't need to update the return type.
          It's more idiomatic rust.
        */
        Self {
            source,
            chars: source.chars(),
        }
    }

    // &mut -> a mutable reference
    fn read_next_kind(&mut self) -> Kind {}
}
