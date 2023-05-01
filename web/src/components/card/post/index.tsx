import { FC } from "react";
import { Stack } from "src/components/stack";
import { StyleProps } from "src/utils/props/style";
import { Text } from "src/components/text";
import { LoneModel } from "src/utils/models/lone-model";
import { CompactPost } from "src/models/post";
import { CompactCategory } from "src/models/category";
import { CompactTag } from "src/models/tag";
import { AccountType, CompactAccount } from "src/models/account";
import "./style.css";
import { Link } from "src/components/link";
import { getAccountName } from "src/utils/models/acount-name";
import { Divider } from "src/components/divider";
import { Tag } from "src/components/tag";
import { getPostUrl } from "src/utils/urls/post-url";

export interface PostCardProps extends StyleProps {
  post: LoneModel<CompactPost> & {
    category: LoneModel<CompactCategory>;
    tags: Array<LoneModel<CompactTag>>;
    poster: LoneModel<CompactAccount> & AccountType;
  };
  stretch?: boolean;
}

export const PostCard: FC<PostCardProps> = ({ margin, post, stretch }) => {
  return (
    <div className={`post-card${stretch ? " width100" : ""}`}>
      <Link variant="v4" to={getPostUrl(post, post.category, post.poster)}>
        <Stack orientation="vertical" margin={margin}>
          <Text variant="v3" margin="0 0 1">
            {post.title}
          </Text>
          <Text variant="v5">{post.short_description}</Text>
          {post.tags.length > 0 && (
            <Stack orientation="horizontal" margin="1 0 0">
              {post.tags.slice(0, 3).map((tag) => (
                <Tag variant="v5" key={tag.id}>
                  {tag.name}
                </Tag>
              ))}
              {post.tags.length > 3 && (
                <Tag variant="v5" key={post.tags[3].id}>
                  +{post.tags.length - 3}
                </Tag>
              )}
            </Stack>
          )}
          <Stack orientation="horizontal" margin="1 0 0">
            <Text variant="v5">{getAccountName(post.poster)}</Text>
            <Divider margin="0 1" orientation="vertical" />
            <Text variant="v5">{post.category.name}</Text>
          </Stack>
        </Stack>
      </Link>
    </div>
  );
};
