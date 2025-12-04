use std::io::Read;

use crate::utils::grid::{Grid, Pos};

#[derive(PartialEq, Eq)]
enum Cell {
    PaperRoll,
    Empty,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Cell::PaperRoll),
            '.' => Ok(Cell::Empty),
            _ => Err(()),
        }
    }
}

type Day4Grid = Grid<Cell>;

fn parse_grid() -> Option<Day4Grid> {
    let mut s = String::new();

    if std::io::stdin().read_to_string(&mut s).is_err() {
        return None;
    }

    Day4Grid::from_str(&s).ok()
}

fn removable_cells(grid: &Day4Grid) -> impl Iterator<Item = Pos> {
    grid.pos_iter()
        .filter(|&pos| {
            match grid.get(pos) {
                Some(Cell::PaperRoll) => {
                    grid.neighbors8(pos)
                        .filter(|&cell| cell == &Cell::PaperRoll)
                        .count() < 4
                },
                _ => false,
            }
        })
}

pub fn part1() -> i64 {
    let grid = parse_grid().unwrap();

    removable_cells(&grid).count() as i64
}

pub fn part2() -> i64 {
    let mut grid = parse_grid().unwrap();
    let mut res = 0i64;

    loop {
        let positions = removable_cells(&grid).collect::<Vec<_>>();

        res += positions.len() as i64;

        if positions.len() == 0 {
            break res;
        }

        for pos in positions {
            *grid.get_mut(pos).unwrap() = Cell::Empty;
        }
    }
}