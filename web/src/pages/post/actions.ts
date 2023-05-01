import { getState, getStateActions } from "src/state";
import Axios from "axios";
import { getConfig } from "src/utils/config/get-config";
import { CompactPost } from "src/models/post";
import { CompactCategory } from "src/models/category";
import { CompactTag } from "src/models/tag";
import { CompactAccount } from "src/models/account";
import { PostPageState } from "./state";

export const fetchPostForPostPage = async (postId: string): Promise<void> => {
  const { postPage } = getStateActions();
  const { post } = getState().postPage;
  if (post === "ERROR") postPage.set({ post: null });

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await Axios.get<{
      post: CompactPost[];
      categories: CompactCategory[];
      tags: CompactTag[];
      posters: CompactAccount[];
    }>(getConfig().api.base_url + "/posts/feed");

    // const post: PostPageState["post"] = data.post.map((post) => {
    //   const category = data.categories.find((category) => category.id === post.category_id);
    //   if (!category)
    //     throw new Error(`Category with id ${post.category_id} not found for post ${post.id}`);

    //   const tags = data.tags.filter((tag) => post.tag_ids.includes(tag.id));
    //   if (tags.length !== post.tag_ids.length)
    //     throw new Error(
    //       `Not all tags found for post ${post.id}: looking for ${post.tag_ids} but found ${tags.map(
    //         (tag) => tag.id
    //       )}}`
    //     );

    //   const poster = data.posters.find((poster) => poster.id === post.poster_id);
    //   if (!poster)
    //     throw new Error(`Poster with id ${post.poster_id} not found for post ${post.id}`);

    //   return {
    //     id: post.id,
    //     slug: post.slug,
    //     title: post.title,
    //     short_description: post.short_description,
    //     category,
    //     tags,
    //     poster,
    //   };
    // });

    // postPage.set({ post });
  } catch (error) {
    postPage.set({ post: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching post for post page", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
