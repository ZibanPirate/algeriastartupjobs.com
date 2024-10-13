import { FC, useEffect, useMemo } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { useSliceSelector } from "src/utils/state/selector";
import { deletePost, fetchPostForPostPage, fetchSimilarPostsForPostPage } from "./actions";
import { usePageTitle } from "src/utils/hooks/page-title";

import { isLoaded } from "src/utils/loadable";
import { useMatch, useNavigate } from "react-router-dom";
import { CREATE_POST_PAGE_URL, JOBS_FOR_PAGE_URL, POST_PAGE_URL } from "src/utils/urls/common";
import { getPostLongTitle } from "src/utils/urls/post-long-title";
import { Button } from "src/components/button";
import { Tag } from "src/components/tag";
import { getAccountName } from "src/utils/models/account-name";
import { Skeleton } from "src/components/skeleton";
import { PostCard } from "src/components/card/post";
import { getStateActions } from "src/state";
import { GlobalSearch } from "src/components/search/global";
import { Icon } from "src/components/icon";
import { fetchPostCountForLandingPage, fetchPostsForLandingPage } from "src/pages/landing/actions";
import { useMediaQuery } from "src/utils/hooks/use-media-query";
import { Footer } from "src/components/footer";
import { Divider } from "src/components/divider";
import { fetchAccountForMePage } from "../me/actions";
import { useIsAuthenticated } from "src/utils/hooks/is-authenticated";
import { Dialog, useDialogProps } from "src/components/dialog";

export const Page: FC = () => {
  const postSlug = useMatch(POST_PAGE_URL)?.params.postSlug;
  const postId = useMemo(() => (/(.*)_(\d+)$/.exec(postSlug || "") || [])[2], [postSlug]);
  const { query, total_post_count } = useSliceSelector("landingPage");
  const { account } = useSliceSelector("mePage");
  const navigate = useNavigate();
  const { landingPage, postsForPage, postPage } = getStateActions();

  useEffect(() => {
    fetchPostsForLandingPage();
  }, [query]);

  useEffect(() => {
    fetchPostCountForLandingPage();
  }, []);

  useEffect(() => {
    getStateActions().postPage.set({ postId });
    if (!postId) return;
    fetchPostForPostPage(postId);
    fetchSimilarPostsForPostPage(postId);
  }, [postId]);

  const { post, similarPosts, deletion_status } = useSliceSelector("postPage");
  const loadedPost = isLoaded(post);
  usePageTitle(loadedPost ? getPostLongTitle(loadedPost, loadedPost.poster) : "Loading Job...", {
    enabled: !!loadedPost,
  });
  const isSmallScreen = useMediaQuery("(max-width: 700px)");

  const { isAuthenticated } = useIsAuthenticated();
  useEffect(() => {
    if (isAuthenticated) fetchAccountForMePage();
  }, [isAuthenticated]);

  const isMyPost = useMemo(() => {
    if (!loadedPost?.poster?.id) return false;

    const loadedAccount = isLoaded(account);
    if (!loadedAccount) return false;

    return loadedPost.poster.id === loadedAccount.id;
  }, [loadedPost, account]);

  const { propsForDeleteDialog } = useDialogProps("Delete");
  const { toggleDeleteDialog } = propsForDeleteDialog;

  return (
    <Stack
      orientation="vertical"
      fullWidth
      align="center"
      minHeight="100vh"
      justifyContent="space-between"
    >
      <Stack orientation="vertical" fullWidth align="center" maxWidth={1600} margin="auto">
        <Stack orientation="vertical" stretch={true} align="stretch" padding="0 1">
          <Stack orientation="horizontal" margin="1 0 0" gap="1" align="space-between">
            <Stack orientation="vertical" align="start">
              <Link
                variant="v4"
                back={POST_PAGE_URL}
                to={"/"}
                vtName={deletion_status === "DELETED" ? "login" : "back"}
              >
                <Icon variant="v4" name="back" /> Back
              </Link>
            </Stack>
            <Stack orientation="vertical" align="center" flex={1}>
              <GlobalSearch
                total_post_count={total_post_count}
                value={query}
                setValue={(value) => landingPage.set({ query: value })}
                onClick={() => navigate("/")}
              />
            </Stack>
            {!isSmallScreen && (
              <Stack orientation="vertical" align="end">
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
            )}
          </Stack>
        </Stack>
        <Stack orientation="horizontal" align="start" stretch={true} gap="3" padding="0 1">
          {post === "ERROR" ? (
            <Stack orientation="vertical" margin="3 0 0" flex={3} minWidth="60%">
              <Stack orientation="horizontal" align="baseline">
                <Text variant="v5" margin="0 0 1">
                  An error occurred while fetching post, please &nbsp;
                </Text>
                <Button variant="v5" onClick={() => fetchPostForPostPage(postId)}>
                  Try Again
                </Button>
              </Stack>
            </Stack>
          ) : (
            <Stack orientation="vertical" margin="1 0 0" flex="auto" minWidth="60%">
              {loadedPost ? (
                <Stack
                  orientation="horizontal"
                  justifyContent="space-between"
                  fullWidth
                  gap="1"
                  align="center"
                  margin="0 0 1"
                >
                  <Text variant="v3" vtName={`post-title-${loadedPost?.id}`}>
                    {loadedPost?.title}
                  </Text>
                  {isMyPost && (
                    <Stack orientation="horizontal">
                      <Icon
                        variant="v3"
                        name="deletePost"
                        onClick={() => toggleDeleteDialog(true)}
                      />
                      <Divider orientation="vertical" />
                      <Icon
                        variant="v3"
                        name="editPost"
                        onClick={() => alert("stay tuned at github.com/zibanpirate/dzjob.io")}
                      />
                    </Stack>
                  )}
                </Stack>
              ) : (
                <Skeleton variant="v3" width="18rem" margin="0 0 1" />
              )}
              <Text variant="v4" vtName={`post-description-${loadedPost?.id}`}>
                {loadedPost?.description !== undefined ? (
                  loadedPost.description ? (
                    <pre className="fade-in-up" style={{ whiteSpace: "pre-line", maxWidth: 600 }}>
                      {loadedPost.description}
                    </pre>
                  ) : (
                    <>
                      No description provided, you can&nbsp;
                      <Link
                        variant="v4"
                        to="#"
                        onClick={() => alert("Stay updated at github.com/zibanpirate/dzjob.io")}
                      >
                        ask
                      </Link>
                      &nbsp;for more information from the poster
                    </>
                  )
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
                    <Tag
                      variant="v4"
                      key={tag.id}
                      onClick={() => {
                        postsForPage.set({ tag });
                        navigate(JOBS_FOR_PAGE_URL.replace(":tagSlug", tag.slug));
                      }}
                    >
                      {tag.name}
                    </Tag>
                  ))}
                </Stack>
              )}
              <Stack orientation="vertical" margin="1 0 0" gap="1" stretch={true}>
                {loadedPost?.poster ? (
                  <>
                    <Text variant="v3" vtName={`post-poster-${loadedPost?.id}`}>
                      {getAccountName(loadedPost.poster)}
                    </Text>
                    {loadedPost.poster?.email ? (
                      <Text variant="v4">{loadedPost.poster?.email}</Text>
                    ) : (
                      <Skeleton variant="v4" width="10rem" />
                    )}
                  </>
                ) : (
                  <>
                    <Skeleton variant="v3" width="10rem" vtName={`post-poster-${loadedPost?.id}`} />
                  </>
                )}
              </Stack>
              <Stack orientation="horizontal" margin="3 0 0" align="center" gap="1">
                <Button
                  variant="v3"
                  onClick={() => alert("Stay updated at github.com/zibanpirate/dzjob.io")}
                >
                  Apply
                </Button>
                <Text variant="v4">or</Text>
                <Link
                  to="#"
                  variant="v4"
                  onClick={() => alert("Stay updated at github.com/zibanpirate/dzjob.io")}
                >
                  share this post
                </Link>
              </Stack>
            </Stack>
          )}
          <Stack orientation="vertical" margin="0 0 0" flex={3} minWidth={300}>
            <Text variant="v3" margin="1 0">
              Similar Jobs
            </Text>
            <Stack orientation="vertical" margin="0 0 3">
              {similarPosts === "ERROR" ? (
                <Stack orientation="horizontal" align="baseline">
                  <Text variant="v5" margin="0 0 1">
                    An error occurred while fetching similar posts, please &nbsp;
                  </Text>
                  <Button variant="v5" onClick={() => fetchSimilarPostsForPostPage(postId)}>
                    Try Again
                  </Button>
                </Stack>
              ) : similarPosts?.length === 0 ? (
                <Text variant="v4">No similar posts found (Raha Nashfa)</Text>
              ) : (
                <Stack orientation="horizontal" gap="1" align="stretch">
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
      </Stack>
      <Dialog {...propsForDeleteDialog}>
        <Stack orientation="vertical" gap="1" align="center" fullHeight={false}>
          <Text variant="v3">Are you sure you want to delete this post?</Text>
          {["DELETING", "DELETED"].includes(deletion_status) ? (
            <Icon
              variant="v3"
              name={deletion_status === "DELETING" ? "loadingSpinner" : "success"}
              animation={deletion_status === "DELETING" ? "rotate" : undefined}
              margin="1"
            />
          ) : (
            <>
              {deletion_status === "ERROR" && (
                <Text variant="v4" margin="1">
                  Something went wrong, please try again
                </Text>
              )}
              <Stack orientation="horizontal" gap="1" fullHeight={false}>
                <Button
                  variant="v3"
                  onClick={() => {
                    postPage.set({ deletion_status: "IDLE" });
                    toggleDeleteDialog(false);
                  }}
                >
                  Cancel
                </Button>
                <Button variant="v3" onClick={() => deletePost(postId)}>
                  Delete
                </Button>
              </Stack>
            </>
          )}
        </Stack>
      </Dialog>
      <Footer />
    </Stack>
  );
};
