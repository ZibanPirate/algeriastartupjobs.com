import type { ForgeConfig } from "@electron-forge/shared-types";
import { MakerDeb } from "@electron-forge/maker-deb";
import { AutoUnpackNativesPlugin } from "@electron-forge/plugin-auto-unpack-natives";
import { WebpackPlugin } from "@electron-forge/plugin-webpack";

const config: ForgeConfig = {
  packagerConfig: { asar: true },
  rebuildConfig: {},
  makers: [new MakerDeb({})],
  plugins: [
    new AutoUnpackNativesPlugin({}),
    new WebpackPlugin({
      mainConfig: {
        entry: "./src/_entry/index.ts",
        module: {
          rules: [
            { test: /native_modules[/\\].+\.node$/, use: "node-loader" },
            {
              test: /[/\\]node_modules[/\\].+\.(m?js|node)$/,
              parser: { amd: false },
              use: {
                loader: "@vercel/webpack-asset-relocator-loader",
                options: { outputAssetBase: "native_modules" },
              },
            },
            {
              test: /\.tsx?$/,
              exclude: /(node_modules|\.webpack)/,
              use: { loader: "ts-loader", options: { transpileOnly: true } },
            },
          ],
        },
        resolve: { extensions: [".js", ".ts", ".jsx", ".tsx", ".css", ".json"] },
      },
      renderer: {
        config: {},
        entryPoints: [
          {
            html: "./src/_ignore/.html",
            js: "./src/_ignore/.ts",
            name: "ignore",
            preload: { js: "./src/_ignore/.ts" },
          },
        ],
      },
    }),
  ],
};

export default config;
