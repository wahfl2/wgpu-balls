use std::{f32::consts::PI, ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg}};

use cgmath::Vector2;
use rand::Rng;

use crate::vertex::Vertex;

pub struct RenderCircle {
    pub(crate) num_vertices: u32,
    pub(crate) color: [f32; 3],

    vertices: Option<Vec<Vertex>>,
    indices: Option<Vec<u32>>,
}

impl RenderCircle {
    pub fn new(num_vertices: u32, color: [f32; 3]) -> Self {
        Self { num_vertices, color, vertices: None, indices: None }
    }

    pub fn get_vertices(&mut self) -> Vec<Vertex> {
        match &self.vertices {
            Some(e) => return e.clone(),
            None => {
                let mut vertices = Vec::new();
                let angle_divisor = self.num_vertices as f32;
        
                for i in 0..self.num_vertices {
                    let angle = (i as f32 / angle_divisor) * PI * 2.0;
                    vertices.push(Vertex {
                        position: [angle.cos(), angle.sin()],
                        color: self.color,
                    });
                }
        
                self.vertices = Some(vertices.clone());
                return vertices;
            }
        }
    }

    pub fn get_indices(&mut self) -> Vec<u32> {
        match &self.indices {
            Some(e) => return e.clone(),
            None => {
                let mut indices = Vec::new();

                for i in 2..self.num_vertices {
                    indices.push(0u32);
                    indices.push(i-1);
                    indices.push(i);
                }

                self.indices = Some(indices.clone());
                return indices;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn fill(n: f32) -> Self {
        Self { x: n, y: n }
    }

    pub fn distance(&self, other: &Vec2) -> f32 {
        let a = self.x - other.x;
        let b = self.y - other.y;

        return ((a*a) + (b*b)).sqrt()
    }

    pub fn length(&self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn normalize(self) -> Self {
        return self / Vec2::fill(self.length())
    }

    pub fn both_greater_eq(&self, other: &Vec2) -> bool {
        self.x >= other.x && self.y >= other.y
    }

    pub fn both_less_eq(&self, other: &Vec2) -> bool {
        self.x <= other.x && self.y <= other.y
    }
}

impl Into<Vector2<f32>> for Vec2 {
    fn into(self) -> Vector2<f32> {
        Vector2 { x: self.x, y: self.y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y 
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y 
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x * rhs.x, 
            y: self.y * rhs.y 
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self * Vec2::fill(rhs)
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { 
            x: self.x / rhs.x, 
            y: self.y / rhs.y 
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self / Vec2::fill(rhs)
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub const RED: Self = Color::new(1.0, 0.0, 0.0);
    pub const GREEN: Self = Color::new(0.0, 1.0, 0.0);
    pub const BLUE: Self = Color::new(0.0, 0.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Color::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn into_arr(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}