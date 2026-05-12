# Bóveda — Password Manager 🔒

Construida bajo el paradigma de **Seguridad por Aislamiento**. Mientras que los "gigantes" de la industria priorizan la comodidad de la nube y la recolección de metadatos, Bóveda nace de la necesidad de soberanía digital total, donde tus secretos nunca tocan el disco sin cifrar ni la red sin tu permiso explícito.

![License](https://img.shields.io/badge/license-Apache--2.0-blue?style=for-the-badge)
![Tauri](https://img.shields.io/badge/Tauri-2.0-orange?style=for-the-badge)
![Svelte](https://img.shields.io/badge/Svelte-5.0-red?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-1.77+-black?style=for-the-badge)

---

## 🏛️ La Filosofía: Seguridad por Aislamiento

A diferencia de las soluciones convencionales, Bóveda se rige por tres pilares fundamentales que nos diferencian:

1.  **Aislamiento de Procesos:** La interfaz de usuario vive en un entorno restringido. Nunca tiene acceso directo a las claves maestras o a la base de datos. Toda operación sensible ocurre en el "Core" de Rust a través de un puente IPC (Inter-Process Communication) estrictamente tipado y auditado.
2.  **Soberanía del Dato:** No hay "nube por defecto". Tus datos te pertenecen y residen exclusivamente en tu hardware. El aislamiento no es solo técnico, es estructural: Bóveda asume que cualquier conexión externa es un vector de ataque potencial.
3.  **Resistencia Forense:** No basta con cifrar. Bóveda implementa medidas para que, incluso si un atacante obtiene acceso físico a la memoria RAM o a los volcados de sistema, no encuentre rastros legibles de tu información.

---

## 🛡️ Ingeniería de Seguridad (Bóveda Core)

El motor `boveda-core` es una pieza independiente encargada de proteger los datos sensibles:

### 🔐 Criptografía de Vanguardia
-   **Almacenamiento Ciego:** Base de datos **SQLite + SQLCipher** con cifrado **AES-256-CBC**. Protegemos no solo las entradas, sino el esquema, los índices y los metadatos.
-   **Secretos:** Cada entrada individual se cifra adicionalmente con **ChaCha20-Poly1305**, proporcionando Cifrado Autenticado con Datos Asociados (AEAD).
-   **Barrera de Fuerza Bruta:** Implementamos **Argon2id** (Parámetros: 64MB RAM, 3 iteraciones, 4 hilos), el estándar ganador de la Password Hashing Competition, configurado para ser costoso en hardware especializado (ASIC/GPU).

### 🧠 Gestión de Memoria "Inmune"
-   **Zeroización:** Uso de `Zeroize` que sobrescriben físicamente la memoria RAM con ceros en cuanto un secreto deja de ser necesario, mitigando ataques de reutilización de memoria.
-   **RAM Inamovible:** Implementamos `mlock` / `VirtualLock` para evitar que las claves maestras terminen en el archivo de intercambio (swap) del disco duro.
-   **Hardening del Proceso:** Desactivamos los `core dumps` y protegemos contra la inspección de procesos mediante políticas de seguridad a nivel de sistema operativo.

---

## 🏗️ Arquitectura de Capas

```mermaid
graph TD
    UI[Interfaz Svelte 5] -- IPC Seguro --> Tauri[Capa de Orquestación]
    Tauri -- Comandos Rust --> Core[Bóveda Core - Rust]
    Core -- Cifrado --> DB[(SQLite + SQLCipher)]
    Core -- Memoria --> RAM[Zeroized RAM / mlock]
```

-   **`crates/boveda-core`**: El núcleo protector. Rust puro, sin dependencias de UI, enfocado 100% en seguridad.
-   **`src-tauri`**: El guardián moderno. Gestiona los permisos y la comunicación entre la webview y el sistema.
-   **`src`**: La cara. Una interfaz premium, rápida y minimalista que hace que la seguridad extrema se sienta natural.

---

## 🛠️ Configuración de Desarrollo

**Requisitos:**
- [Node.js](https://nodejs.org/) (v20+)
- [Rust](https://rustup.rs/) (v1.77+)
- [Tauri Prerequisites](https://tauri.app/v2/guides/getting-started/prerequisites)

```bash
# Instalar dependencias
npm install

# Ejecutar en modo desarrollo
npm run tauri dev

# Compilar binario de producción
npm run tauri build
```

## 🛡️ Auditoría y Calidad

Mantenemos un estándar de "Cero Advertencias". Puedes verificar la integridad del proyecto con:

```bash
# Auditoría completa (Rust + JS)
npm run security
```

O por separado:
- `cargo audit`: Verifica vulnerabilidades en dependencias de Rust.
- `cargo clippy`: Linter estricto para asegurar código idiomático y seguro.
- `npm audit`: Verifica el ecosistema de Node.js.

---

## 🤝 Contribuciones

Si compartes nuestra visión de una privacidad sin compromisos, tus PRs son bienvenidos. Consulta el [ROADMAP.md](./ROADMAP.md) para ver en qué estamos trabajando.

## 📜 Licencia

Bóveda es software libre bajo la licencia **Apache-2.0**. Tu seguridad no debería ser una caja negra.
