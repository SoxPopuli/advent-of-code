use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use crate::Pos;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,

    pub items: HashMap<Pos, T>,
}
impl<T> Grid<T> {
    pub fn get(&self, p: &Pos) -> Option<&T> {
        self.items.get(p)
    }

    pub fn is_inside(&self, Pos { x, y }: &Pos) -> bool {
        let is_negative = *x < 0 || *y < 0;
        let is_outside = *x >= self.width || *y >= self.height;

        !is_outside && !is_negative
    }

    pub fn from_grid_string(grid: &str, mut handle_cell: impl FnMut(Pos, u8) -> Option<T> ) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut items = HashMap::new();

        for (y, line) in grid.lines().take_while(|x| !x.is_empty()).enumerate() {
            let line = line.trim();

            width = width.max(line.len() as isize);
            height += 1;

            for (x, cell) in line.as_bytes().iter().enumerate() {
                let pos = Pos {
                    x: x as isize,
                    y: y as isize,
                };
                if let Some(cell) = handle_cell(pos, *cell) {
                    items.insert(pos, cell);
                }
            }
        }

        Self {
            width,
            height,
            items,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Pos, &'_ T)> {
        self.items.iter()
    }

    /// Iterates over all valid diagonal directions
    pub fn iter_adjacent_diagonal(&self, p: Pos) -> impl Iterator<Item = (&'_ T, Pos)> {
        crate::vectors::DIAGONAL
            .iter()
            .map(move |v| p + *v)
            .filter_map(|p| self.get(&p).map(|x| (x, p)))
    }

    /// Iterates over all valid cardinal directions
    pub fn iter_adjacent_cardinal(&self, p: Pos) -> impl Iterator<Item = (&'_ T, Pos)> {
        crate::vectors::CARDINAL
            .iter()
            .map(move |v| p + *v)
            .filter_map(|p| self.get(&p).map(|x| (x, p)))
    }

    /// Iterates over all valid directions
    pub fn iter_adjacent(&self, p: Pos) -> impl Iterator<Item = (&'_ T, Pos)> {
        crate::vectors::ALL
            .iter()
            .map(move |v| p + *v)
            .filter_map(|p| self.get(&p).map(|x| (x, p)))
    }

    pub fn swap(&mut self, a: Pos, b: Pos) {
        let first = self.items.contains_key(&a);
        let second = self.items.contains_key(&b);

        match (first, second) {
            (true, true) => {
                let a = self.items.get_mut(&a).unwrap() as *mut T;
                let b = self.items.get_mut(&b).unwrap() as *mut T;

                unsafe {
                    std::ptr::swap(a, b);
                }
            }
            (true, false) => {
                let first = self.items.remove(&a).unwrap();
                self.items.insert(b, first);
            }
            (false, true) => {
                let second = self.items.remove(&b).unwrap();
                self.items.insert(a, second);
            }
            (false, false) => {}
        }
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(&Pos { x, y }) {
                    Some(cell) => cell.fmt(f)?,
                    None => f.write_char('.')?,
                }
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
