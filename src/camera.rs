pub mod camera {
    use crate::ray::Ray;
    use num::Float;
    use r_vector::vector::Vector;
    pub struct Camera<T>
    where
        T: Float,
    {
        origin: Vector<T>,
        lower_left_corner: Vector<T>,
        horizontal: Vector<T>,
        vertical: Vector<T>,
    }

    impl<T> Camera<T>
    where
        T: Float,
    {
        pub fn new(
            origin: Vector<T>,
            lower_left_corner: Vector<T>,
            horizontal: Vector<T>,
            vertical: Vector<T>,
        ) -> Camera<T> {
            Camera {
                origin,
                lower_left_corner,
                horizontal,
                vertical,
            }
        }
        pub fn get_ray(&self, u: T, v: T) -> Ray<T> {
            Ray::new(
                self.origin.clone(),
                &self.lower_left_corner + &(&self.horizontal * u) + &(&self.vertical * v)
                    - &self.origin,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::camera::Camera;
    use crate::ray::Ray;
    use r_vector::vector::Vector;

    #[test]
    fn camera() {
        let lower_left_corner = Vector::<f32>::new(-2.0, -1.0, -1.0);
        let horizontal = Vector::<f32>::new(4.0, 0.0, 0.0);
        let vertical = Vector::<f32>::new(0.0, 2.0, 0.0);
        let origin = Vector::<f32>::new(0.0, 0.0, 0.0);
        let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

        assert_eq!(
            camera.get_ray(1.0, 1.0),
            Ray::<f32>::new(
                Vector::<f32>::new(0.0, 0.0, 0.0),
                Vector::<f32>::new(2.0, 1.0, -1.0)
            )
        );
    }
}
