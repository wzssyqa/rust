// compile-flags: -Z borrowck=mir -Z two-phase-borrows

// This is similar to two-phase-reservation-sharing-interference.rs
// in that it shows a reservation that overlaps with a shared borrow.

fn main() {
    let mut v = vec![0, 1, 2];
    let shared = &v;

    v.push(shared.len()); //~ ERROR cannot borrow `v` as mutable

    assert_eq!(v, [0, 1, 2, 3]);
}
