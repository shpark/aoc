fn parse_banks() -> impl Iterator<Item = String> {
    std::io::stdin().lines().flatten()
}

fn find_max_joltage(bank: &str, num_batteries: usize) -> i64 {
    (1..=num_batteries)
        .rev()
        .fold((0i64, bank), |(acc, bank_rem), i| {
            let max_radix = bank_rem[..(bank_rem.len() - (i - 1))].chars().max().unwrap();

            let max_radix_index = bank_rem[..(bank_rem.len() - (i - 1))].find(max_radix).unwrap();

            (acc * 10 + max_radix.to_digit(10).unwrap() as i64, &bank_rem[max_radix_index + 1..])
        })
        .0
}

pub fn part1() -> i64 {
    parse_banks()
        .map(|bank| find_max_joltage(&bank, 2))
        .sum()
}

pub fn part2() -> i64 {
    parse_banks()
        .map(|bank| find_max_joltage(&bank, 12))
        .sum()
}

#[cfg(test)]
mod test {
    use super::find_max_joltage;

    #[test]
    fn test_find_max_joltage2() {
        assert_eq!(find_max_joltage("987654321111111", 2), 98);
        assert_eq!(find_max_joltage("811111111111119", 2), 89);
        assert_eq!(find_max_joltage("234234234234278", 2), 78);
        assert_eq!(find_max_joltage("818181911112111", 2), 92);
    }

    #[test]
    fn test_find_max_joltage12() {
        assert_eq!(find_max_joltage("987654321111111", 12), 987654321111);
        assert_eq!(find_max_joltage("811111111111119", 12), 811111111119);
        assert_eq!(find_max_joltage("234234234234278", 12), 434234234278);
        assert_eq!(find_max_joltage("818181911112111", 12), 888911112111);
    }
}
