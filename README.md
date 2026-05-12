# MiniKV

MiniKV es una base de datos **key-value** desarrollada en **Rust** como trabajo práctico individual de la materia **Taller de Programación**.

El proyecto implementa almacenamiento de pares clave-valor, persistencia en archivos y una versión cliente-servidor utilizando sockets TCP.

---

## Características principales

- Almacenamiento de datos en formato clave-valor.
- Soporte para operaciones `set`, `get`, `length` y `snapshot`.
- Eliminación de claves mediante `set clave`.
- Persistencia mediante archivos de log.
- Compactación del estado actual mediante snapshots.
- Versión cliente-servidor usando sockets TCP.
- Manejo de múltiples clientes mediante concurrencia.
- Separación entre lógica local y lógica de servidor.

---

## Tecnologías utilizadas

- Rust
- Cargo
- TCP sockets
- Threads
- Channels
- File System
- Makefile

---

## Estructura del proyecto

```text
Minikv/
├── minikv/          # Versión local del key-value store
├── minikvserver/    # Versión cliente-servidor
└── README.md
```

---

## Funcionamiento general

MiniKV guarda información como pares clave-valor.

Por ejemplo:

```text
nombre -> Alexis
lenguaje -> Rust
materia -> Taller de Programación
```

Las operaciones realizadas se almacenan en un archivo de log.  
Cuando se ejecuta un snapshot, el sistema compacta el estado actual y genera una representación más limpia de la base de datos.

---

## Comandos soportados

### `set clave valor`

Guarda una clave con un valor asociado.

```bash
set nombre Alexis
```

---

### `get clave`

Obtiene el valor asociado a una clave.

```bash
get nombre
```

---

### `set clave`

Elimina una clave existente.

```bash
set nombre
```

---

### `length`

Devuelve la cantidad de claves almacenadas actualmente.

```bash
length
```

---

### `snapshot`

Compacta el estado actual de la base de datos.

```bash
snapshot
```

---

## Ejecución de la versión local

Para ejecutar la versión local del proyecto:

```bash
cd minikv
cargo run -- set nombre Alexis
cargo run -- get nombre
cargo run -- length
cargo run -- snapshot
```

Ejemplo de uso:

```bash
cargo run -- set lenguaje Rust
cargo run -- get lenguaje
```

---

## Ejecución de la versión cliente-servidor

Para iniciar el servidor:

```bash
cd minikvserver
cargo run -- 127.0.0.1:8080
```

Luego, un cliente puede conectarse al servidor y enviar comandos como:

```text
set nombre Alexis
get nombre
length
snapshot
```

---

## Persistencia

MiniKV utiliza archivos para mantener el estado de la base de datos entre ejecuciones.

Archivos principales:

- `.minikv.log`: almacena las operaciones realizadas.
- `.minikv.data`: almacena el estado compactado luego de ejecutar `snapshot`.

La idea general es:

1. Las operaciones se escriben en el log.
2. Al iniciar el programa, se reconstruye el estado leyendo los archivos persistidos.
3. Al ejecutar `snapshot`, se compacta el estado actual.

---

## Objetivos del proyecto

Este proyecto fue desarrollado con el objetivo de practicar conceptos como:

- Programación en Rust.
- Manejo de errores.
- Entrada y salida de archivos.
- Persistencia de datos.
- Diseño de una base key-value simple.
- Comunicación cliente-servidor.
- Uso de sockets TCP.
- Concurrencia con threads y channels.
- Organización de código en módulos.

---

## Decisiones de diseño

El proyecto separa la lógica de almacenamiento de la lógica de comunicación por red.

En la versión cliente-servidor, el servidor recibe comandos de los clientes, procesa las operaciones correspondientes y devuelve una respuesta. Esto permite practicar una arquitectura similar a la de servicios backend simples.

La persistencia basada en log permite registrar las operaciones de forma incremental, mientras que el snapshot permite compactar el estado para evitar que el log crezca indefinidamente.

---

## Estado del proyecto

Proyecto académico desarrollado como trabajo práctico individual.

No está pensado para ser utilizado en producción, sino como una implementación educativa de una base de datos key-value simple con persistencia y comunicación por red.

---

## Autor

**Alexis Maximiliano Torrez Vargas**

Estudiante de Ingeniería Informática  
Universidad de Buenos Aires