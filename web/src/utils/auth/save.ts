import { setStorageItem } from "../storage";
import { notifySubscribers } from "./subscribe";

export const authSave = (authToken: string) => {
  setStorageItem("auth_token", authToken);
  notifySubscribers();
};
