export type UnionWithOptionalDiscriminatedProps<T, S> = Partial<Exclude<T, keyof S>> & S;
