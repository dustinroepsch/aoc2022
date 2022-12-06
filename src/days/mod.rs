pub mod four;
pub mod one;
pub mod three;
pub mod two;

pub type Solution = fn(&str) -> String;

pub struct Day {
    pub part_one: Solution,
    pub part_two: Solution,
}

pub const DAYS: [Day; 4] = [one::DAY_ONE, two::DAY_TWO, three::DAY_THREE, four::DAY_FOUR];
