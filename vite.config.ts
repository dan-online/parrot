import { defineConfig } from "vite";
import WindiCSS from 'vite-plugin-windicss'
import Icons from 'unplugin-icons/vite'
import vue from "@vitejs/plugin-vue";

const mobile =
  process.env.TAURI_PLATFORM === "android" ||
  process.env.TAURI_PLATFORM === "ios";

export default defineConfig(async () => ({
  plugins: [Icons(), WindiCSS(), vue()],

  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
}));
