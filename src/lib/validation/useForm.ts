import { z, type ZodSchema } from "zod";

/**
 * A Svelte 5 rune-based form handler with Zod validation
 */
export function useForm<T extends Record<string, any>>(
  schema: ZodSchema<T>,
  initialValues: T,
  onSubmit: (values: T) => Promise<void>
) {
  let values = $state<T>({ ...initialValues });
  let errors = $state<Partial<Record<keyof T, string>>>({});
  let loading = $state(false);
  let globalError = $state("");

  async function handleSubmit() {
    globalError = "";
    errors = {};
    
    const result = schema.safeParse(values);
    
    if (!result.success) {
      const formattedErrors: any = {};
      result.error.issues.forEach((issue) => {
        const path = issue.path[0] as keyof T;
        if (!formattedErrors[path]) {
          formattedErrors[path] = issue.message;
        }
      });
      errors = formattedErrors;
      return;
    }

    loading = true;
    try {
      await onSubmit(result.data);
    } catch (e: any) {
      globalError = e.message || e.toString();
    } finally {
      loading = false;
    }
  }

  function reset() {
    values = { ...initialValues };
    errors = {};
    globalError = "";
  }

  return {
    get values() { return values; },
    get errors() { return errors; },
    get loading() { return loading; },
    get globalError() { return globalError; },
    handleSubmit,
    reset
  };
}
