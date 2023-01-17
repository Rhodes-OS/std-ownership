#[inline]
pub fn remove_element_in_vec<E: PartialEq>(mut vec: Vec<E>, element: E) -> E {
    match vec.iter().position(|e| *e == element).unwrap() {
        pos => vec.remove(pos)
    }
}   