#[derive(Debug)]
struct SimpleHashMap<T> {
    num_buckets: usize,
    buckets: Vec<Vec<(usize, T)>>,
    elements: usize,
}
impl<T> SimpleHashMap<T> {
    fn new() -> SimpleHashMap<T> {
        let buckets = (0..7).map(|_| vec![]).collect();
        SimpleHashMap {
            num_buckets: 7,
            buckets,
            elements: 0,
        }
    }
}
