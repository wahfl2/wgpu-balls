use std::{rc::Rc, collections::HashSet};

use itertools::Itertools;

use crate::{util::Vec2, physics::Ball};

#[derive(Debug, Clone)]
pub struct QuadTree {
    node: QuadTreeNode,
    pub(crate) max_depth: usize,
    pub(crate) max_size: usize,
}

#[derive(Debug, Clone)]
pub struct QuadTreeNode {
    pos: Vec2,
    size: Vec2,
    depth: usize,
    children: Option<Rc<[QuadTreeNode; 4]>>,
    contents: Vec<QuadTreeEntry>,
    max_size: usize,
    max_depth: usize,
}

#[derive(Debug, Clone)]
pub struct QuadTreeEntry {
    pos: Vec2,
    size: Vec2,
    ball_index: usize,
}

impl QuadTree {
    pub fn new(pos: Vec2, size: Vec2, max_depth: usize, max_size: usize) -> Self {
        Self {
            node: QuadTreeNode::new(pos, size, 0, max_size, max_depth),
            max_depth,
            max_size,
        }
    }

    pub fn insert_ball(&mut self, ball: &Ball, ball_index: usize) {
        self.insert(QuadTreeEntry::new(ball.pos - Vec2::fill(ball.radius), Vec2::fill(ball.radius * 2.0), ball_index))
    }

    pub fn insert(&mut self, entry: QuadTreeEntry) {
        self.node.insert(entry);
    }

    pub fn get_possible_collisions(&self) -> Vec<(usize, usize)> {
        let leaf_contents = self.node.get_leaf_contents();
        let mut collision_set = HashSet::new();

        for leaf in leaf_contents.into_iter() {
            let len = leaf.len();
            for i in 0..len {
                for j in (i+1)..len {
                    let object_i = leaf[i].ball_index;
                    let object_j = leaf[j].ball_index;

                    if object_i > object_j {
                        collision_set.insert((object_i, object_j));
                    } else {
                        collision_set.insert((object_j, object_i));
                    }
                }
            }
        }

        collision_set.into_iter().collect()
    }
}

impl QuadTreeNode {
    pub fn new(pos: Vec2, size: Vec2, depth: usize, max_size: usize, max_depth: usize) -> Self {
        Self { 
            pos,
            size,
            depth,
            children: None,
            contents: Vec::with_capacity(max_size + 1),
            max_size,
            max_depth,
        }
    }
    
    fn create_child(&self, pos: Vec2, size: Vec2) -> Self {
        QuadTreeNode::new(pos, size, self.depth + 1, self.max_size, self.max_depth)
    }

    fn create_children(&self) -> [Self; 4] {
        let half_size = self.size / 2.0;
        let half_pos = self.pos + half_size;
        
        [
            self.create_child(self.pos, half_size),
            self.create_child(Vec2::new(self.pos.x, half_pos.y), half_size),
            self.create_child(Vec2::new(half_pos.x, self.pos.y), half_size),
            self.create_child(half_pos, half_size),
        ]
    }

    fn split(&mut self) {
        self.children = Some(Rc::new(self.create_children()));
    }

    fn insert(&mut self, entry: QuadTreeEntry) {
        if !entry.colliding(&self.pos, &self.size) {
            return;
        }

        if self.children.is_some() {
            self.push_to_children(entry);
        } else {
            self.contents.push(entry);
            if self.contents.len() > self.max_size && self.depth < self.max_depth {
                self.split();
                for object in self.contents.clone().into_iter() {
                    self.push_to_children(object);
                }
                self.contents.clear();
            }
        }
    }

    fn push_to_children(&mut self, entry: QuadTreeEntry) {
        if self.children.is_none() { return }

        let children = Rc::get_mut(self.children.as_mut().unwrap());
        for child in children.unwrap().iter_mut() {
            child.insert(entry.clone());
        }
    }

    fn get_leaf_contents(&self) -> Vec<Vec<QuadTreeEntry>> {
        if self.children.is_some() {
            let mut ret = Vec::new();
            for child in self.children.as_ref().unwrap().iter() {
                ret.append(&mut child.get_leaf_contents());
            }
            return ret
        } else {
            return vec![self.contents.clone()]
        }
    }
}

impl QuadTreeEntry {
    pub fn new(pos: Vec2, size: Vec2, index: usize) -> Self {
        Self { pos, size, ball_index: index }
    }

    pub fn colliding(&self, collider_pos: &Vec2, collider_size: &Vec2) -> bool {
        let translated = *collider_pos - self.pos;

        // don't ask
        translated.both_less_eq(&self.size) && (-translated).both_less_eq(collider_size)
    }
}