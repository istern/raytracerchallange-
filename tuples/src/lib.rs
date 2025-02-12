use std::ops::{Add, Sub, Neg};

#[derive(Debug,PartialEq, Default, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple{
    pub fn is_point(& self)->bool{
        self.w == 1.0
    }

    pub fn is_vector(& self)->bool{
        self.w == 0.0
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self  {
        Self  {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// Implement PartialEq so Point can be compared with Tuple
impl PartialEq<Tuple> for Point {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        other.w == 1.0
    }
}

impl PartialEq<Tuple> for Vector {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        other.w == 0.0
    }
}

#[derive(Debug,PartialEq, Default, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, other: Self) -> Vector  {
        Vector  {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, other: Vector) -> Point  {
        Point  {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl PartialEq<Point> for Tuple {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == 1.0
    }
}

#[derive(Debug,PartialEq, Default, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl PartialEq<Vector> for Tuple {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == 0.0
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self  {
        Self  {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Point> for Vector {
    type Output = Point;
    fn sub(self, other: Point) -> Point  {
        Point  {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}





