import { defineConfig } from "vite";
import path from "path";
import react from "@vitejs/plugin-react";
import viteCompression from "vite-plugin-compression";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    react(),
    viteCompression({ algorithm: 'brotliCompress' }) // Compress assets for faster load
  ],

  // Prevent Vite from hiding Rust errors
  clearScreen: false,

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  server: {
    port: 1420,  // Use a fixed port
    strictPort: true, // Fail if port is unavailable
    host: host || false, // Allow for remote debugging if TAURI_DEV_HOST is set
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421, // HMR runs on a separate port
      }
      : undefined,
    watch: {
      // Ignore changes inside src-tauri to avoid unnecessary reloads
      ignored: ["**/src-tauri/**"],
    },
  },

  build: {
    target: "esnext", // Use modern JavaScript features
    minify: "terser", // Use Terser for advanced minification
    cssCodeSplit: true, // Split CSS for better caching
    terserOptions: {
      compress: {
        drop_console: true, // Remove console logs in production
        drop_debugger: true, // Remove debugger statements
      },
      format: {
        comments: false, // Remove comments from final build
      },
    },
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes("node_modules")) {
            return id
              .toString()
              .split("node_modules/")[1]
              .split("/")[0]; // Split dependencies into separate chunks
          }
        },
      },
    },
  },

  esbuild: {
    minifyIdentifiers: true, // Shorten variable names
    minifySyntax: true, // Optimize syntax
    minifyWhitespace: true, // Remove whitespace
  },
}));
