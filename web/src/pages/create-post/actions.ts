import Axios from "axios";
import { Account } from "src/models/account";
import { getState, getStateActions } from "src/state";
import { getConfig } from "src/utils/config/get-config";
import { initialStateForCreatePostPage } from "./state";
import { getBrowserRouter } from "src/components/router-provider";
import { CONFIRM_EMAIL_PAGE_URL } from "src/utils/urls/common";
import { initialStateForConfirmEmailPage } from "../confirm-email/state";
import { Post } from "src/models/post";

export const fetchAccountForCreatePostPage = async (): Promise<void> => {
  const { accountEntities, createPostPage } = getStateActions();
  const { poster_contact } = getState().createPostPage;

  if (!poster_contact) return;

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await Axios.get<{
      account: Account;
    }>(getConfig().api.base_url + "/account/by_email?email=" + encodeURIComponent(poster_contact));

    createPostPage.set({ poster: data.account });

    // update cache:
    accountEntities.upsertMany([data.account]);
  } catch (error) {
    // @TODO-ZM: set it to null when status is 404
    createPostPage.set({ poster: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching posts for landing page", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};

export const createPost = async (): Promise<void> => {
  const { createPostPage, confirmEmailPage } = getStateActions();
  createPostPage.set({ creation_status: "CREATING" });

  try {
    const {
      title,
      poster_type,
      poster_name,
      poster_first_name,
      poster_last_name,
      poster_contact,
      post_description = "",
    } = getState().createPostPage;

    const {
      data: { confirmation_id, post_id },
    } = await Axios.post<{
      post_id: number;
      poster_id: number;
      confirmation_id: string;
    }>(getConfig().api.base_url + "/posts", {
      poster: {
        email: poster_contact,
        slug: "",
        type: poster_type,
        ...(poster_type === "Company"
          ? {
              company_name: poster_name,
            }
          : {
              first_name: poster_first_name,
              last_name: poster_last_name,
            }),
      } as Omit<Account, "id">,
      post: {
        title,
        slug: "",
        short_description: "",
        description: post_description,
        poster_id: 0,
        category_id: 0,
        tag_ids: [],
        is_confirmed: false,
      } as Omit<Post, "id">,
    });

    createPostPage.overwrite({ ...initialStateForCreatePostPage, creation_status: "CREATED" });
    confirmEmailPage.set({ ...initialStateForConfirmEmailPage, confirmation_id, post_id });
    getBrowserRouter().navigate(CONFIRM_EMAIL_PAGE_URL);
  } catch (error) {
    createPostPage.set({ creation_status: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console
    console.log("Error creating post", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
