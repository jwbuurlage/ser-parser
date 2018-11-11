use pyo3::prelude::*;

use super::data::*;
use super::parsers::*;

fn convert_to_float<T>(raw: Vec<T>) -> Vec<f32> {
    raw.iter().map(|x| *x as f32).collect()
}

#[pyfunction]
fn parse(filename: String) -> PyResult<((i32, i32), Vec<f32>)> {
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

    let float_data = match data.1 {
        SerData::TwoDim(the_data) => match the_data.data {
            DataU8(raw) | DataU16(raw) | DataU32(raw) | DataI8(raw)
            | DataI16(raw) | DataI32(raw) | DataF32(raw) | DataF64(raw) => {
                convert_to_float(raw)
            }
            _ => None,
        },
        _ => None,
    }
    .expect("could not convert to float");

    ((the_data.array_size_y, the_data.array_size_x), float_data)
}

#[pymodinit]
fn ser_parser(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_function!(parse))?;

    Ok(())
}
