use crate::{util::Vec2, quadtree::QuadTree};

pub const CENTER_OF_SCREEN: Vec2 = Vec2::new(960.0, 515.0);
const DAMPING: Vec2 = Vec2::fill(0.9995);

pub struct Physics {
    pub(crate) balls: Vec<Ball>,
    pub(crate) substeps: u32,
    pub(crate) iterations: u32,
}

impl Physics {
    pub fn new(substeps: u32, iterations: u32) -> Self {
        Self { balls: Vec::new(), substeps, iterations }
    }

    pub fn update(&mut self) {
        for ball in self.balls.iter_mut() {
            ball.apply();
            ball.update_pos();
        }

        for _ in 0..self.iterations {
            for (i, j) in self.broad_phase_collisions().iter() {
                self.collide(*i, *j);
            }

            for ball in self.balls.iter_mut() {
                ball.circle_boundary();
            }
        }
    }

    fn broad_phase_collisions(&self) -> Vec<(usize, usize)> {
        let mut quad_tree = QuadTree::new(
            Vec2::new(420.0, 0.0), 
            Vec2::new(1080.0, 1080.0), 
            8, 
            4
        );

        for (i, ball) in self.balls.iter().enumerate() {
            quad_tree.insert_ball(&ball, i);
        }

        quad_tree.get_possible_collisions()
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    // ewwww
    fn collide(&mut self, i: usize, j: usize) {
        let ball_1 = self.balls[i].clone();
        let ball_2 = self.balls[j].clone();

        let b = &mut self.balls;

        let added_radii = ball_1.radius + ball_2.radius;
        if (ball_1.pos.x - ball_2.pos.x).abs() >= added_radii { return }
        if (ball_1.pos.y - ball_2.pos.y).abs() >= added_radii { return }

        let distance = ball_1.pos.distance(&ball_2.pos);
        if distance >= added_radii { return }

        let move_dist = added_radii - distance;
        let resolution_vec = (ball_1.pos - ball_2.pos).normalize() * Vec2::fill(move_dist * 0.5);

        b[i].pos += resolution_vec;
        b[i].vel += resolution_vec;

        b[j].pos -= resolution_vec;
        b[j].vel -= resolution_vec;
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self { balls: Vec::new(), substeps: 1, iterations: 6 }
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

    pub fn apply(&mut self) {
        self.vel.y += 0.2;
        self.vel *= DAMPING;
    }

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
        self.circle_boundary();
    }

    pub fn circle_boundary(&mut self) {
        let distance = self.pos.distance(&CENTER_OF_SCREEN);
        let allowed_distance = 500.0 - self.radius;

        if distance > allowed_distance {
            let move_dist = distance - allowed_distance;
            let resolution_vec = (self.pos - CENTER_OF_SCREEN).normalize() * move_dist;

            self.pos -= resolution_vec;
            self.vel -= resolution_vec;
        }
    }
}