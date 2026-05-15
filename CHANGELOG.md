# CHANGELOG 📝

Todas las novedades y cambios notables de **Bóveda** se documentarán en este archivo.

El formato se basa en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y este proyecto adhiere a [Versionado Semántico](https://semver.org/lang/es/).

## [1.0.0] - 2026-05-14

### Añadido
- **Gestión de PINs**: Nueva sección dedicada para almacenar códigos PIN de tarjetas, aplicaciones y accesos de forma segura.
- **Códigos de Recuperación**: Integración de campos específicos para códigos de recuperación en cada credencial.
- **Exportación/Importación Segura de Paquetes (`.bvda.pack`)**: 
  - Ahora permite exportar e importar datos mediante paquetes JSON cifrados con una contraseña independiente.
  - Soporte para estrategias de importación: **Fusionar (Merge)** o **Reemplazar (Replace)**.
  - Inclusión automática de **PINs** y **Códigos de Recuperación** en el proceso de exportación/importación.
- **Asistente de Seguridad (Bóveda Bot)**: Un nuevo asistente interactivo en la exportación que analiza la fortaleza de tu contraseña y te da recomendaciones en tiempo real.
- **Internacionalización (i18n)**: Soporte completo para Español e Inglés en toda la interfaz.

### Cambiado
- **Arquitectura de Comandos (Facade)**: Migración masiva de la lógica de comandos hacia `boveda-core` para una mayor seguridad, aislamiento y facilidad de testeo.
- **Persistencia de Temas**: La configuración del tema visual ahora se desacopla del baúl cifrado y se almacena en el navegador para una carga inmediata al iniciar.
- **UI Polishing**: Mejoras en el diseño de `CredentialCard`, iconos más consistentes y dimensiones fijas para evitar saltos visuales en temporizadores.

### Seguridad
- **Endurecimiento de Memoria**: Implementación de `mlock` (Unix) y `VirtualLock` (Windows) en el núcleo para evitar que la llave maestra sea escrita en el disco (swap).
- **Zeroize**: Limpieza activa de datos sensibles en memoria tras su uso mediante el trait `Zeroize`.
- **Política de Seguridad**: Creación de `SECURITY.md` y guía de contribución técnica.

---

## [0.9.0] - 2026-05-10
- Versión previa al lanzamiento estable con motor Rust refinado.
- Implementación de Cifrado SQLCipher.
- Sistema de Temas dinámicos.
