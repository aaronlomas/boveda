# Política de Seguridad 🛡️

Valoramos mucho la seguridad de **Bóveda**. Si crees haber encontrado una vulnerabilidad de seguridad, te agradecemos que nos lo informes de manera responsable.

## Versiones Soportadas

Actualmente, solo proporcionamos parches de seguridad para las versiones estables más recientes de cada componente.


| Componente | Versión Soportada | Estado |
| :--- | :--- | :--- |
| **Bóveda** (Aplicación) | `1.1.x` | ✅ Soportada |
| **Bóveda** (Aplicación) | `< 1.1.0` | ❌ No soportada |
| **boveda-core** (Motor) | `1.0.x` | ✅ Soportada |
| **boveda-core** (Motor) | `< 1.0.0` | ❌ No soportada |

Para detalles técnicos sobre nuestra implementación criptográfica y medidas de endurecimiento de memoria, consulta el [Libro Blanco de Seguridad](./crates/boveda-core/docs/SECURITY_WHITE_PAPER.md).

## Reportar una Vulnerabilidad

**Por favor, NO abras un Issue público para reportar una vulnerabilidad.**

Para reportar un fallo de seguridad, por favor sigue estos pasos:

1. Envía un correo electrónico detallado a **[security@boveda.app]** (o el canal de contacto del mantenedor).
2. Incluye una descripción del fallo, los pasos para reproducirlo y, si es posible, una prueba de concepto (PoC).
3. Danos un tiempo razonable para investigar y solucionar el problema antes de hacerlo público.

Prometemos:
- Acusar recibo de tu reporte en un plazo de 48-72 horas.
- Mantenerte informado sobre el progreso del parche.
- Darte el crédito correspondiente en nuestras notas de lanzamiento (si así lo deseas) una vez solucionado.

## Filosofía de Respuesta
Bóveda se toma en serio la soberanía digital. Cualquier fallo que comprometa el aislamiento de los secretos o la integridad del cifrado será tratado con la máxima prioridad.

Gracias por ayudarnos a mantener Bóveda segura.
