/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]

#![cfg(target_os="macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
pub trait Foo {
    unsafe fn func(self)
    -> ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;
    unsafe fn setFunc_(self,
                       func:
                           ::std::option::Option<unsafe extern "C" fn()
                                                     ->
                                                         ::std::os::raw::c_int>);
}
impl Foo for id {
    unsafe fn func(self)
     ->
         ::std::option::Option<unsafe extern "C" fn()
                                   -> ::std::os::raw::c_int> {
        msg_send!(self , func)
    }
    unsafe fn setFunc_(self,
                       func:
                           ::std::option::Option<unsafe extern "C" fn()
                                                     ->
                                                         ::std::os::raw::c_int>) {
        msg_send!(self , setFunc:func )
    }
}
