pub mod sphere {
    use crate::hittable::hittable::{HitRecord, Hittable};
    use crate::material::material::Material;
    use crate::ray::Ray;
    use r_float::Float;
    use r_vector::vector::{Vector, VectorOperations};
    pub struct Sphere<M, T>
    where
        M: Material<T>,
        T: Float,
    {
        center: Vector<T>,
        radius: T,
        pub material: M,
    }

    impl<M, T> Sphere<M, T>
    where
        M: Material<T>,
        T: Float,
    {
        pub fn new(center: Vector<T>, radius: T, material: M) -> Sphere<M, T> {
            Sphere {
                center,
                radius,
                material,
            }
        }
    }

    impl<M, T> Hittable<T> for Sphere<M, T>
    where
        M: Material<T>,
        T: Float,
    {
        fn hit(
            &mut self,
            ray: &Ray<T>,
            t_min: T,
            t_max: T,
            hit_record: &mut HitRecord<T>,
        ) -> Option<Box<&mut dyn Material<T>>> {
            let oc = ray.origin() - &self.center;

            let a = ray.direction().dot(&ray.direction());
            let b = oc.dot(&ray.direction());
            let c = oc.dot(&oc) - (self.radius * self.radius);
            let discriminant = b * b - a * c;

            if discriminant.is_sign_negative() {
                return Option::None;
            }

            let temp_a = (-b - (b * b - a * c).sqrt()) / a;

            if temp_a < t_max && temp_a > t_min {
                hit_record.t = temp_a;
                hit_record.point_at_t = ray.point_at_time(temp_a);
                hit_record.normal = (&hit_record.point_at_t - &self.center) / self.radius;
                return Option::Some(Box::new(&mut self.material));
            }

            let temp_b = (-b + (b * b - a * c).sqrt()) / a;
            if temp_b < t_max && temp_b > t_min {
                hit_record.t = temp_b;
                hit_record.point_at_t = ray.point_at_time(temp_b);
                hit_record.normal = (&hit_record.point_at_t - &self.center) / self.radius;
                return Option::Some(Box::new(&mut self.material));
            }
            Option::None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::sphere::Sphere;
    use crate::material::materials_impl::Lambertian;
    use crate::material::materials_impl::Metal;
    use r_vector::vector::Vector;
    #[test]
    fn sphere_lambertian() {
        let _sphere = Sphere::new(
            Vector::<f32>::new(0.0, 0.0, 0.0),
            1.0,
            Lambertian::new(Vector::new(0.8, 0.3, 0.3)),
        );
    }

    #[test]
    fn sphere_metal() {
        let _sphere = Sphere::new(
            Vector::<f32>::new(0.0, 0.0, 0.0),
            1.0,
            Metal::new(Vector::new(0.8, 0.3, 0.3)),
        );
    }
}
