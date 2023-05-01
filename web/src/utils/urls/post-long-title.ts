import memoize from "lodash/memoize";
import { AccountType } from "src/models/account";
import { Post } from "src/models/post";

export const _getPostLongTitle = (post: Pick<Post, "title">, poster?: AccountType) => {
  const posterExtension = poster
    ? " needed " +
      (poster.type === "Company"
        ? `at ${poster.company_name}`
        : `by ${poster.first_name} ${poster.last_name}`)
    : "";

  return `${post.title}${posterExtension}`;
};

export const getPostLongTitle = memoize(_getPostLongTitle);
