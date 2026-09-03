use std::{fmt::Display, mem};

#[derive(Debug)]
enum Element<K, V> {
    Occupied(K, V),
    Empty,
    Deleted,
}

impl<K, V> Element<K, V> {
    fn take(&mut self) -> Option<V> {
        match self {
            Element::Occupied(_, _) => {
                let taken = mem::replace(self, Element::Deleted);

                match taken {
                    Element::Occupied(_, v) => Some(v),
                    _ => unreachable!(),
                }
            }
            Element::Empty => None,
            Element::Deleted => None,
        }
    }

    fn replace_value(&mut self, new_value: V) -> Option<V> {
        match self {
            Element::Occupied(_, v) => Some(mem::replace(v, new_value)),
            _ => None,
        }
    }
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

        if self.buscar(&key).is_some() {
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
                _ => {
                    *slot = Element::Occupied(key, value);
                    self.elements += 1;
                    return Ok(());
                }
            }
        }

        Err(value)
    }

    pub fn buscar(&self, key: &usize) -> Option<&T> {
        for offset in 0..self.capacity {
            let index = self.hash(key + offset);
            let slot = self.slots.get(index).unwrap();

            match slot {
                Element::Occupied(k, value) => {
                    if k == key {
                        return Some(value);
                    }
                }
                Element::Deleted => continue,
                Element::Empty => return None,
            }
        }

        None
    }

    pub fn eliminar(&mut self, key: &usize) -> Option<T> {
        for offset in 0..self.capacity {
            let index = self.hash(key + offset);
            let slot = self.slots.get_mut(index).unwrap();

            match slot {
                Element::Occupied(k, _) => {
                    if k == key {
                        self.elements -= 1;
                        return slot.take();
                    }
                }
                Element::Deleted => continue,
                Element::Empty => return None,
            }
        }

        None
    }

    pub fn upsert(&mut self, key: usize, value: T) -> Option<T> {
        for offset in 0..self.capacity {
            let index = self.hash(key + offset);
            let slot = self.slots.get_mut(index).unwrap();

            match slot {
                Element::Occupied(k, _) if *k == key => {
                    return slot.replace_value(value);
                }
                Element::Deleted | Element::Empty => {
                    *slot = Element::Occupied(key, value);
                    self.elements += 1;
                    return None;
                }
                _ => {}
            }
        }

        None
    }
}

impl<T: Display, K: Display> OpenAddressingHashMap<K, T> {
    pub fn imprime(&self) {
        for slot in &self.slots {
            if let Element::Occupied(k, v) = slot {
                println!("{} => {}", k, v)
            }
        }
    }
}

impl<T> Default for OpenAddressingHashMap<usize, T> {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_inserta_busca() {
        let mut hm = OpenAddressingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let _ = hm.insertar(10, "mundo");

        let val = hm.buscar(&3);
        assert_eq!(val, Some(&"hola"));

        let val = hm.buscar(&10);
        assert_eq!(val, Some(&"mundo"))
    }

    #[test]
    fn take_element() {
        let mut occupied = Element::Occupied(3, "hola");

        let mut empty: Element<(), ()> = Element::Empty;

        let val1 = occupied.take();
        let val2 = empty.take();

        assert_eq!(val1, Some("hola"));
        assert!(matches!(occupied, Element::Deleted));
        assert_eq!(val2, None);
    }

    #[test]
    fn inserta_elimina_busca() {
        let mut hm = OpenAddressingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let _ = hm.insertar(10, "mundo");

        let val = hm.buscar(&3);
        assert_eq!(val, Some(&"hola"));

        let eliminado = hm.eliminar(&3);

        assert_eq!(eliminado, Some("hola"));

        let val = hm.buscar(&10);
        assert_eq!(val, Some(&"mundo"));
    }

    #[test]
    fn insertar_reusa_deleted_slots() {
        let mut hm = OpenAddressingHashMap::new();

        let _ = hm.insertar(3, "Temporary");
        hm.eliminar(&3);

        let _ = hm.insertar(10, "Persistent");

        match &hm.slots[3] {
            Element::Occupied(k, _) => assert_eq!(*k, 10, "No se reusó el slot"),
            _ => panic!("El slot 3 se debió reusar para la llave 10"),
        }
    }

    #[test]
    fn eliminar_disminuye_conteo() {
        let mut hm = OpenAddressingHashMap::new();

        for i in 0..7 {
            assert!(hm.insertar(i, i * 10).is_ok());
        }

        assert!(hm.insertar(100, 0).is_err());

        hm.eliminar(&2);
        hm.eliminar(&4);

        let res = hm.insertar(100, 999);
        assert!(res.is_ok(), "No se pudo insertar");
    }

    #[test]
    fn test_cadena_compleja_con_tombstones() {
        let mut hm = OpenAddressingHashMap::new();

        assert!(hm.insertar(0, "A").is_ok());
        assert!(hm.insertar(7, "B").is_ok());
        assert!(hm.insertar(14, "C").is_ok());
        assert!(hm.insertar(21, "D").is_ok());
        assert!(hm.insertar(28, "E").is_ok());

        // El estado interno debe ser:
        // Slot 0: Occupied(0)
        // Slot 1: Occupied(7)
        // Slot 2: Occupied(14)
        // Slot 3: Occupied(21)
        // Slot 4: Occupied(28)
        // Slots 5, 6: Empty

        assert_eq!(hm.eliminar(&7), Some("B"));
        assert_eq!(hm.eliminar(&21), Some("D"));

        // Slot 0: Occupied(0)
        // Slot 1: Deleted
        // Slot 2: Occupied(14)
        // Slot 3: Deleted
        // Slot 4: Occupied(28)

        assert_eq!(
            hm.buscar(&28),
            Some(&"E"),
            "Fallo al buscar 28: La busqueda se rompio al pasar por slots Eliminados/Ocupados"
        );
        assert_eq!(
            hm.buscar(&14),
            Some(&"C"),
            "Fallo al buscar 14 despues de un slot Eliminado"
        );

        assert_eq!(
            hm.buscar(&21),
            None,
            "Encontro una llave que debia estar eliminada!"
        );

        assert!(hm.insertar(14, "F").is_err());

        assert!(hm.insertar(35, "F").is_ok());

        // Slot 0: Occupied(0)
        // Slot 1: Occupied(35)
        // Slot 2: Occupied(14)
        // Slot 3: Deleted
        // Slot 4: Occupied(28)

        assert_eq!(
            hm.buscar(&35),
            Some(&"F"),
            "Fallo al encontrar la llave 35 recien insertada"
        );
        assert_eq!(
            hm.buscar(&28),
            Some(&"E"),
            "La cadena se rompio para 28 despues de reusar un slot Eliminado"
        );

        assert_eq!(hm.eliminar(&0), Some("A"));

        // Slot 0: Deleted
        // Slot 1: Occupied(35)
        // Slot 2: Occupied(14)
        // Slot 3: Deleted
        // Slot 4: Occupied(28)

        assert_eq!(
            hm.buscar(&28),
            Some(&"E"),
            "Fallo al encontrar 28 cuando la cadena de prueba comienza con un slot Eliminado"
        );
    }

    #[test]
    fn test_upsert() {
        let mut hm = OpenAddressingHashMap::new();

        let _ = hm.insertar(3, "hola");
        let res = hm.upsert(3, "mundo");
        assert_eq!(res, Some("hola"));

        let replaced = hm.buscar(&3);
        assert_eq!(replaced, Some(&"mundo"));

        let res = hm.upsert(10, "nuevo");
        assert_eq!(res, None);
        assert_eq!(hm.buscar(&10), Some(&"nuevo"));
    }
}
