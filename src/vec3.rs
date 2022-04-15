use std::ops;

use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    const EPSILON: f64 = 1e-8;

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn length(self: &Vec3) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn dot(self: &Vec3, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn unit_vector(self: &Vec3) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self: &Vec3) -> bool {
        self.x.abs() < Vec3::EPSILON && self.y.abs() < Vec3::EPSILON && self.z.abs() < Vec3::EPSILON
    }

    pub fn reflect(self: &Vec3, normal: &Vec3) -> Vec3 {
        self - normal * self.dot(normal) * 2.0
    }

    pub fn refract(unit_vector: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(normal.dot(&-unit_vector), 1.0);
        let res_perp = (unit_vector + normal * cos_theta) * etai_over_etat;
        let res_parallel = normal * -(1.0 - res_perp.dot(&res_perp)).abs().sqrt();

        res_perp + res_parallel
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            thread_rng().gen_range(min..max),
            thread_rng().gen_range(min..max),
            thread_rng().gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.dot(&p) >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(&normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    fn assert_almost_eq(f1: f64, f2: f64) {
        assert!((f1 - f2).abs() <= Vec3::EPSILON);
    }

    fn assert_vec3_almost_eq(v1: &Vec3, v2: &Vec3) {
        assert!((v1 - v2).near_zero())
    }

    #[test]
    fn test_addition_references() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_vec3_almost_eq(&(&v1 + &v2), &Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_addition_value_and_reference() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_vec3_almost_eq(&(&v1 + v2), &Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_addition_values() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_vec3_almost_eq(&(v1 + v2), &Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_subtraction_references() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-1.0, 4.0, 2.0);
        assert_vec3_almost_eq(&(&v1 - &v2), &Vec3::new(2.0, -2.0, 1.0));
    }

    #[test]
    fn test_subtraction_value_and_reference() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-1.0, 4.0, 2.0);
        assert_vec3_almost_eq(&(&v1 - v2), &Vec3::new(2.0, -2.0, 1.0));
    }

    #[test]
    fn test_subtraction_values() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-1.0, 4.0, 2.0);
        assert_vec3_almost_eq(&(v1 - v2), &Vec3::new(2.0, -2.0, 1.0));
    }

    #[test]
    fn test_multiplication_reference() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_vec3_almost_eq(&(&v1 * 2.0), &Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_multiplication_value() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_vec3_almost_eq(&(v1 * 4.0), &Vec3::new(4.0, 8.0, 12.0));
    }

    #[test]
    fn test_division_by_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 2.0;
        assert_vec3_almost_eq(&(&v1 / scalar), &Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(5.0, 2.0, 1.0);
        assert_almost_eq(v1.dot(&v2), 12.0);
    }

    #[test]
    fn test_length() {
        let v1 = Vec3::new(1.0, 2.0, 2.0);
        assert_almost_eq(v1.length(), 3.0);
    }

    #[test]
    fn test_unit_vector() {
        let v1 = Vec3::new(1.0, 2.0, 2.0);
        assert_vec3_almost_eq(
            &v1.unit_vector(),
            &Vec3::new(0.3333333333, 0.66666666666, 0.66666666666),
        );
    }
}
