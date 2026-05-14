# 🛡️ Bóveda Core: Security White Paper (Libro Blanco)

Este documento detalla la arquitectura de seguridad, el diseño criptográfico y las medidas de endurecimiento de memoria implementadas en **Bóveda Core**. Nuestro objetivo es proporcionar transparencia total para usuarios y auditores de seguridad.

---

## 1. Filosofía de Seguridad: "Security by Isolation"
Bóveda Core está diseñado bajo el principio de aislamiento. El núcleo (engine) funciona como una caja negra que maneja secretos solo en memoria protegida, exponiendo una interfaz mínima (Facade) para reducir la superficie de ataque. El sistema es **Zero-Knowledge**: los datos nunca salen del dispositivo del usuario y el desarrollador no tiene forma de recuperar contraseñas maestras.

## 2. Pila Criptográfica (Cryptographic Stack)
Bóveda utiliza algoritmos modernos, resistentes y de alto rendimiento:

| Función | Algoritmo | Razón |
| :--- | :--- | :--- |
| **KDF (Derivación de Llaves)** | Argon2id | Ganador de la Password Hashing Competition; resistente a ataques de GPU/ASIC. |
| **Cifrado Simétrico** | ChaCha20-Poly1305 | Cifrado autenticado (AEAD) extremadamente rápido y seguro, preferido sobre AES en sistemas sin aceleración por hardware dedicada. |
| **Cifrado de Base de Datos** | SQLCipher (AES-256-GCM) | Estándar de la industria para cifrado transparente de bases de datos SQLite con verificación de integridad por página. |
| **Generación de Entropía** | `rand::rngs::OsRng` | Utiliza el generador de números aleatorios criptográficamente seguro del sistema operativo. |

---

## 3. Gestión y Jerarquía de Llaves

### 3.1 Derivación de la Master Key
La contraseña maestra nunca se almacena. Al desbloquear el baúl:
1. Se lee el `vault.salt` (32 bytes aleatorios generados en la inicialización).
2. Se utiliza **Argon2id** para derivar una llave de 256 bits.
3. Los parámetros por defecto están optimizados para seguridad local:
   - Tiempo: 3 iteraciones.
   - Memoria: 64 MB.
   - Paralelismo: 4 hilos.

### 3.2 Protección de la Llave en Memoria
Una vez derivada, la **Master Key** se envuelve en un struct `MasterKey` que implementa:
- **mlock / VirtualLock**: Bloquea la memoria física para evitar que el sistema operativo mueva la llave al archivo de intercambio (Swap/Pagefile) en el disco.
- **Zeroize**: Sobrescribe los bytes de la llave con ceros en el momento exacto en que el struct sale de alcance (`Drop`).

---

## 4. Endurecimiento del Proceso (Process Hardening)

### 4.1 Anti-Forensics (Linux/Unix/Windows)
Al iniciar, Bóveda ejecuta medidas preventivas:
- **Linux:** Llama a `prctl(PR_SET_DUMPABLE, 0)` para desactivar Core Dumps e impedir que procesos no privilegiados se adjunten vía `ptrace`.
- **Windows:** Configura `SetErrorMode` para evitar diálogos de error que puedan exponer volcados de memoria.

### 4.2 Cifrado Transparente y Doble Capa
Toda la base de datos de SQLite está cifrada por SQLCipher. Adicionalmente, Bóveda aplica una **segunda capa de cifrado** (Application-Layer Encryption): cada campo sensible (usuario, contraseña, notas, URL) se cifra con ChaCha20-Poly1305 antes de ser insertado. Incluso si se comprometiera la llave de SQLCipher, los secretos seguirían protegidos.

---

## 5. Autenticación Multifactor (TOTP)
Bóveda Core soporta TOTP (RFC 6238) como factor de protección adicional para el desbloqueo:
- El secreto TOTP se almacena cifrado con la Master Key del usuario.
- Se generan **códigos de recuperación** de un solo uso, también almacenados bajo cifrado, para evitar el bloqueo permanente en caso de pérdida del dispositivo de autenticación.

---

## 6. Trazabilidad y Auditoría (SOC2 Readiness)
Cada operación sensible es registrada en un log de auditoría inmutable dentro de la base de datos cifrada:
- Desbloqueos exitosos/fallidos.
- Revelación de secretos (cada vez que el usuario ve una contraseña).
- Exportaciones e importaciones de datos.
- Cambios en la configuración de seguridad.

---

## 7. Formato de Exportación (.bvda.pack)
El formato de exportación es un contenedor binario cifrado que utiliza:
- Una llave de exportación independiente (derivada con un nuevo Salt y una contraseña de exportación específica).
- Un pipeline de datos que desencripta y vuelve a encriptar en memoria, garantizando que los secretos nunca toquen el disco en texto plano durante el proceso.

---

## 8. Consideraciones de Seguridad
- **Resistencia a Side-Channel:** Se utilizan comparaciones de tiempo constante (`subtle` crate) para verificaciones críticas de autenticación y TOTP.
- **Validación de Datos:** Todos los inputs son saneados y validados para prevenir ataques de inyección o desbordamientos antes de ser procesados por la capa criptográfica.
