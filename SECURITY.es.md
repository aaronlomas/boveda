# Política de Seguridad 🛡️

Valoramos mucho la seguridad de **Bóveda**. Si crees haber encontrado una vulnerabilidad de seguridad, te agradecemos que nos lo informes de manera responsable.

## Versiones Soportadas

Actualmente, solo proporcionamos parches de seguridad para las versiones estables más recientes de cada componente.


| Componente | Versión Soportada | Estado |
| :--- | :--- | :--- |
| **Bóveda** (Aplicación) | `1.x.x` | ✅ Soportada |
| **Bóveda** (Aplicación) | `< 1.0.0` | ❌ No soportada |
| **boveda-core** (Motor) | `1.x.x` | ✅ Soportada |
| **boveda-core** (Motor) | `< 1.0.0` | ❌ No soportada |

Para detalles técnicos sobre nuestra implementación criptográfica y medidas de endurecimiento de memoria, consulta el [Libro Blanco de Seguridad](./crates/boveda-core/docs/SECURITY_WHITE_PAPER.md).

## Reportar una Vulnerabilidad

**Por favor, NO abras un Issue público para reportar una vulnerabilidad.**

Para reportar un fallo de seguridad de forma privada y segura, por favor utiliza la función de **Reportes Privados de GitHub**:

1. Ve a la pestaña de [Seguridad (Security)](https://github.com/aaronlomas/boveda/security/advisories/new) de este repositorio.
2. Haz clic en **"Report a vulnerability"**.
3. Incluye una descripción detallada del fallo, los pasos para reproducirlo y, si es posible, una prueba de concepto (PoC).

Este método permite que el equipo de mantenimiento investigue y solucione el problema de forma privada antes de que se haga público un boletín de seguridad (Security Advisory).

Prometemos:
- Acusar recibo de tu reporte en un plazo de 48-72 horas.
- Mantenerte informado sobre el progreso del parche.
- Darte el crédito correspondiente en nuestras notas de lanzamiento (si así lo deseas) una vez solucionado.

## Filosofía de Respuesta
Bóveda se toma en serio la soberanía digital. Cualquier fallo que comprometa el aislamiento de los secretos o la integridad del cifrado será tratado con la máxima prioridad.

Gracias por ayudarnos a mantener Bóveda segura.