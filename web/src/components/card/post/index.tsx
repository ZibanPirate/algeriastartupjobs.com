import { FC } from "react";
import { Input } from "src/components/input";
import { Stack } from "src/components/stack";
import { StyleProps } from "src/utils/props/style";
import { LandingPageState } from "src/pages/landing/state";
import { Text } from "src/components/text";
import { LoneModel } from "src/utils/models/lone-model";
import { CompactPost } from "src/models/post";
import { CompactCategory } from "src/models/category";
import { CompactTag } from "src/models/tag";
import { CompactAccount } from "src/models/account";
import "./style.css";
import { Link } from "src/components/link";

export interface PostCardProps extends StyleProps {
  post: LoneModel<CompactPost> & {
    category: LoneModel<CompactCategory>;
    tags: Array<LoneModel<CompactTag>>;
    poster: LoneModel<CompactAccount>;
  };
  stretch?: boolean;
}

export const PostCard: FC<PostCardProps> = ({ margin, post, stretch }) => {
  return (
    <div className={`post-card${stretch ? " width100" : ""}`}>
      <Link variant="v4" to={"#"}>
        <Stack orientation="vertical" margin={margin}>
          <Text variant="v3">{post.title}</Text>
          <Text variant="v5">{post.short_description}</Text>
        </Stack>
      </Link>
    </div>
  );
};
