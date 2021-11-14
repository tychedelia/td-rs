use crate::ffi::Chop;

#[cxx::bridge]
mod ffi {
    pub struct Chop {
        pub foo: u8
    }

    // impl Chop {
    //     fn execute(&self, data: f64) {}
    // }

    extern "Rust" {
        fn exec(chop: Chop);
    }

}

fn exec(chop: Chop) {
    println!("{}", chop.foo);
}