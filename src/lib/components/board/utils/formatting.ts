/**
 * Ejecuta un comando nativo de edición de texto en el documento.
 * @param {string} command - El nombre del comando (ej. 'bold', 'italic').
 * @param {string | undefined} [value] - Valor opcional para el comando.
 */
export function execBoardCommand(command: string, value: string | undefined = undefined): void {
  if (typeof document !== "undefined") {
    document.execCommand(command, false, value);
  }
}

/**
 * Aplica un tamaño de fuente exacto en puntos (pt) a la selección actual.
 * Crea un <span> con estilos inline para evitar las limitaciones del scale 1-7 del navegador.
 * @param {number} size - El tamaño de la fuente en puntos.
 */
export function setFontSize(size: number): void {
  if (typeof document === "undefined") return;

  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return;

  const range = sel.getRangeAt(0);

  if (range.collapsed) {
    // No text selected — store the size for the next character typed
    // using a zero-width span so future typed text inherits the size.
    const span = document.createElement("span");
    span.style.fontSize = `${size}pt`;
    // Insert a zero-width space so the caret enters the span
    span.innerHTML = "\u200B";
    range.insertNode(span);
    // Move caret inside the span, after the ZWS
    const newRange = document.createRange();
    newRange.setStart(span.firstChild!, 1);
    newRange.collapse(true);
    sel.removeAllRanges();
    sel.addRange(newRange);
    return;
  }

  // Text is selected — wrap it
  try {
    const span = document.createElement("span");
    span.style.fontSize = `${size}pt`;
    // surroundContents throws if selection crosses block boundaries;
    // fall back to execCommand in that case.
    range.surroundContents(span);
  } catch {
    // Fallback for complex selections: clone content, wrap, delete & insert
    const fragment = range.extractContents();
    const span = document.createElement("span");
    span.style.fontSize = `${size}pt`;
    span.appendChild(fragment);
    range.insertNode(span);
  }

  // Restore selection
  sel.removeAllRanges();
  const newRange = document.createRange();
  const inserted = range.commonAncestorContainer.lastChild as Node;
  newRange.selectNodeContents(inserted ?? range.commonAncestorContainer);
  sel.addRange(newRange);
}

/**
 * Maneja eventos de pegado con saneamiento de seguridad y detección de patrones peligrosos.
 * @param {ClipboardEvent} e - El evento de portapapeles.
 * @param {(msg: string) => void} onWarning - Callback para notificar advertencias de seguridad.
 * @returns {string | null} El texto saneado o null si ya se manejó la inserción.
 */
export function handlePasteSecurity(e: ClipboardEvent, onWarning: (msg: string) => void): string | null {
  e.preventDefault();
  let text = e.clipboardData?.getData("text/plain");
  
  if (!text) return null;

  // 1. Sanitize: Remove zero-width spaces and dangerous non-printable control characters
  const sanitizedText = text.replace(/[\u200B-\u200D\uFEFF\u0000-\u0008\u000B\u000C\u000E-\u001F\u007F]/g, "");
  
  // 2. Detect: Expanded dangerous patterns
  const dangerousPatterns = /(\bsudo\b|\bcurl\b|\bwget\b|\bpowershell\b|\bbash\b|\bsh -c\b|\brm -rf\b|\biex\b|\binvoke-expression\b|\bshutdown\b|\bnet user\b|\bformat\b|\bdel \/f\b)/gi;
  
  if (dangerousPatterns.test(sanitizedText)) {
    // This will be translated in the component if needed, but here we provide a default or let the component handle it
    onWarning("dangerous_patterns_detected");
    
    // 3. Unmasking: Highlight malicious parts in red
    // Escape HTML to prevent XSS from the text itself
    const escaped = sanitizedText.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
    const highlightedHTML = escaped.replace(dangerousPatterns, (match) => {
      return `<span style="color: #ef4444; font-weight: bold; background: rgba(239, 68, 68, 0.15); padding: 1px 4px; border-radius: 4px; border: 1px solid rgba(239, 68, 68, 0.3);" title="Detección de Bóveda: Comando peligroso">${match}</span>`;
    });
    
    document.execCommand("insertHTML", false, highlightedHTML);
    return null; // Return null because we already inserted the HTML
  } else {
    document.execCommand("insertText", false, sanitizedText);
    return sanitizedText;
  }
}
