import memoize from "lodash/memoize";
import { Account, AccountType } from "src/models/account";
import { Category } from "src/models/category";
import { Post } from "src/models/post";

/**
 * @returns String `/jobs/[category_slug]/[post_slug_without_id]_by_[poster_slug_without_id]_[post_id]`
 */

export const _getPostUrl = (
  post: Pick<Post, "slug" | "id">,
  category: Pick<Category, "slug">,
  poster: Pick<Account, "slug"> & AccountType
) => {
  const where = poster.type === "Company" ? "at" : "by";
  return `/jobs/${category.slug}/${[post.slug]}_${where}_${poster.slug}_${post.id}`;
};

export const getPostUrl = memoize(_getPostUrl);
