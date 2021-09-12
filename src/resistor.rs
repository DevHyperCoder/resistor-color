use crate::band_color::BandColor;
use std::str::FromStr;

/// Resistor struct. Supports resistors with 3 - 6 color bands.
/// TODO Tolerance and PPM
///
/// Use Resistor::from_str() or directly provide the BandColors
///
/// ```
/// use resistor_color::{resistor::Resistor,band_color::BandColor};
/// use std::str::FromStr;
/// let resistor = Resistor::from_str("brown black red").unwrap();
///
/// // Get resistor band colors
///
/// let bands = resistor.get_bands();
/// assert_eq!(bands[0],BandColor::Brown);
///
/// // Get resistor value
/// assert_eq!(resistor.get_value(),1000);
/// ```
///
/// Use BandColor directly
///
/// ```
/// use resistor_color::{resistor::Resistor,band_color::BandColor};
/// use std::str::FromStr;
///
/// let resistor = Resistor::with_bands(
///      vec![
///          BandColor::Brown,
///          BandColor::Black,
///          BandColor::Red,
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
