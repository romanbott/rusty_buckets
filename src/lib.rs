//! rusty_buckets - Implementaciones de tablas de dispersión
//!
//! Esta _crate_ proporciona dos implementaciones clásicas de tablas de dispersión:
//! - *Encadenamiento* (`ChainingHashMap`): Cada _bucket_ es un `Vec` de pares llave-valor.
//! - *Direccionamiento abierto* (`OpenAddressingHashMap`): Sondeo lineal.
//!
//! Ambos implementan el _trait_ [`HashMapOps`] para una interfaz unificada.

use std::fmt::Display;

pub mod direccionamiento_abierto;
pub mod encadenamiento;

pub use direccionamiento_abierto::OpenAddressingHashMap;
pub use encadenamiento::ChainingHashMap;

/// Operaciones comunes para implementaciones de tablas de dispersión.
///
/// Este _trait_ define la interfaz estándar para las tablas de dispersión en este crate.
/// Tanto `ChainingHashMap` como `OpenAddressingHashMap` lo implementan.
///
/// # Variables de tipo:
/// - `K`: Tipo de llave (actualmente solo se soporta `usize`)
/// - `V`: Tipo de valor
pub trait HashMapOps<K, V> {
    /// Crea una tabla de dispersión vacía con capacidad por defecto (7).
    fn new() -> Self
    where
        Self: Sized;

    /// Crea una nueva tabla de dispersión con la capacidad especificada.
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized;

    /// Retorna el factor de carga (elementos / capacidad).
    fn factor_carga(&self) -> f32;

    /// Retorna `true` si la tabla no contiene elementos.
    fn esta_vacio(&self) -> bool;

    /// Inserta un par llave-valor.
    ///
    /// # Errores
    /// Retorna `Err(valor)` si no se pudo insertar.
    fn insertar(&mut self, key: K, value: V) -> Result<(), V>;

    /// Inserta o actualiza un par llave-valor.
    ///
    /// Retorna el valor anterior si la llave existía, `None` si fue una nueva inserción.
    fn upsert(&mut self, key: K, value: V) -> Option<V>;

    /// Busca un valor por su llave.
    fn buscar(&self, key: &K) -> Option<&V>;

    /// Elimina un par llave-valor por su llave.
    ///
    /// Retorna el valor eliminado, o `None` si la llave no se encontró.
    fn eliminar(&mut self, key: &K) -> Option<V>;

    /// Imprime todos los pares llave-valor.
    fn imprime(&self)
    where
        K: Display,
        V: Display;
}
