import { defineConfig } from "vite";
import vitePluginFaviconsInject from "vite-plugin-favicons-inject";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    vitePluginFaviconsInject("./src/assets/svg/logo-square.svg"),
  ],
  publicDir: "dist",
  resolve: {
    alias: {
      src: "/src",
    },
  },
});
