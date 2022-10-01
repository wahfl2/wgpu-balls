use crate::util::Vec2;

pub const CENTER_OF_SCREEN: Vec2 = Vec2::new(960.0, 505.5);
const NUM_ITERATIONS: u32 = 4;

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

        let balls_len = self.balls.len();

        for _ in 0..NUM_ITERATIONS {
            for i in 0..balls_len {
                for j in 0..balls_len {
                    if i == j { continue }
                    self.collide(i, j);
                }
            }
        }
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
        self.vel.y += 0.2;
        self.vel *= Vec2::fill(0.9995);

        self.circle_boundary();
    }

    fn circle_boundary(&mut self) {
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