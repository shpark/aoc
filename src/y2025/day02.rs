fn parse_ranges() -> impl Iterator<Item = (u64, u64)> {
    std::io::stdin().lines()
        .flatten()
        .flat_map(|line| {
            line.split(',')
                .filter_map(|range| {
                    let parts: Vec<_> = range.split('-')
                        .map(|part| part.parse::<u64>())
                        .flatten()
                        .collect();

                    if parts.len() > 0 {
                        Some((parts[0], parts[1]))
                    } else {
                        None
                    }
                })
            .collect::<Vec<_>>()
        })
}

fn is_invalid(x: u64) -> bool {
    let s = x.to_string();
    let ss = s.clone() + &s;
    ss[1..ss.len() - 1].contains(&s)
}

fn multiplier(len_pattern: u32, num_repeats: u32) -> u64 {
    (0..num_repeats - 1)
        .fold(1, |acc, _| {
            acc * 10u64.pow(len_pattern as u32) + 1
        })
}

// TODO: `part1` argument is ugly, but won't fix.
fn sum_invalid_ids_in_range(lo: u64, hi: u64, part1: bool) -> u64 {
    let mut res = 0u64;

    let num_max_digits = hi.ilog10() + 1;

    for len_pattern in 1..=(num_max_digits / 2) {
        for num_repeats in 2..=(num_max_digits / len_pattern) {
            if part1 && num_repeats % 2 != 0 {
                continue;
            }

            let multiplier = multiplier(len_pattern, num_repeats);

            let lo = lo.next_multiple_of(multiplier);
            let hi = (hi + 1).next_multiple_of(multiplier) - multiplier;

            // TODO: this is really ugly workaround for `len_pattern == 1` case, but won't fix.
            res += (std::cmp::max(if len_pattern == 1 {
                1
            } else {
                10u64.pow(len_pattern - 1)
            }, lo / multiplier)..=std::cmp::min(10u64.pow(len_pattern) - 1, hi / multiplier))
                .filter_map(|maybe_atom| {
                    if !is_invalid(maybe_atom) {
                        Some(maybe_atom * multiplier)
                    } else {
                        None
                    }
                })
                .sum::<u64>();
        }
    }

    res
}

pub fn part1() -> i64 {
    parse_ranges()
        .map(|(lo, hi)| {
            sum_invalid_ids_in_range(lo, hi, true)
        })
        .sum::<u64>() as i64
}

pub fn part2() -> i64 {
    parse_ranges()
        .map(|(lo, hi)| {
            sum_invalid_ids_in_range(lo, hi, false)
        })
        .sum::<u64>() as i64
}


#[cfg(test)]
mod test {
    use super::{is_invalid, multiplier, sum_invalid_ids_in_range};

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(3, 3), 1001001);
        assert_eq!(multiplier(1, 3), 111);
        assert_eq!(multiplier(5, 2), 100001);
    }

    #[test]
    fn test_is_invalid() {
        assert_eq!(is_invalid(123123), true);
        assert_eq!(is_invalid(1231), false);
        assert_eq!(is_invalid(123123123), true);
        assert_eq!(is_invalid(1231231), false);
    }

    #[test]
    fn test_sum_invalid_ids_in_range() {
        assert_eq!(sum_invalid_ids_in_range(998, 1012, false), 999 + 1010);
    }
}
