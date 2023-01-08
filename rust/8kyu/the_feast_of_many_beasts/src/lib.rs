fn feast(beast: &str, dish: &str) -> bool {
    let mut beast_chars = beast.chars();
    let mut dish_chars = dish.chars();
    beast_chars.nth(0) == dish_chars.nth(0) && beast_chars.last() == dish_chars.last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(feast("great blue heron", "garlic naan"), true);
        assert_eq!(feast("chickadee", "chocolate cake"), true);
        assert_eq!(feast("brown bear", "bear claw"), false);
    }
}
