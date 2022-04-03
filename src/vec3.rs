use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    const EPSILON: f64 = 0.0001;

    fn assert_almost_eq(v1: &Vec3, v2: &Vec3) {
        assert!(
            f64::abs(v1.x - v2.x) <= EPSILON
                && f64::abs(v1.y - v2.y) <= EPSILON
                && f64::abs(v1.z - v2.z) <= EPSILON
        );
    }

    #[test]
    fn test_addition_references() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_almost_eq(&(&v1 + &v2), &Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_addition_value_and_reference() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_almost_eq(&(&v1 + v2), &Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_addition_values() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_almost_eq(&(v1 + v2), &Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_subtraction_references() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-1.0, 4.0, 2.0);
        assert_almost_eq(&(&v1 - &v2), &Vec3::new(2.0, -2.0, 1.0));
    }

    #[test]
    fn test_subtraction_value_and_reference() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-1.0, 4.0, 2.0);
        assert_almost_eq(&(&v1 - v2), &Vec3::new(2.0, -2.0, 1.0));
    }

    #[test]
    fn test_subtraction_values() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-1.0, 4.0, 2.0);
        assert_almost_eq(&(v1 - v2), &Vec3::new(2.0, -2.0, 1.0));
    }

    #[test]
    fn test_division_by_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 2.0;
        assert_almost_eq(&(&v1 / scalar), &Vec3::new(0.5, 1.0, 1.5));
    }
}
