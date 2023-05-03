import { getState, getStateActions } from "src/state";
import Axios from "axios";
import { getConfig } from "src/utils/config/get-config";
import { CompactPost, Post } from "src/models/post";
import { CompactCategory } from "src/models/category";
import { CompactTag, Tag } from "src/models/tag";
import { CompactAccount } from "src/models/account";
import { PostPageState } from "./state";
import { isLoaded } from "src/utils/loadable";
import { TagEntity } from "src/state/entities/tag";

export const fetchPostForPostPage = async (postId: string): Promise<void> => {
  const { postPage, postEntities, categoryEntities, tagEntities, accountEntities } =
    getStateActions();
  const { post } = getState().postPage;
  if (post === "ERROR") postPage.set({ post: null });
  if (String(isLoaded(post)?.id) !== postId) postPage.set({ post: null });

  // load from cache if possible:
  const cachedPost = getState().postEntities.entities[postId];
  if (cachedPost) {
    const { category_id, tag_ids, poster_id, ...loneCachedPost } = cachedPost;

    const cachedPoster = getState().accountEntities.entities[poster_id];
    const cachedCategory = getState().categoryEntities.entities[category_id];
    const cachedTags = tag_ids
      .map((tagId) => getState().tagEntities.entities[tagId])
      .filter((tag): tag is TagEntity => !!tag);

    postPage.set({
      post: { ...loneCachedPost, tags: cachedTags, category: cachedCategory, poster: cachedPoster },
    });
  }

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

    // update cache:
    postEntities.upsertMany([data.post]);
    categoryEntities.upsertMany([data.category]);
    tagEntities.upsertMany(data.tags);
    accountEntities.upsertMany([data.poster]);
  } catch (error) {
    postPage.set({ post: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching post for post page", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};

export const fetchSimilarPostsForPostPage = async (postId: string): Promise<void> => {
  const { postPage, postEntities, categoryEntities, tagEntities, accountEntities } =
    getStateActions();

  postPage.set({ similarPosts: null });

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await Axios.get<{
      posts: CompactPost[];
      categories: CompactCategory[];
      tags: CompactTag[];
      posters: CompactAccount[];
    }>(getConfig().api.base_url + "/posts/" + postId + "/similar");

    const similarPosts: PostPageState["similarPosts"] = data.posts.map((post) => {
      const { category_id, tag_ids, poster_id, ...lonePost } = post;

      const category = data.categories.find((category) => category.id === category_id);
      if (!category)
        throw new Error(`Category with id ${category_id} not found for post ${post.id}`);

      const tags = data.tags.filter((tag) => tag_ids.includes(tag.id));
      if (tags.length !== tag_ids.length)
        throw new Error(
          `Not all tags with ids ${tag_ids} found for post ${post.id}. Found tags: ${tags.map(
            (tag) => tag.id
          )}`
        );

      const poster = data.posters.find((poster) => poster.id === poster_id);
      if (!poster) throw new Error(`Poster with id ${poster_id} not found for post ${post.id}`);

      return {
        ...lonePost,
        category,
        tags,
        poster,
      };
    });

    if (getState().postPage.postId === postId) postPage.set({ similarPosts });

    // update cache:
    postEntities.upsertMany(data.posts);
    categoryEntities.upsertMany(data.categories);
    tagEntities.upsertMany(data.tags);
    accountEntities.upsertMany(data.posters);
  } catch (error) {
    postPage.set({ similarPosts: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching similar posts for post page", error);
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
