import { FC, useEffect, useMemo } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { fetchPostForPostPage } from "./actions";
import { usePageTitle } from "src/utils/hooks/page-title";

import { isLoaded } from "src/utils/loadable";
import { useMatch } from "react-router-dom";
import { POST_PAGE_URL } from "src/utils/urls/common";

export const Page: FC = () => {
  const postSlug = useMatch(POST_PAGE_URL)?.params.postSlug;
  const postId = useMemo(() => (/(.*)_(\d+)$/.exec(postSlug || "") || [])[2], [postSlug]);

  useEffect(() => {
    if (!postId) return;
    fetchPostForPostPage(postId);
  }, [postId]);

  const { post } = useSliceSelector("postPage");
  usePageTitle(isLoaded(post)?.title || "Loading Job...");

  return (
    <Stack orientation="vertical">
      <pre>{JSON.stringify({ postId, post }, null, 2)}</pre>
      {/* Header */}
      <Stack orientation="vertical" margin="1 0 0" stretch={true} align="center">
        <Text variant="v1" margin="0 1">
          Join a startup in Algeria
        </Text>
        <Text variant="v4" margin="1 1">
          Source code is publicly available at&nbsp;
          <Link to="https://github.com/algeriastartupjobs/algeriastartupjobs.com" variant="v4">
            Github
          </Link>
        </Text>
      </Stack>
    </Stack>
  );
};
