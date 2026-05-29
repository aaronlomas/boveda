/**
 * @module csvImporter
 * @description Responsable de convertir archivos CSV exportados por gestores
 * de contraseñas y navegadores externos al formato interno de Bóveda.
 *
 * ## Formatos soportados (detección automática por cabecera)
 *
 * | Navegador / Gestor     | Columnas clave reconocidas                        |
 * |------------------------|---------------------------------------------------|
 * | Chrome / Edge / Brave  | name, url, username, password, note               |
 * | Firefox                | url, username, password, httpRealm               |
 * | Safari (Keychain)      | Title, URL, Username, Password, Notes             |
 * | Bitwarden              | login_uri, login_username, login_password, name  |
 * | LastPass               | url, username, password, name, extra             |
 * | 1Password              | Title, Website, Username, Password, Notes        |
 * | Dashlane               | url, username, password, name, note              |
 *
 * ## Contrato de salida
 * Cada entrada importada se normaliza a `ImportedCredential`.
 * El módulo nunca lanza excepciones por filas individuales vacías o inválidas;
 * las registra en `ImportResult.skipped` para que la UI pueda informarlas.
 *
 * @security El contenido del CSV se procesa íntegramente en memoria y
 * nunca se persiste en texto plano. La función devuelve estructuras en RAM
 * que el llamador debe cifrar inmediatamente mediante `addAccount()`.
 */

// ─── Tipos ────────────────────────────────────────────────────────────────────

/**
 * Representación normalizada de una credencial importada desde cualquier
 * fuente externa. Todos los campos son strings para facilitar el cifrado.
 */
export interface ImportedCredential {
  /** Nombre del sitio o app. Nunca vacío (usa fallback si la fuente lo omite). */
  site: string;
  /** URL completa si está disponible, o cadena vacía. */
  url: string;
  /** Nombre de usuario. Usa `"(sin usuario)"` si la fuente lo omite. */
  username: string;
  /** Contraseña en texto plano. El llamador debe cifrarla de inmediato. */
  password: string;
  /** Notas adicionales. Puede incluir la URL si `site` ya la usó. */
  notes: string;
}

/** Resultado completo de una importación de CSV. */
export interface ImportResult {
  /** Credenciales listas para cifrar e insertar. */
  credentials: ImportedCredential[];
  /** Número de filas ignoradas (sin contraseña, malformadas, etc.). */
  skipped: number;
  /** Formato detectado automáticamente. */
  detectedFormat: CsvFormat;
}

/** Formatos de CSV soportados. */
export type CsvFormat =
  | "chrome"
  | "firefox"
  | "safari"
  | "bitwarden"
  | "lastpass"
  | "1password"
  | "dashlane"
  | "generic";

// ─── Detección de formato ─────────────────────────────────────────────────────

/**
 * Mapeo de formato → función de extracción de índices.
 * Cada entrada define los nombres de columna que identifican ese formato.
 *
 * Si la cabecera del CSV contiene todas las columnas requeridas (`required`),
 * el formato queda detectado. Las columnas opcionales (`optional`) se usarán
 * si están presentes.
 */
const FORMAT_SIGNATURES: Record<
  CsvFormat,
  { required: string[]; optional?: string[] }
> = {
  bitwarden:  { required: ["login_password", "login_username", "login_uri"] },
  lastpass:   { required: ["password", "username", "url", "extra"] },
  "1password":{ required: ["password", "username", "website", "title"] },
  safari:     { required: ["password", "username", "url", "title"] },
  firefox:    { required: ["password", "username", "url", "httprealm"] },
  chrome:     { required: ["password", "username", "url", "name"] },
  dashlane:   { required: ["password", "username", "url", "name"], optional: ["note"] },
  generic:    { required: ["password"] },
};

/**
 * Detecta el formato de un CSV a partir de su cabecera normalizada.
 * La detección es ordenada de más específico a más genérico.
 */
function detectFormat(header: string[]): CsvFormat {
  const order: CsvFormat[] = [
    "bitwarden", "lastpass", "1password", "safari", "firefox", "chrome", "dashlane", "generic",
  ];
  for (const fmt of order) {
    const sig = FORMAT_SIGNATURES[fmt];
    if (sig.required.every((col) => header.includes(col))) {
      return fmt;
    }
  }
  return "generic";
}

// ─── Parseo de CSV ────────────────────────────────────────────────────────────

/**
 * Parsea una línea de CSV respetando campos entrecomillados con `"`.
 * Maneja: comillas dobles escapadas (`""`), espacios al inicio/fin, y
 * separadores dentro de campos entrecomillados.
 *
 * @param line Línea cruda del archivo CSV.
 * @returns Array de valores, sin comillas externas ni espacios superfluos.
 */
export function parseCsvLine(line: string): string[] {
  const result: string[] = [];
  let current = "";
  let inQuotes = false;

  for (let i = 0; i < line.length; i++) {
    const ch = line[i];
    const next = line[i + 1];

    if (ch === '"') {
      if (inQuotes && next === '"') {
        // Comilla escapada dentro de campo ("" → ")
        current += '"';
        i++;
      } else {
        inQuotes = !inQuotes;
      }
    } else if (ch === ',' && !inQuotes) {
      result.push(current.trim());
      current = "";
    } else {
      current += ch;
    }
  }
  result.push(current.trim());
  return result;
}

// ─── Extractores por formato ──────────────────────────────────────────────────

/**
 * Extrae un valor de `row` usando una lista de posibles nombres de columna.
 * Devuelve la primera coincidencia encontrada, o `""` si ninguna existe.
 */
function pick(row: string[], header: string[], ...candidates: string[]): string {
  for (const name of candidates) {
    const idx = header.indexOf(name);
    if (idx !== -1 && idx < row.length) return row[idx] || "";
  }
  return "";
}

/**
 * Convierte una fila raw en una `ImportedCredential` según el formato detectado.
 * Aplica fallbacks para campos requeridos por el backend (site, username).
 *
 * @param row   Valores de la fila ya parseados.
 * @param header Cabecera normalizada (minúsculas).
 * @param format Formato detectado.
 * @returns `ImportedCredential` o `null` si la fila no tiene contraseña.
 */
function extractCredential(
  row: string[],
  header: string[],
  format: CsvFormat
): ImportedCredential | null {
  let site = "";
  let url = "";
  let username = "";
  let password = "";
  let notes = "";

  switch (format) {
    case "bitwarden":
      site     = pick(row, header, "name");
      url      = pick(row, header, "login_uri");
      username = pick(row, header, "login_username");
      password = pick(row, header, "login_password");
      notes    = pick(row, header, "notes");
      break;

    case "lastpass":
      site     = pick(row, header, "name", "url");
      url      = pick(row, header, "url");
      username = pick(row, header, "username");
      password = pick(row, header, "password");
      notes    = pick(row, header, "extra", "grouping");
      break;

    case "1password":
      site     = pick(row, header, "title");
      url      = pick(row, header, "website");
      username = pick(row, header, "username");
      password = pick(row, header, "password");
      notes    = pick(row, header, "notes");
      break;

    case "safari":
      site     = pick(row, header, "title");
      url      = pick(row, header, "url");
      username = pick(row, header, "username");
      password = pick(row, header, "password");
      notes    = pick(row, header, "notes");
      break;

    case "firefox":
      // Firefox no tiene "name"; usa el hostname como identificador
      url      = pick(row, header, "url");
      username = pick(row, header, "username");
      password = pick(row, header, "password");
      notes    = pick(row, header, "httprealm");
      site     = url;
      break;

    case "dashlane":
      site     = pick(row, header, "name", "title");
      url      = pick(row, header, "url");
      username = pick(row, header, "username", "login");
      password = pick(row, header, "password");
      notes    = pick(row, header, "note", "notes");
      break;

    // chrome / generic (fallback)
    default:
      site     = pick(row, header, "name", "title");
      url      = pick(row, header, "url", "website");
      username = pick(row, header, "username", "login", "email");
      password = pick(row, header, "password");
      notes    = pick(row, header, "note", "notes", "extra");
      break;
  }

  // Sin contraseña → fila ignorable
  if (!password.trim()) return null;

  // ── Fallbacks requeridos por el backend ───────────────────────────────────
  // El motor de Bóveda requiere `site` y `username` no vacíos.
  // Si la URL fue usada como site, no la duplicamos en notes.
  const resolvedSite = (site || url || "").trim() || "Cuenta Importada";
  const resolvedUser = username.trim() || "(sin usuario)";

  let resolvedNotes = notes.trim();
  if (url && url !== resolvedSite) {
    resolvedNotes = resolvedNotes ? `${url}\n${resolvedNotes}` : url;
  }

  return {
    site:     resolvedSite,
    url,
    username: resolvedUser,
    password: password.trim(),
    notes:    resolvedNotes,
  };
}

// ─── API pública ──────────────────────────────────────────────────────────────

/**
 * Procesa el contenido de un archivo CSV exportado por un navegador o
 * gestor de contraseñas y devuelve las credenciales normalizadas.
 *
 * **Nunca persiste datos**: toda la operación ocurre en memoria.
 *
 * @param content Contenido completo del archivo CSV como string.
 * @returns `ImportResult` con las credenciales listas y estadísticas.
 * @throws Error si el CSV está vacío, no tiene cabecera, o no contiene
 *         ninguna columna de contraseña reconocible.
 *
 * @example
 * ```ts
 * const raw = await readExternalFile(path);
 * const { credentials, detectedFormat } = parseCsv(raw);
 * for (const cred of credentials) {
 *   await addAccount(cred.site, cred.username, cred.password, "", cred.notes);
 * }
 * ```
 */
export function parseCsv(content: string): ImportResult {
  const lines = content
    .split(/\r?\n/)
    .filter((l) => l.trim() !== "");

  if (lines.length < 2) {
    throw new Error("Archivo CSV vacío o sin datos");
  }

  // Normalizar cabecera: minúsculas, sin espacios, sin comillas
  const header = parseCsvLine(lines[0]).map((h) =>
    h.toLowerCase().replace(/["\s]/g, "")
  );

  if (!header.some((h) => h.includes("password"))) {
    throw new Error(
      "CSV no reconocido: no se encontró ninguna columna de contraseña. " +
      "Asegúrate de exportar desde un gestor compatible (Chrome, Firefox, Bitwarden, LastPass, 1Password, Safari, Dashlane)."
    );
  }

  const detectedFormat = detectFormat(header);
  const credentials: ImportedCredential[] = [];
  let skipped = 0;

  for (let i = 1; i < lines.length; i++) {
    const row = parseCsvLine(lines[i]);
    const cred = extractCredential(row, header, detectedFormat);
    if (cred) {
      credentials.push(cred);
    } else {
      skipped++;
    }
  }

  return { credentials, skipped, detectedFormat };
}
