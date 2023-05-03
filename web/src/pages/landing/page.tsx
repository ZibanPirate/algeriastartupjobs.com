import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { fetchPostsForLandingPage } from "./actions";
import { usePageTitle } from "src/utils/hooks/page-title";
import { GlobalSearch } from "src/components/search/global";

import { PostCard } from "src/components/card/post";
import { Button } from "src/components/button";
import { Skeleton } from "src/components/skeleton";

export const Page: FC = () => {
  usePageTitle("Join a startup in Algeria");

  const { posts } = useSliceSelector("landingPage");

  useEffect(() => {
    fetchPostsForLandingPage();
  }, []);

  return (
    <Stack orientation="vertical" maxWidth={1200} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" margin="3 0 0" stretch={true} align="center">
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
      {/* Global Search */}
      <Stack orientation="vertical" margin="1 0 2" stretch={true} align="center">
        <GlobalSearch margin="0 1" />
      </Stack>
      {/* Posts */}
      <Stack orientation="vertical" margin="0 0 3" stretch={true} align="center">
        {posts === "ERROR" ? (
          <Stack orientation="horizontal" align="baseline" margin="0 1">
            <Text variant="v5" margin="0 0 1">
              An error occurred while fetching posts, please &nbsp;
            </Text>
            <Button variant="v5" onClick={fetchPostsForLandingPage}>
              Try Again
            </Button>
          </Stack>
        ) : (
          <Stack orientation="horizontal" gap="1" margin="0 1" align="stretch">
            {posts
              ? posts.map((post) => <PostCard key={post.id} post={post} />)
              : "|"
                  .repeat(4)
                  .split("|")
                  .map(() => <Skeleton variant="v3" width="80vw" maxWidth={600} height="6rem" />)}
          </Stack>
        )}
      </Stack>
      {/* Footer */}
    </Stack>
  );
};
