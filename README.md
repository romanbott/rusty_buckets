# rusty_buckets

Implementaciones de tablas de dispersión.

## Contenido

- **ChainingHashMap** (`encadenamiento`): tabla de dispersión con encadenamiento. Cada _bucket_ es un `Vec<(usize, V)>`.
- **OpenAddressingHashMap** (`direccionamiento_abierto`): tabla de dispersión con direccionamiento abierto (sondeo lineal).
- **HashMapOps**: _Trait_ que define la interfaz común para ambas implementaciones.
- **REPL interactivo**: Binario `repl` para probar ambas tablas desde la línea de comandos.

## Ejecutar las pruebas

```bash
cargo test
```

## Ejecutar el REPL

```bash
cargo run --bin repl
```

### Comandos disponibles

| Comando | Descripción |
|---------|-------------|
| `insertar <llave> <valor>` | Inserta un par (falla si la llave existe) |
| `upsert <llave> <valor>` | Inserta o actualiza (reemplaza si existe) |
| `buscar <llave>` | Busca un valor por su llave |
| `eliminar <llave>` | Elimina un par por su llave |
| `lista` | Muestra todos los pares llave-valor |
| `carga` | Muestra el factor de carga actual |
| `exit` | Sale del REPL |
