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
    pub calibration_offset: f64,
    pub calibration_delta: f64,
    pub calibration_element: i32,
    pub data_type: i16,
    pub array_length: i32,
    pub data: SerRawData,
}

#[derive(Debug, PartialEq)]
pub struct SerDataTwoDim {
    pub calibration_offset_x: f64,
    pub calibration_delta_x: f64,
    pub calibration_element_x: i32,
    pub calibration_offset_y: f64,
    pub calibration_delta_y: f64,
    pub calibration_element_y: i32,
    pub data_type: i16,
    pub array_size_x: i32,
    pub array_size_y: i32,
    pub data: SerRawData,
}

#[derive(Debug, PartialEq)]
pub enum SerData {
    OneDim(SerDataOneDim),
    TwoDim(SerDataTwoDim),
}

#[derive(Debug, PartialEq)]
pub struct SerDataTagTime {
    // FIXME this should be f32 according to spec, i32 according to reference parser
    pub time: i32,
}

#[derive(Debug, PartialEq)]
pub struct SerDataTagTimePos {
    // FIXME this should be f32 according to spec, i32 according to reference parser
    pub time: i32,
    pub position_x: f64,
    pub position_y: f64,
}

#[derive(Debug, PartialEq)]
pub enum SerDataTag {
    Time(SerDataTagTime),
    TimePos(SerDataTagTimePos),
}

