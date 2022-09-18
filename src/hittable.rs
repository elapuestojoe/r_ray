pub mod hittable {
    use crate::ray::Ray;
    use num::Float;
    use r_vector::vector::Vector;
    pub struct HitRecord<T>
    where
        T: Float,
    {
        pub t: T,
        pub point_at_t: Vector<T>,
        pub normal: Vector<T>,
    }

    impl<T> HitRecord<T>
    where
        T: Float,
    {
        pub fn new() -> HitRecord<T> {
            HitRecord {
                t: T::zero(),
                point_at_t: Vector::new(T::zero(), T::zero(), T::zero()),
                normal: Vector::new(T::zero(), T::zero(), T::zero()),
            }
        }
    }

    pub trait Hittable<T>
    where
        T: Float,
    {
        fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T, hit_record: &mut HitRecord<T>) -> bool;
    }
}
