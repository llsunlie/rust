//! # My Crate
//!
//! `my_crate` 是一个使得特定计算更方便的
//! 工具集合

/// 将给定的数字加一
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = c30_cargo_release_profiles::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod art;