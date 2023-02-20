// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

// This function returns how much ice cream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_ice_cream(time_of_day: u16) -> Option<u16> {
    if time_of_day > 24 {
        return None;
    }

    if time_of_day >= 22 {
        return Some(0);
    }

    return Some(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(10), Some(5));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(25), None);
    }

    #[test]
    fn raw_value() {
        let ice_creams = maybe_ice_cream(12);
        assert_eq!(ice_creams, Some(5));
    }
}
