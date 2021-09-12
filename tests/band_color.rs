use resistor_color::band_color::BandColor;
use std::{str::FromStr, vec};

#[test]
fn test_band_color_from_str() {
    assert_eq!(BandColor::from_str("black").unwrap(), BandColor::Black);
    assert_eq!(BandColor::from_str("brown").unwrap(), BandColor::Brown);
    assert_eq!(BandColor::from_str("red").unwrap(), BandColor::Red);
    assert_eq!(BandColor::from_str("orange").unwrap(), BandColor::Orange);
    assert_eq!(BandColor::from_str("yellow").unwrap(), BandColor::Yellow);
    assert_eq!(BandColor::from_str("green").unwrap(), BandColor::Green);
    assert_eq!(BandColor::from_str("blue").unwrap(), BandColor::Blue);
    assert_eq!(BandColor::from_str("violet").unwrap(), BandColor::Violet);
    assert_eq!(BandColor::from_str("grey").unwrap(), BandColor::Grey);
    assert_eq!(BandColor::from_str("white").unwrap(), BandColor::White);
}

#[test]
fn test_invalid_band_color() {
    assert!(BandColor::from_str("asdf").is_err());
    assert!(BandColor::from_str("bluee").is_err());
    assert!(BandColor::from_str("redder").is_err());
}

#[test]
fn test_brand_color_from_str_all_case() {
    assert_eq!(BandColor::from_str("BLACK").unwrap(), BandColor::Black);
    assert_eq!(BandColor::from_str("BlacK").unwrap(), BandColor::Black);
    assert_eq!(BandColor::from_str("black").unwrap(), BandColor::Black);
    assert_eq!(BandColor::from_str("BlAcK").unwrap(), BandColor::Black);
}

#[test]
fn band_color_insuffcient() {
    assert!(BandColor::get_bands(vec!["brown", "brown"]).is_err());
    assert!(BandColor::get_bands(vec![]).is_err());
    assert!(BandColor::get_bands(vec!["black"]).is_err());
}

#[test]
fn get_bands_bad_color() {
    assert!(BandColor::get_bands(vec!["asdf", "asdf", "asdf"]).is_err());
}

#[test]
fn test_get_bands() {
    assert_eq!(
        BandColor::get_bands(vec!["brown", "brown", "red"]).unwrap(),
        vec![BandColor::Brown, BandColor::Brown, BandColor::Red,]
    )
}
