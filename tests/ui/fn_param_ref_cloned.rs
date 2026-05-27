#![allow(unused)]
#![warn(clippy::fn_param_ref_cloned_info)]

// Impl methods
#[derive(Clone)]
pub struct A;

pub struct B;

impl A {
    // pub fn no_ref(&self) {
    //     let a = A;
    //     let cloned_no_ref = a.clone();
    // }

    // pub fn cloning_ref(&self, moje: &A) {
    //     // #[clippy::dump]
    //     let cloned_ref_param = moje.clone();
    // }

    // pub fn using_ref(&self, tvoje: &A) {
    //     let x = "";
    //     let b = tvoje;
    // }
}

pub fn cloning_ref(vase: A, nase: &A, ich: &A) {
    let test = vase;
    let cloned_ref_param = nase.clone();
    //~^ fn_param_ref_cloned
}

pub fn regular_ref(vase: B, nase: &B, ich: &B) {
    let test = vase;
    let cloned_ref_param = nase;
}

fn main() {
    let a = A;
    let b = A;
    let c = A;

    let x = B;
    let y = B;
    let z = B;
    // a.cloning_ref(&A);
    cloning_ref(a, &b, &c);
    regular_ref(x, &y, &z);
}
