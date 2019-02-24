use std::ops::Add;

#[derive(Default, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn from_real(re: f64) -> Complex {
        Complex { re, im: 0.0 }
    }

    pub fn from_imaginary(im: f64) -> Complex {
        Complex { re: 0.0, im }
    }

    pub fn from_polar(magnitude: f64, angle: f64) -> Complex {
        let re = magnitude * angle.cos();
        let im = magnitude * angle.sin();
        Complex { re, im }
    }

    pub fn magnitude(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    pub fn angle(&self) -> f64 {
        self.im.atan2(self.re)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}
#[cfg(test)]
mod complex_test_suite {
    use super::*;

    #[test]
    fn test_default() {
        let z: Complex = Default::default();
        assert_eq!(0.0, z.im);
        assert_eq!(0.0, z.re);
    }

    #[test]
    fn test_from_real() {
        let z = Complex::from_real(1.0);
        assert_eq!(1.0, z.re);
        assert_eq!(0.0, z.im);
    }

    #[test]
    fn test_from_imaginary() {
        let z = Complex::from_imaginary(1.0);
        assert_eq!(0.0, z.re);
        assert_eq!(1.0, z.im);
    }

    #[test]
    fn test_from_polar() {
        let z = Complex::from_polar(1.0, 0.0);
        assert_eq!(1.0, z.re);
        assert_eq!(0.0, z.im);
    }

    #[test]
    fn test_magnitude() {
        let z = Complex::from_polar(1.0, 0.0);
        assert_eq!(1.0, z.magnitude());
    }

    #[test]
    fn test_angle() {
        let z = Complex::from_polar(1.0, 0.0);
        assert_eq!(0.0, z.angle());
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Complex { re: 1.0, im: 0.0 } + Complex { re: 2.0, im: 1.0 },
            Complex { re: 3.0, im: 1.0 }
        );
    }
}
