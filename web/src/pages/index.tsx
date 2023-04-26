import { globImportToLazyNodes } from "src/utils/vite-only/glob-import-to-lazy-nodes";

const componentModules = import.meta.glob("./*/page.tsx", {
  import: "Page",
});

// @TODO-ZM: nice to have static types for LazyPages properties, Vite plugin?
export const LazyPages = globImportToLazyNodes(componentModules, "page");
