import { getStorageItem } from "../storage";

export const authVerify = () => {
  const authToken = getStorageItem("auth_token");
  if (authToken) return true;
  return false;
};
