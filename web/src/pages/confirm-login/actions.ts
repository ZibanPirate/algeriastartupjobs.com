import { Account } from "src/models/account";
import { getState, getStateActions } from "src/state";
import { initialStateForConfirmLoginPage } from "./state";
import { initialStateForLoginPage } from "../login/state";
import { getBrowserRouter } from "src/components/router-provider";
import { ME_PAGE_URL } from "src/utils/urls/common";
import { authSave } from "src/utils/auth/save";
import { fetch } from "src/utils/fetch/fetch";

export const confirmLogin = async (): Promise<void> => {
  const { confirmLoginPage, loginPage, mePage } = getStateActions();
  confirmLoginPage.set({ confirmation_status: "CONFIRMING" });

  try {
    const { email, confirmation_id, confirmation_code } = getState().confirmLoginPage;

    const { data } = await fetch.post<{
      account: Account;
      auth_token: string;
    }>("/auth/confirm-login", {
      email,
      confirmation_id,
      confirmation_code,
    });

    confirmLoginPage.set({ ...initialStateForConfirmLoginPage, confirmation_status: "CONFIRMED" });
    loginPage.set(initialStateForLoginPage);

    const { account, auth_token } = data;
    authSave(auth_token);
    mePage.set({ account });
    getBrowserRouter().navigate(ME_PAGE_URL);
  } catch (error) {
    confirmLoginPage.set({ confirmation_status: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console
    console.log("Error confirming login", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
