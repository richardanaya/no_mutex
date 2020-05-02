#![no_std]
#![feature(const_fn)]
use core::cell::UnsafeCell;
use core::cell::RefCell;

pub struct Mutex<T> where T:Sync+Send+Default+Sized{
    instance: UnsafeCell<Option<RefCell<T>>>
}

unsafe impl<T:Sync+Send+Default> Send for Mutex<T> {}

unsafe impl<T:Sync+Send+Default> Sync for Mutex<T> {}

impl<T:Sync+Send+Default+Sized> Mutex<T>{
    pub const fn default() -> Self {
        Mutex {
            instance: UnsafeCell::new(None)
        }
    }
    
    pub fn lock(&self) -> core::cell::RefMut<'_, T> {
        unsafe {
            let instance = self.instance.get();
            if (*instance).is_none() {
                *instance = Some(RefCell::new(T::default()));
            }
            (*instance).as_ref().unwrap().borrow_mut()
        }
    }
}