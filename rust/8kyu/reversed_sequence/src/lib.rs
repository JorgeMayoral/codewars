pub fn reverse_seq(n: u32) -> Vec<u32> {
    let mut result = (1..=n).collect::<Vec<u32>>();
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    }
}
