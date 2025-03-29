#[derive(Copy,Clone)]
pub struct Voxel {
    id: u8,
}

impl Voxel {
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    pub fn default() -> Self {
        Self { id: 0 }
    }

    pub fn id(&self) -> u8 {
        self.id
    }
}
