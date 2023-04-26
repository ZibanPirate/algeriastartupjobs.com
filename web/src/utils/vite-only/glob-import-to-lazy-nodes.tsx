import { ReactNode } from "react";
import { PreconfiguredLoadable } from "src/components/loadable-factory";

export const globImportToLazyNodes = (
  modules: Record<string, () => Promise<unknown>>,
  index = "index"
) => {
  const lazyNodes: Record<string, ReactNode> = {};

  Object.keys(modules).forEach((path) => {
    const importer = modules[path];
    const [, componentName] =
      path.match(new RegExp(`/(\\S+)\/${index}\\.tsx?$`)) ?? [];
    lazyNodes[componentName] = (
      <PreconfiguredLoadable path={componentName} importer={importer} />
    );
  });

  return lazyNodes;
};
