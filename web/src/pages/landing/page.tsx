import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { fetchPostCountForLandingPage, fetchPostsForLandingPage } from "./actions";
import { usePageTitle } from "src/utils/hooks/page-title";
import { GlobalSearch } from "src/components/search/global";

import { PostCard } from "src/components/card/post";
import { Button } from "src/components/button";
import { Skeleton } from "src/components/skeleton";
import { getStateActions } from "src/state";
import { Icon } from "src/components/icon";
import { CREATE_POST_PAGE_URL, LOGIN_PAGE_URL } from "src/utils/urls/common";
import { useNavigate } from "react-router-dom";
import { Divider } from "src/components/divider";

export const Page: FC = () => {
  usePageTitle("Join a startup in Algeria");
  const navigate = useNavigate();

  const { posts, query, total_post_count } = useSliceSelector("landingPage");
  const { set } = getStateActions().landingPage;

  useEffect(() => {
    fetchPostsForLandingPage();
  }, [query]);

  useEffect(() => {
    fetchPostCountForLandingPage();
  }, []);

  return (
    <Stack orientation="vertical" fullWidth maxWidth={1600} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" margin="1 0 0" stretch={true} align="center">
        <Stack orientation="vertical" stretch={true} align="end">
          <Stack orientation="horizontal" align="center" margin="0 1">
            <Link to={LOGIN_PAGE_URL} variant="v4" vtName="login">
              Login
            </Link>
            <Divider orientation="vertical" margin="0 1" />
            <Button
              variant="v3"
              paddingPreset="rectangle-end"
              margin="0 1"
              onClick={() => navigate(CREATE_POST_PAGE_URL)}
              vtName="new-post"
            >
              <Icon variant="v3" name="newPost" />
              Free Post
            </Button>
          </Stack>
        </Stack>
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
        <GlobalSearch
          margin="0 1"
          total_post_count={total_post_count}
          value={query}
          setValue={(value) => set({ query: value })}
          onClick={fetchPostsForLandingPage}
        />
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
