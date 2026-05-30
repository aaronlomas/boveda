# Guía de Contribución 🤝

¡Gracias por tu interés en mejorar **Bóveda**! Este es un proyecto impulsado por la comunidad para proporcionar soberanía digital y seguridad sin compromisos. Al ser un gestor de contraseñas, mantenemos estándares de calidad y seguridad extremadamente altos.

---

## Nuestra Filosofía: "Security by Isolation"

Antes de proponer cambios, es vital entender nuestra arquitectura:
- **Aislamiento:** La lógica de negocio y criptografía vive en `crates/boveda-core`. La UI (Svelte) y el puente (Tauri) son solo consumidores de la fachada de comandos.
- **Cero Conocimiento:** El diseño debe garantizar que ni los desarrolladores ni terceros puedan acceder a los datos de los usuarios.
- **Endurecimiento de Memoria:** Cada secreto debe ser borrado físicamente (`Zeroize`) y protegido contra volcados (`mlock`).

---

## 🚀 Cómo Empezar

### Requisitos Previos
- **Rust:** (Última versión estable).
- **Node.js & pnpm:** Usamos `pnpm` para la gestión de dependencias de frontend.
- **Librerías del sistema:** Consulta el [README.md](./README.md) para las dependencias de Tauri según tu OS.

### Configuración del Entorno
1. Fork del repositorio.
2. Clona tu fork: `git clone https://github.com/aaronlomas/boveda.git`
3. Instala dependencias: `pnpm install`
4. Ejecuta en modo desarrollo: `pnpm tauri dev`

---

## 🛠️ Estándares de Código

### Backend (Rust)
Mantenemos una política de **"Cero Advertencias"**. Tu código debe pasar:
- **Formateo:** `cargo fmt --all`
- **Lints:** `cargo clippy --workspace --all-targets -- -D warnings`
- **Seguridad:** `cargo audit` (para nuevas dependencias).

**Reglas:**
- Secretos en memoria: Usa `SecretString` o `SecretBytes` para cualquier dato sensible en memoria; estas envolturas aplican `Zeroize` y protecciones de memoria.
- No serializar wrappers secretos: no derives Serialize/Deserialize en SecretString/SecretBytes. Si el tipo necesita serialización, implementa una serialización que redacte el valor (por ejemplo devuelve "[REDACTED]") para evitar fugas accidentales.
- Nunca imprimas secretos en logs o errores (usa `{:?}` con wrappers redactados).

### Frontend (Svelte 5)
- Usamos **Svelte 5 (Runes)**. Evita patrones antiguos de Svelte 4.
- Ejecuta `pnpm check` para validar tipos y consistencia de componentes.
- Mantén el diseño premium: usa el sistema de diseño basado en variables CSS en `src/app.css`.

---

## 🧪 Pruebas (Testing)

No aceptamos PRs que reduzcan la cobertura de pruebas o rompan el CI.
- **Pruebas unitarias:** `cargo test --package boveda-core`
- **Integración:** Asegúrate de que el flujo de desbloqueo y gestión de cuentas funcione correctamente en modo `dev`.

---

## 📝 Compromisos (Commits)

Sugerimos seguir la convención de [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` Nueva característica.
- `ui:` Cambios visuales o de experiencia de usuario.
- `fix:` Corrección de error.
- `refactor:` Mejora de código sin cambio funcional.
- `sec/security:` Cambios específicos de seguridad o endurecimiento.
- `chore:` configuración y dependencias.
- `docs:` Documentación.
- `ci:` Sistema de integración continua.
---

## 🔒 Reportar Vulnerabilidades

Si encuentras un fallo de seguridad, por favor **NO abras un Issue público**. Consulta nuestra [Política de Seguridad](./SECURITY.md) para saber cómo reportarlo de manera responsable.

---

Al contribuir, aceptas que tus aportaciones estarán bajo la licencia **AGPL-3.0**.
