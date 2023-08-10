// import Axios from "axios";
import { getState, getStateActions } from "src/state";
// import { getConfig } from "src/utils/config/get-config";
import { initialStateForLoginPage } from "./state";
import { getBrowserRouter } from "src/components/router-provider";
import { CONFIRM_LOGIN_PAGE_URL } from "src/utils/urls/common";

export const login = async () => {
  const { loginPage } = getStateActions();
  loginPage.set({ login_status: "LOGGING_IN" });

  try {
    const { email } = getState().loginPage;

    await new Promise((resolve) => setTimeout(resolve, 1000));
    // const {
    //   data: { confirmation_id },
    // } = await Axios.post<{ confirmation_id: string }>(getConfig().api.base_url + "/auth/login", {
    //   email,
    // });

    // @TODO-ZM: fix typing on overwrite reducer.
    loginPage.overwrite({ ...initialStateForLoginPage, login_status: "CODE_SENT" });
    //   confirmEmailPage.set({ ...initialStateForConfirmEmailPage, confirmation_id, post_id });
    getBrowserRouter().navigate(CONFIRM_LOGIN_PAGE_URL);
  } catch (error) {
    console.log("Error creating post", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
    loginPage.set({ login_status: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console
  }
};
