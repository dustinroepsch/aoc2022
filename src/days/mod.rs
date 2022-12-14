pub mod eight;
pub mod five;
pub mod four;
pub mod one;
pub mod seven;
pub mod six;
pub mod three;
pub mod two;

pub type Solution = fn(&str) -> String;

pub struct Day {
    pub part_one: Solution,
    pub part_two: Solution,
}

pub const DAYS: [Day; 8] = [
    one::DAY_ONE,
    two::DAY_TWO,
    three::DAY_THREE,
    four::DAY_FOUR,
    five::DAY_FIVE,
    six::DAY_SIX,
    seven::DAY_SEVEN,
    eight::DAY_EIGHT,
];
