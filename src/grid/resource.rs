use bevy::{math::USizeVec2, prelude::*};

#[derive(Resource)]
pub struct CollisionGrid {
    pub cell_size: f32,
    pub size: usize,
    pub cells: Vec<Vec<Entity>>,
}

impl CollisionGrid {
    pub fn new(world_size: f32, cell_size: f32) -> Self {
        let size = (world_size / cell_size).ceil() as usize;
        Self {
            cell_size,
            size: size,
            cells: vec![Vec::new(); size * size],
        }
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.clear();
        }
    }

    pub fn get_cell_index_from_grid_coord(&self, coord: USizeVec2) -> Option<usize> {
        if coord.x < self.size && coord.y < self.size {
            Some((coord.y * self.size) + coord.x)
        } else {
            None
        }
    }

    pub fn get_cell_index_from_particle_pos(&self, pos: Vec2) -> Option<usize> {
        let x = ((pos.x + (self.size as f32 * self.cell_size / 2.0)) / self.cell_size) as i32;
        let y = ((pos.y + (self.size as f32 * self.cell_size / 2.0)) / self.cell_size) as i32;

        if x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32 {
            Some((y as usize * self.size) + x as usize)
        } else {
            None
        }
    }
}
