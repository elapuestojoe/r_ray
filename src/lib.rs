pub mod geometry;
pub mod hittable;
pub mod ray {
    use num::Float;
    use r_vector::vector::Vector;

    #[derive(PartialEq, Debug)]
    pub struct Ray<T>
    where
        T: Float,
    {
        origin: Vector<T>,
        direction: Vector<T>,
    }

    impl<T> Ray<T>
    where
        T: Float,
    {
        pub fn new(origin: Vector<T>, direction: Vector<T>) -> Ray<T> {
            Ray {
                origin: origin,
                direction: direction,
            }
        }
        pub fn origin(&self) -> &Vector<T> {
            &self.origin
        }
        pub fn direction(&self) -> &Vector<T> {
            &self.direction
        }
        pub fn point_at_time(&self, time: T) -> Vector<T> {
            &self.origin + &(&self.direction * time)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use r_vector::vector::Vector;
    #[test]
    fn point_at_time() {
        let ray = Ray::new(
            Vector::<f32>::new(2.0, 2.0, 3.0),
            Vector::<f32>::new(10.0, 5.0, 10.0),
        );
        assert_eq!(ray.point_at_time(5.0), Vector::<f32>::new(52.0, 27.0, 53.0));
    }
}
