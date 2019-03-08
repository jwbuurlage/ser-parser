use cpython::{PyResult, Python};

use super::data::*;
use super::parsers::*;
use std::convert::Into;
use std::fs;

fn convert_to_float<T: Copy>(raw: Vec<T>) -> Vec<f32>
where
    f32: std::convert::From<T>,
{
    return raw.into_iter().map(|x| f32::from(x)).collect();
}

fn convert_unsafe(raw: Vec<i32>) -> Vec<f32>
{
    return raw.into_iter().map(|x| x as f32).collect();
}

fn parse(_: Python, filename: String) -> PyResult<((i32, i32), Vec<f32>)> {
    let file = fs::read(filename).expect("failed to open .ser file");

    let result = ser_header_parser(&file).expect("could not parse .ser file");

    let offsets = ser_offsets_parser(
        &file[(result.1.array_offset as usize)..],
        result.1.series_version,
        result.1.total_element_count,
    )
    .expect("could not parse offsets");

    // FIXME for each tag
    let tags = ser_data_tag_parser(
        &file[(offsets.1.tag_offset[0] as usize)..],
        result.1.tag_type,
    )
    .expect("could not parse tag");

    // FIXME for each offset
    let data = ser_data_parser(
        &file[(offsets.1.data_offset[0] as usize)..],
        result.1.array_dim,
    )
    .expect("could not parse data");

    use self::SerRawData::*;

    let result_data = match data.1 {
        SerData::TwoDim(the_data) => Some((
            (the_data.array_size_y, the_data.array_size_x),
            match the_data.data {
                DataU8(raw) => Some(convert_to_float(raw)),
                DataU16(raw) => Some(convert_to_float(raw)),
                DataI8(raw) => Some(convert_to_float(raw)),
                DataI16(raw) => Some(convert_to_float(raw)),
                DataF32(raw) => Some(convert_to_float(raw)),
                DataI32(raw) => Some(convert_unsafe(raw)),
                // DataI32(raw) => Some(convert_to_float(raw)), // \
                // DataU32(raw) => Some(convert_to_float(raw)), //  }~~~~> need other way
                // DataF64(raw) => Some(convert_to_float(raw)), // /
                _ => None,
            }
            .expect("could not convert data to float"),
        )),
        _ => None,
    }
    .expect("could not return data");

    Ok(result_data)
}

py_module_initializer!(ser_parser, initser_parser, PyInit_ser_parser, |py, m| {
    m.add(py, "__doc__", "SER parser written in Rust.")?;
    m.add(py, "parser", py_fn!(py, parse(filename: String)))?;
    Ok(())
});
