import { getState, getStateActions } from "src/state";
import Axios from "axios";
import { getConfig } from "src/utils/config/get-config";
import { CompactPost, Post } from "src/models/post";
import { CompactCategory } from "src/models/category";
import { CompactTag } from "src/models/tag";
import { CompactAccount } from "src/models/account";
import { PostPageState } from "./state";
import { isLoaded } from "src/utils/loadable";

export const fetchPostForPostPage = async (postId: string): Promise<void> => {
  const { postPage } = getStateActions();
  const { post } = getState().postPage;
  if (post === "ERROR") postPage.set({ post: null });
  if (String(isLoaded(post)?.id) !== postId) postPage.set({ post: null });

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await Axios.get<{
      post: Post;
      category: CompactCategory;
      tags: CompactTag[];
      poster: CompactAccount;
    }>(getConfig().api.base_url + "/posts/" + postId);

    const { category_id, tag_ids, poster_id, ...lonePost } = data.post;
    const post: PostPageState["post"] = {
      ...lonePost,
      category: data.category,
      tags: data.tags,
      poster: data.poster,
    };

    postPage.set({ post });
  } catch (error) {
    postPage.set({ post: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching post for post page", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
