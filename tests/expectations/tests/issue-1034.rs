/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct S2 {
    pub _bitfield_1: u16,
}
#[test]
fn bindgen_test_layout_S2() {
    assert_eq!(
        ::std::mem::size_of::<S2>(),
        2usize,
        concat!("Size of: ", stringify!(S2))
    );
    assert_eq!(
        ::std::mem::align_of::<S2>(),
        1usize,
        concat!("Alignment of ", stringify!(S2))
    );
}
impl S2 {
    #[inline]
    pub fn new_bitfield_1() -> u16 {
        0
    }
}
