export type LoneModel<T> = Omit<T, `${string}_${"id" | "ids"}`>;
