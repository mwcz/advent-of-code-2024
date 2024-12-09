use std::{
    fmt::Display,
    ops::{Add, Neg, Sub},
};

use crate::{
    direction::{CardDir, CardOrdDir},
    grid::Grid,
};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct Point<const D: usize> {
    pub coords: [i64; D],
}

impl<const D: usize> Sub for Point<D> {
    type Output = Point<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_coords = self.coords;
        for (i, c) in new_coords.iter_mut().enumerate() {
            *c -= rhs.coords[i];
        }
        new_coords.into()
    }
}

impl<const D: usize> Add for Point<D> {
    type Output = Point<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_coords = self.coords;
        for (i, c) in new_coords.iter_mut().enumerate() {
            *c += rhs.coords[i];
        }
        new_coords.into()
    }
}

impl<const D: usize> Neg for Point<D> {
    type Output = Point<D>;

    fn neg(self) -> Self::Output {
        let mut new_coords = self.coords;
        for (i, c) in new_coords.iter_mut().enumerate() {
            *c *= -1;
        }
        new_coords.into()
    }
}

impl<const D: usize> Point<D> {
    pub fn new(coords: &[i64; D]) -> Self {
        Self { coords: *coords }
    }

    pub fn x(&self) -> i64 {
        self.coords[0]
    }
    pub fn y(&self) -> i64 {
        self.coords[1]
    }
    pub fn z(&self) -> i64 {
        self.coords[2]
    }

    /// Set a new value for the x coordinate.
    pub fn set_x(&mut self, new_x: i64) {
        self.coords[0] = new_x;
    }

    /// Set a new value for the y coordinate.
    pub fn set_y(&mut self, new_y: i64) {
        self.coords[1] = new_y;
    }

    /// Set a new value for the z coordinate.
    pub fn set_z(&mut self, new_z: i64) {
        self.coords[2] = new_z;
    }

    /// Get the magnitude of the point, considered as a vector.
    pub fn mag(&self) -> i64 {
        self.coords.iter().product::<i64>() / self.coords.len() as i64
    }

    /// Attempt to move the point one unit in the given direction (no diagonals), within a grid
    /// bounds.  Returns None if the move would push the point outside the bounds of the grid.
    pub fn move_in_grid<T: Copy>(&self, dir: CardDir, grid: &Grid<T>) -> Option<Point<D>> {
        let mut p = *self;

        match dir {
            CardDir::Up => p.set_y(p.y().checked_sub(1)?),
            CardDir::Down => p.set_y(p.y().checked_add(1)?),
            CardDir::Left => p.set_x(p.x().checked_sub(1)?),
            CardDir::Right => p.set_x(p.x().checked_add(1)?),
        }

        if p.x() > 0 && grid.width() as i64 > p.x() && p.y() > 0 && grid.height() as i64 > p.y() {
            Some(p)
        } else {
            None
        }
    }

    /// Attempt to move the point one unit in the given direction (diagonals allowed), within a
    /// grid bounds.  Returns None if the move would push the point outside the bounds of the grid.
    pub fn move_in_grid_diag<T: Copy>(&self, dir: CardOrdDir, grid: &Grid<T>) -> Option<Point<D>> {
        let mut p = *self;
        match dir {
            CardOrdDir::Up => {
                p.set_y(p.y().checked_sub(1)?);
            }
            CardOrdDir::Down => {
                p.set_y(p.y().checked_add(1)?);
            }
            CardOrdDir::Left => {
                p.set_x(p.x().checked_sub(1)?);
            }
            CardOrdDir::Right => {
                p.set_x(p.x().checked_add(1)?);
            }
            CardOrdDir::UpLeft => {
                p.set_x(p.x().checked_sub(1)?);
                p.set_y(p.y().checked_sub(1)?);
            }
            CardOrdDir::UpRight => {
                p.set_x(p.x().checked_add(1)?);
                p.set_y(p.y().checked_sub(1)?);
            }
            CardOrdDir::DownLeft => {
                p.set_x(p.x().checked_sub(1)?);
                p.set_y(p.y().checked_add(1)?);
            }
            CardOrdDir::DownRight => {
                p.set_x(p.x().checked_add(1)?);
                p.set_y(p.y().checked_add(1)?);
            }
        }

        if p.x() > 0 && grid.width() as i64 > p.x() && p.y() > 0 && grid.height() as i64 > p.y() {
            Some(p)
        } else {
            None
        }
    }
}

// Make possible the nice pattern `&[1,2,3].into()` to create a Point.
impl<const D: usize> From<&[i32; D]> for Point<D> {
    fn from(coords: &[i32; D]) -> Self {
        Point {
            coords: coords.map(|n| n as i64),
        }
    }
}

// Make possible the nice pattern `[1,2,3].into()` to create a Point.
impl<const D: usize> From<[i32; D]> for Point<D> {
    fn from(coords: [i32; D]) -> Self {
        Point {
            coords: coords.map(|n| n as i64),
        }
    }
}

// Make possible the nice pattern `&[1,2,3].into()` to create a Point.
impl<const D: usize> From<&[i64; D]> for Point<D> {
    fn from(coords: &[i64; D]) -> Self {
        Point { coords: *coords }
    }
}

// Make possible the nice pattern `[1,2,3].into()` to create a Point.
impl<const D: usize> From<[i64; D]> for Point<D> {
    fn from(coords: [i64; D]) -> Self {
        Point { coords }
    }
}

// Make possible the nice pattern `&[1,2,3].into()` to create a Point.
impl<const D: usize> From<&[usize; D]> for Point<D> {
    fn from(coords: &[usize; D]) -> Self {
        Point {
            coords: coords.map(|n| n as i64),
        }
    }
}

// Make possible the nice pattern `[1,2,3].into()` to create a Point.
impl<const D: usize> From<[usize; D]> for Point<D> {
    fn from(coords: [usize; D]) -> Self {
        Point {
            coords: coords.map(|n| n as i64),
        }
    }
}

impl<const D: usize> Display for Point<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, n) in self.coords.iter().enumerate() {
            write!(f, "{}", n)?;
            if i < self.coords.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}
