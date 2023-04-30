import { getState, getStateActions } from "src/state";
import Axios from "axios";
import { getConfig } from "src/utils/config/get-config";
import { CompactPost } from "src/models/post";
import { CompactCategory } from "src/models/category";
import { CompactTag } from "src/models/tag";
import { CompactAccount } from "src/models/account";
import { LandingPageState } from "./state";

export const fetchPostsForLandingPage = async (): Promise<void> => {
  const { landingPage } = getStateActions();
  const { posts } = getState().landingPage;
  if (posts === "ERROR") landingPage.set({ posts: null });

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await Axios.get<{
      posts: CompactPost[];
      categories: CompactCategory[];
      tags: CompactTag[];
      posters: CompactAccount[];
    }>(getConfig().api.base_url + "/posts/feed");

    const posts: LandingPageState["posts"] = data.posts.map((post) => {
      const category = data.categories.find((category) => category.id === post.category_id);
      if (!category)
        throw new Error(`Category with id ${post.category_id} not found for post ${post.id}`);

      const tags = data.tags.filter((tag) => post.tag_ids.includes(tag.id));
      if (tags.length !== post.tag_ids.length)
        throw new Error(
          `Not all tags found for post ${post.id}: looking for ${post.tag_ids} but found ${tags.map(
            (tag) => tag.id
          )}}`
        );

      const poster = data.posters.find((poster) => poster.id === post.poster_id);
      if (!poster)
        throw new Error(`Poster with id ${post.poster_id} not found for post ${post.id}`);

      return {
        id: post.id,
        slug: post.slug,
        title: post.title,
        short_description: post.short_description,
        category,
        tags,
        poster,
      };
    });

    landingPage.set({ posts });
  } catch (error) {
    landingPage.set({ posts: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching posts for landing page", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
