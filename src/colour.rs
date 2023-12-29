#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Colour {
    pub fn diff(c1: Colour, c2: Colour) -> Colour {
        return Colour {
            r: c1.r.wrapping_sub(c2.r),
            g: c1.g.wrapping_sub(c2.g),
            b: c1.b.wrapping_sub(c2.b),
            a: c1.a.wrapping_sub(c2.a),
        }
    }

    pub fn set_r(&mut self, value: u8) {
        self.r = value;
    }

    pub fn set_g(&mut self, value: u8) {
        self.g = value;
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::colour::Colour;

    #[test]
    fn equal() {
        let x = Colour { r: 20, g: 30, b: 40, a: 255 };
        let y = Colour { r: 20, g: 30, b: 40, a: 255 };

        assert_eq!(x, y)
    }
}

