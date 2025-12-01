const N: i32 = 100;

enum Dir {
    Left,
    Right,
}

fn parse_rotations() -> impl Iterator<Item = (Dir, i32)> {
    std::io::stdin().lines()
        .filter_map(|line| {
            match line {
                Err(_) => None,
                Ok(line) => Some((
                    match line.chars().nth(0).unwrap() {
                        'L' => Dir::Left,
                        'R' => Dir::Right,
                        _ => return None,
                    },
                    line[1..].parse::<i32>().unwrap_or(0)
                )),
            }
        })
}

pub fn part1() -> i64 {
    let mut curr = 50i32;
    let mut cnt = 0i64;

    for (dir, amount) in parse_rotations() {
        match dir {
            Dir::Right => curr += amount,
            Dir::Left => curr -= amount,
        }

        curr = curr.rem_euclid(N);

        // check if the dial points to zero as a result of current rotation.
        if curr == 0 {
            cnt += 1;
        }
    }

    cnt
}

pub fn part2() -> i64 {
    let mut curr = 50i32;
    let mut next;
    let mut cnt = 0i64;

    for (dir, amount) in parse_rotations() {
        // if `N * m <= amount < N * (m + 1)`, then the dial shall point to
        // zero for `m` times.
        cnt += (amount / N) as i64;

        match dir {
            Dir::Right => next = curr + amount.rem_euclid(N),
            Dir::Left => next = curr - amount.rem_euclid(N),
        }

        // check if remaining clicks cause an underflow or overflow
        if curr * next < 0 || next > N {
            cnt += 1;
        }

        curr = next.rem_euclid(N);

        // check if the dial points to zero as a result of current rotation.
        if curr == 0 {
            cnt += 1;
        }
    }

    cnt
}
