#![allow(unused)]
#![warn(clippy::fn_param_ref_cloned_info)]

// Impl methods
#[derive(Clone)]
pub struct A;

impl A {
    pub fn no_ref(&self) {
        let a = A;
        let cloned_no_ref = a.clone();
    }

    pub fn cloning_ref(&self, item: &A) {
        // #[clippy::dump]
        let cloned_ref_param = item.clone();
    }
    //~^ fn_param_ref_cloned_info

    pub fn using_ref(&self, item: &A) {
        let x = "";
        let b = item;
    }
}

pub fn cloning_ref(test: A, item: &A) {
    let cloned_ref_param = item.clone();
}
//~^ fn_param_ref_cloned_info

fn main() {
    let a = A;
    let b = A;
    a.cloning_ref(&A);
    cloning_ref(b, &A);
}
