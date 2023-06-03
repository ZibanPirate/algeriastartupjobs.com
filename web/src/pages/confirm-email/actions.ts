import { getStateActions } from "src/state";
import { initialStateForConfirmEmailPage } from "./state";
import { getBrowserRouter } from "src/components/router-provider";
import { initialStateForCreatePostPage } from "../create-post/state";

export const confirmEmail = async (): Promise<void> => {
  const { confirmEmailPage, createPostPage } = getStateActions();
  confirmEmailPage.set({ confirmation_status: "CONFIRMING" });

  // @TODO-ZM: Call backend, get post id, and render a link to the post
  await new Promise((resolve) => setTimeout(resolve, 1000));
  // confirmEmailPage.set({ ...initialStateForConfirmEmailPage, confirmation_status: "ERROR" });

  confirmEmailPage.set({ ...initialStateForConfirmEmailPage, confirmation_status: "CONFIRMED" });
  createPostPage.set(initialStateForCreatePostPage);

  // @TODO-ZM: generate post url and navigate to it
  getBrowserRouter().navigate("/jobs/itaque-corporis/software-developer-0_at_deckow-inc_0");
};
