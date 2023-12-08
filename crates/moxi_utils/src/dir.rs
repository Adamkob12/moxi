//! Directional utilities
use crate::face::*;
use bevy_math::IVec2;
use std::fmt;

/// Notical Directions in the world
#[derive(Clone, Copy, PartialEq)]
pub enum NDir {
    /// +z
    North,
    /// -z
    South,
    /// +x
    East,
    /// -x
    West,
    /// +x +z
    NoEast,
    /// -x +z
    NoWest,
    /// +x -z
    SoEast,
    /// -x -z
    SoWest,
}
use NDir::*;

/// All possible directions
pub const DIRECTIONS: [NDir; 8] = [North, South, East, West, NoEast, NoWest, SoEast, SoWest];

impl Into<&'static str> for NDir {
    fn into(self) -> &'static str {
        match self {
            North => "North",
            South => "South",
            East => "East",
            West => "West",
            NoEast => "NorthEast",
            NoWest => "NorthWest",
            SoEast => "SouthEast",
            SoWest => "SouthWest",
        }
    }
}

impl fmt::Debug for NDir {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "Direction::{}", Into::<&'static str>::into(*self))
    }
}

impl NDir {
    pub fn add_direction(
        dir1: Option<NDir>,
        dir2: Option<NDir>,
    ) -> Option<NDir> {
        if dir1.is_none() {
            return dir2;
        }
        if dir2.is_none() {
            return dir1;
        }

        let d1 = to_cords(dir1);
        let d2 = to_cords(dir2);
        from_cords_change(d1 + d2)
    }

    pub fn opposite(&self) -> NDir {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
            SoEast => NoWest,
            NoWest => SoEast,
            SoWest => NoEast,
            NoEast => SoWest,
        }
    }

    pub fn decompose(&self) -> (Option<NDir>, Option<NDir>) {
        match self {
            North => (Some(North), None),
            South => (Some(South), None),
            East => (None, Some(East)),
            West => (None, Some(West)),
            SoEast => (Some(South), Some(East)),
            NoWest => (Some(North), Some(West)),
            SoWest => (Some(South), Some(West)),
            NoEast => (Some(North), Some(East)),
        }
    }
}

pub fn to_cords(dir: Option<NDir>) -> IVec2 {
    if dir.is_none() {
        return [0, 0].into();
    }
    let dir = dir.unwrap();
    match dir {
        North => [0, 1],
        South => [0, -1],
        East => [1, 0],
        West => [-1, 0],
        NoEast => [1, 1],
        NoWest => [-1, 1],
        SoEast => [1, -1],
        SoWest => [-1, -1],
    }
    .into()
}

pub fn from_cords_change(change: IVec2) -> Option<NDir> {
    let change: [i32; 2] = change.into();
    if change[1] < 0 {
        if change[0] > 0 {
            return Some(NDir::SoEast);
        }
        if change[0] == 0 {
            return Some(NDir::South);
        }
        if change[0] < 0 {
            return Some(NDir::SoWest);
        }
    }
    if change[1] == 0 {
        if change[0] > 0 {
            return Some(NDir::East);
        }
        if change[0] == 0 {
            return None;
        }
        if change[0] < 0 {
            return Some(NDir::West);
        }
    }
    if change[1] > 0 {
        if change[0] > 0 {
            return Some(NDir::NoEast);
        }
        if change[0] == 0 {
            return Some(NDir::North);
        }
        if change[0] < 0 {
            return Some(NDir::NoWest);
        }
    }
    unreachable!();
}

impl From<Face> for NDir {
    fn from(value: Face) -> Self {
        match value {
            Face::Back => Self::North,
            Face::Front => Self::South,
            Face::Right => Self::East,
            Face::Left => Self::West,
            Face::Top => {
                #[cfg(debug_assertions)]
                debug_assert!(false, "Face::Top cannot be cast as Direction");
                Self::North
            }
            Face::Bottom => {
                #[cfg(debug_assertions)]
                debug_assert!(false, "Face::Bottom cannot be cast as Direction");
                Self::South
            }
        }
    }
}

impl Into<usize> for NDir {
    fn into(self) -> usize {
        match self {
            NDir::North => 0,
            NDir::South => 1,
            NDir::East => 2,
            NDir::West => 3,
            NDir::NoEast => 4,
            NDir::NoWest => 5,
            NDir::SoEast => 6,
            NDir::SoWest => 7,
        }
    }
}

impl Into<Face> for NDir {
    fn into(self) -> Face {
        match self {
            Self::NoWest | Self::NoEast | NDir::North => Face::Back,
            Self::SoWest | Self::SoEast | NDir::South => Face::Front,
            Self::West => Face::Left,
            Self::East => Face::Right,
        }
    }
}
