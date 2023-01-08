fn fake_bin(s: &str) -> String {
    s.chars()
        .map(|c| match c.to_digit(10) {
            Some(d) => {
                if d < 5 {
                    '0'
                } else {
                    '1'
                }
            }
            None => panic!("The string should only contain numbers!"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::fake_bin;

    #[test]
    fn basic_tests() {
        assert_eq!(fake_bin("45385593107843568"), "01011110001100111");
        assert_eq!(fake_bin("509321967506747"), "101000111101101");
        assert_eq!(
            fake_bin("366058562030849490134388085"),
            "011011110000101010000011011"
        );
        assert_eq!(fake_bin("15889923"), "01111100");
        assert_eq!(fake_bin("800857237867"), "100111001111");
    }
}
