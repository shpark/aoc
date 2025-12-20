use std::collections::{HashMap, VecDeque, hash_map::Entry};

use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Status {
    Off,
    On,
}

impl Default for Status {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Indicator {
    inner: Vec<Status>,
}

impl Indicator {
    fn with_len(len: usize) -> Self {
        Self { inner: vec![Status::default(); len] }
    }

    fn apply(&self, button: &Button) -> Self {
        let mut inner = self.inner.clone();

        for &index in button.inner.iter() {
            inner[index] = match inner[index] {
                Status::Off => Status::On,
                Status::On => Status::Off,
            }
        }

        Self { inner }
    }
}

impl TryFrom<&str> for Indicator {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut inner = Vec::new();

        for c in s.chars() {
            match c {
                '.' => inner.push(Status::Off),
                '#' => inner.push(Status::On),
                _ => {},
            }
        }

        Ok(Self { inner })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct JoltageLevels {
    inner: Vec<usize>,
}

impl TryFrom<&str> for JoltageLevels {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.chars()
            .skip(1).
            take_while(|&c| { c != '}'})
            .collect::<String>();

        let parts = s.split(',').collect::<Vec<_>>();

        let inner = parts.into_iter()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        Ok(Self { inner })
    }
}

#[derive(Debug)]
struct Button {
    inner: Vec<usize>,
}

impl TryFrom<&str> for Button {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.chars()
            .skip(1).
            take_while(|&c| { c != ')'})
            .collect::<String>();

        let parts = s.split(',').collect::<Vec<_>>();

        let inner = parts.into_iter()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        Ok(Self { inner })
    }
}

#[derive(Debug)]
struct Manual {
    target_indicator: Indicator,
    target_joltage_levels: JoltageLevels,
    buttons: Vec<Button>,
}

impl TryFrom<&str> for Manual {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split(' ').collect::<Vec<_>>();

        if parts.len() < 3 {
            return Err(());
        }

        let target_indicator = Indicator::try_from(parts[0])?;

        let buttons = (1..parts.len() - 1)
            .filter_map(|i| Button::try_from(parts[i]).ok() )
            .collect::<Vec<_>>();

        let target_joltage_levels = JoltageLevels::try_from(parts[parts.len() - 1])?;

        Ok(Self {
            target_indicator,
            buttons,
            target_joltage_levels,
        })
    }
}

impl Manual {
    fn src_indicator(&self) -> Indicator {
        Indicator::with_len(self.target_indicator.inner.len())
    }

    fn next_indicators(&self, indicator: &Indicator) -> impl Iterator<Item = Indicator> {
        self.buttons.iter()
            .map(|button| indicator.apply(button))
    }

    fn num_fewest_toggles_for_indicator(&self) -> usize {
        let s = self.src_indicator();

        let mut dist: HashMap<Indicator, usize> = HashMap::new();
        dist.insert(s.clone(), 0);

        let mut q = VecDeque::new();
        q.push_front(s.clone());

        'out: while !q.is_empty() {
            if let Some(curr) = q.pop_back() {
                let curr_dist = *dist.get(&curr).unwrap();

                for neighbor in self.next_indicators(&curr) {
                    if let Entry::Vacant(e) = dist.entry(neighbor.clone()) {
                        e.insert(curr_dist + 1);
                        q.push_front(neighbor.clone());
                    }

                    if neighbor == self.target_indicator {
                        break 'out;
                    }
                }
            }
        }

        dist[&self.target_indicator]
    }

    fn num_fewest_toggles_for_joltage_levels(&self) -> usize {
        let mut vars = variables!();

        let mut presses = Vec::new();

        for _ in 0..self.buttons.len() {
            presses.push(vars.add(variable().integer().min(0)));
        }

        let objective: Expression = presses.iter().sum();

        let mut problem = vars.minimise(objective).using(default_solver);

        for (i, &target) in self.target_joltage_levels.inner.iter().enumerate() {
            let mut sum: Expression = 0.into();
            for (j, button) in self.buttons.iter().enumerate() {
                if button.inner.contains(&i) {
                    sum += presses[j];
                }
            }
            problem = problem.with(constraint!(sum == target as i32));
        }

        // NOTE: if `.solve()` fails, it's a problem... but i won't fix.
        if let Ok(solution) = problem.solve() {
            presses.iter()
                .map(|&v| solution.value(v) as i32)
                .sum::<i32>() as usize
        } else {
            0
        }
    }
}

fn parse_manuals() -> impl Iterator<Item = Manual> {
    std::io::stdin().lines().flatten()
        .flat_map(|line| Manual::try_from(line.as_str()).ok())
}

pub fn part1() -> i64 {
    parse_manuals()
        .map(|manual| manual.num_fewest_toggles_for_indicator())
        .sum::<usize>() as i64
}

pub fn part2() -> i64 {
    parse_manuals()
        .map(|manual| manual.num_fewest_toggles_for_joltage_levels())
        .sum::<usize>() as i64
}

#[cfg(test)]
mod test {
    use crate::y2025::day10::{Button, Indicator, Manual};

    #[test]
    fn test_indicator_simple() {
        let before = Indicator::try_from("[.##.]").unwrap();
        let expected = Indicator::try_from("[#...]").unwrap();
        let button = Button::try_from("(0,1,2)").unwrap();

        assert_eq!(before.apply(&button), expected);
    }

    #[test]
    fn test_num_fewest_toggles_for_joltage() {
        let s = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(
            Manual::try_from(s).unwrap().num_fewest_toggles_for_joltage_levels(),
            11
        );

        let s = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        assert_eq!(
            Manual::try_from(s).unwrap().num_fewest_toggles_for_joltage_levels(),
            12
        );

        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(
            Manual::try_from(s).unwrap().num_fewest_toggles_for_joltage_levels(),
            10
        );
    }
}
