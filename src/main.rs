use std::ptr::NonNull;
use std::alloc;

#[derive(Debug)]
struct JVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl <T> Drop for JVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            let layout = alloc::Layout::from_size_align(size, align).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
        }
    }
}

impl <T> JVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.as_ptr().add(index)) }
        } else {
            None
        }
    }

    pub fn push(&mut self, value: T) {
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("Failed to allocate memory");
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            unsafe { ptr.write(value) }
            self.ptr = NonNull::new(ptr).expect("Failed to allocate memory");
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            unsafe { self.ptr.as_ptr().add(1).write(value) }
            self.len += 1;
        } else {
            let new_capacity = self.capacity * 2;
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_mul(2).expect("capacity overflow");

            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr =  alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocated memory");
                ptr.as_ptr().add(self.len).write(value);
                self.ptr = ptr;
            }
            self.len += 1;
            self.capacity = new_capacity;
        }
    }
}

fn main() {
    let mut vec = JVec::<u8>::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    println!("Vec {:?}", vec);
    println!("get by index {:?}", vec.get(4));

}
