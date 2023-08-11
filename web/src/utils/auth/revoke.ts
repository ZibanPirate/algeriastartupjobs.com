import { setStorageItem } from "../storage";
import { notifySubscribers } from "./subscribe";

export const authRevoke = () => {
  setStorageItem("auth_token", null);
  notifySubscribers();
};
