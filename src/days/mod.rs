pub mod one;

pub type Solution = fn(&str) -> String;

pub struct Day {
    pub part_one: Solution,
    pub part_two: Solution,
}
