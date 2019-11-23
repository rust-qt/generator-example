// This file will be included in the generated crate.

use clipper::{IntPoint, VectorOfIntPoint};

#[test]
fn area() {
    unsafe {
        let mut vec = VectorOfIntPoint::new();
        vec.push_back(&IntPoint::new_2a(0, 0));
        vec.push_back(&IntPoint::new_2a(10, 0));
        vec.push_back(&IntPoint::new_2a(10, 10));
        assert!((clipper::area(&vec) - 50.0).abs() < 0.01);
    }
}
