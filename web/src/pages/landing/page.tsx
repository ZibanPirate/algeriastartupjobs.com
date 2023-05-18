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
import { getStateActions } from "src/state";
import { Icon } from "src/components/icon";

export const Page: FC = () => {
  usePageTitle("Join a startup in Algeria");

  const { posts, query } = useSliceSelector("landingPage");

  useEffect(() => {
    fetchPostsForLandingPage();
  }, [query]);

  return (
    <Stack orientation="vertical" maxWidth={1600} margin="auto">
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
      <Stack orientation="vertical" stretch={true} align="stretch">
        <Stack orientation="horizontal" margin="1 1 2" gap="1" align="space-between">
          <Stack orientation="vertical" flex={1} />
          <Stack orientation="vertical" align="center" flex={4}>
            <GlobalSearch
              margin="0 1"
              value={query}
              setValue={(value) => getStateActions().landingPage.set({ query: value })}
            />
          </Stack>
          <Stack orientation="vertical" flex={1} align="end">
            <Button
              variant="v3"
              padding="rectangle-end"
              onClick={() => alert("Stay updated at github.com/algeriastartupjobs")}
              vtName="new-post"
            >
              <Icon variant="v3" name="newPost" />
              Free Post
            </Button>
          </Stack>
        </Stack>
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
          <Stack orientation="horizontal" gap="1" margin="0 1" align="stretch" animation={true}>
            {posts ? (
              posts.length > 0 ? (
                posts.map((post) => <PostCard key={post.id} post={post} />)
              ) : (
                <Text variant="v5" margin="1">
                  No posts found
                </Text>
              )
            ) : (
              "|"
                .repeat(4)
                .split("|")
                .map(() => <Skeleton variant="v3" width="20rem" maxWidth="80vw" height="6rem" />)
            )}
          </Stack>
        )}
      </Stack>
      {/* Footer */}
    </Stack>
  );
};
