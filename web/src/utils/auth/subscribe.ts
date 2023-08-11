import { setStorageItem } from "../storage";

let authSubscribers: Array<() => void> = [];

export const authSubscribe = (callback: () => void) => {
  authSubscribers.push(callback);

  return () => {
    authSubscribers = authSubscribers.filter((subscriber) => subscriber !== callback);
  };
};

export const notifySubscribers = () => {
  authSubscribers.forEach((subscriber) => subscriber());
};
