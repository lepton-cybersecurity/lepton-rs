use std::mem;
use libc::{mlock, munlock, c_void};

pub struct SecureData {
    data: Vec<u8>,
}

impl Drop for SecureData {
    fn drop(&mut self) {
        let ptr = self.data.as_ptr() as *const c_void;
        let size = self.data.len() * mem::size_of::<u8>();
        unsafe {
            munlock(ptr, size);
        }

        self.data.fill(0);
    }
}

impl SecureData {
    pub fn from(data: &mut Vec<u8>) -> SecureData {
        let result = SecureData {
            data: data.clone()
        };

        let ptr = result.data.as_ptr() as *const c_void;
        let size = result.data.len() * mem::size_of::<u8>();
        unsafe {
            mlock(ptr, size);
        }

        data.fill(0);
        result
    }

    pub fn read(self) -> Vec<u8> {
        return self.data.clone();
    }
}
