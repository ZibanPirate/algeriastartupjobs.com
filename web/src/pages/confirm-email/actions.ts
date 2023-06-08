import Axios from "axios";
import { getState, getStateActions } from "src/state";
import { initialStateForConfirmEmailPage } from "./state";
import { getBrowserRouter } from "src/components/router-provider";
import { initialStateForCreatePostPage } from "../create-post/state";
import { Post } from "src/models/post";
import { CompactCategory } from "src/models/category";
import { Account } from "src/models/account";
import { CompactTag } from "src/models/tag";
import { getConfig } from "src/utils/config/get-config";
import { PostPageState } from "../post/state";
import { getPostUrl } from "src/utils/urls/post-url";

export const confirmEmail = async (): Promise<void> => {
  const {
    confirmEmailPage,
    createPostPage,
    postPage,
    postEntities,
    categoryEntities,
    tagEntities,
    accountEntities,
  } = getStateActions();
  confirmEmailPage.set({ confirmation_status: "CONFIRMING" });

  try {
    const { post_id, confirmation_id, confirmation_code } = getState().confirmEmailPage;

    const { data } = await Axios.post<{
      post: Post;
      category: CompactCategory;
      poster: Account;
      tags: CompactTag[];
    }>(getConfig().api.base_url + "/posts/confirm", {
      post_id,
      confirmation_id,
      confirmation_code,
    });

    confirmEmailPage.set({ ...initialStateForConfirmEmailPage, confirmation_status: "CONFIRMED" });
    createPostPage.set(initialStateForCreatePostPage);

    const { category_id, tag_ids, poster_id, ...lonePost } = data.post;
    const post: PostPageState["post"] = {
      ...lonePost,
      category: data.category,
      tags: data.tags,
      poster: data.poster,
    };

    postPage.set({ post });

    const postUrl = getPostUrl(data.post, data.category, data.poster);
    getBrowserRouter().navigate(postUrl);

    // update cache:
    postEntities.upsertMany([data.post]);
    categoryEntities.upsertMany([data.category]);
    tagEntities.upsertMany(data.tags);
    accountEntities.upsertMany([data.poster]);
  } catch (error) {
    confirmEmailPage.set({ confirmation_status: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console
    console.log("Error confirming email", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
