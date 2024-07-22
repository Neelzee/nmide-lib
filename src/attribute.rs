use crate::{Location, Msg};

pub type Func = Box<dyn Fn() -> Msg>;

pub enum Attribute {
    Styling(Css),
    Alt(String),
    Src(String),
    Id(String),
    Class(String),
    Location(Location),
    OnClick(Func),
    OnHover(Func),
    OnLeave(Func),
    OnEnter(Func),
}

pub enum Css {
    Width(i32, Unit),
    Height(i32, Unit),
    Padding(Direction, i32, Unit),
    BackgroundColor(Color),
    Color(Color),
}

pub enum Color {
    White,
    Black,
    Red,
    Blue,
    RGB(u8, u8, u8),
    Hex(String),
}

pub enum Unit {
    Pixel,
    Percentage,
}

pub enum Direction {
    Top,
    Bottom,
    Right,
    Left,
}
