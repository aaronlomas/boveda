# CHANGELOG 📝

Todas las novedades y cambios notables de **Bóveda** se documentarán en este archivo.

El formato se basa en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y este proyecto adhiere a [Versionado Semántico](https://semver.org/lang/es/).

## [1.0.0] - 2026-05-14

### Funciones iniciales
- **Gestión de PINs**: Sección dedicada para almacenar códigos PIN de tarjetas, aplicaciones y accesos de forma segura.
- **Códigos de Recuperación**: Integración de campos específicos para códigos de recuperación en cada credencial.
- **Exportación/Importación Segura de Paquetes (`.bvda.pack`)**: 
  - Ahora permite exportar e importar datos mediante paquetes JSON cifrados con una contraseña independiente.
  - Soporte para estrategias de importación: **Fusionar (Merge)** o **Reemplazar (Replace)**.
  - Inclusión automática de **PINs** y **Códigos de Recuperación** en el proceso de exportación/importación.
- **Importación de fuentes externas**:
  - Solo se permiten archivos CSV, este pasa por un filtro para ser guardado en el núcleo seguro de Bóveda.
- **Internacionalización (i18n)**: Soporte completo para Español e Inglés en toda la interfaz.

### Seguridad
- **Endurecimiento de Memoria**: Implementación de `mlock` (Unix) y `VirtualLock` (Windows) en el núcleo para evitar que la llave maestra sea escrita en el disco (swap).
- **Zeroize**: Limpieza activa de datos sensibles en memoria tras su uso mediante el trait `Zeroize`.
- **Política de Seguridad**: Creación de `SECURITY.md` y guía de contribución técnica.

---

## [1.0.0] - 2026-05-10
- Versión previa al lanzamiento estable
