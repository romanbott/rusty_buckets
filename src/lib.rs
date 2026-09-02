use std::fmt::Display;

#[derive(Debug)]
pub struct BucketHashMap<T> {
    num_buckets: usize,
    buckets: Vec<Vec<(usize, T)>>,
    elements: usize,
}

impl<T> BucketHashMap<T> {
    pub fn new() -> BucketHashMap<T> {
        let buckets = (0..7).map(|_| vec![]).collect();
        BucketHashMap {
            num_buckets: 7,
            buckets,
            elements: 0,
        }
    }

    pub fn factor_carga(&self) -> f32 {
        self.elements as f32 / self.num_buckets as f32
    }

    pub fn esta_vacio(&self) -> bool {
        self.elements == 0
    }

    fn hash(&self, key: usize) -> usize {
        key % self.num_buckets
    }

    pub fn insertar(&mut self, key: usize, value: T) -> Result<(), T> {
        let index = self.hash(key);
        let bucket = self.buckets.get_mut(index).unwrap();

        if bucket.iter().position(|(k, _)| *k == key).is_some() {
            return Err(value);
        }

        self.elements += 1;
        bucket.push((key, value));

        Ok(())
    }

    pub fn upsert(&mut self, key: usize, value: T) -> Option<T> {
        let index = self.hash(key);

        let bucket = self.buckets.get_mut(index).unwrap();

        let n = bucket.iter().enumerate().find(|(_, (i, _))| key == *i);

        match n {
            Some((n, _)) => {
                let (_, v) = bucket.remove(n);

                bucket.push((key, value));

                Some(v)
            }
            None => None,
        }
    }

    pub fn buscar(&self, key: usize) -> Option<&T> {
        let index = self.hash(key);
        self.buckets[index]
            .iter()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    pub fn eliminar(&mut self, key: usize) -> Option<T> {
        let index = self.hash(key);

        let bucket = self.buckets.get_mut(index).unwrap();

        let n = bucket.iter().enumerate().find(|(_, (i, _))| key == *i);

        match n {
            Some((n, _)) => {
                let (_, v) = bucket.remove(n);
                self.elements -= 1;
                Some(v)
            }
            None => None,
        }
    }
}

impl<T: Display> BucketHashMap<T> {
    pub fn imprime(&self) {
        for bucket in &self.buckets {
            for (k, v) in bucket {
                println!("{} => {}", k, v)
            }
        }
    }
}

impl<T> Default for BucketHashMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inserta_simple() {
        let mut hm = BucketHashMap::new();

        let res = hm.insertar(3, "hola");

        dbg!(&hm);
        assert!(res.is_ok());
    }

    #[test]
    fn test_inserta_repetido_falla() {
        let mut hm = BucketHashMap::new();

        let _ = hm.insertar(3, "hola");
        let res = hm.insertar(3, "hola");
        assert!(res.is_err());
    }

    #[test]
    fn test_inserta_busca() {
        let mut hm = BucketHashMap::new();

        let _ = hm.insertar(3, "hola");
        let val = hm.buscar(3);

        assert_eq!(val, Some(&"hola"))
    }

    #[test]
    fn test_inserta_busca_colision() {
        let mut hm = BucketHashMap::new();

        let _ = hm.insertar(3, "hola");
        let _ = hm.insertar(10, "mundo");
        let val = hm.buscar(3);
        let val2 = hm.buscar(10);

        assert_eq!(val, Some(&"hola"));
        assert_eq!(val2, Some(&"mundo"))
    }

    #[test]
    fn test_busca_inexistente() {
        let mut hm = BucketHashMap::new();

        let _ = hm.insertar(3, "hola");
        let val = hm.buscar(4);

        assert_eq!(val, None)
    }

    #[test]
    fn test_upsert() {
        let mut hm = BucketHashMap::new();

        let _ = hm.insertar(3, "hola");
        let res = hm.upsert(3, "mundo");
        assert_eq!(res, Some("hola"));

        let replaced = hm.buscar(3);
        assert_eq!(replaced, Some(&"mundo"))
    }

    #[test]
    fn test_elimina() {
        let mut hm = BucketHashMap::new();

        let _ = hm.insertar(3, "hola");
        let res_elim = hm.eliminar(3);

        let val = hm.buscar(3);

        // La eliminación funcionó correctamente
        assert_eq!(res_elim, Some("hola"));

        // La búsqueda ya no encuentra nada
        assert_eq!(val, None);
    }

    #[test]
    fn test_elimina_con_colision() {
        let mut hm = BucketHashMap::new();

        let _ = hm.insertar(3, "hola");
        let _ = hm.insertar(10, "mundo");
        let res_elim = hm.eliminar(10);

        let val = hm.buscar(3);

        // La eliminación funcionó correctamente
        assert_eq!(res_elim, Some("mundo"));

        // La búsqueda ya no encuentra nada
        assert_eq!(val, Some(&"hola"));
    }
}
