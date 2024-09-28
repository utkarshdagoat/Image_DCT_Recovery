use std::collections::HashMap;

// to store the image row's
#[derive(Debug)]
pub struct RowBuffer(Vec<u8>);

#[derive(Debug)]
pub struct MetaData {}

/// A struct to hold the image it contains information about rows and columns
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Image {
    data: Vec<RowBuffer>,
    width: u32,
    height: u32,
    metadata: MetaData, // Hashmap for meta data which are key
                        // value pairs byte=> bytes
}
