pub mod hittable {
    use crate::{material::material::Material, ray::Ray};
    use r_float::Float;
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
        fn hit(
            &mut self,
            ray: &Ray<T>,
            t_min: T,
            t_max: T,
            hit_record: &mut HitRecord<T>,
        ) -> Option<Box<&mut dyn Material<T>>>;
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
        fn hit(
            &mut self,
            ray: &Ray<T>,
            t_min: T,
            t_max: T,
            hit_record: &mut HitRecord<T>,
        ) -> Option<Box<&mut dyn Material<T>>> {
            let mut temporal_record = HitRecord::new();
            let mut closest_hit: Option<Box<&mut dyn Material<T>>> = Option::None;
            let mut closest_so_far = t_max;
            for element in self.elements.iter_mut() {
                let hit = element.hit(ray, t_min, closest_so_far, &mut temporal_record);
                if hit.is_some() {
                    closest_hit = hit;
                    closest_so_far = temporal_record.t;
                }
            }
            if closest_hit.is_some() {
                *hit_record = temporal_record;
            }
            closest_hit
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::sphere::Sphere;
    use crate::material::materials_impl::{Lambertian, Metal};
    use r_vector::vector::Vector;

    use super::hittable::HittableList;

    #[test]
    fn spheres_hittable_list() {
        let sphere_1 = Sphere::new(
            Vector::<f32>::new(0.0, 0.0, 0.0),
            1.0,
            Lambertian::new(Vector::new(0.8, 0.3, 0.3)),
        );
        let sphere_2 = Sphere::new(
            Vector::new(0f32, -100.5, -1f32),
            100f32,
            Lambertian::new(Vector::new(0.8, 0.8, 0f32)),
        );
        let sphere_3 = Sphere::new(
            Vector::new(1f32, 0f32, -1f32),
            0.5,
            Metal::new(Vector::new(0.8, 0.6, 0.2)),
        );
        let sphere_4 = Sphere::new(
            Vector::new(-1f32, 0f32, -1f32),
            0.5,
            Metal::new(Vector::new(0.8, 0.8, 0.8)),
        );
        HittableList::new(vec![
            Box::new(sphere_1),
            Box::new(sphere_2),
            Box::new(sphere_3),
            Box::new(sphere_4),
        ]);
    }
}
