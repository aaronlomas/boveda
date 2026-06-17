# Auditoría técnica de `boveda-core`

## Alcance

Esta auditoría cubre el crate `crates/boveda-core`, con énfasis en:

- Autenticación y desbloqueo de bóveda
- Derivación de clave y cifrado de datos
- Gestión de secretos en memoria
- Almacenamiento local y protecciones de SQLCipher
- 2FA/TOTP
- Registro de auditoría
- Flujos de exportación/importación

## Resumen de arquitectura

`boveda-core` expone una capa de motor de bóveda basada en SQLite cifrado y una fachada de comandos (`src/commands.rs`).

Componentes clave:

- `src/vault/authentication/mod.rs`: desbloqueo de bóveda, creación de salt, conexión con SQLCipher, bloqueo y detección de sesiones remotas.
- `src/vault/mod.rs`: motor de bóveda, llave maestra dentro de memoria segura, bloqueo, verificación de estado y utilidad `with_key`.
- `src/crypto/mod.rs`: derivación de clave con Argon2id y cifrado AEAD ChaCha20-Poly1305.
- `src/crypto/secret.rs`: wrappers seguros `SecretString`, `SecretKey`, `SecretBytes` con `zeroize` y comparaciones de tiempo constante.
- `src/storage/mod.rs`: esquema SQLite, tablas de cuentas, pines, documentos, preferencias, TOTP y registro de auditoría.
- `src/vault/totp/mod.rs`: configuración de TOTP, verificación, códigos de recuperación y desactivación.

El diseño es de software local. Los datos persistentes están en un archivo SQLite cifrado (`vault.bvda`) y un archivo de sal separado (`vault.salt`) dentro del directorio de datos local.

## Hallazgos principales

### Seguridad del cifrado y derivación de clave

Fortalezas:

- Derivación de clave de contraseña con `argon2id`, parámetros robustos: `m=65536`, `t=3`, `p=4`.
- Sal aleatoria de 32 bytes generada con `OsRng` y almacenada en `vault.salt`.
- Uso de SQLCipher con parámetros explícitos en `open_encrypted()` para fortalecer la base de datos cifrada.
- Cifrado adicional para datos de usuario con ChaCha20-Poly1305 en `crypto::encrypt()`/`decrypt()`.
- `generate_pragma_key()` construye la clave SQLCipher en formato hexadecimal seguro, evitando inyecciones.
- `secure_delete = ON` y `journal_mode = WAL` en la configuración de SQLite.

Observaciones:

- El archivo `vault.salt` es crítico; su protección depende de permisos a nivel de sistema. La creación de permisos `0o600` en Unix es adecuada.
- Argumentos SQLCipher están bien fijados, pero dependen de que la biblioteca SQLCipher se compile y funcione correctamente en todas las plataformas objetivo.

### Gestión de secretos en memoria

Fortalezas:

- `SecretString`, `SecretKey` y `SecretBytes` usan `zeroize` para borrar datos sensibles al liberar memoria.
- `MasterKey` usa `mlock`/`VirtualLock` para evitar el intercambio a disco de la llave maestra.
- `SecretString` es redacted en `Debug` y no expone valores reales al serializar.

Observaciones:

- `with_key()` reconstruye un `SecretKey` temporal desde el `MasterKey` en memoria, lo que duplica la llave en stack/memoria breve. Esto es funcional, pero deja una pequeña superficie adicional donde la clave existe en dos ubicaciones en memoria durante la operación.
- El método `export_vault()` materializa datos desencriptados en memoria para empaquetado. Aunque normal para exportación, aumenta la exposición de datos sensibles si el proceso es comprometido.

### Almacenamiento local y protección de datos

Fortalezas:

- El esquema SQLite está segregado con tablas separadas para cuentas, pines, documentos, preferencias, TOTP y auditoría.
- Todos los datos sensibles se almacenan como campos cifrados (`TEXT`).
- La base de datos no almacena la sal KDF; `vault.salt` permanece fuera de la DB.
- SQL `INSERT`/`SELECT` usa parámetros enlazados (`bind`), evitando inyección SQL en la mayoría de flujos.

Observaciones:

- La tabla `audit_log` almacena acciones y metadata en texto sin cifrar. Esto es adecuado para auditoría, pero debe manejarse como información sensible del sistema.
- `preferences` usa clave/valor en texto claro. Algunos valores (como `security.block_remote`) son no sensibles, pero no hay segregación entre metadatos y configuraciones de seguridad.

### 2FA / TOTP

Fortalezas:

- TOTP se protege con semilla cifrada en la bóveda (`totp_secret_cipher`).
- Los códigos de recuperación se almacenan como filas individuales en `totp_recovery_codes` y también cifrados.
- `setup_totp()` limpia formatos legados y evita sobreescribir una configuración TOTP activa sin desactivación previa.
- `verify_totp()` implementa un limitador de intentos que bloquea después de 5 fallos y exige una espera de 5 minutos.
- La desactivación de TOTP borra la semilla y los códigos de recuperación.

Observaciones:

- Si `verify_totp()` falla, incrementa los contadores de intentos mediante preferencias en la base de datos. Esto es sólido, pero los contadores residen en almacenamiento persistente y podrían ser manipulados por actores con acceso a la DB.
- `verify_totp_recovery()` realiza comparaciones de longitud y tiempo constante para evitar fugas por temporización al recorrer códigos.

### Autenticación, sesiones y bloqueo

Fortalezas:

- `AppState` exige verificación de sesión TOTP antes de permitir operaciones críticas.
- `cmd_lock_vault()` borra el motor y anula la sesión verificada.
- `BovedaEngine::is_locked()` trata bloqueos de mutex envenenados de forma conservadora, devolviendo el estado de bóveda bloqueado si no se puede garantizar la integridad del bloqueo.

## Registro de auditoría

Fortalezas:

- La función `storage::add_audit_log()` registra eventos clave con `action`, `metadata` y `created_at`.
- `boveda-core` audita eventos críticos como desbloqueo, bloqueo, importación, exportación, creación/eliminación de cuentas, PINs y documentos, y cambios TOTP.
- Los errores se abstraen para evitar la fuga de detalles técnicos al usuario.

Observaciones:

- La metadata de auditoría se almacena en texto claro. Si se incluye información sensible en `metadata`, puede requerir un tratamiento adicional de acceso o encriptado.
- El registro de auditoría no incluye un mecanismo de rotación o limpieza automática; con tiempo, la tabla `audit_log` puede crecer sin límites.

## Exportación e importación

Fortalezas:

- El paquete de exportación utiliza su propio salt y KDF independiente del vault principal.
- Exportación e importación dependen de `SecretString` y `SecretBytes`, manteniendo los datos sensibles en estructuras redacted.
- El flujo de importación evita sobrescribir configuraciones TOTP y filtra preferencias sensibles de 2FA.

Observaciones:

- `export_vault()` desencripta todo el contenido de la bóveda en memoria antes de volver a cifrarlo. Este es un riesgo inherente al proceso y debe manejarse con cuidado en el lado de la aplicación.
- La importación en modo `Merge` omite duplicados por sitio/usuario, pero no ofrece opciones de conflicto más finas.

## Recomendaciones

1. Revisar `with_key()` para minimizar las duplicaciones de clave maestra en memoria. Considerar un wrapper que opere directamente sobre la llave confinada sin reconstruir un `SecretKey` temporal.
2. Añadir protección o encriptado opcional para los metadatos de auditoría cuando contengan información sensible.
3. Evaluar un mecanismo de limpieza/rotación de `audit_log` o límites de retención, especialmente en dispositivos con espacio limitado.
4. Auditar la dependencia `libsqlite3-sys` + SQLCipher para asegurar que el binario utiliza la configuración esperada y no depende de bibliotecas externas variables.
5. Verificar el acceso al archivo `vault.salt` en plataformas no Unix y documentar el comportamiento de permisos para Windows.

## Conclusión

`boveda-core` muestra una implementación sólida para un software local de almacenamiento cifrado. La bóveda utiliza modernas prácticas criptográficas, protege secretos en memoria y separa claramente el estado de desbloqueo de la lógica de aplicación.

Las áreas que merecen atención prioritaria están relacionadas con la superficie de memoria durante operaciones con la clave maestra, la protección de metadatos de auditoría y la gestión del ciclo de vida del registro.

---

Documento generado por auditoría de `boveda-core` y guardado en `docs/auditoria-boveda-core.md`.
