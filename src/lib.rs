//! resistor-color | Rust library to convert values to/from resistor color codes
//! Use Resistor::from_str() or directly provide the BandColors
//!
//! ```
//! use resistor_color::{resistor::Resistor,band_color::BandColor};
//! use std::str::FromStr;
//! let resistor = Resistor::from_str("brown black red").unwrap();
//!
//! // Get resistor band colors
//!
//! let bands = resistor.get_bands();
//! assert_eq!(bands[0],BandColor::Brown);
//!
//! // Get resistor value
//! assert_eq!(resistor.get_value(),1000);
//! ```
//!
//! Use BandColor directly
//!
//! ```
//! use resistor_color::{resistor::Resistor,band_color::BandColor};
//! use std::str::FromStr;
//!
//! let resistor = Resistor::with_bands(
//!      vec![
//!          BandColor::Brown,
//!          BandColor::Black,
//!          BandColor::Red,
//!      ]
//! ).unwrap();
//!
//! // Get resistor band colors
//!
//! let bands = &resistor.get_bands();
//! assert_eq!(bands[0],BandColor::Brown);
//!
//! // Get resistor value
//!
//! assert_eq!(resistor.get_value(),1000);

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Band Color enum and impls
pub mod band_color;

/// Resistor struct and impls
pub mod resistor;
