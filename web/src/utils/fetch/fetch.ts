import axios from "axios";
import { getConfig } from "src/utils/config/get-config";
import { authSubscribe } from "src/utils/auth/subscribe";
import { authExtract } from "src/utils/auth/extract";
import { authVerify } from "src/utils/auth/verify";
import { authRevoke } from "../auth/revoke";
import { authRefresh } from "../auth/refresh";

export const fetch = axios.create({
  baseURL: getConfig().api.base_url,
});

const AUTH_TOKEN_EXPIRATION_MINUTES = 5;
const REFRESH_AUTH_TOKEN_INTERVAL = (AUTH_TOKEN_EXPIRATION_MINUTES - 1) * 60 * 1000;

let refreshIntervalId: NodeJS.Timeout;

const setupAuth = () => {
  clearInterval(refreshIntervalId);

  if (authVerify()) {
    const auth_token = authExtract();
    fetch.defaults.headers.common["Authorization"] = `Bearer ${auth_token}`;
    refreshIntervalId = setInterval(authRefresh, REFRESH_AUTH_TOKEN_INTERVAL);
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
if (authVerify()) authRefresh();
