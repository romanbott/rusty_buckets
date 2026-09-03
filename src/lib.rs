use std::fmt::Display;

pub mod direccionamiento_abierto;
pub mod encadenamiento;

pub use direccionamiento_abierto::OpenAddressingHashMap;
pub use encadenamiento::ChainingHashMap;

pub trait HashMapOps<K, V> {
    fn new() -> Self
    where
        Self: Sized;

    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized;

    fn factor_carga(&self) -> f32;

    fn esta_vacio(&self) -> bool;

    fn insertar(&mut self, key: K, value: V) -> Result<(), V>;

    fn upsert(&mut self, key: K, value: V) -> Option<V>;

    fn buscar(&self, key: &K) -> Option<&V>;

    fn eliminar(&mut self, key: &K) -> Option<V>;

    fn imprime(&self)
    where
        K: Display,
        V: Display;
}
