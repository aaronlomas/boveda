/**
 * @module documents/context
 * @description Typed context key for the documents feature.
 * Using a Symbol as the key ensures type safety and prevents accidental key collisions
 * across different features of the application.
 */
import type { useDocuments } from "$lib/composables/useDocuments.svelte";

export type DocumentsContext = ReturnType<typeof useDocuments>;

/**
 * Symbol key used with Svelte's setContext / getContext for the documents feature.
 * Import this instead of using the raw string "DOCUMENTS_STATE".
 *
 * @example
 * // In the provider (DocumentsView.svelte):
 * import { setContext } from "svelte";
 * import { DOCUMENTS_CTX } from "../features/documents/context";
 * setContext(DOCUMENTS_CTX, docState);
 *
 * @example
 * // In any descendant component:
 * import { getContext } from "svelte";
 * import { DOCUMENTS_CTX, type DocumentsContext } from "../context";
 * const docState = getContext<DocumentsContext>(DOCUMENTS_CTX);
 */
export const DOCUMENTS_CTX = Symbol("documents-state");
