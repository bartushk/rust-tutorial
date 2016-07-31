
#![doc(html_logo_url = "https://avatars3.githubusercontent.com/u/8077780?v=3&s=460",
   html_favicon_url = "https://www.rust-lang.org/favicon.ico",
   html_root_url = "https://github.com/bartushk")]

//! The 'adder' crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```
//!



/// This function adds two to its argument.
///
/// # Examples
///
/// ```.
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    if cfg!(feature = "secure-password") {
        println!("We're using a secure password to add two.");
    }
    a + 2
}



/// The first line should be a quick description.
///
/// Then you can add a second paragraph that goes into further detail about the
/// function that couldn't be covered in the quick description. This sectin is
/// optional
///
/// # Panics
///
/// # Errors
///
/// # Safety
///
/// # Examples
///
/// ```
/// let t = adder::for_comment(1, 2.0);
/// assert_eq!(33, t)
/// ```
///
/// ```c
/// the c makes this not rust code 
/// ```
///
pub fn for_comment(one: i32, two: f32) -> i64 {
    33
}


/// Panic with a given message unless an expression evaluates to true.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate adder;
/// # fn main() {
/// panic_unless!(1 + 1 == 2, "Math is broken!");
/// # }
/// ```
///
#[macro_export]
macro_rules! panic_unless {
    ($condition:expr, $($rest:expr),+) => ({ if ! $condition {panic!($($rest),+);}
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "assertion failed")]
    #[cfg(feature = "secure-password")]
    fn it_works() {
        assert_eq!("Hello", "world");
    }
    
    #[test]
    #[ignore]
    fn it_works2() {
        assert!(true);
    }

    #[test]
    fn adds_two(){
        assert_eq!(4, add_two(2));
    }
}
