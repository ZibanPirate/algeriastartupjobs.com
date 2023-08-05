import Axios from "axios";
import { Account } from "src/models/account";
import { getState, getStateActions } from "src/state";
import { getConfig } from "src/utils/config/get-config";
import { initialStateForCreatePostPage } from "./state";
import { getBrowserRouter } from "src/components/router-provider";
import { CONFIRM_EMAIL_PAGE_URL } from "src/utils/urls/common";
import { initialStateForConfirmEmailPage } from "../confirm-email/state";
import { Post } from "src/models/post";
import { CompactTag } from "src/models/tag";
import { onceAtATime } from "src/utils/concurrency/once-at-a-time";

export const fetchAccountForCreatePostPage = async (): Promise<void> => {
  const { poster_contact } = getState().createPostPage;
  if (!poster_contact) return;

  const { accountEntities, createPostPage } = getStateActions();

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

const concurrentFetchTagsForCreatePostPage = async (): Promise<void> => {
  const { compact, title, post_description } = getState().createPostPage;
  const { tagEntities, createPostPage } = getStateActions();

  if (!post_description) {
    createPostPage.set({ suggested_tags: [] });
    if (!title) createPostPage.set({ suggested_categories: [] });
  }
  if (compact || !post_description) return;

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await Axios.post<{
      tags: CompactTag[];
    }>(getConfig().api.base_url + "/tags/suggestions_for_post", {
      description: post_description,
      title,
    });

    createPostPage.set({ suggested_tags: data.tags });

    // update cache:
    tagEntities.upsertMany(data.tags);
  } catch (error) {
    // @TODO-ZM: set it to null when status is 404
    createPostPage.set({ suggested_tags: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching suggested tags for create post page", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};

export const fetchTagsForCreatePostPage = onceAtATime(concurrentFetchTagsForCreatePostPage);

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
      tags = [],
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
        tag_ids: tags.map((tag) => tag.id),
        is_confirmed: false,
      } as Omit<Post, "id">,
    });

    createPostPage.overwrite({ ...initialStateForCreatePostPage, creation_status: "CREATED" });
    confirmEmailPage.set({ ...initialStateForConfirmEmailPage, confirmation_id, post_id });
    getBrowserRouter().navigate(CONFIRM_EMAIL_PAGE_URL);
  } catch (error) {
    console.log("Error creating post", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
    // @TODO-ZM: skip when 429
    createPostPage.set({ creation_status: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console
  }
};
