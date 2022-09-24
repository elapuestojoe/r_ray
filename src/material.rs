mod internal {
    use r_float::Float;
    use r_vector::vector::Vector;
    pub fn random_in_unit_sphere<T>() -> Vector<T>
    where
        T: Float,
    {
        let mut vec = Vector::random_range(-T::one(), T::one());
        while vec.length_squared() >= T::one() {
            vec = Vector::random_range(-T::one(), T::one());
        }
        vec
    }
}
pub mod material {
    use crate::hittable::hittable::HitRecord;
    use crate::ray::Ray;
    use r_float::Float;
    use r_vector::vector::Vector;

    pub trait Material<T>: Sized
    where
        T: Float,
    {
        fn scatter(
            &self,
            ray: &Ray<T>,
            hit_record: &HitRecord<T>,
            attenuation: &mut Vector<T>,
            scattered: &mut Ray<T>,
        ) -> bool;
    }
}

pub mod materials_impl {
    use crate::hittable::hittable::HitRecord;
    use crate::material::material::Material;
    use crate::ray::Ray;
    use r_float::Float;
    use r_vector::vector::{Vector, VectorOperations};

    use super::internal;

    pub struct Lambertian<T>
    where
        T: Float,
    {
        albedo: Vector<T>,
    }

    impl<T> Lambertian<T>
    where
        T: Float,
    {
        pub fn new(albedo: Vector<T>) -> Lambertian<T> {
            Lambertian { albedo }
        }
    }

    impl<T> Material<T> for Lambertian<T>
    where
        T: Float,
    {
        fn scatter(
            &self,
            _ray: &Ray<T>,
            hit_record: &HitRecord<T>,
            attenuation: &mut Vector<T>,
            scattered: &mut Ray<T>,
        ) -> bool {
            let target =
                &hit_record.point_at_t + &hit_record.normal + internal::random_in_unit_sphere();
            *scattered = Ray::new(
                hit_record.point_at_t.clone(),
                target - &hit_record.point_at_t,
            );
            *attenuation = self.albedo.clone();
            true
        }
    }

    pub struct Metal<T>
    where
        T: Float,
    {
        albedo: Vector<T>,
    }

    impl<T> Metal<T>
    where
        T: Float,
    {
        pub fn new(albedo: Vector<T>) -> Metal<T> {
            Metal { albedo }
        }
        fn reflect(vector: &Vector<T>, n: &Vector<T>) -> Vector<T> {
            vector - &(n * vector.dot(n) * T::from_i32(2))
        }
    }

    impl<T> Material<T> for Metal<T>
    where
        T: Float,
    {
        fn scatter(
            &self,
            ray: &Ray<T>,
            hit_record: &HitRecord<T>,
            attenuation: &mut Vector<T>,
            scattered: &mut Ray<T>,
        ) -> bool {
            let reflected = Self::reflect(ray.direction(), &hit_record.normal);
            *scattered = Ray::new(hit_record.point_at_t.clone(), reflected);
            *attenuation = self.albedo.clone();
            scattered.direction().dot(&hit_record.normal) > T::zero()
        }
    }
}

mod tests {
    #[test]
    fn point_at_time() {}
}
