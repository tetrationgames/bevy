use crate::{
    color_difference::EuclideanDistance, impl_componentwise_vector_space, impl_const_color_ops,
    ColorToComponents, ColorToPacked, Gray, LinearRgba, Luminance, StandardColor, Xyza,
};
#[cfg(feature = "alloc")]
use alloc::{format, string::String};
use bevy_math::{ops, Vec3, Vec4};
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::prelude::*;
use thiserror::Error;

/// Non-linear standard RGB with alpha.
///
/// SDR colors are in `[0.0, 1.0]`. Values above `1.0` are HDR intensities.
#[doc = include_str!("../docs/conversion.md")]
/// <div>
#[doc = include_str!("../docs/diagrams/model_graph.svg")]
/// </div>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Clone, PartialEq, Default)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub struct Srgba {
    /// The red channel. [0.0, 1.0] for SDR colors.
    pub red: f32,
    /// The green channel. [0.0, 1.0] for SDR colors.
    pub green: f32,
    /// The blue channel. [0.0, 1.0] for SDR colors.
    pub blue: f32,
    /// The alpha channel. [0.0, 1.0]
    pub alpha: f32,
}

impl StandardColor for Srgba {}

impl_componentwise_vector_space!(Srgba, [red, green, blue, alpha]);

impl_const_color_ops!(Srgba, [red, green, blue]);

impl Srgba {
    // The standard VGA colors, with alpha set to 1.0.
    // https://en.wikipedia.org/wiki/Web_colors#Basic_colors

    /// <div style="background-color:rgb(0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully black color with full alpha.
    pub const BLACK: Srgba = Srgba::new(0.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgba(0%, 0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully transparent color with no alpha (alpha = 0.0).
    #[doc(alias = "transparent")]
    pub const NONE: Srgba = Srgba::new(0.0, 0.0, 0.0, 0.0);
    /// <div style="background-color:rgb(100%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully white color with full alpha.
    pub const WHITE: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);
    /// <div style="background-color:rgb(100%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully red color with full alpha.
    pub const RED: Srgba = Srgba::new(1.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(0%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully green color with full alpha.
    pub const GREEN: Srgba = Srgba::new(0.0, 1.0, 0.0, 1.0);
    /// <div style="background-color:rgb(0%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully blue color with full alpha.
    pub const BLUE: Srgba = Srgba::new(0.0, 0.0, 1.0, 1.0);

    /// Construct a new [`Srgba`] color from components.
    ///
    /// # Arguments
    ///
    /// * `red` - Red channel. [0.0, 1.0]
    /// * `green` - Green channel. [0.0, 1.0]
    /// * `blue` - Blue channel. [0.0, 1.0]
    /// * `alpha` - Alpha channel. [0.0, 1.0]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Construct a new [`Srgba`] color from (r, g, b) components, with the default alpha (1.0).
    ///
    /// # Arguments
    ///
    /// * `red` - Red channel. [0.0, 1.0]
    /// * `green` - Green channel. [0.0, 1.0]
    /// * `blue` - Blue channel. [0.0, 1.0]
    pub const fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::new(red, green, blue, 1.)
    }

    /// New `Srgba` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `red` - Red channel. [0, 255]
    /// * `green` - Green channel. [0, 255]
    /// * `blue` - Blue channel. [0, 255]
    /// * `alpha` - Alpha channel. [0, 255]
    ///
    /// See also [`Srgba::new`], [`Srgba::rgb`], [`Srgba::rgb_u8`], [`Srgba::hex`].
    pub const fn rgba_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::new(
            red as f32 / 255.,
            green as f32 / 255.,
            blue as f32 / 255.,
            alpha as f32 / 255.,
        )
    }

    /// New `Srgba` from sRGB colorspace, with the default alpha (255).
    ///
    /// # Arguments
    ///
    /// * `red` - Red channel. [0, 255]
    /// * `green` - Green channel. [0, 255]
    /// * `blue` - Blue channel. [0, 255]
    ///
    /// See also [`Srgba::new`],  [`Srgba::rgb`], [`Srgba::rgba_u8`], [`Srgba::hex`].
    pub const fn rgb_u8(red: u8, green: u8, blue: u8) -> Self {
        Self::rgba_u8(red, green, blue, 255)
    }

    /// Return a copy of this color with the red channel set to the given value.
    pub const fn with_red(self, red: f32) -> Self {
        Self { red, ..self }
    }

    /// Return a copy of this color with the green channel set to the given value.
    pub const fn with_green(self, green: f32) -> Self {
        Self { green, ..self }
    }

    /// Return a copy of this color with the blue channel set to the given value.
    pub const fn with_blue(self, blue: f32) -> Self {
        Self { blue, ..self }
    }

    /// New `Srgba` from a CSS-style hexadecimal string, in a `const` context.
    ///
    /// Equivalent to [`Srgba::hex`] for valid CSS hex strings; `hex` accepts any
    /// `AsRef<str>` and therefore cannot be `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_color::Srgba;
    /// const FUCHSIA: Srgba = match Srgba::from_hex("FF00FF") {
    ///     Ok(color) => color,
    ///     Err(_) => panic!("invalid color"),
    /// };
    /// assert_eq!(FUCHSIA, Srgba::new(1.0, 0.0, 1.0, 1.0));
    /// ```
    #[inline]
    pub const fn from_hex(hex: &str) -> Result<Self, HexColorError> {
        let bytes = hex.as_bytes();
        let offset = if !bytes.is_empty() && bytes[0] == b'#' {
            1
        } else {
            0
        };
        let digits = bytes.len() - offset;

        // Digits are 3, 4, 6, or 8: short forms repeat each nibble.
        let (step, has_alpha) = match digits {
            3 => (1, false),
            4 => (1, true),
            6 => (2, false),
            8 => (2, true),
            _ => return Err(HexColorError::Length),
        };
        let red = match hex_channel(bytes, offset, step) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let green = match hex_channel(bytes, offset + step, step) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let blue = match hex_channel(bytes, offset + 2 * step, step) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let alpha = if has_alpha {
            match hex_channel(bytes, offset + 3 * step, step) {
                Ok(value) => value,
                Err(error) => return Err(error),
            }
        } else {
            255
        };
        Ok(Self::rgba_u8(red, green, blue, alpha))
    }

    /// New `Srgba` from a CSS-style hexadecimal string, panicking on invalid input.
    ///
    /// This is the `const`-friendly counterpart of `Srgba::from_hex(hex).unwrap()`, which
    /// cannot be written in a `const` context because `Result::unwrap` is not `const`. In a
    /// `const` item, a malformed string fails the build with a const-evaluation error.
    /// At runtime it panics, so prefer [`Srgba::from_hex`] for user-supplied input.
    ///
    /// # Panics
    ///
    /// Panics if `hex` is not a valid CSS-style hexadecimal color, either because of its
    /// length or because it contains an invalid character.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_color::Srgba;
    /// const FUCHSIA: Srgba = Srgba::from_hex_or_panic("FF00FF");
    /// assert_eq!(FUCHSIA, Srgba::new(1.0, 0.0, 1.0, 1.0));
    /// ```
    ///
    /// Invalid input fails to compile:
    ///
    /// ```compile_fail
    /// # use bevy_color::Srgba;
    /// const INVALID: Srgba = Srgba::from_hex_or_panic("nope!");
    /// ```
    #[inline]
    pub const fn from_hex_or_panic(hex: &str) -> Self {
        match Self::from_hex(hex) {
            Ok(color) => color,
            Err(HexColorError::Length) => panic!("invalid hex color: unexpected length"),
            Err(HexColorError::Char(_)) => panic!("invalid hex color: invalid character"),
        }
    }

    /// New `Srgba` from a CSS-style hexadecimal string.
    ///
    /// See [`Srgba::from_hex`] for the `const` equivalent.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_color::Srgba;
    /// let color = Srgba::hex("FF00FF").unwrap(); // fuchsia
    /// let color = Srgba::hex("FF00FF7F").unwrap(); // partially transparent fuchsia
    ///
    /// // A standard hex color notation is also available
    /// assert_eq!(Srgba::hex("#FFFFFF").unwrap(), Srgba::new(1.0, 1.0, 1.0, 1.0));
    /// ```
    pub fn hex<T: AsRef<str>>(hex: T) -> Result<Self, HexColorError> {
        Self::from_hex(hex.as_ref())
    }

    /// Convert this color to CSS-style hexadecimal notation.
    #[cfg(feature = "alloc")]
    pub fn to_hex(&self) -> String {
        let [r, g, b, a] = self.to_u8_array();
        match a {
            255 => format!("#{r:02X}{g:02X}{b:02X}"),
            _ => format!("#{r:02X}{g:02X}{b:02X}{a:02X}"),
        }
    }

    /// Converts a non-linear sRGB value to a linear one via [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction).
    pub fn gamma_function(value: f32) -> f32 {
        if value <= 0.0 {
            return value;
        }
        if value <= 0.04045 {
            value / 12.92 // linear falloff in dark values
        } else {
            ops::powf((value + 0.055) / 1.055, 2.4) // gamma curve in other area
        }
    }

    /// Converts a linear sRGB value to a non-linear one via [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction).
    pub fn gamma_function_inverse(value: f32) -> f32 {
        if value <= 0.0 {
            return value;
        }

        if value <= 0.0031308 {
            value * 12.92 // linear falloff in dark values
        } else {
            (1.055 * ops::powf(value, 1.0 / 2.4)) - 0.055 // gamma curve in other area
        }
    }
}

impl Default for Srgba {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Luminance for Srgba {
    #[inline]
    fn luminance(&self) -> f32 {
        let linear: LinearRgba = (*self).into();
        linear.luminance()
    }

    #[inline]
    fn with_luminance(&self, luminance: f32) -> Self {
        let linear: LinearRgba = (*self).into();
        linear
            .with_luminance(Srgba::gamma_function(luminance))
            .into()
    }

    #[inline]
    fn darker(&self, amount: f32) -> Self {
        let linear: LinearRgba = (*self).into();
        linear.darker(amount).into()
    }

    #[inline]
    fn lighter(&self, amount: f32) -> Self {
        let linear: LinearRgba = (*self).into();
        linear.lighter(amount).into()
    }
}

impl EuclideanDistance for Srgba {
    #[inline]
    fn distance_squared(&self, other: &Self) -> f32 {
        let dr = self.red - other.red;
        let dg = self.green - other.green;
        let db = self.blue - other.blue;
        dr * dr + dg * dg + db * db
    }
}

impl Gray for Srgba {
    const BLACK: Self = Self::BLACK;
    const WHITE: Self = Self::WHITE;
}

impl ColorToComponents for Srgba {
    fn to_f32_array(self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    fn to_f32_array_no_alpha(self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }

    fn to_vec4(self) -> Vec4 {
        Vec4::new(self.red, self.green, self.blue, self.alpha)
    }

    fn to_vec3(self) -> Vec3 {
        Vec3::new(self.red, self.green, self.blue)
    }

    fn from_f32_array(color: [f32; 4]) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: color[3],
        }
    }

    fn from_f32_array_no_alpha(color: [f32; 3]) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: 1.0,
        }
    }

    fn from_vec4(color: Vec4) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: color[3],
        }
    }

    fn from_vec3(color: Vec3) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: 1.0,
        }
    }
}

impl ColorToPacked for Srgba {
    fn to_u8_array(self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
            .map(|v| ops::round(v.clamp(0.0, 1.0) * 255.0) as u8)
    }

    fn to_u8_array_no_alpha(self) -> [u8; 3] {
        [self.red, self.green, self.blue].map(|v| ops::round(v.clamp(0.0, 1.0) * 255.0) as u8)
    }

    fn from_u8_array(color: [u8; 4]) -> Self {
        Self::from_f32_array(color.map(|u| u as f32 / 255.0))
    }

    fn from_u8_array_no_alpha(color: [u8; 3]) -> Self {
        Self::from_f32_array_no_alpha(color.map(|u| u as f32 / 255.0))
    }
}

impl From<LinearRgba> for Srgba {
    #[inline]
    fn from(value: LinearRgba) -> Self {
        Self {
            red: Srgba::gamma_function_inverse(value.red),
            green: Srgba::gamma_function_inverse(value.green),
            blue: Srgba::gamma_function_inverse(value.blue),
            alpha: value.alpha,
        }
    }
}

impl From<Srgba> for LinearRgba {
    #[inline]
    fn from(value: Srgba) -> Self {
        Self {
            red: Srgba::gamma_function(value.red),
            green: Srgba::gamma_function(value.green),
            blue: Srgba::gamma_function(value.blue),
            alpha: value.alpha,
        }
    }
}

// Derived Conversions

impl From<Xyza> for Srgba {
    fn from(value: Xyza) -> Self {
        LinearRgba::from(value).into()
    }
}

impl From<Srgba> for Xyza {
    fn from(value: Srgba) -> Self {
        LinearRgba::from(value).into()
    }
}

#[cfg(feature = "wgpu-types")]
impl From<Srgba> for wgpu_types::Color {
    fn from(color: Srgba) -> Self {
        wgpu_types::Color {
            r: color.red as f64,
            g: color.green as f64,
            b: color.blue as f64,
            a: color.alpha as f64,
        }
    }
}

/// Decode the character starting at `at`.
///
/// The bytes come from a `&str`, so every error is at a character boundary and this is exact.
/// The fallback only exists to keep the function total; `from_hex` never reaches it.
const fn char_at(bytes: &[u8], at: usize) -> char {
    const REPLACEMENT: char = char::REPLACEMENT_CHARACTER;

    let first = bytes[at];
    let (len, mut code) = if first < 0x80 {
        return first as char;
    } else if first & 0xE0 == 0xC0 {
        (2, (first & 0x1F) as u32)
    } else if first & 0xF0 == 0xE0 {
        (3, (first & 0x0F) as u32)
    } else if first & 0xF8 == 0xF0 {
        (4, (first & 0x07) as u32)
    } else {
        return REPLACEMENT;
    };
    if at + len > bytes.len() {
        return REPLACEMENT;
    }
    let mut i = 1;
    while i < len {
        let continuation = bytes[at + i];
        if continuation & 0xC0 != 0x80 {
            return REPLACEMENT;
        }
        code = (code << 6) | (continuation & 0x3F) as u32;
        i += 1;
    }
    match char::from_u32(code) {
        Some(character) => character,
        None => REPLACEMENT,
    }
}

/// Decode a single hexadecimal digit at `at`.
const fn hex_digit(bytes: &[u8], at: usize) -> Result<u8, HexColorError> {
    let byte = bytes[at];
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(HexColorError::Char(char_at(bytes, at))),
    }
}

/// Decode one channel: a pair of digits when `step` is 2, else one repeated digit.
const fn hex_channel(bytes: &[u8], at: usize, step: usize) -> Result<u8, HexColorError> {
    let high = match hex_digit(bytes, at) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    if step == 2 {
        let low = match hex_digit(bytes, at + 1) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        Ok((high << 4) | low)
    } else {
        Ok((high << 4) | high)
    }
}

/// Error returned if a hex string could not be parsed as a color.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum HexColorError {
    /// Invalid length.
    #[error("Unexpected length of hex string")]
    Length,
    /// Invalid character.
    #[error("Invalid hex character '{0}'")]
    Char(char),
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_approx_eq;

    use super::*;

    #[test]
    fn test_to_from_linear() {
        let srgba = Srgba::new(0.0, 0.5, 1.0, 1.0);
        let linear_rgba: LinearRgba = srgba.into();
        assert_eq!(linear_rgba.red, 0.0);
        assert_approx_eq!(linear_rgba.green, 0.2140, 0.0001);
        assert_approx_eq!(linear_rgba.blue, 1.0, 0.0001);
        assert_eq!(linear_rgba.alpha, 1.0);
        let srgba2: Srgba = linear_rgba.into();
        assert_eq!(srgba2.red, 0.0);
        assert_approx_eq!(srgba2.green, 0.5, 0.0001);
        assert_approx_eq!(srgba2.blue, 1.0, 0.0001);
        assert_eq!(srgba2.alpha, 1.0);
    }

    #[test]
    fn euclidean_distance() {
        // White to black
        let a = Srgba::new(0.0, 0.0, 0.0, 1.0);
        let b = Srgba::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(a.distance_squared(&b), 3.0);

        // Alpha shouldn't matter
        let a = Srgba::new(0.0, 0.0, 0.0, 1.0);
        let b = Srgba::new(1.0, 1.0, 1.0, 0.0);
        assert_eq!(a.distance_squared(&b), 3.0);

        // Red to green
        let a = Srgba::new(0.0, 0.0, 0.0, 1.0);
        let b = Srgba::new(1.0, 0.0, 0.0, 1.0);
        assert_eq!(a.distance_squared(&b), 1.0);
    }

    #[test]
    fn darker_lighter() {
        // Darker and lighter should be commutative.
        let color = Srgba::new(0.4, 0.5, 0.6, 1.0);
        let darker1 = color.darker(0.1);
        let darker2 = darker1.darker(0.1);
        let twice_as_dark = color.darker(0.2);
        assert!(darker2.distance_squared(&twice_as_dark) < 0.0001);

        let lighter1 = color.lighter(0.1);
        let lighter2 = lighter1.lighter(0.1);
        let twice_as_light = color.lighter(0.2);
        assert!(lighter2.distance_squared(&twice_as_light) < 0.0001);
    }

    #[test]
    fn hex_color() {
        assert_eq!(Srgba::hex("FFF"), Ok(Srgba::WHITE));
        assert_eq!(Srgba::hex("FFFF"), Ok(Srgba::WHITE));
        assert_eq!(Srgba::hex("FFFFFF"), Ok(Srgba::WHITE));
        assert_eq!(Srgba::hex("FFFFFFFF"), Ok(Srgba::WHITE));
        assert_eq!(Srgba::hex("000"), Ok(Srgba::BLACK));
        assert_eq!(Srgba::hex("000F"), Ok(Srgba::BLACK));
        assert_eq!(Srgba::hex("000000"), Ok(Srgba::BLACK));
        assert_eq!(Srgba::hex("000000FF"), Ok(Srgba::BLACK));
        assert_eq!(Srgba::hex("03a9f4"), Ok(Srgba::rgb_u8(3, 169, 244)));
        assert_eq!(Srgba::hex("yy"), Err(HexColorError::Length));
        assert_eq!(Srgba::hex("#f2a"), Ok(Srgba::rgb_u8(255, 34, 170)));
        assert_eq!(Srgba::hex("#e23030"), Ok(Srgba::rgb_u8(226, 48, 48)));
        assert_eq!(Srgba::hex("#ff"), Err(HexColorError::Length));
        assert_eq!(Srgba::hex("11223344"), Ok(Srgba::rgba_u8(17, 34, 51, 68)));
        assert_eq!(Srgba::hex("1234"), Ok(Srgba::rgba_u8(17, 34, 51, 68)));
        assert_eq!(Srgba::hex("12345678"), Ok(Srgba::rgba_u8(18, 52, 86, 120)));
        assert_eq!(Srgba::hex("4321"), Ok(Srgba::rgba_u8(68, 51, 34, 17)));

        assert!(matches!(Srgba::hex("yyy"), Err(HexColorError::Char('y'))));
        assert!(matches!(Srgba::hex("##fff"), Err(HexColorError::Char('#'))));
    }

    #[test]
    fn hex_in_const() {
        const FUCHSIA: Srgba = match Srgba::from_hex("#FF00FF") {
            Ok(color) => color,
            Err(_) => panic!("invalid color"),
        };
        const TRANSLUCENT: Srgba = match Srgba::from_hex("FF00FF7F") {
            Ok(color) => color,
            Err(_) => panic!("invalid color"),
        };
        assert_eq!(FUCHSIA, Srgba::rgb_u8(255, 0, 255));
        assert_eq!(TRANSLUCENT, Srgba::rgba_u8(255, 0, 255, 127));
        assert_eq!(Srgba::from_hex("nope!"), Err(HexColorError::Length));
        assert_eq!(Srgba::from_hex("#12g456"), Err(HexColorError::Char('g')));
        assert_eq!(Srgba::from_hex("+fff"), Err(HexColorError::Char('+')));
        assert_eq!(Srgba::from_hex("ééé"), Err(HexColorError::Char('é')));
    }

    #[test]
    #[should_panic(expected = "invalid hex color: invalid character")]
    fn from_hex_or_panic_on_invalid_character() {
        let _ = Srgba::from_hex_or_panic("nope");
    }
}
