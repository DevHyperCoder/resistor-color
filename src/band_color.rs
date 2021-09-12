use std::str::FromStr;

/// Band Colors for resistor.
/// TODO Gold and Silver
///
/// Use BandColor::from_str("color")
///
/// ```
/// use resistor_color::band_color::BandColor;
/// use std::str::FromStr;
///
/// // "brown" can be of any case, ie, "Brown", "BROWN" etc are all acceptable
/// let band_color = BandColor::from_str("brown").unwrap();
///
/// assert_eq!(band_color,BandColor::Brown);
///
/// // Get numeric value
///
/// let band_num = band_color as u32;
/// assert_eq!(band_num,1);
/// ```
/// You can also use BandColor::get_bands()
///
/// ```
/// use resistor_color::band_color::BandColor;
/// use std::str::FromStr;
///
/// let colors = vec!["brown", "brown", "red"];
/// let bands = BandColor::get_bands(colors).unwrap();
///
/// assert_eq!(bands[0], BandColor::Brown);
/// ```
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
