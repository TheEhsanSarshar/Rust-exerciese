use int_enum::{IntEnum};
use enum_iterator::{Sequence, all};
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value() as u32
    // unimplemented!("convert a color into a numerical representation")
}

pub fn value_to_color_string(value: u32) -> String {
    let colors_array = all::<ResistorColor>().collect::<Vec<_>>();

    for color in colors_array {
        if color.int_value() as u32 == value {
            let res = format!("{:#?}", color).to_string();
            return res
        } 
    }
    String::from("value out of range")
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
