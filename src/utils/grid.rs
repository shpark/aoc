#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos(i64, i64);

const DIRS8: &[Pos; 8] = &[
    Pos(-1, -1), Pos(-1, 0), Pos(-1, 1),
    Pos(0, -1), Pos(0, 1),
    Pos(1, -1), Pos(1, 0), Pos(1, 1),
];

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct Grid<T: PartialEq + Eq + TryFrom<char>> {
    pub nrows: usize,
    pub ncols: usize,
    cells: Vec<T>,
}

impl<T: PartialEq + Eq + TryFrom<char>> Grid<T> {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        let mut nrows = 0usize;
        let mut ncols = 0usize;
        let mut cells: Vec<_> = vec![];

        // TODO: flatten() is enough?
        s.lines().for_each(|row| {
            ncols = row.chars().count();
            nrows += 1;
            cells.extend(row.chars().map(|c| c.try_into()).flatten());
        });

        if cells.len() == nrows * ncols {
            Ok(Self {
                nrows,
                ncols,
                cells,
            })
        } else {
            Err(())
        }
    }

    pub fn get(&self, Pos(row, col): Pos) -> Option<&T> {
        if row < 0 || row >= self.nrows as i64 || col < 0 || col >= self.ncols as i64 {
            None
        } else {
            Some(&self.cells[row as usize * self.ncols + col as usize])
        }
    }

    pub fn get_mut(&mut self, Pos(row, col): Pos) -> Option<&mut T> {
        if row < 0 || row >= self.nrows as i64 || col < 0 || col >= self.ncols as i64 {
            None
        } else {
            Some(&mut self.cells[row as usize * self.ncols + col as usize])
        }
    }

    pub fn neighbors8(&self, pos: Pos) -> impl Iterator<Item = &T> {
        DIRS8.iter().map(move |&dir| self.get(pos + dir)).flatten()
    }

    pub fn pos_iter(&self) -> PosIterator {
        PosIterator {
            curr: 0,
            nrows: self.nrows,
            ncols: self.ncols,
        }
    }
}

pub struct PosIterator {
    curr: usize,
    nrows: usize,
    ncols: usize,
}

impl Iterator for PosIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.nrows * self.ncols {
            return None
        }

        let curr_pos = Pos((self.curr / self.nrows) as i64, (self.curr % self.nrows) as i64);
        self.curr += 1;
        Some(curr_pos)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::grid::{Grid, Pos};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    enum Cell {
        A,
        B,
        C,
    }

    impl TryFrom<char> for Cell {
        type Error = ();

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Cell::A),
                'B' => Ok(Cell::B),
                'C' => Ok(Cell::C),
                _ => Err(()),
            }
        }
    }

    #[test]
    fn test_grid() {
        let grid = Grid::<Cell>::from_str("AAB
BCC
CCA
CAB");

        assert!(grid.is_ok());

        let mut grid = grid.unwrap();

        assert_eq!(grid.nrows, 4);
        assert_eq!(grid.ncols, 3);

        assert_eq!(grid.get(Pos(2, 2)), Some(&Cell::A));
        assert_eq!(grid.get(Pos(4, 0)), None);

        let mut neighbors8 = grid.neighbors8(Pos(0, 0)).collect::<Vec<_>>();
        neighbors8.sort();

        assert_eq!(
            neighbors8,
            vec![&Cell::A, &Cell::B, &Cell::C],
        );

        let mut neighbors8 = grid.neighbors8(Pos(1, 1)).collect::<Vec<_>>();
        neighbors8.sort();

        assert_eq!(
            neighbors8,
            vec![&Cell::A, &Cell::A, &Cell::A, &Cell::B, &Cell::B, &Cell::C, &Cell::C, &Cell::C],
        );

        *grid.get_mut(Pos(2, 2)).unwrap() = Cell::B;

        assert_eq!(grid.get(Pos(2, 2)), Some(&Cell::B));

        assert_eq!(grid.pos_iter().count(), 12);
    }
}
