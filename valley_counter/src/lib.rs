pub fn valley_count(steps: &str) -> u32 {
    let mut sum = 0;
    let mut valley_count = 0;
    let steps = steps.chars().map(|c| if c == 'U' { 1 } else { -1 });

    for c in steps {
        if c == 1 && sum == -1 {
            valley_count += 1
        }
        sum += c
    }
    valley_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_trip_has_no_valleys() {
        let trip = String::from("");
        let actual = valley_count(&trip);
        assert_eq!(0, actual)
    }

    #[test]
    fn simple_trip_has_one_valley() {
        let trip = String::from("DU");
        let actual = valley_count(&trip);
        assert_eq!(1, actual)
    }

    #[test]
    fn complex_trip() {
        let trip = String::from("UDDDUDUU");
        let actual = valley_count(&trip);
        assert_eq!(1, actual)
    }

    #[test]
    fn more_complex_trip() {
        let trip = String::from("UDDDUDUUDDDUUU");
        let actual = valley_count(&trip);
        assert_eq!(2, actual)
    }
}
