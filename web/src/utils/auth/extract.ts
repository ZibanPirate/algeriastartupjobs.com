import { getStorageItem } from "../storage";

export const authExtract = () => {
  return getStorageItem("auth_token");
};
