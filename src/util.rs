use std::f32::consts::PI;

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
                    let angle = (i as f32 / angle_divisor) * PI;
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