use num::Zero;

#[derive(Debug, PartialEq)]
enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
}

struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: PartialOrd + Copy + std::ops::Add<Output = T> + Zero,
{
    fn new(a: T, b: T, c: T) -> Self {
        Triangle { a, b, c }
    }

    fn is_valid(&self) -> bool {
        self.a > T::zero() && self.b > T::zero() && self.c > T::zero() &&
        self.a + self.b >= self.c &&
        self.b + self.c >= self.a &&
        self.a + self.c >= self.b
    }

    fn determine_type(&self) -> Option<TriangleType> {
        if !self.is_valid() {
            return None;
        }

        if self.a == self.b && self.b == self.c {
            Some(TriangleType::Equilateral)
        } else if self.a == self.b || self.b == self.c || self.a == self.c {
            Some(TriangleType::Isosceles)
        } else {
            Some(TriangleType::Scalene)
        }
    }
}

fn main() {
    let triangle = Triangle::new(1, 2, 3);
    match triangle.determine_type() {
        Some(triangle_type) => println!("{:?}", triangle_type),
        None => println!("Invalid triangle"),
    }
}