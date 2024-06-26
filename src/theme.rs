use crate::math::{normalize, FloatNumber};
use crate::Color;

/// The theme of a color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    /// Basic theme.
    Basic,
    /// Vivid theme.
    Vivid,
    /// Muted theme.
    Muted,
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
}

impl Theme {
    /// Scores the given color based on the theme.
    ///
    /// # Arguments
    /// * `color` - The color to score.
    ///
    /// # Returns
    /// The score of the color.
    #[inline]
    #[must_use]
    pub(crate) fn score<T>(&self, color: &Color<T>) -> T
    where
        T: FloatNumber,
    {
        match self {
            Theme::Basic => score_basic(color),
            Theme::Vivid => score_vivid(color),
            Theme::Muted => score_muted(color),
            Theme::Light => score_light(color),
            Theme::Dark => score_dark(color),
        }
    }
}

#[inline]
fn score_basic<T>(_color: &Color<T>) -> T
where
    T: FloatNumber,
{
    T::one()
}

#[inline]
fn score_vivid<T>(color: &Color<T>) -> T
where
    T: FloatNumber,
{
    let chroma = color.chroma();
    if chroma <= T::from_u32(60) {
        T::zero()
    } else {
        normalize(chroma, Color::<T>::min_chroma(), Color::<T>::max_chroma())
    }
}

#[inline]
fn score_muted<T>(color: &Color<T>) -> T
where
    T: FloatNumber,
{
    let chroma = color.chroma();
    if chroma <= T::from_u32(80) {
        T::one() - normalize(chroma, Color::<T>::min_chroma(), Color::<T>::max_chroma())
    } else {
        T::zero()
    }
}

#[inline]
fn score_light<T>(color: &Color<T>) -> T
where
    T: FloatNumber,
{
    if color.is_light() {
        normalize(
            color.lightness(),
            Color::<T>::min_lightness(),
            Color::<T>::max_lightness(),
        )
    } else {
        T::zero()
    }
}

#[inline]
fn score_dark<T>(color: &Color<T>) -> T
where
    T: FloatNumber,
{
    if color.is_dark() {
        T::one()
            - normalize(
                color.lightness(),
                Color::<T>::min_lightness(),
                Color::<T>::max_lightness(),
            )
    } else {
        T::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case::black("#000000", 1.0)]
    #[case::gray("#808080", 1.0)]
    #[case::white("#ffffff", 1.0)]
    #[case::red("#ff0000", 1.0)]
    #[case::green("#00ff00", 1.0)]
    #[case::blue("#0000ff", 1.0)]
    #[case::yellow("#ffff00", 1.0)]
    #[case::cyan("#00ffff", 1.0)]
    #[case::magenta("#ff00ff", 1.0)]
    fn test_score_basic(#[case] hex: &str, #[case] expected: f32) {
        // Act
        let color: Color<f32> = Color::from_str(hex).unwrap();
        let score = Theme::Basic.score(&color);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }

    #[rstest]
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.0)]
    #[case::white("#ffffff", 0.0)]
    #[case::red("#ff0000", 0.580)]
    #[case::green("#00ff00", 0.665)]
    #[case::blue("#0000ff", 0.743)]
    #[case::yellow("#ffff00", 0.538)]
    #[case::cyan("#00ffff", 0.0)]
    #[case::magenta("#ff00ff", 0.641)]
    #[case::orange("#ff8000", 0.475)]
    #[case::purple("#8000ff", 0.694)]
    #[case::lime("#80ff00", 0.607)]
    fn test_score_vivid(#[case] hex: &str, #[case] expected: f32) {
        // Act
        let color: Color<f32> = Color::from_str(hex).unwrap();
        let score = Theme::Vivid.score(&color);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }

    #[rstest]
    #[case::black("#000000", 1.0)]
    #[case::gray("#808080", 1.0)]
    #[case::white("#ffffff", 1.0)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.0)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.721)]
    #[case::magenta("#ff00ff", 0.0)]
    #[case::orange("#ff8000", 0.0)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.0)]
    fn test_score_muted(#[case] hex: &str, #[case] expected: f32) {
        // Act
        let color: Color<f32> = Color::from_str(hex).unwrap();
        let score = Theme::Muted.score(&color);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }

    #[rstest]
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.535)]
    #[case::white("#ffffff", 1.0)]
    #[case::red("#ff0000", 0.532)]
    #[case::green("#00ff00", 0.877)]
    #[case::blue("#0000ff", 0.0)]
    #[case::yellow("#ffff00", 0.971)]
    #[case::cyan("#00ffff", 0.911)]
    #[case::magenta("#ff00ff", 0.603)]
    fn test_score_light(#[case] hex: &str, #[case] expected: f32) {
        // Act
        let color: Color<f32> = Color::from_str(hex).unwrap();
        let score = Theme::Light.score(&color);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }

    #[rstest]
    #[case::black("#000000", 1.0)]
    #[case::gray("#808080", 0.0)]
    #[case::white("#ffffff", 0.0)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.676)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.0)]
    #[case::magenta("#ff00ff", 0.0)]
    fn test_score_dark(#[case] hex: &str, #[case] expected: f32) {
        // Act
        let color: Color<f32> = Color::from_str(hex).unwrap();
        let score = Theme::Dark.score(&color);

        println!("{} - {}", color, score);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }
}
