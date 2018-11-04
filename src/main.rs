#[macro_use]
extern crate nom;

use nom::{be_i16,be_u64};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

named!(hex_color<&str, Color>,
       do_parse!(
           tag!("#") >>
           (Color {red: 1, green: 1, blue: 1})
       )
);

#[test]
fn test_parser() {
    assert_eq!(
        hex_color("#"),
        Ok((
            "",
            Color {
                red: 1,
                green: 1,
                blue: 1
            }
        ))
    );
}

// --------------------------------------

#[derive(Debug, PartialEq)]
pub struct Ser {
    pub byte_order: i16,
}

named!(ser_reader<&[u8], Ser>,
       do_parse!(
           tag!(b"test") >>
           byte_order: be_i16 >>
           (Ser { byte_order })
       ));

#[test]

fn test_ser_parser() {
    assert_eq!(
        match ser_reader(b"test                 ") {
            Ok((_, b)) => Some(b),
            _ => None
        },
        None
    );

    // empty slice: &b""[..]
}

fn main() {
    println!("Hello, world!");
}
