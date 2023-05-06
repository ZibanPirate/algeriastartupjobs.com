import { FC, useEffect, useMemo } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { fetchPostForPostPage, fetchSimilarPostsForPostPage } from "./actions";
import { usePageTitle } from "src/utils/hooks/page-title";

import { isLoaded } from "src/utils/loadable";
import { useMatch } from "react-router-dom";
import { POST_PAGE_URL } from "src/utils/urls/common";
import { getPostLongTitle } from "src/utils/urls/post-long-title";
import { Button } from "src/components/button";
import { Tag } from "src/components/tag";
import { getAccountName } from "src/utils/models/acount-name";
import { Skeleton } from "src/components/skeleton";
import { PostCard } from "src/components/card/post";
import { getStateActions } from "src/state";
import { viewTransition } from "src/utils/animation/view-transition";
import { useComponentDidMount } from "src/utils/hooks/component-did-mount";

export const Page: FC = () => {
  const postSlug = useMatch(POST_PAGE_URL)?.params.postSlug;
  const postId = useMemo(() => (/(.*)_(\d+)$/.exec(postSlug || "") || [])[2], [postSlug]);

  const componentDidMount = useComponentDidMount();

  useEffect(() => {
    getStateActions().postPage.set({ postId });
    if (!postId) return;
    viewTransition(
      () => {
        fetchPostForPostPage(postId);
        fetchSimilarPostsForPostPage(postId);
      },
      { skip: !componentDidMount }
    );
  }, [postId]);

  const { post, similarPosts } = useSliceSelector("postPage");
  const loadedPost = isLoaded(post);
  usePageTitle(loadedPost ? getPostLongTitle(loadedPost, loadedPost.poster) : "Loading Job...");

  return (
    <Stack orientation="vertical" stretch align="center" maxWidth={1600} margin="auto">
      <Stack orientation="horizontal" align="start" stretch={true} gap="3">
        {post === "ERROR" ? (
          <Stack orientation="vertical" margin="3 0 0" flex={3} minWidth="60%">
            <Stack orientation="horizontal" align="baseline" margin="0 1">
              <Text variant="v5" margin="0 0 1">
                An error occurred while fetching post, please &nbsp;
              </Text>
              <Button variant="v5" onClick={() => fetchPostForPostPage(postId)}>
                Try Again
              </Button>
            </Stack>
          </Stack>
        ) : (
          <Stack orientation="vertical" margin="1 1 0" flex={3} minWidth="60%">
            {loadedPost ? (
              <Text variant="v3" margin="1 0" vtName={`post-title-${loadedPost?.id}`}>
                {loadedPost?.title}
              </Text>
            ) : (
              <Skeleton variant="v3" width="18rem" margin="0 0 1" />
            )}
            <Text variant="v4" vtName={`post-description-${loadedPost?.id}`}>
              {loadedPost?.description ? (
                <pre style={{ whiteSpace: "pre-line", maxWidth: 600 }}>
                  {loadedPost.description}
                </pre>
              ) : (
                <Stack orientation="vertical" gap="1">
                  <Skeleton variant="v4" width="20rem" />
                  <Skeleton variant="v4" width="20rem" />
                  <Skeleton variant="v4" width="16rem" />
                  <Skeleton variant="v4" width="20rem" />
                  <Skeleton variant="v4" width="4rem" />
                  <Skeleton variant="v4" width="20rem" />
                  <Skeleton variant="v4" width="18rem" />
                  <Skeleton variant="v4" width="16rem" />
                  <Skeleton variant="v4" width="20rem" />
                  <Skeleton variant="v4" width="4rem" />
                </Stack>
              )}
            </Text>
            {loadedPost?.tags && loadedPost?.tags.length > 0 && (
              <Stack
                orientation="horizontal"
                margin="2 0 1"
                gap="1"
                stretch={true}
                vtName={`post-tags-${loadedPost.id}`}
              >
                {loadedPost?.tags.map((tag) => (
                  <Tag variant="v4" key={tag.id}>
                    {tag.name}
                  </Tag>
                ))}
              </Stack>
            )}
            <Stack orientation="vertical" margin="1 0 0" gap="1" stretch={true}>
              {loadedPost?.poster ? (
                <Text variant="v3" vtName={`post-poster-${loadedPost?.id}`}>
                  {getAccountName(loadedPost.poster)}
                </Text>
              ) : (
                <Skeleton variant="v3" width="10rem" vtName={`post-poster-${loadedPost?.id}`} />
              )}
              {loadedPost?.category ? (
                <Text variant="v4" vtName={`post-category-${loadedPost?.id}`}>
                  {loadedPost.category.name}
                </Text>
              ) : (
                <Skeleton variant="v4" width="10rem" vtName={`post-category-${loadedPost?.id}`} />
              )}
            </Stack>
            <Stack orientation="horizontal" margin="3 0 0" align="center" gap="1">
              <Button
                variant="v3"
                onClick={() => alert("Stay updated at github.com/algeriastartupjobs")}
              >
                Apply
              </Button>
              <Text variant="v4">or</Text>
              <Link back={true} to="/" variant="v4">
                go Back
              </Link>
            </Stack>
          </Stack>
        )}
        <Stack orientation="vertical" margin="0 0 0" flex={2}>
          <Text variant="v3" margin="1">
            Similar Jobs
          </Text>
          {/* Posts */}
          <Stack orientation="vertical" margin="0 0 3">
            {similarPosts === "ERROR" ? (
              <Stack orientation="horizontal" align="baseline" margin="0 1">
                <Text variant="v5" margin="0 0 1">
                  An error occurred while fetching similar posts, please &nbsp;
                </Text>
                <Button variant="v5" onClick={() => fetchSimilarPostsForPostPage(postId)}>
                  Try Again
                </Button>
              </Stack>
            ) : similarPosts?.length === 0 ? (
              <Text variant="v4" margin="0 1">
                No similar posts found (Raha Nashfa)
              </Text>
            ) : (
              <Stack orientation="horizontal" gap="1" margin="0 1" align="stretch">
                {similarPosts
                  ? similarPosts.map((post) => <PostCard key={post.id} post={post} />)
                  : "|"
                      .repeat(4)
                      .split("|")
                      .map(() => (
                        <Skeleton variant="v3" width="20rem" maxWidth="80vw" height="6rem" />
                      ))}
              </Stack>
            )}
          </Stack>
        </Stack>
      </Stack>
      <Text variant="v4" margin="2 1">
        Source code is publicly available at&nbsp;
        <Link to="https://github.com/algeriastartupjobs/algeriastartupjobs.com" variant="v4">
          Github
        </Link>
      </Text>
    </Stack>
  );
};
