use std::fmt::Display;

use crate::HashMapOps;

#[derive(Debug, Default)]
pub struct ChainingHashMap<T> {
    pub(crate) num_buckets: usize,
    pub(crate) buckets: Vec<Vec<(usize, T)>>,
    pub(crate) elements: usize,
}

impl<T> ChainingHashMap<T> {
    pub(crate) fn hash(&self, key: usize) -> usize {
        key % self.num_buckets
    }
}

impl<T> HashMapOps<usize, T> for ChainingHashMap<T> {
    fn new() -> Self {
        let buckets = (0..7).map(|_| vec![]).collect();
        ChainingHashMap {
            num_buckets: 7,
            buckets,
            elements: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        let buckets = (0..capacity).map(|_| vec![]).collect();
        ChainingHashMap {
            num_buckets: capacity,
            buckets,
            elements: 0,
        }
    }

    fn factor_carga(&self) -> f32 {
        self.elements as f32 / self.num_buckets as f32
    }

    fn esta_vacio(&self) -> bool {
        self.elements == 0
    }

    fn insertar(&mut self, key: usize, value: T) -> Result<(), T> {
        let index = self.hash(key);
        let bucket = self.buckets.get_mut(index).unwrap();

        if bucket.iter().position(|(k, _)| *k == key).is_some() {
            return Err(value);
        }

        self.elements += 1;
        bucket.push((key, value));

        Ok(())
    }

    fn upsert(&mut self, key: usize, value: T) -> Option<T> {
        let index = self.hash(key);

        let bucket = self.buckets.get_mut(index).unwrap();

        let n = bucket.iter().enumerate().find(|(_, (i, _))| key == *i);

        match n {
            Some((n, _)) => {
                let (_, v) = bucket.remove(n);

                bucket.push((key, value));

                Some(v)
            }
            None => {
                self.elements += 1;
                bucket.push((key, value));
                None
            }
        }
    }

    fn buscar(&self, key: &usize) -> Option<&T> {
        let index = self.hash(*key);
        self.buckets[index]
            .iter()
            .find_map(|(k, v)| if *k == *key { Some(v) } else { None })
    }

    fn eliminar(&mut self, key: &usize) -> Option<T> {
        let index = self.hash(*key);

        let bucket = self.buckets.get_mut(index).unwrap();

        let n = bucket.iter().enumerate().find(|(_, (i, _))| *i == *key);

        match n {
            Some((n, _)) => {
                let (_, v) = bucket.remove(n);
                self.elements -= 1;
                Some(v)
            }
            None => None,
        }
    }

    fn imprime(&self)
    where
        usize: Display,
        T: Display,
    {
        for bucket in &self.buckets {
            for (k, v) in bucket {
                println!("{} => {}", k, v)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inserta_simple() {
        let mut hm = ChainingHashMap::new();

        let res = hm.insertar(3, "hola");

        dbg!(&hm);
        assert!(res.is_ok());
    }

    #[test]
    fn test_inserta_repetido_falla() {
        let mut hm = ChainingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let res = hm.insertar(3, "hola");
        assert!(res.is_err());
    }

    #[test]
    fn test_inserta_busca() {
        let mut hm = ChainingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let val = hm.buscar(&3);

        assert_eq!(val, Some(&"hola"))
    }

    #[test]
    fn test_inserta_busca_colision() {
        let mut hm = ChainingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let _ = hm.insertar(10, "mundo");
        let val = hm.buscar(&3);
        let val2 = hm.buscar(&10);

        assert_eq!(val, Some(&"hola"));
        assert_eq!(val2, Some(&"mundo"))
    }

    #[test]
    fn test_busca_inexistente() {
        let mut hm = ChainingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let val = hm.buscar(&4);

        assert_eq!(val, None)
    }

    #[test]
    fn test_upsert() {
        let mut hm = ChainingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let res = hm.upsert(3, "mundo");
        assert_eq!(res, Some("hola"));

        let replaced = hm.buscar(&3);
        assert_eq!(replaced, Some(&"mundo"));

        // upsert on non-existing key
        let res = hm.upsert(10, "nuevo");
        assert_eq!(res, None);
        assert_eq!(hm.buscar(&10), Some(&"nuevo"));
    }

    #[test]
    fn test_elimina() {
        let mut hm = ChainingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let res_elim = hm.eliminar(&3);

        let val = hm.buscar(&3);

        assert_eq!(res_elim, Some("hola"));
        assert_eq!(val, None);
    }

    #[test]
    fn test_elimina_con_colision() {
        let mut hm = ChainingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let _ = hm.insertar(10, "mundo");
        let res_elim = hm.eliminar(&10);

        let val = hm.buscar(&3);

        assert_eq!(res_elim, Some("mundo"));
        assert_eq!(val, Some(&"hola"));
    }

    #[test]
    fn test_with_capacity() {
        let hm = ChainingHashMap::<String>::with_capacity(100);
        assert_eq!(hm.num_buckets, 100);
        assert_eq!(hm.factor_carga(), 0.0);
    }
}

