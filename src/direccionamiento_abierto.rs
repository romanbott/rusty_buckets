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
impl<T> OpenAddressingHashMap<usize, T> {
    pub fn new() -> OpenAddressingHashMap<usize, T> {
        let slots = (0..7).map(|_| Element::Empty).collect();
        OpenAddressingHashMap {
            slots,
            elements: 0,
            capacity: 7,
        }
    }

    pub fn with_capacity(capacity: usize) -> OpenAddressingHashMap<usize, T> {
        let slots = (0..capacity).map(|_| Element::Empty).collect();
        OpenAddressingHashMap {
            slots,
            elements: 0,
            capacity,
        }
    }

    pub fn factor_carga(&self) -> f32 {
        self.elements as f32 / self.capacity as f32
    }

    pub fn esta_vacio(&self) -> bool {
        self.elements == 0
    }

    fn hash(&self, key: usize) -> usize {
        key % self.capacity
    }

    pub fn insertar(&mut self, key: usize, value: T) -> Result<(), T> {
        if self.elements == self.capacity {
            return Err(value);
        }

        for offset in 0..self.capacity {
            let index = self.hash(key + offset);
            let slot = self.slots.get_mut(index).unwrap();

            match slot {
                Element::Occupied(k, _) => {
                    if *k == key {
                        return Err(value);
                    }
                }
                Element::Deleted => continue,
                Element::Empty => {
                    *slot = Element::Occupied(key, value);
                    self.elements += 1;
                    return Ok(());
                }
            }
        }

        Err(value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inserta_simple() {
        let mut hm = OpenAddressingHashMap::new();

        let res = hm.insertar(3, "hola");

        dbg!(&hm);
        assert!(res.is_ok());
    }

    #[test]
    fn test_inserta_lleno() {
        let mut hm = OpenAddressingHashMap::new();

        for i in 0..7 {
            let res = hm.insertar(3 + 7 * i, i);
            assert!(res.is_ok());
            dbg!(&hm);
        }

        let res = hm.insertar(4, 0);

        dbg!(&hm);
        assert!(res.is_err());
    }
}
