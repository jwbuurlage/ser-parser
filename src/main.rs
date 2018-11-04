#[macro_use]
extern crate nom;

#[derive(Debug,PartialEq)]
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
    assert_eq!(hex_color("#"), Ok(("", Color{red: 1, green:1, blue:1})));
}

fn main() {
    println!("Hello, world!");
}
