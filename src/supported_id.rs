use std::{collections::BTreeSet, sync::LazyLock, fs::File};
use poem_openapi::{types::{ParseFromParameter, ParseResult, ParseError}};
use serde::{Deserialize, Serialize};

pub static SUPPORTED_CHAIN_IDS: LazyLock<BTreeSet<u64>> =
    LazyLock::new(|| {
        let path = std::path::Path::new("./chain_ids.json");
        serde_json::from_reader(
            File::open(path).expect("failed to open config file"),
        )
        .expect("failed to parse config file")
    });


#[derive(
    Serialize, Deserialize, PartialEq, Eq, Hash,  poem_openapi::NewType,
)]
#[oai(from_parameter = false)]
#[repr(transparent)]
/// a currently supported chain ID
pub struct SupportedChainId(pub u64);

impl ParseFromParameter for SupportedChainId {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        // first, parse to plain u64, propagating any error
        let number = value.parse::<u64>()
            .map_err(|_| ParseError::custom("must be a uint64"))?;

        // then, use the static set we setup earlier to check support
        match SUPPORTED_CHAIN_IDS.contains(&number) {
            // wrapping in our type encodes that we've checked the number
            true => Ok(SupportedChainId(number)),
            // represent lack of support as a parsing error
            false => Err(ParseError::custom(
                format!("unsupported chain ID: {}", number)
            )),
        }
    }
}