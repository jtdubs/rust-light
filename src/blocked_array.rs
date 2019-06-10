use std::default::Default;

pub struct BlockedArray<T> {
    storage : Vec<T>,
    storage_width : u32,
    width : u32,
    height : u32,
}

impl<T : Default> BlockedArray<T> {
    pub fn new(width : u32, height : u32) -> BlockedArray<T> {
        let storage_width = width.next_power_of_two();
        let storage_height = height.next_power_of_two();
        BlockedArray { 
            storage: Vec::from_fn(storage_width * storage_height, |_| { Default::default() }),
            storage_width: storage_width,
            width: width,
            height: height,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x : u32, y : u32) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);
        let bx = x >> 2;
        let by = y >> 2;
        let ox = x & 3;
        let oy = y & 3;
        let idx = (by * self.storage_width * 4) + (bx * 16) + (oy * 4) + ox;
        &self.storage[idx]
    }

    pub fn get_mut(&mut self, x : u32, y : u32) -> &mut T {
        assert!(x < self.width);
        assert!(y < self.height);
        let bx = x >> 2;
        let by = y >> 2;
        let ox = x & 3;
        let oy = y & 3;
        let idx = (by * self.storage_width * 4) + (bx * 16) + (oy * 4) + ox;
        &mut self.storage[idx]
    }        
}

