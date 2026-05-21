import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import pkg from "./package.json";

// process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    tailwindcss(),
    sveltekit(),
  ],
  define: {
    "import.meta.env.APP_VERSION": JSON.stringify(pkg.version),
    "import.meta.env.APP_STATUS": JSON.stringify(pkg.status),
  },

  test: {
    include: ["src/**/*.{test,spec}.{js,ts}"],
    environment: "jsdom",
    globals: true,
    setupFiles: ["./vitest-setup.ts"],
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
      lines: 80,
      functions: 80,
      branches: 80,
      statements: 80,
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
