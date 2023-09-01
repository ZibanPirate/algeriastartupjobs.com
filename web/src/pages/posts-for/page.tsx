import { FC, useEffect } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { usePageTitle } from "src/utils/hooks/page-title";
import { PostCard } from "src/components/card/post";
import { Button } from "src/components/button";
import { Skeleton } from "src/components/skeleton";
import { Icon } from "src/components/icon";
import {
  CREATE_POST_PAGE_URL,
  JOBS_FOR_PAGE_URL,
  LOGIN_PAGE_URL,
  ME_PAGE_URL,
  POST_PAGE_URL,
} from "src/utils/urls/common";
import { useMatch, useNavigate } from "react-router-dom";
import { Divider } from "src/components/divider";
import { useIsAuthenticated } from "src/utils/hooks/is-authenticated";
import { Footer } from "src/components/footer";
import { fetchPostsForTag } from "./actions";
import { isLoaded } from "src/utils/loadable";

export const Page: FC = () => {
  const navigate = useNavigate();

  const { posts, tag } = useSliceSelector("postsForPage");
  const { isAuthenticated } = useIsAuthenticated();

  const tagSlug = useMatch(JOBS_FOR_PAGE_URL)?.params.tagSlug;

  useEffect(() => {
    if (!tagSlug) return;

    fetchPostsForTag(tagSlug);
  }, [tagSlug]);

  const loadedTag = isLoaded(tag);

  usePageTitle(`Startup Jobs for ${loadedTag?.name} in Algeria`, { enabled: !!loadedTag });

  return (
    <Stack
      orientation="vertical"
      fullWidth
      align="center"
      minHeight="100vh"
      justifyContent="space-between"
    >
      <Stack orientation="vertical" fullWidth maxWidth={1600} margin="auto">
        <Stack orientation="vertical" stretch={true} align="center" padding="1 1 0">
          <Stack orientation="vertical" stretch={true} align="end">
            <Stack orientation="horizontal" align="center" gap="1" stretch={true}>
              <Stack orientation="vertical" align="start" flex={1}>
                <Link variant="v4" back={POST_PAGE_URL} to={"/"} vtName="back">
                  <Icon variant="v4" name="back" /> Back
                </Link>
              </Stack>
              <Link to={isAuthenticated ? ME_PAGE_URL : LOGIN_PAGE_URL} variant="v4" vtName="login">
                {isAuthenticated ? "My Account" : "Login"}
              </Link>
              <Divider orientation="vertical" />
              <Button
                variant="v3"
                paddingPreset="rectangle-end"
                onClick={() => navigate(CREATE_POST_PAGE_URL)}
                vtName="new-post"
              >
                <Icon variant="v3" name="newPost" />
                Free Post
              </Button>
            </Stack>
          </Stack>
          <Text variant="v2" margin="2 0">
            Startup jobs{loadedTag ? ` for ${loadedTag.name} ` : " "}in Algeria
          </Text>
        </Stack>
        <Stack orientation="vertical" stretch={true} align="center">
          {posts === "ERROR" ? (
            <Stack orientation="horizontal" align="baseline" margin="0 1">
              <Text variant="v5" margin="0 0 1">
                An error occurred while fetching posts, please &nbsp;
              </Text>
              <Button variant="v5" onClick={() => !!tagSlug && fetchPostsForTag(tagSlug)}>
                Try Again
              </Button>
            </Stack>
          ) : (
            <Stack
              orientation="horizontal"
              gap="1"
              margin="0 1"
              align="stretch"
              animation={!!posts?.length}
            >
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
      </Stack>
      <Footer />
    </Stack>
  );
};
