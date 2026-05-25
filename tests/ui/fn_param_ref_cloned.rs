#![allow(unused)]
#![warn(clippy::fn_param_ref_cloned_info)]

// Impl methods
#[derive(Clone)]
pub struct A;

impl A {
    pub fn fo(&self) {}
    pub fn foo(&self, item: &A) {
        let x = item.clone();
    }
    //~^ fn_param_ref_cloned_info
    pub fn food(&self) {}
}

pub fn foo(item: &A) {
    let y = item.clone();
}
//~^ fn_param_ref_cloned_info

fn main() {
    let a = A;
    a.foo(&A);
    foo(&A);
}
