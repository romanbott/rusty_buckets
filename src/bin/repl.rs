use rusty_buckets::{ChainingHashMap, HashMapOps, OpenAddressingHashMap};
use std::io::{self, Write};

fn main() {
    println!("--- RustyBuckets REPL ---");
    println!("Seleccione implementación:");
    println!("  1) Encadenamiento");
    println!("  2) Direccionamiento abierto");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    let mut map: Box<dyn HashMapOps<usize, String>> = match choice {
        "1" => {
            println!("Usando ChainingHashMap");
            Box::new(ChainingHashMap::new())
        }
        "2" => {
            println!("Usando OpenAddressingHashMap");
            Box::new(OpenAddressingHashMap::new())
        }
        _ => {
            println!("Opción inválida, usando ChainingHashMap por defecto");
            Box::new(ChainingHashMap::new())
        }
    };

    println!(
        "\nComandos: insertar <key> <val> | upsert <key> <val> | buscar <key> | eliminar <key> | lista | carga | exit\n"
    );

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0].to_lowercase();
        let args = &parts[1..];

        match cmd.as_str() {
            "insertar" => cmd_insertar(&mut *map, args),
            "upsert" => cmd_upsert(&mut *map, args),
            "buscar" => cmd_buscar(&*map, args),
            "eliminar" => cmd_eliminar(&mut *map, args),
            "lista" => cmd_lista(&*map),
            "carga" => cmd_carga(&*map),
            "exit" => {
                println!("Saliendo.");
                break;
            }
            _ => println!(
                "Comando no reconocido. Sintaxis: insertar <llave> <val> | upsert <llave> <val> | buscar <llave> | eliminar <llave> | lista | carga | exit"
            ),
        }
    }
}

fn parse_key(s: &str) -> Option<usize> {
    s.parse().ok()
}

fn cmd_insertar(map: &mut dyn HashMapOps<usize, String>, args: &[&str]) {
    let [key_str, val @ ..] = args else {
        println!("Uso: insertar <llave> <valor>");
        return;
    };
    if val.is_empty() {
        println!("Error: valor requerido");
        return;
    }
    let Some(key) = parse_key(key_str) else {
        println!("Error: llave inválida");
        return;
    };
    let value = val.join(" ");
    match map.insertar(key, value) {
        Ok(_) => println!("Llave insertada {}", key),
        Err(_) => println!("Colisión! Llave {} no insertada", key),
    }
}

fn cmd_buscar(map: &dyn HashMapOps<usize, String>, args: &[&str]) {
    let [key_str] = args else {
        println!("Uso: buscar <llave>");
        return;
    };
    let Some(key) = parse_key(key_str) else {
        println!("Error: llave inválida");
        return;
    };
    match map.buscar(&key) {
        Some(val) => println!("Valor: {}", val),
        None => println!("Llave {} no encontrada.", key),
    }
}

fn cmd_eliminar(map: &mut dyn HashMapOps<usize, String>, args: &[&str]) {
    let [key_str] = args else {
        println!("Uso: eliminar <llave>");
        return;
    };
    let Some(key) = parse_key(key_str) else {
        println!("Error: llave inválida");
        return;
    };
    match map.eliminar(&key) {
        Some(_) => println!("Llave eliminada {}", key),
        None => println!("Llave {} no encontrada.", key),
    }
}

fn cmd_lista(map: &dyn HashMapOps<usize, String>) {
    if map.esta_vacio() {
        println!("La tabla está vacía");
    } else {
        map.imprime();
    }
}

fn cmd_carga(map: &dyn HashMapOps<usize, String>) {
    println!("El factor de carga es: {}", map.factor_carga());
}

fn cmd_upsert(map: &mut dyn HashMapOps<usize, String>, args: &[&str]) {
    let [key_str, val @ ..] = args else {
        println!("Uso: upsert <llave> <valor>");
        return;
    };
    if val.is_empty() {
        println!("Error: valor requerido");
        return;
    }
    let Some(key) = parse_key(key_str) else {
        println!("Error: llave inválida");
        return;
    };
    let value = val.join(" ");
    match map.upsert(key, value) {
        Some(old) => println!("Llave actualizada {} (valor anterior: {})", key, old),
        None => println!("Llave insertada {}", key),
    }
}
