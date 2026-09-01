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

    fn buscar(&self, key: usize) -> Option<&T> {
        self.buckets[key]
            .iter()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inserta_simple() {
        let mut hm = SimpleHashMap::new();

        let succes = hm.insertar(3, "hola");

        dbg!(hm);
        assert!(succes.is_some());
    }

    #[test]
    fn test_inserta_repetido_falla() {
        let mut hm = SimpleHashMap::new();

        let _ = hm.insertar(3, "hola");
        let succes = hm.insertar(3, "hola");
        assert!(succes.is_none());
    }
}
