pub mod sphere {
    use crate::hittable::hittable::{HitRecord, Hittable};
    use crate::ray::Ray;
    use num::Float;
    use r_vector::vector::{Vector, VectorOperations};
    pub struct Sphere<T>
    where
        T: Float,
    {
        center: Vector<T>,
        radius: T,
    }

    impl<T> Sphere<T>
    where
        T: Float,
    {
        pub fn new(center: Vector<T>, radius: T) -> Sphere<T> {
            Sphere {
                center: center,
                radius: radius,
            }
        }
    }

    impl<T> Hittable<T> for Sphere<T>
    where
        T: Float,
    {
        fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T, hit_record: &mut HitRecord<T>) -> bool {
            let oc = ray.origin() - &self.center;

            let a = ray.direction().dot(&ray.direction());
            let b = oc.dot(&ray.direction());
            let c = oc.dot(&oc) - (self.radius * self.radius);
            let discriminant = b * b - a * c;

            if discriminant.is_sign_negative() {
                return false;
            }

            let temp_a = (-b - (b * b - a * c).sqrt()) / a;

            if temp_a < t_max && temp_a > t_min {
                hit_record.t = temp_a;
                hit_record.point_at_t = ray.point_at_time(temp_a);
                hit_record.normal = (&hit_record.point_at_t - &self.center) / self.radius;
                return true;
            }

            let temp_b = (-b + (b * b - a * c).sqrt()) / a;
            if temp_b < t_max && temp_b > t_min {
                hit_record.t = temp_b;
                hit_record.point_at_t = ray.point_at_time(temp_b);
                hit_record.normal = (&hit_record.point_at_t - &self.center) / self.radius;
                return true;
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hittable::hittable::HitRecord;
    use crate::ray::Ray;
    use crate::{geometry::sphere::Sphere, hittable::hittable::Hittable};
    use r_vector::vector::Vector;
    #[test]
    fn sphere_hits() {
        let sphere = Sphere::<f32>::new(Vector::<f32>::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::<f32>::new(
            Vector::<f32>::new(0.0, 0.0, 0.0),
            Vector::<f32>::new(1.0, 0.0, 0.0),
        );
        let mut hit_record = HitRecord::new();
        assert_eq!(sphere.hit(&ray, 0.0, 10.0, &mut hit_record), true);
    }

    #[test]
    fn sphere_no_hits() {
        let sphere = Sphere::<f32>::new(Vector::<f32>::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::<f32>::new(
            Vector::<f32>::new(0.0, 0.0, 0.0),
            Vector::<f32>::new(0.0, 0.0, 0.0),
        );
        let mut hit_record = HitRecord::new();
        assert_eq!(sphere.hit(&ray, 0.0, 10.0, &mut hit_record), false);
    }
}
