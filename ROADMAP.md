# 🗺️ Bóveda Core Roadmap

Este documento detalla la visión estratégica y los objetivos técnicos para el desarrollo de **Bóveda Core**. El objetivo es convertirnos en el estándar de código abierto para la gestión de secretos locales y multiplataforma.

---

## 🎯 Fase 1: Estabilización y Confianza (Q3 2026)
*Enfoque: Calidad de código, auditoría interna y robustez.*

- [ ] **Automatización de Pruebas (CI/CD):**
    - [x] Integrar `cargo test`, `cargo clippy` y `cargo audit` en GitHub Actions para bloquear código no seguro.
    - Reportes de cobertura de código (Code Coverage) superiores al 90%.
- [ ] **Fuzz Testing:** Implementar `cargo-fuzz` en el motor de descifrado y procesamiento de base de datos para detectar fallos de memoria raros.
- [ ] **Libro Blanco de Seguridad:** Documentar exhaustivamente el diseño criptográfico, la jerarquía de claves y las medidas de endurecimiento de memoria.
- [x] **Refactorización de Errores:** Completar la migración a errores estructurados y localizables en todos los módulos del núcleo.

## 🚀 Fase 2: Expansión del Ecosistema (Q4 2026 - Q1 2027)
*Enfoque: Portabilidad y acceso desde múltiples interfaces.*

- [ ] **Soporte WebAssembly (WASM):** Adaptar el núcleo para ser compilado a WASM, permitiendo su uso en extensiones de navegador y aplicaciones web seguras.
- [ ] **CLI de Bóveda:** Lanzar una interfaz de línea de comandos oficial para gestión de baúles en servidores y entornos headless.
- [ ] **SDK para Móviles:** Crear bindings oficiales para Android (Kotlin/JNI) e iOS (Swift/UniFFI) utilizando el mismo motor de Rust.
- [ ] **Contenedor de Exportación Blindado:** Implementar un formato de exportación (.bvda.pack) estrictamente cifrado con pipeline de memoria zeroized, rechazando formatos en texto plano (CSV/JSON) para garantizar la integridad forense.

## 🛡️ Fase 3: Hardening Avanzado y Funciones Pro (Q2 2027+)
*Enfoque: Integración con hardware y funciones de nivel empresarial.*

- [ ] **Integración de Hardware Security (HSM):** Soporte nativo para llaves físicas (YubiKey) y Secure Enclave / TPM para el desbloqueo del baúl.
- [x] **software Security (U2F):** Implementar soporte para TOTP (Google/Microsoft Authenticator) como paso adicional de desbloqueo, interfaz de configuración con generación de QR y advertencia sobre la dependencia de ecosistemas externos.
- [ ] **Biometría Nativa:** Desbloqueo mediante Windows Hello, TouchID y FaceID integrado 
directamente en la lógica del motor.
- [ ] **Sincronización E2EE Opcional:** Protocolo de sincronización entre dispositivos con cifrado de extremo a extremo, donde el servidor nunca tiene acceso a los datos.
- [ ] **Auditoría Externa:** Someter el código a una auditoría de seguridad profesional por una firma independiente.

---

> **Nota:** Este roadmap es un documento vivo y puede cambiar según las necesidades de la comunidad y los avances en el campo de la ciberseguridad.
