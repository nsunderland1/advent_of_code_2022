use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    slice,
};

use itertools::Itertools;

#[macro_export]
macro_rules! grid {
    [$e:expr; $width:expr, $height:expr] => {
        $crate::grid::Grid::new_filled($e, $width, $height)
    };
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{} ", self[(x, y)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(width * height);
        grid.resize_with(width * height, T::default);

        Self {
            width,
            height,
            grid,
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new_filled(element: T, width: usize, height: usize) -> Self {
        let grid = vec![element; width * height];

        Self {
            width,
            height,
            grid,
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn neighbours(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let x_plus_one = x.checked_add(1).filter(|&xi| xi < self.width());
        let x_sub_one = x.checked_sub(1);
        let y_plus_one = y.checked_add(1).filter(|&yi| yi < self.height());
        let y_sub_one = y.checked_sub(1);

        let make_point = |(l, r)| -> Option<_> { Some((l?, r?)) };

        [
            (x_sub_one, y_sub_one),
            (Some(x), y_sub_one),
            (x_plus_one, y_sub_one),
            (x_sub_one, Some(y)),
            (x_plus_one, Some(y)),
            (x_sub_one, y_plus_one),
            (Some(x), y_plus_one),
            (x_plus_one, y_plus_one),
        ]
        .into_iter()
        .filter_map(make_point)
    }

    pub fn neighbours_orthogonal(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        [
            x.checked_sub(1).map(|xi| (xi, y)),
            x.checked_add(1)
                .filter(|&xi| xi < self.width())
                .map(|xi| (xi, y)),
            y.checked_sub(1).map(|yi| (x, yi)),
            y.checked_add(1)
                .filter(|&yi| yi < self.height())
                .map(|yi| (x, yi)),
        ]
        .into_iter()
        .filter_map(|v| v)
    }

    pub fn into_flat_iter(self) -> impl DoubleEndedIterator<Item = T> {
        self.grid.into_iter()
    }

    pub fn flat_iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.grid.iter_mut()
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        if (0..self.width()).contains(&x) && (0..self.height()).contains(&y) {
            Some(&self.grid[y * self.width + x])
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[y * self.width + x]
    }
}

impl<T, I> FromIterator<I> for Grid<T>
where
    I: IntoIterator<Item = T>,
{
    fn from_iter<S: IntoIterator<Item = I>>(iter: S) -> Self {
        let mut height = 0;

        let grid = iter
            .into_iter()
            .flat_map(|row_iter| {
                height += 1; // hacky but it works!
                row_iter
            })
            .collect_vec();

        Self {
            width: grid.len() / height, // TODO: check that this is actually true?
            height,
            grid,
        }
    }
}

pub struct WindowMut<'a, T> {
    grid: &'a mut Grid<T>,
    min: (usize, usize),
    max: (usize, usize),
}

impl<'a, T> WindowMut<'a, T> {
    pub fn new(grid: &'a mut Grid<T>, min: (usize, usize), max: (usize, usize)) -> Self {
        assert!((0..grid.width()).contains(&min.0));
        assert!((0..grid.width()).contains(&(min.0 + max.0 - 1)));
        assert!((0..grid.height()).contains(&min.1));
        assert!((0..grid.height()).contains(&(min.1 + max.1 - 1)));
        assert!(min.0 < max.0);
        assert!(min.1 < max.1);

        Self { grid, min, max }
    }

    pub fn grow_once(self) -> Self {
        Self::new(
            self.grid,
            (self.min.0 - 1, self.min.1 - 1),
            (self.max.0 + 1, self.max.1 + 1),
        )
    }

    pub fn width(&self) -> usize {
        self.max.0 - self.min.0
    }

    pub fn height(&self) -> usize {
        self.max.1 - self.min.1
    }
}

impl<'a, T> Index<(usize, usize)> for WindowMut<'a, T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[(x + self.min.0, y + self.min.1)]
    }
}

impl<'a, T> IndexMut<(usize, usize)> for WindowMut<'a, T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[(x + self.min.0, y + self.min.1)]
    }
}
