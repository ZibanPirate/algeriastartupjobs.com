import axios from "axios";
import { getConfig } from "src/utils/config/get-config";
import { authSubscribe } from "src/utils/auth/subscribe";
import { authExtract } from "src/utils/auth/extract";
import { authVerify } from "src/utils/auth/verify";
import { authRevoke } from "../auth/revoke";

const fetch = axios.create({
  baseURL: getConfig().api.base_url,
});

const setupAuth = () => {
  // @TODO: Add refresh token logic
  if (authVerify()) {
    const auth_token = authExtract();
    fetch.defaults.headers.common["Authorization"] = `Bearer ${auth_token}`;
  } else {
    delete fetch.defaults.headers.common["Authorization"];
  }
};

fetch.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response.status === 401) authRevoke();
    return Promise.reject(error);
  }
);

authSubscribe(setupAuth);
setupAuth();

export { fetch };
