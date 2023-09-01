import { getState, getStateActions } from "src/state";
import { CompactPost } from "src/models/post";
import { CompactTag, Tag } from "src/models/tag";
import { CompactAccount } from "src/models/account";
import { fetch } from "src/utils/fetch/fetch";
import * as Sentry from "@sentry/react";
import { PostsForPageState } from "./state";

export const fetchPostsForTag = async (tagSlug: string): Promise<void> => {
  const { postsForPage, postEntities, tagEntities, accountEntities } = getStateActions();
  const { posts } = getState().postsForPage;
  if (posts === "ERROR") postsForPage.set({ posts: null });

  try {
    // @TODO-ZM: auto-generate types for API endpoints
    const { data } = await fetch.get<{
      tag: Tag;
      posts: CompactPost[];
      tags: CompactTag[];
      posters: CompactAccount[];
    }>(`/posts/for_tag/${tagSlug}`);
    const posts: PostsForPageState["posts"] = data.posts.map((post) => {
      const { tag_ids, poster_id, ...lonePost } = post;
      const tags = data.tags.filter((tag) => tag_ids.includes(tag.id));
      if (tags.length !== tag_ids.length)
        throw new Error(
          `Not all tags found for post ${post.id}: looking for ${tag_ids} but found ${tags.map(
            (tag) => tag.id
          )}}`
        );
      const poster = data.posters.find((poster) => poster.id === poster_id);
      if (!poster) throw new Error(`Poster with id ${poster_id} not found for post ${post.id}`);
      return {
        ...lonePost,
        tags,
        poster,
      };
    });

    postsForPage.set({ posts, tag: data.tag });

    // update cache:
    postEntities.upsertMany(data.posts);
    tagEntities.upsertMany(data.tags);
    accountEntities.upsertMany(data.posters);
  } catch (error) {
    postsForPage.set({ posts: "ERROR" });
    // @TODO-ZM: use Logger abstraction instead of console.log
    console.log("Error fetching posts for posts for tag", error);
    Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
