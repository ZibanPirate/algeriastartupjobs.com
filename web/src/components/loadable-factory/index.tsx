import loadable from "@loadable/component";

export const PreconfiguredLoadable = loadable<{
  path: string;
  importer: () => Promise<any>;
}>(({ importer }) => importer(), { cacheKey: ({ path }) => path });
