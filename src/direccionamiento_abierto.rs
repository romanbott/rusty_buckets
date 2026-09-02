#[derive(Debug)]
enum Element<K, V> {
    Occupied(K, V),
    Empty,
    Deleted,
}

#[derive(Debug)]
pub struct OpenAddressingHashMap<K, V> {
    slots: Vec<Element<K, V>>,
    elements: usize,
    capacity: usize,
}
