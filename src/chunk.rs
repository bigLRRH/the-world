// todo 改十个bug
use crate::voxel::{self, Voxel};
use bevy::{prelude::*, utils::HashMap};

const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    position: IVec3,
    voxels: [[[Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    is_dirty: bool,
}

impl Chunk {
    fn new(position: IVec3) -> Self {
        Self {
            position,
            voxels: [[[Voxel::default(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            is_dirty: false,
        }
    }
}

pub struct ChunkManager {
    chunks: HashMap<IVec3, Chunk>,
}

impl ChunkManager {
    fn get_voxel(&self, position: IVec3) -> Option<&Voxel> {
        let chunk_pos = position / CHUNK_SIZE as i32;
        self.chunks.get(&chunk_pos).map(|chunk| {
            let voxel_pos = position - chunk_pos;
            &chunk.voxels[voxel_pos.x as usize][voxel_pos.y as usize][voxel_pos.z as usize]
        })
    }

    fn set_voxel(&mut self, position: IVec3, voxel: Voxel) {
        let chunk_pos = position / CHUNK_SIZE as i32;

        // 获取或创建 Chunk
        let chunk = self
            .chunks
            .entry(chunk_pos)
            .or_insert_with(|| Chunk::new(chunk_pos));

        // 修改体素
        let voxel_pos = position - chunk_pos;
        chunk.voxels[voxel_pos.x as usize][voxel_pos.y as usize][voxel_pos.z as usize] = voxel;
        chunk.is_dirty = true;
    }
}
