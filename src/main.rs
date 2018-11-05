#[macro_use]
extern crate nom;

extern crate hex;

use nom::{le_f64, le_i16, le_i32, le_i64};
use std::fs;

#[derive(Debug, PartialEq)]
pub enum ArrayDim {
    One,
    Two,
}

#[derive(Debug, PartialEq)]
pub enum TagType {
    TimeOnly,
    TimeAnd2D,
}

#[derive(Debug, PartialEq)]
pub struct DimArray<'a> {
    pub dimension_size: i32,
    pub calibration_offset: f64,
    pub calibration_delta: f64,
    pub calibration_element: i32,
    pub description: &'a str,
    pub units: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Ser<'a> {
    pub series_id: i16,
    pub series_version: i16,
    pub data_type: ArrayDim,
    pub tag_type: TagType,
    pub total_element_count: i32,
    pub valid_element_count: i32,
    pub array_offset: i64,
    pub dimension_arrays: Vec<DimArray<'a>>,
}

named!(
    tag_type<TagType>,
    alt!(
            tag!(&(hex::decode("52410000").unwrap())[..]) => { |_| TagType::TimeOnly } |
            tag!(&(hex::decode("42410000").unwrap())[..]) => { |_| TagType::TimeAnd2D }
        )
);

named!(
    array_dim<ArrayDim>,
    alt!(
        tag!(&(hex::decode("20410000").unwrap())[..]) => { |_| ArrayDim::One } |
        tag!(&(hex::decode("22410000").unwrap())[..]) => { |_| ArrayDim::Two }
    )
);

named!(
    dim_array<DimArray>,
    do_parse!(
        dimension_size: le_i32
            >> calibration_offset: le_f64
            >> calibration_delta: le_f64
            >> calibration_element: le_i32
            >> description_length: le_i32
            >> description: take_str!(description_length)
            >> units_length: le_i32
            >> units: take_str!(units_length)
            >> (DimArray {
                dimension_size,
                calibration_offset,
                calibration_delta,
                calibration_element,
                description,
                units
            })
    )
);

named!(ser_reader<&[u8], Ser>,
       do_parse!(
            tag!(&hex::decode("4949").unwrap()[..]) >>
            series_id: le_i16 >>
            series_version: le_i16 >>
            data_type: array_dim >>
            tag_type: tag_type >>
            total_element_count: le_i32 >>
            valid_element_count: le_i32 >>
            array_offset: alt!(
                cond_reduce!(series_version >= 544, le_i64) |
                map!(le_i32, |y| y as i64)
            ) >>
            number_dimensions: le_i32 >>
            dimension_arrays: count!(dim_array, number_dimensions as usize) >>
            (Ser { series_id, series_version, data_type, tag_type,
                   total_element_count, valid_element_count, array_offset, dimension_arrays })
       ));

#[test]
fn test_ser_parser() {
    let file = fs::read("test_data/-3_1.ser").expect("failed to open .ser file");

    let result = ser_reader(&file).expect("could not parse .ser file");
    println!("{:#?}", result.1)

    // assert_eq!(
    //     match ser_reader(&file) {
    //         Ok((_, b)) => Some(b),
    //         _ => None,
    //     },
    //     None
    // );

    // empty slice: &b""[..]
}

fn main() {
    let file = fs::read("test_data/-3_1.ser").expect("failed to open .ser file");

    let result = ser_reader(&file).expect("could not parse .ser file");
    println!("{:#?}", result.1);
}
