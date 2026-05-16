import { z } from "zod";

/**
 * Shared validation schemas for the application
 */

export const credentialSchema = z.object({
  site: z.string().min(1, "error_site_required"),
  username: z.string().min(1, "error_username_required"),
  password: z.string().min(1, "error_password_required"),
  recoveryCode: z.string().optional(),
  notes: z.string().optional()
});

export const pinSchema = z.object({
  name: z.string().min(1, "error_name_required"),
  pin: z.string().min(1, "error_pin_required"),
  notes: z.string().optional()
});

export const noteSchema = z.object({
  title: z.string().min(1, "error_title_required"),
  description: z.string().optional()
});

export type CredentialForm = z.infer<typeof credentialSchema>;
export type PinForm = z.infer<typeof pinSchema>;
export type NoteForm = z.infer<typeof noteSchema>;
