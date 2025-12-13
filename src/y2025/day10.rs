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

#[derive(PartialEq, Eq, Hash, Debug)]
struct Indicator {
    inner: Vec<Status>,
}

impl Indicator {
    fn with_len(len: usize) -> Self {
        Self { inner: vec![Status::default(); len] }
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
    target: Indicator,
    buttons: Vec<Button>,
}

impl TryFrom<&str> for Manual {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split(' ').collect::<Vec<_>>();

        if parts.len() < 3 {
            return Err(());
        }

        let indicator = Indicator::try_from(parts[0])?;

        let buttons = (1..parts.len() - 1)
            .filter_map(|i| Button::try_from(parts[i]).ok() )
            .collect::<Vec<_>>();

        Ok(Self {
            target: indicator,
            buttons,
        })
    }
}

impl Manual {
    fn src(&self) -> Indicator {
        Indicator::with_len(self.target.inner.len())
    }

    fn neighbors(&self, indicator: &Indicator) -> impl Iterator<Item = Indicator> {
        todo!()
    }

    // TODO: BFS
    fn num_fewest_toggles(&self) -> usize {
        let s = self.src();

        todo!()
    }
}

fn parse_manuals() -> impl Iterator<Item = Manual> {
    std::io::stdin().lines().flatten()
        .flat_map(|line| Manual::try_from(line.as_str()).ok())
}

pub fn part1() -> i64 {
    println!("{:?}", Indicator::with_len(10));

    parse_manuals()
        .map(|manual| manual.num_fewest_toggles())
        .sum::<usize>() as i64
}

pub fn part2() -> i64 {
    todo!()
}