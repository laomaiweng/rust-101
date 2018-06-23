//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    ///
    /// # Examples
    ///
    /// ```
    /// use art::PrimaryColor::{Red, Blue};
    /// use art::SecondaryColor::Purple;
    ///
    /// assert_eq!(art::mix(Red, Blue), Purple);
    /// ```
    ///
    /// # Panics
    ///
    /// Don't try to mix the same primary color twice!
    ///
    /// # Errors
    ///
    /// N/A
    ///
    /// Just for completeness.
    ///
    /// # Safety
    ///
    /// N/A
    ///
    /// Just for completeness.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == c2 {
            panic!("Can't mix the same color twice! Sorry about the crap API.");
        }
        let orange_mix = (PrimaryColor::Red, PrimaryColor::Yellow);
        if (c1, c2) == orange_mix || (c2, c1) == orange_mix {
            return SecondaryColor::Orange;
        }
        let green_mix = (PrimaryColor::Blue, PrimaryColor::Yellow);
        if (c1, c2) == green_mix || (c2, c1) == green_mix {
            return SecondaryColor::Green;
        }
        let purple_mix = (PrimaryColor::Blue, PrimaryColor::Red);
        if (c1, c2) == purple_mix || (c2, c1) == purple_mix {
            return SecondaryColor::Purple;
        }
        panic!("What did you do? Those should never be mixed!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_valid() {
        assert_eq!(mix(PrimaryColor::Blue, PrimaryColor::Red), SecondaryColor::Purple);
        assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Blue), SecondaryColor::Purple);
        assert_eq!(mix(PrimaryColor::Blue, PrimaryColor::Yellow), SecondaryColor::Green);
        assert_eq!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green);
        assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Yellow), SecondaryColor::Orange);
        assert_eq!(mix(PrimaryColor::Yellow, PrimaryColor::Red), SecondaryColor::Orange);
    }

    #[test]
    #[should_panic]
    fn mix_invalid_red() {
        mix(PrimaryColor::Red, PrimaryColor::Red);
    }

    #[test]
    #[should_panic]
    fn mix_invalid_yellow() {
        mix(PrimaryColor::Yellow, PrimaryColor::Yellow);
    }

    #[test]
    #[should_panic]
    fn mix_invalid_blue() {
        mix(PrimaryColor::Blue, PrimaryColor::Blue);
    }
}
