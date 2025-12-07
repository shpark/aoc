use std::io::Read;

use crate::utils::grid::{Grid, Pos};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Source,
    Empty,
    Ray,
    Splitter,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Cell::Source),
            '.' => Ok(Cell::Empty),
            '^' => Ok(Cell::Splitter),
            _ => Err(())
        }
    }
}

pub fn part1() -> i64 {
    let mut rays: Vec<Cell> = vec![];
    let mut num_splits = 0i64;

    std::io::stdin().lines().flatten()
        .for_each(|line| {
            if rays.len() < line.len() {
                line.char_indices().for_each(|_| {
                    rays.push(Cell::Empty);
                });
            }

            line.chars().enumerate().for_each(|(i, char)| {
                match Cell::try_from(char) {
                    Ok(Cell::Source) => {
                        rays[i] = Cell::Ray;
                    },
                    Ok(Cell::Splitter) => {
                        match rays[i] {
                            Cell::Ray => {
                                num_splits += 1;

                                if let Some(cell) = rays.get_mut(i - 1) {
                                    *cell = Cell::Ray;
                                }

                                if let Some(cell) = rays.get_mut(i + 1) {
                                    *cell = Cell::Ray;
                                }

                                if let Some(cell)  = rays.get_mut(i) {
                                    *cell = Cell::Empty;
                                }
                            },
                            _ => {},
                        }
                    },
                    Ok(Cell::Empty) => {},
                    _ => {}
                }
            })
        });

    num_splits
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AugmentedCell(Cell, usize);

impl TryFrom<char> for AugmentedCell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(AugmentedCell(Cell::Source, 0)),
            '.' => Ok(AugmentedCell(Cell::Empty, 0)),
            '^' => Ok(AugmentedCell(Cell::Splitter, 0)),
            _ => Err(())
        }
    }
}

type Laboratory = Grid<AugmentedCell>;

pub fn part2() -> i64 {
    let mut s =  String::new();

    std::io::stdin().read_to_string(&mut s).unwrap();

    let mut lab = Laboratory::from_str(&s).unwrap();

    for i in 0..lab.nrows as i64 {
        (0..lab.ncols as i64).for_each(|j| {
            let cell = lab.get_mut(Pos(i, j)).unwrap().clone();

            if cell.0 == Cell::Source {
                lab.get_mut(Pos(i, j)).unwrap().1 = 1;
                lab.get_mut(Pos(i + 1, j)).unwrap().1 = 1;
            }

            match cell.0 {
                Cell::Empty=> {
                    if let Some(next_cell) = lab.get_mut(Pos(i + 1, j)) {
                        next_cell.1 += cell.1;
                    }
                },
                Cell::Splitter => {
                    if let Some(next_cell) = lab.get_mut(Pos(i + 1, j - 1)) {
                        next_cell.1 += cell.1;
                    }

                    if let Some(next_cell) = lab.get_mut(Pos(i + 1, j + 1)) {
                        next_cell.1 += cell.1;
                    }
                },
                _ => {},
            }
        });
    }

    (0..lab.ncols as i64)
        .filter_map(|j| lab.get(Pos(lab.nrows as i64 - 1, j)).map(|cell| cell.1))
        .sum::<usize>() as i64
}