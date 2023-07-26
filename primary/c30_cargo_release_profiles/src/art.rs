//! # Art
//!
//! 一个描述美术信息的库。

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// 采用 RGB 色彩模式的主要颜色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 采用 RGB 色彩模式的次要颜色。
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::{PrimaryColor, SecondaryColor};

    /// 等量的混合两个主要颜色
    /// 来创建一个次要颜色。
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        return SecondaryColor::Green;
    }
}