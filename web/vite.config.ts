import { sentryVitePlugin } from "@sentry/vite-plugin";
import { defineConfig } from "vite";
import vitePluginFaviconsInject from "vite-plugin-favicons-inject";
import react from "@vitejs/plugin-react-swc";
import { ViteEjsPlugin } from "vite-plugin-ejs";
import { qrcode } from "vite-plugin-qrcode";
import autoprefixer from "autoprefixer";
import { ViteImageOptimizer } from "vite-plugin-image-optimizer";
import ogPlugin from "vite-plugin-open-graph";

// https://vitejs.dev/config/
export default defineConfig(({ mode: stage }) => ({
  plugins: [
    react(),
    vitePluginFaviconsInject("./src/assets/svg/logo-square.svg"),
    ViteEjsPlugin(({ mode }) => ({ mode })),
    qrcode(),
    ViteImageOptimizer(),
    sentryVitePlugin({
      org: "algeria-startup-jobs-qh",
      project: "web",
    }),
    ogPlugin({
      basic: {
        title: "{{HTML_TITLE}}",
        description: "{{HTML_DESCRIPTION}}",
        image: "{{HTML_IMAGE}}",
      },
    }),
  ],
  base: stage === "development" ? "/" : `https://${stage}.assets.algeriastartupjobs.com/`,
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
  define: {
    APP_VERSION: JSON.stringify(require("./package.json").version),
  },
}));
