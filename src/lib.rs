use std::ops::Index;

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

    fn hash(&self, key: usize) -> usize {
        key % self.num_buckets
    }

    fn insertar(&mut self, key: usize, value: T) -> Option<()> {
        let index = self.hash(key);

        let bucket = self.buckets.get_mut(index).unwrap();

        if let Some(_) = bucket.iter().find(|(i, _)| key == *i) {
            None
        } else {
            bucket.push((key, value));

            Some(())
        }
    }

    fn buscar(&self, key: usize) -> Option<&T> {
        let index = self.hash(key);
        self.buckets[index]
            .iter()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    fn eliminar(&mut self, key: usize) -> Option<T> {
        let index = self.hash(key);

        let mut bucket = self.buckets.get_mut(index).unwrap();

        let n = bucket.iter().enumerate().find(|(_, (i, _))| key == *i);

        match n {
            Some((n, _)) => {
                let (_, v) = bucket.remove(n);
                Some(v)
            }
            None => None,
        }
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

    #[test]
    fn test_inserta_busca() {
        let mut hm = SimpleHashMap::new();

        let _ = hm.insertar(3, "hola");
        let val = hm.buscar(3);

        assert_eq!(val, Some(&"hola"))
    }

    #[test]
    fn test_inserta_busca_colision() {
        let mut hm = SimpleHashMap::new();

        let _ = hm.insertar(3, "hola");
        let _ = hm.insertar(10, "mundo");
        let val = hm.buscar(3);
        let val2 = hm.buscar(10);

        assert_eq!(val, Some(&"hola"));
        assert_eq!(val2, Some(&"mundo"))
    }

    #[test]
    fn test_busca_inexistente() {
        let mut hm = SimpleHashMap::new();

        let _ = hm.insertar(3, "hola");
        let val = hm.buscar(4);

        assert_eq!(val, None)
    }

    #[test]
    fn test_elimina() {
        let mut hm = SimpleHashMap::new();

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
        let mut hm = SimpleHashMap::new();

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
