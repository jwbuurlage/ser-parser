extern crate image;
extern crate ser_parser;

use ser_parser::data::*;
use ser_parser::parsers::*;

use std::fs;

fn main() {
    let file = fs::read("test_data/-3_1.ser").expect("failed to open .ser file");

    let result = ser_header_parser(&file).expect("could not parse .ser file");
    println!("header: {:#?}", result.1);

    let offsets = ser_offsets_parser(
        &file[(result.1.array_offset as usize)..],
        result.1.series_version,
        result.1.total_element_count,
    )
    .expect("could not parse offsets");
    println!("offsets: {:#?}", offsets.1);

    // FIXME for each tag
    let tags = ser_data_tag_parser(
        &file[(offsets.1.tag_offset[0] as usize)..],
        result.1.tag_type,
    )
    .expect("could not parse tag");
    println!("tags: {:#?}", tags.1);

    // FIXME for each offset
    let data = ser_data_parser(
        &file[(offsets.1.data_offset[0] as usize)..],
        result.1.array_dim,
    )
    .expect("could not parse data");
    // FIXME what about valid elements

    match data.1 {
        SerData::TwoDim(the_data) => match the_data.data {
            SerRawData::DataU16(raw) => {
                let max: f64 = *raw.iter().max().unwrap() as f64;
                let raw_as_u8: Vec<u8> = raw
                    .iter()
                    .map(|x| (((*x as f64) / max) * 255.0) as u8)
                    .collect();
                image::save_buffer(
                    "test.png",
                    &raw_as_u8[..],
                    the_data.array_size_y as u32,
                    the_data.array_size_x as u32,
                    image::ColorType::L8,
                )
                .unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
