import { getState, getStateActions } from "src/state";
import { CompactPost, Post } from "src/models/post";
import { CompactTag } from "src/models/tag";
import { Account, CompactAccount } from "src/models/account";
import { PostPageState } from "./state";
import { isLoaded } from "src/utils/loadable";
import { TagEntity } from "src/state/entities/tag";
import { fetch } from "src/utils/fetch/fetch";
import * as Sentry from "@sentry/react";

export const fetchPostForPostPage = async (postId: string): Promise<void> => {
  const { postPage, postEntities, tagEntities, accountEntities } = getStateActions();
  const { post } = getState().postPage;
  if (post === "ERROR") postPage.set({ post: null });
  if (String(isLoaded(post)?.id) !== postId) postPage.set({ post: null });

  // load from cache if possible:
  const cachedPost = getState().postEntities.entities[postId];
  if (cachedPost) {
    const { tag_ids, poster_id, ...loneCachedPost } = cachedPost;

    const cachedPoster = getState().accountEntities.entities[poster_id];
    const cachedTags = tag_ids
      .map((tagId) => getState().tagEntities.entities[tagId])
      .filter((tag): tag is TagEntity => !!tag);

    postPage.set({ post: { ...loneCachedPost, tags: cachedTags, poster: cachedPoster } });
  }

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await fetch.get<{
      post: Post;
      tags: CompactTag[];
      poster: Account;
    }>("/posts/" + postId);

    const { tag_ids, poster_id, ...lonePost } = data.post;
    const post: PostPageState["post"] = {
      ...lonePost,
      tags: data.tags,
      poster: data.poster,
    };

    postPage.set({ post });

    // update cache:
    postEntities.upsertMany([data.post]);
    tagEntities.upsertMany(data.tags);
    accountEntities.upsertMany([data.poster]);
  } catch (error) {
    postPage.set({ post: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching post for post page", error);
    Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};

export const fetchSimilarPostsForPostPage = async (postId: string): Promise<void> => {
  const { postPage, postEntities, tagEntities, accountEntities } = getStateActions();

  postPage.set({ similarPosts: null });

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await fetch.get<{
      posts: CompactPost[];
      tags: CompactTag[];
      posters: CompactAccount[];
    }>("/posts/" + postId + "/similar");

    const similarPosts: PostPageState["similarPosts"] = data.posts.map((post) => {
      const { tag_ids, poster_id, ...lonePost } = post;

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
        tags,
        poster,
      };
    });

    if (getState().postPage.postId === postId) postPage.set({ similarPosts });

    // update cache:
    postEntities.upsertMany(data.posts);
    tagEntities.upsertMany(data.tags);
    accountEntities.upsertMany(data.posters);
  } catch (error) {
    postPage.set({ similarPosts: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching similar posts for post page", error);
    Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
