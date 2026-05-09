# Bóveda — Password Manager 🔒

Bóveda es un gestor de contraseñas de código abierto, moderno y ultra-seguro construido con **Tauri 2**, **Svelte 5** y **Rust**. Diseñado bajo el principio de "Privacidad por Diseño", asegura que tus credenciales nunca salgan de tu dispositivo y estén protegidas contra los vectores de ataque más avanzados.

![License](https://img.shields.io/badge/license-Apache--2.0-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.0-orange)
![Svelte](https://img.shields.io/badge/Svelte-5.0-red)
![Rust](https://img.shields.io/badge/Rust-1.77+-black)

## 🛡️ Seguridad de Grado Militar (Bóveda Core)

El corazón de Bóveda es un motor independiente (`boveda-core`) diseñado para ofrecer resistencia forense y criptográfica máxima:

### 🔐 Criptografía y Almacenamiento
- **Cifrado Total en Reposo:** La base de datos SQLite está cifrada íntegramente mediante **SQLCipher (AES-256 bit en modo CBC)**, protegiendo no solo tus contraseñas sino también los metadatos y la estructura de la base de datos.
- **Protocolos de Vanguardia:** Implementa **ChaCha20-Poly1305** para el cifrado autenticado de secretos individuales, garantizando integridad y confidencialidad.
- **Derivación de Claves Hardened:** Utiliza **Argon2id** (Parámetros: 64MB RAM, 3 iteraciones, 4 hilos) para transformar tu contraseña maestra en una clave criptográfica, ofreciendo la mejor resistencia actual contra ataques de fuerza bruta por GPU/ASIC.

### 🧠 Protección de Memoria Activa
- **Memory Locking:** La clave maestra se bloquea en la RAM física mediante `mlock` (Unix) o `VirtualLock` (Windows), impidiendo que el sistema operativo la escriba en el archivo de intercambio (swap) o paginación en disco.
- **Defensa Anti-Forense:** Bóveda desactiva los volcados de memoria (`core dumps`) a nivel de proceso para evitar que la memoria sensible sea volcada al disco tras un fallo.
- **Arquitectura Zero-Knowledge:** Los secretos se descifran solo bajo demanda y se mantienen en contenedores `Zeroize` que limpian físicamente los bytes de la memoria RAM inmediatamente después de su uso.

## 🏗️ Arquitectura del Proyecto

Bóveda utiliza una arquitectura desacoplada para facilitar auditorías y mantenimiento:

- **`crates/boveda-core`**: Biblioteca pura de Rust que contiene toda la lógica de seguridad, cifrado y base de datos.
- **`src-tauri`**: Capa de orquestación nativa que comunica el núcleo de Rust con la interfaz gráfica mediante IPC seguro.
- **`src`**: Interfaz de usuario reactiva construida con Svelte 5, ofreciendo una experiencia fluida, rápida y estética.

## 🛠️ Configuración de Desarrollo

**Requisitos previos:**
- [Node.js](https://nodejs.org/) (v20+)
- [Rust](https://rustup.rs/) (v1.77+)
- Herramientas de compilación de Tauri ([Guía oficial](https://tauri.app/v2/guides/getting-started/prerequisites))

```bash
# 1. Instalar dependencias del frontend
npm install

# 2. Ejecutar en modo desarrollo (Hot Reload)
npm run tauri dev

# 3. Compilar para producción (Instaladores nativos)
npm run tauri build

## 🛡️ Auditoría de Seguridad

Para garantizar que las dependencias no tengan vulnerabilidades conocidas, puedes ejecutar:

```bash
# Auditoría de dependencias de Rust y JavaScript
npm run security
```

O individualmente:
```bash
cargo audit  # Para el motor de Rust
npm audit    # Para el frontend
```
```

## 🤝 Contribuciones

Bóveda es un proyecto comunitario. Valoramos las contribuciones que mejoren la seguridad o la experiencia de usuario. Por favor, revisa nuestras guías de estilo antes de enviar un Pull Request.

## 📜 Licencia

Este proyecto está bajo la Licencia **Apache-2.0**. Consulta el archivo `LICENSE` para más detalles.
