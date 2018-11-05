#[macro_use]
extern crate nom;
extern crate hex;
extern crate image;

use nom::{le_f32, le_f64, le_i16, le_i32, le_i64, le_i8, le_u16, le_u32, le_u8};
use std::fs;

#[derive(Debug, PartialEq)]
pub enum ArrayDim {
    One,
    Two,
}

#[derive(Debug, PartialEq)]
pub enum TagType {
    Time,
    TimePos,
}

#[derive(Debug, PartialEq)]
pub enum SerRawData {
    DataU8(Vec<u8>),
    DataU16(Vec<u16>),
    DataU32(Vec<u32>),
    DataI8(Vec<i8>),
    DataI16(Vec<i16>),
    DataI32(Vec<i32>),
    DataF32(Vec<f32>),
    DataF64(Vec<f64>),
}

#[derive(Debug, PartialEq)]
pub struct DimArray {
    pub dimension_size: i32,
    pub calibration_offset: f64,
    pub calibration_delta: f64,
    pub calibration_element: i32,
    pub description: String,
    pub units: String,
}

#[derive(Debug, PartialEq)]
pub struct SerHeader {
    pub series_id: i16,
    pub series_version: i16,
    pub array_dim: ArrayDim,
    pub tag_type: TagType,
    pub total_element_count: i32,
    pub valid_element_count: i32,
    pub array_offset: i64,
    pub dimension_arrays: Vec<DimArray>,
}

#[derive(Debug, PartialEq)]
pub struct SerOffsets {
    pub data_offset: Vec<i64>,
    pub tag_offset: Vec<i64>,
}

#[derive(Debug, PartialEq)]
pub struct SerDataOneDim {
    calibration_offset: f64,
    calibration_delta: f64,
    calibration_element: i32,
    data_type: i16,
    array_length: i32,
    data: SerRawData,
}

#[derive(Debug, PartialEq)]
pub struct SerDataTwoDim {
    calibration_offset_x: f64,
    calibration_delta_x: f64,
    calibration_element_x: i32,
    calibration_offset_y: f64,
    calibration_delta_y: f64,
    calibration_element_y: i32,
    data_type: i16,
    array_size_x: i32,
    array_size_y: i32,
    data: SerRawData,
}

#[derive(Debug, PartialEq)]
pub enum SerData {
    OneDim(SerDataOneDim),
    TwoDim(SerDataTwoDim),
}

#[derive(Debug, PartialEq)]
pub struct SerDataTagTime {
    // FIXME this should be f32 according to spec, i32 according to reference parser
    time: i32,
}

#[derive(Debug, PartialEq)]
pub struct SerDataTagTimePos {
    // FIXME this should be f32 according to spec, i32 according to reference parser
    time: i32,
    position_x: f64,
    position_y: f64,
}

#[derive(Debug, PartialEq)]
pub enum SerDataTag {
    Time(SerDataTagTime),
    TimePos(SerDataTagTimePos),
}

/// Parse if the data is time only or time and 2D
named!(
    tag_type<TagType>,
    alt!(
            tag!(&(hex::decode("52410000").unwrap())[..]) => { |_| TagType::Time } |
            tag!(&(hex::decode("42410000").unwrap())[..]) => { |_| TagType::TimePos }
        )
);

/// Parse if the array dimensions are 1D or 2D
named!(
    array_dim<ArrayDim>,
    alt!(
        tag!(&(hex::decode("20410000").unwrap())[..]) => { |_| ArrayDim::One } |
        tag!(&(hex::decode("22410000").unwrap())[..]) => { |_| ArrayDim::Two }
    )
);

/// Parse information of a dimension
named!(
    dim_array<DimArray>,
    do_parse!(
        dimension_size: le_i32
            >> calibration_offset: le_f64
            >> calibration_delta: le_f64
            >> calibration_element: le_i32
            >> description_length: le_i32
            >> description: map!(take_str!(description_length), |s| s.to_string())
            >> units_length: le_i32
            >> units: map!(take_str!(units_length), |s| s.to_string())
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

/// Parse an offset depending on version number
named_args!(parse_offset(series_version: i16)<i64>,
    alt!(
        cond_reduce!(series_version >= 544, le_i64) |
        map!(le_i32, |y| y as i64)
    )
);

/// Parse a FEI .ser file
named!(
    ser_header_parser<SerHeader>,
    do_parse!(
        tag!(&hex::decode("4949").unwrap()[..])
            >> series_id: le_i16
            >> series_version: le_i16
            >> array_dim: array_dim
            >> tag_type: tag_type
            >> total_element_count: le_i32
            >> valid_element_count: le_i32
            >> array_offset: call!(parse_offset, series_version)
            >> number_dimensions: le_i32
            >> dimension_arrays: count!(dim_array, number_dimensions as usize)
            >> (SerHeader {
                series_id,
                series_version,
                array_dim,
                tag_type,
                total_element_count,
                valid_element_count,
                array_offset,
                dimension_arrays
            })
    )
);

/// Parse ser offsets
named_args!(ser_offsets_parser(series_version: i16, total_element_count: i32)<SerOffsets>,
       do_parse!(
           data_offset: count!(call!(parse_offset, series_version), total_element_count as usize)
           >> tag_offset: count!(call!(parse_offset, series_version), total_element_count as usize)
           >> (SerOffsets { data_offset, tag_offset })
       )
);

named_args!(ser_raw_data_parser(data_type: i16, elements: usize)<SerRawData>,
            alt!(
                cond_reduce!(data_type == 1, map!(count!(le_u8, elements), SerRawData::DataU8)) |
                cond_reduce!(data_type == 2, map!(count!(le_u16, elements), SerRawData::DataU16)) |
                cond_reduce!(data_type == 3, map!(count!(le_u32, elements), SerRawData::DataU32)) |
                cond_reduce!(data_type == 4, map!(count!(le_i8, elements), SerRawData::DataI8)) |
                cond_reduce!(data_type == 5, map!(count!(le_i16, elements), SerRawData::DataI16)) |
                cond_reduce!(data_type == 6, map!(count!(le_i32, elements), SerRawData::DataI32)) |
                cond_reduce!(data_type == 7, map!(count!(le_f32, elements), SerRawData::DataF32)) |
                cond_reduce!(data_type == 8, map!(count!(le_f64, elements), SerRawData::DataF64))
            )
);

named!(
    ser_data_one_dim_parser<SerDataOneDim>,
    do_parse!(
        calibration_offset: le_f64
            >> calibration_delta: le_f64
            >> calibration_element: le_i32
            >> data_type: le_i16
            >> array_length: le_i32
            >> data: call!(ser_raw_data_parser, data_type, array_length as usize)
            >> (SerDataOneDim {
                calibration_offset,
                calibration_delta,
                calibration_element,
                data_type,
                array_length,
                data
            })
    )
);

named!(
    ser_data_two_dim_parser<SerDataTwoDim>,
    do_parse!(
        calibration_offset_x: le_f64
            >> calibration_delta_x: le_f64
            >> calibration_element_x: le_i32
            >> calibration_offset_y: le_f64
            >> calibration_delta_y: le_f64
            >> calibration_element_y: le_i32
            >> data_type: le_i16
            >> array_size_x: le_i32
            >> array_size_y: le_i32
            >> data: call!(
                ser_raw_data_parser,
                data_type,
                (array_size_x * array_size_y) as usize
            )
            >> (SerDataTwoDim {
                calibration_offset_x,
                calibration_delta_x,
                calibration_element_x,
                calibration_offset_y,
                calibration_delta_y,
                calibration_element_y,
                data_type,
                array_size_x,
                array_size_y,
                data
            })
    )
);

named!(
    ser_data_tag_time_parser<SerDataTagTime>,
    do_parse!(tag!(&hex::decode("5241").unwrap()[..]) >> time: le_i32 >> (SerDataTagTime { time }))
);
named!(
    ser_data_tag_time_pos_parser<SerDataTagTimePos>,
    do_parse!(
        tag!(&hex::decode("4241").unwrap()[..])
            >> time: le_i32
            >> position_x: le_f64
            >> position_y: le_f64
            >> (SerDataTagTimePos {
                time,
                position_x,
                position_y
            })
    )
);

/// Parse tags depending on tag type id
named_args!(ser_data_tag_parser(tag_type: TagType)<SerDataTag>,
            alt!(
                cond_reduce!(tag_type == TagType::Time, map!(ser_data_tag_time_parser, SerDataTag::Time)) |
                cond_reduce!(tag_type == TagType::TimePos, map!(ser_data_tag_time_pos_parser, SerDataTag::TimePos))
            )
);

/// Parse tags depending on tag type id
named_args!(ser_data_parser(array_dim: ArrayDim)<SerData>,
            alt!(
                cond_reduce!(array_dim == ArrayDim::One, map!(ser_data_one_dim_parser, SerData::OneDim)) |
                cond_reduce!(array_dim == ArrayDim::Two, map!(ser_data_two_dim_parser, SerData::TwoDim))
            )
);

#[test]
fn test_ser_parser() {}

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

    let tags = ser_data_tag_parser(
        &file[(offsets.1.tag_offset[0] as usize)..],
        result.1.tag_type,
    )
    .expect("could not parse tag");
    println!("tags: {:#?}", tags.1);

    let data = ser_data_parser(
        &file[(offsets.1.data_offset[0] as usize)..],
        result.1.array_dim,
    )
    .expect("could not parse data");

    match data.1 {
        SerData::TwoDim(the_data) => match the_data.data {
            SerRawData::DataU16(raw) => {
                let raw_as_u8: Vec<u8> = raw.iter().map(|x| *x as u8).collect();
                image::save_buffer(
                    "test.png",
                    &raw_as_u8[..],
                    the_data.array_size_y as u32,
                    the_data.array_size_x as u32,
                    image::Gray(8),
                ).unwrap();
            },
            _ => {}
        },
        _ => {}
    }
}
