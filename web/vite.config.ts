import { defineConfig } from "vite";
import vitePluginFaviconsInject from "vite-plugin-favicons-inject";
import react from "@vitejs/plugin-react-swc";
import { ViteEjsPlugin } from "vite-plugin-ejs";
import { qrcode } from "vite-plugin-qrcode";
import autoprefixer from "autoprefixer";
import { ViteImageOptimizer } from "vite-plugin-image-optimizer";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    vitePluginFaviconsInject("./src/assets/svg/logo-square.svg"),
    ViteEjsPlugin(({ mode }) => ({ mode })),
    qrcode(),
    ViteImageOptimizer(),
  ],
  publicDir: "dist",
  resolve: {
    alias: {
      src: "/src",
    },
  },
  server: {
    host: true,
  },
  css: {
    postcss: {
      plugins: [autoprefixer()],
    },
  },
});
