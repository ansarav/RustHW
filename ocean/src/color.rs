#[derive(Eq, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn new_red() -> Color {
        Color::new(255, 0, 0)
    }

    pub fn new_green() -> Color {
        Color::new(0, 255, 0)
    }

    pub fn new_blue() -> Color {
        Color::new(0, 0, 255)
    }


    pub fn cross(c1: &Color, c2: &Color) -> Color {
        //could also use let to to declare set variable fields
        Color {
            //set the field instance
            r: c1.r.wrapping_add(c2.r),
            g: c1.g.wrapping_add(c2.g),
            b: c1.b.wrapping_add(c2.b),
        }
    }
}

