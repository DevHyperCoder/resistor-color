//! resistor-color | Rust library to convert values to/from resistor color codes
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use std::str::FromStr;

/// Band Colors for resistor.
/// TODO Gold and Silver
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BandColor {
    /// Black 0
    Black = 0,
    /// Brown 1
    Brown = 1,
    /// Red 2
    Red = 2,
    /// Orange 3
    Orange = 3,
    /// Yellow 4
    Yellow = 4,
    /// Green 5
    Green = 5,
    /// Blue 6
    Blue = 6,
    /// Violet 7
    Violet = 7,
    /// Grey 8
    Grey = 8,
    /// White 9
    White = 9,
}

impl FromStr for BandColor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "black" => BandColor::Black,
            "brown" => BandColor::Brown,
            "red" => BandColor::Red,
            "orange" => BandColor::Orange,
            "yellow" => BandColor::Yellow,
            "green" => BandColor::Green,
            "blue" => BandColor::Blue,
            "violet" => BandColor::Violet,
            "grey" => BandColor::Grey,
            "white" => BandColor::White,
            _ => return Err(format!("Could not find BandColor for {}", s)),
        })
    }
}

impl BandColor {
    /// Convert given vector of band color names to a vector of BandColor
    /// Errors if less than 3 colors are provided
    pub fn get_bands(band_colors: Vec<&str>) -> Result<Vec<Self>, String> {
        if band_colors.len() < 3 {
            return Err(format!(
                "Insufficient bands provided! 3 or more required, {} provided",
                band_colors.len()
            ));
        }

        let mut band_vec = vec![];
        for band in band_colors {
            match BandColor::from_str(band) {
                Ok(band_c) => band_vec.push(band_c),
                Err(why) => return Err(why),
            }
        }

        Ok(band_vec)
    }
}

/// 3 band resistor
///
/// Use Resistor::from_str() or directly provide the BandColors
///
/// ```
/// use resistor_color::{Resistor,BandColor};
/// use std::str::FromStr;
/// let resistor = Resistor::from_str("brown black red").unwrap();
///
/// // Get resistor band colors
///
/// let bands = resistor.get_bands();
/// assert_eq!(bands[0],BandColor::Brown);
///
/// // Get resistor value
///
/// assert_eq!(resistor.get_value(),1000);
/// ```
///
/// Use BandColor directly
///
/// ```
/// use resistor_color::{Resistor,BandColor};
/// use std::str::FromStr;
/// let resistor = Resistor::with_bands(
///      vec![
///      BandColor::Brown,
///      BandColor::Black,
///      BandColor::Red,
///      ]
/// ).unwrap();
///
/// // Get resistor band colors
///
/// let bands = &resistor.get_bands();
/// assert_eq!(bands[0],BandColor::Brown);
///
/// // Get resistor value
///
/// assert_eq!(resistor.get_value(),1000);
/// ```
#[derive(Debug, Clone)]
pub struct Resistor {
    /// Color bands of resistor
    bands: Vec<BandColor>,
}

impl FromStr for Resistor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let band_colors: Vec<&str> = s.trim().split(' ').collect();

        match BandColor::get_bands(band_colors) {
            Ok(bands) => Ok(Resistor { bands }),
            Err(why) => Err(why),
        }
    }
}

impl Resistor {
    /// Get numeric value from resistor.
    pub fn get_value(self) -> u32 {
        let (value, multiplier) = match self.bands.len() {
            3 | 4 => {
                let first = self.bands[0] as u32;
                let second = self.bands[1] as u32;
                ((first * 10) + second, self.bands[2] as u32)
            }

            5 | 6 => {
                let first = self.bands[0] as u32;
                let second = self.bands[1] as u32;
                let third = self.bands[2] as u32;

                let value = first * 100 + second * 10 + third;
                (value, self.bands[3] as u32)
            }
            _ => panic!("Poison!!"),
        };

        let multiplier = 10_u32.pow(multiplier as u32);

        (value) * multiplier
    }

    /// Get internal band state for resistor
    pub fn get_bands(&self) -> &Vec<BandColor> {
        &self.bands
    }

    /// Returns a Resistor with given color bands
    pub fn with_bands(bands: Vec<BandColor>) -> Result<Self, String> {
        if bands.len() < 3 {
            return Err(format!("{}", bands.len()));
        }

        Ok(Resistor { bands })
    }
}
