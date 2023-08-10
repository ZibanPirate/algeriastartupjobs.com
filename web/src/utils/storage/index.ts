export type StorageKeys = "auth_token";

/**
 * Store a value in local storage. or remove it if value is null/undefined/empty string.
 */
export const setStorageItem = (key: StorageKeys, value: string) => {
  if (typeof value === "undefined" || value === null || value === "") localStorage.removeItem(key);
  else localStorage.setItem(key, value);
};

/**
 * Get a value from local storage. Returns undefined if value is not found/empty.
 */
export const getStorageItem = (key: StorageKeys): string | undefined => {
  const value = localStorage.getItem(key);
  if (typeof value === "undefined" || value === null || value === "") return undefined;
  return value;
};
