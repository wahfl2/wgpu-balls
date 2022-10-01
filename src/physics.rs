use itertools::Itertools;

use crate::util::Vec2;

pub struct Physics {
    pub(crate) balls: Vec<Ball>,
}

impl Physics {
    pub fn new() -> Self {
        Self { balls: Vec::new() }
    }

    pub fn update(&mut self) {
        for ball in self.balls.iter_mut() {
            ball.update_pos();
        }

        for p in (0..self.balls.len()).permutations(2) {
            self.collide(p[0], p[1]);
        }
    }

    // ewwww
    fn collide(&mut self, i: usize, j: usize) {
        let b = &mut self.balls;
        let added_radii = b[i].radius + b[j].radius;
        if (b[i].pos.x - b[j].pos.x).abs() < added_radii && (b[i].pos.y - b[j].pos.y).abs() < added_radii {
            let distance = b[i].pos.distance(&b[j].pos);
            if distance < added_radii {
                let move_dist = added_radii - distance;
                let resolution_vec = (b[i].pos - b[j].pos).normalize() * Vec2::fill(move_dist * 0.5);

                b[i].pos += resolution_vec;
                b[i].vel += resolution_vec;

                b[j].pos -= resolution_vec;
                b[j].vel -= resolution_vec;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ball {
    pub(crate) radius: f32,
    pub(crate) pos: Vec2,
    pub(crate) vel: Vec2,
}

impl Ball {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            radius,
            pos: Vec2::new(x, y),
            vel: Vec2::new(0., 0.),
        }
    }

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
        self.vel.y -= 0.1;
        self.vel *= Vec2::fill(0.9995);

        self.circle_boundary();
    }

    fn circle_boundary(&mut self) {
        let distance = self.pos.length();
        let allowed_distance = 900.0 - self.radius;

        if distance > allowed_distance {
            let move_dist = distance - allowed_distance;
            let resolution_vec = self.pos.normalize() * move_dist;

            self.pos -= resolution_vec;
            self.vel -= resolution_vec;
        }
    }
}