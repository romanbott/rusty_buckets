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
    fn insertar(&mut self, key: usize, value: T) -> Option<()> {
        let index = key % self.num_buckets;

        let bucket = self.buckets.get_mut(index).unwrap();

        if let Some(_) = bucket.iter().find(|(i, _)| index == *i) {
            None
        } else {
            bucket.push((index, value));

            Some(())
        }
    }
}
