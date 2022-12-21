use std::{fmt::Display, str::FromStr};

use super::Day;

pub const DAY_EIGHT: Day = Day { part_one, part_two };

type Grid = Vec<Vec<u8>>;

struct TreeHeightMap {
    height: Grid,
    from_north: Grid,
    from_south: Grid,
    from_east: Grid,
    from_west: Grid,
}

impl TreeHeightMap {
    pub fn visible(&self, row: usize, col: usize) -> bool {
        let height = self.height[row][col];

        let from_north = self.from_north[row][col];
        let from_south = self.from_south[row][col];
        let from_east = self.from_east[row][col];
        let from_west = self.from_west[row][col];

        height > from_north || height > from_south || height >= from_east || height >= from_west
    }

    pub fn width(&self) -> usize {
        self.height[0].len()
    }

    pub fn height(&self) -> usize {
        self.height.len()
    }
}

impl Display for TreeHeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_grid = |f: &mut std::fmt::Formatter<'_>, grid: &Grid| -> std::fmt::Result {
            for row in grid {
                for col in row {
                    write!(f, "{}", col)?;
                }
                writeln!(f)?;
            }
            Ok(())
        };

        writeln!(f, "Height:")?;
        fmt_grid(f, &self.height)?;

        writeln!(f, "From North:")?;
        fmt_grid(f, &self.from_north)?;

        writeln!(f, "From South:")?;
        fmt_grid(f, &self.from_south)?;

        writeln!(f, "From East:")?;
        fmt_grid(f, &self.from_east)?;

        writeln!(f, "From West:")?;
        fmt_grid(f, &self.from_west)?;

        Ok(())
    }
}

impl FromStr for TreeHeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = {
            let mut height: Vec<Vec<u8>> = Vec::new();

            for line in s.lines() {
                let mut row: Vec<u8> = Vec::new();

                for c in line.chars() {
                    row.push(c.to_digit(10).unwrap() as u8);
                }

                height.push(row);
            }
            height
        };

        let from_west: Vec<Vec<u8>> = {
            let mut from_west = height.clone();
            for row in from_west.iter_mut() {
                let mut max = 0;
                for col in row.iter_mut() {
                    if *col > max {
                        max = *col;
                    } else {
                        *col = max;
                    }
                }
            }
            from_west
        };

        let from_east: Vec<Vec<u8>> = {
            let mut from_east = height.clone();
            for row in from_east.iter_mut() {
                let mut max = 0;
                for col in row.iter_mut().rev() {
                    if *col > max {
                        max = *col;
                    } else {
                        *col = max;
                    }
                }
            }
            from_east
        };

        let from_north: Vec<Vec<u8>> = {
            let mut from_north = height.clone();
            for col in 0..from_north[0].len() {
                let mut max = 0;
                for row in from_north.iter_mut() {
                    if row[col] > max {
                        max = row[col];
                    } else {
                        row[col] = max;
                    }
                }
            }
            from_north
        };

        let from_south: Vec<Vec<u8>> = {
            let mut from_south = height.clone();
            for col in 0..from_south[0].len() {
                let mut max = 0;
                for row in from_south.iter_mut().rev() {
                    if row[col] > max {
                        max = row[col];
                    } else {
                        row[col] = max;
                    }
                }
            }
            from_south
        };

        Ok(TreeHeightMap {
            height,
            from_north,
            from_south,
            from_east,
            from_west,
        })
    }
}

fn part_one(input: &str) -> String {
    let m: TreeHeightMap = input.parse().unwrap();
    let mut count = 0;
    for row in 0..m.height() {
        for col in 0..m.width() {
            if m.visible(row, col) {
                count = count + 1;
            }
        }
    }
    count.to_string()
}

fn part_two(_input: &str) -> String {
    todo!()
}
