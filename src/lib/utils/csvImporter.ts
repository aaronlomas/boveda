/**
 * @module csvImporter
 * @description Responsible for converting CSV files exported by external password managers and browsers to Bóveda internal format.
 *
 *
 * ## Supported formats (automatic detection by header)
 *
 * | Navegador / Manager    | Columnas                                          |
 * |------------------------|---------------------------------------------------|
 * | Chrome / Edge / Brave  | name, url, username, password, note               |
 * | Firefox                | url, username, password, httpRealm               |
 * | Safari (Keychain)      | Title, URL, Username, Password, Notes             |
 * | Bitwarden              | login_uri, login_username, login_password, name  |
 * | LastPass               | url, username, password, name, extra             |
 * | 1Password              | Title, Website, Username, Password, Notes        |
 * | Dashlane               | url, username, password, name, note              |
 *
 * ## Output Contract
 * Each imported entry is normalized to `ImportedCredential`.
 * The module never throws exceptions for single empty or invalid rows;
 * it logs them to `ImportResult.skipped` so the UI can report them.
 * @security The CSV content is processed entirely in memory and
 * is never persisted in plain text. The function returns structures in RAM
 * which the caller must immediately encrypt using `addAccount()`.
 */

// ─── Types ────────────────────────────────────────────────────────────────────

/**
 * Standardized representation of a credential imported from any
 * external source. All fields are strings to facilitate encryption.
 */
export interface ImportedCredential {
  /** Site or app name. Never empty (use fallback if the source omits it). */
  site: string;
  /** Full URL if available, or empty string. */
  url: string;
  /** Username. Use `"(sin usuario)"` if the source omits it. */
  username: string;
  /** Password in plain text. The caller must encrypt it immediately. */
  password: string;
  /** Additional notes. May include the URL if `site` already used it. */
  notes: string;
}

/** Complete result of a CSV import. */
export interface ImportResult {
  /** Credentials ready to be encrypted and inserted. */
  credentials: ImportedCredential[];
  /** Number of skipped rows (no password, malformed, etc.). */
  skipped: number;
  /** Automatically detected format. */
  detectedFormat: CsvFormat;
}

/** Supported CSV formats. */
export type CsvFormat =
  | "chrome"
  | "firefox"
  | "safari"
  | "bitwarden"
  | "lastpass"
  | "1password"
  | "dashlane"
  | "generic";

// ─── Format detection ─────────────────────────────────────────────────────

/**
 * Format mapping → index extraction function.
 * Each entry defines the column names that identify that format.
 *
 * If the CSV header contains all required columns (`required`),
 * the format is detected. Optional columns (`optional`) will be used
 * if they are present.
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
 * Detects the format of a CSV from its normalized header.
 * Detection is ordered from most specific to most generic.
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

// ─── CSV Parse ────────────────────────────────────────────────────────────

/**
 * Parses a CSV line respecting fields enclosed in `"`.
 * Handles: escaped double quotes (`""`), leading/trailing spaces, and
 * separators within quoted fields.
 *
 * @param line Raw CSV line.
 * @returns Array of values, without external quotes or superfluous spaces.
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
        // Escaped double quote within field ("" → ")
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

// ─── Format extractors ────────────────────────────────────────────────────────────

/**
 * Extracts a value from `row` using a list of possible column names.
 * Returns the first match found, or `""` if none exists.
 */
function pick(row: string[], header: string[], ...candidates: string[]): string {
  for (const name of candidates) {
    const idx = header.indexOf(name);
    if (idx !== -1 && idx < row.length) return row[idx] || "";
  }
  return "";
}

/**
 * Converts a raw row into an `ImportedCredential` according to the detected format.
 * Applies fallbacks for fields required by the backend (site, username).
 *
 * @param row   Parsed row values.
 * @param header Normalized header (lowercase).
 * @param format Detected format.
 * @returns `ImportedCredential` or `null` if the row has no password.
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
      // Firefox does not have "name"; use the hostname as an identifier
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

  // No password → ignorable row
  if (!password.trim()) return null;

  // ── Fallbacks required by the backend ───────────────────────────────────
  // The Bóveda engine requires non-empty `site` and `username`.
  // If the URL was used as the site, do not duplicate it in notes.
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

// ─── Public API ──────────────────────────────────────────────────────────────

/**
 * Processes the content of a CSV file exported by a browser or
 * password manager and returns the normalized credentials.
 *
 * **Never persists data**: the entire operation takes place in memory.
 *
 * @param content Complete content of the CSV file as a string.
 * @returns `ImportResult` with ready credentials and statistics.
 * @throws Error if the CSV is empty, has no header, or does not contain
 *         any recognizable password column.
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
    throw new Error("CSV file empty or contains no data");
  }

  // Normalize header: lowercase, no spaces, no quotes
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
