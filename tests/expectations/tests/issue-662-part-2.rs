/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]

#[derive(Clone, Copy, Debug)] pub struct RefPtr<T>(T);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsMainThreadPtrHolder<T> {
    pub a: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for nsMainThreadPtrHolder<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsMainThreadPtrHandle<U> {
    pub mPtr: RefPtr<nsMainThreadPtrHolder<U>>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
impl <U> Default for nsMainThreadPtrHandle<U> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
