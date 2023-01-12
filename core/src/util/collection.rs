#[inline]
pub fn remove_element_in_vec<P: PartialEq>(mut vec: Vec<P>, element: P) -> P {
    match vec.iter().position(|e| *e == element).unwrap() {
        pos => vec.remove(pos)
    }
}   