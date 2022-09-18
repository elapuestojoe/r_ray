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

    pub struct HittableList<T>
    where
        T: Float,
    {
        elements: Vec<Box<dyn Hittable<T>>>,
    }

    impl<T> HittableList<T>
    where
        T: Float,
    {
        pub fn new(elements: Vec<Box<dyn Hittable<T>>>) -> HittableList<T> {
            HittableList { elements }
        }
    }

    impl<T> Hittable<T> for HittableList<T>
    where
        T: Float,
    {
        fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T, hit_record: &mut HitRecord<T>) -> bool {
            let mut temporal_record = HitRecord::<T>::new();
            let mut hit_anything = false;
            let mut closest_so_far = t_max;
            for element in self.elements.iter() {
                if element.hit(ray, t_min, closest_so_far, &mut temporal_record) {
                    hit_anything = true;
                    closest_so_far = temporal_record.t;
                }
            }
            if hit_anything {
                *hit_record = temporal_record;
            }
            hit_anything
        }
    }
}