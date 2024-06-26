pub struct Grid<T> {
    rows: Box<[Box<[T]>]>,
}

impl<T> Grid<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let rows = rows.into_iter().map(|row| row.into()).collect();
        Self { rows }
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell<'_, T>> {
        (0..self.rows.len()).flat_map(move |row| {
            (0..self.rows[row].len()).map(move |col| Cell {
                grid: self,
                row,
                col,
            })
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
        .into_iter()
    }
}

#[derive(Clone, Copy)]
pub struct Cell<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    col: usize,
}

impl<'a, T> Cell<'a, T> {
    pub fn neighbour(&self, direction: Direction) -> Option<Cell<'a, T>> {
        let (row, col) = match direction {
            Direction::Up => (self.row.checked_sub(1)?, self.col),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col.checked_sub(1)?),
            Direction::Right => (self.row, self.col + 1),
            Direction::UpLeft => (self.row.checked_sub(1)?, self.col.checked_sub(1)?),
            Direction::UpRight => (self.row.checked_sub(1)?, self.col + 1),
            Direction::DownLeft => (self.row + 1, self.col.checked_sub(1)?),
            Direction::DownRight => (self.row + 1, self.col + 1),
        };

        self.grid.rows.get(row)?.get(col).map(|_| Cell {
            grid: self.grid,
            row,
            col,
        })
    }

    pub fn value(&'a self) -> &'a T {
        &self.grid.rows[self.row][self.col]
    }
}
