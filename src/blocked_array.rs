use std::num::next_power_of_two;
use std::default::Default;

pub struct BlockedArray<T> {
    storage : Vec<T>,
    storage_width : uint,
    width : uint,
    height : uint,
}

impl<T : Default> BlockedArray<T> {
    pub fn new(width : uint, height : uint) -> BlockedArray<T> {
        let storage_width = next_power_of_two(width);
        let storage_height = next_power_of_two(height);
        BlockedArray { 
            storage: Vec::from_fn(storage_width * storage_height, |_| { Default::default() }),
            storage_width: storage_width,
            width: width,
            height: height,
        }
    }

    pub fn get_width(&self) -> uint {
        self.width
    }

    pub fn get_height(&self) -> uint {
        self.height
    }

    pub fn get(&self, x : uint, y : uint) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);
        let bx = x >> 2;
        let by = y >> 2;
        let ox = x & 3;
        let oy = y & 3;
        let idx = (by * self.storage_width * 4) + (bx * 16) + (oy * 4) + ox;
        &self.storage[idx]
    }

    pub fn get_mut(&mut self, x : uint, y : uint) -> &mut T {
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

