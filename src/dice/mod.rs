use rand::Rng;

/// # Representation of each common dice type
pub enum Dice {
    D4(i16, i16),
    D6(i16, i16),
    D8(i16, i16),
    D10(i16, i16),
    D12(i16, i16),
    D20(i16, i16),
}

/// # Calculates common dice roll total, per dice type.
///
/// # Arguments
///
/// * `count` - Total number of dice.
/// * `bias` - Single value representing adjusted modifiers.
///
/// # Example
///
/// ```
/// // Roll two D20 dice, each with a -2 chance of success
/// let roll = dice::roll(dice::Dice::D20(4, -2));
/// ```
pub fn roll(d: Dice) -> i16 {
    match d {
        Dice::D4(count, bias) => (
            rand::thread_rng().gen_range(count + bias, count * (4 + bias))
        ),
        Dice::D6(count, bias) => (
            rand::thread_rng().gen_range(count + bias, count * (6 + bias))
        ),
        Dice::D8(count, bias) => (
            rand::thread_rng().gen_range(count + bias, count * (8 + bias))
        ),
        Dice::D10(count, bias) => (
            rand::thread_rng().gen_range(count + bias, count * (10 + bias))
        ),
        Dice::D12(count, bias) => (
            rand::thread_rng().gen_range(count + bias, count * (12 + bias))
        ),
        Dice::D20(count, bias) => (
            rand::thread_rng().gen_range(count + bias, count * (20 + bias))
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Range

    #[test]
    fn d4_roll_in_range() {
        let d4 = roll(Dice::D4(1, 0));
        assert!(d4 > 0 && d4 < 5, "d4: {}", d4);
    }

    #[test]
    fn d6_roll_in_range() {
        let d6 = roll(Dice::D6(1, 0));
        assert!(d6 > 0 && d6 < 7, "d6: {}", d6);
    }

    #[test]
    fn d8_roll_in_range() {
        let d8 = roll(Dice::D8(1, 0));
        assert!(d8 > 0 && d8 < 9, "d8: {}", d8);
    }

    #[test]
    fn d10_roll_in_range() {
        let d10 = roll(Dice::D10(1, 0));
        assert!(d10 > 0 && d10 < 11, "d10: {}", d10);
    }

    #[test]
    fn d12_roll_in_range() {
        let d12 = roll(Dice::D12(1, 0));
        assert!(d12 > 0 && d12 < 13, "d12: {}", d12);
    }

    #[test]
    fn d20_roll_in_range() {
        let d20 = roll(Dice::D20(1, 0));
        assert!(d20 > 0 && d20 < 21, "d20: {}", d20);
    }

    // Bias

    #[test]
    fn d4_bias() {
        let d4 = roll(Dice::D4(1, 4));
        assert!(d4 > 4 && d4 < 9, "d4: {}", d4);
    }

    #[test]
    fn d6_bias() {
        let d6 = roll(Dice::D6(1, 6));
        assert!(d6 > 6 && d6 < 13, "d6: {}", d6);
    }

    #[test]
    fn d8_bias() {
        let d8 = roll(Dice::D8(1, 8));
        assert!(d8 > 8 && d8 < 17, "d8: {}", d8);
    }

    #[test]
    fn d10_bias() {
        let d10 = roll(Dice::D10(1, 10));
        assert!(d10 > 10 && d10 < 21, "d10: {}", d10);
    }

    #[test]
    fn d12_bias() {
        let d12 = roll(Dice::D12(1, 12));
        assert!(d12 > 12 && d12 < 25, "d12: {}", d12);
    }

    #[test]
    fn d20_bias() {
        let d20 = roll(Dice::D20(1, 20));
        assert!(d20 > 20 && d20 < 41, "d20: {}", d20);
    }

    // Count

    #[test]
    fn d4_count() {
        let d4 = roll(Dice::D4(4, 0));
        assert!(d4 > 3 && d4 < 17, "d6: {}", d4);
    }

    #[test]
    fn d6_count() {
        let d6 = roll(Dice::D6(4, 0));
        assert!(d6 > 3 && d6 < 25, "d6: {}", d6);
    }

    #[test]
    fn d8_count() {
        let d8 = roll(Dice::D8(4, 0));
        assert!(d8 > 3 && d8 < 33, "d8: {}", d8);
    }

    #[test]
    fn d10_count() {
        let d10 = roll(Dice::D10(4, 0));
        assert!(d10 > 3 && d10 < 41, "d10: {}", d10);
    }

    #[test]
    fn d12_count() {
        let d12 = roll(Dice::D12(4, 0));
        assert!(d12 > 3 && d12 < 49, "d12: {}", d12);
    }

    #[test]
    fn d20_count() {
        let d20 = roll(Dice::D20(4, 0));
        assert!(d20 > 3 && d20 < 81, "d20: {}", d20);
    }
}
