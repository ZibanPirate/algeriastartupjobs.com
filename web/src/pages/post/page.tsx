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
import { getPostLongTitle } from "src/utils/urls/post-long-title";
import { Button } from "src/components/button";
import { Tag } from "src/components/tag";
import { Divider } from "src/components/divider";
import { getAccountName } from "src/utils/models/acount-name";

export const Page: FC = () => {
  const postSlug = useMatch(POST_PAGE_URL)?.params.postSlug;
  const postId = useMemo(() => (/(.*)_(\d+)$/.exec(postSlug || "") || [])[2], [postSlug]);

  useEffect(() => {
    if (!postId) return;
    fetchPostForPostPage(postId);
  }, [postId]);

  const { post } = useSliceSelector("postPage");
  const loadedPost = isLoaded(post);
  usePageTitle(loadedPost ? getPostLongTitle(loadedPost, loadedPost.poster) : "Loading Job...");

  return (
    <Stack orientation="vertical" stretch align="center">
      {post === "ERROR" ? (
        <Stack orientation="vertical" margin="3 0 0">
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
        <Stack orientation="vertical" margin="1 1 0" align="center">
          <Text variant="v3" margin="0 0 1">
            {loadedPost?.title || "Loading Job..."}
          </Text>
          <Text variant="v5">
            <pre style={{ whiteSpace: "pre-line", maxWidth: 600 }}>
              {loadedPost?.description || "Loading job description..."}
            </pre>
          </Text>
          {loadedPost?.tags && loadedPost?.tags.length > 0 && (
            <Stack orientation="horizontal" margin="1 0 0" stretch={true}>
              {loadedPost?.tags.map((tag) => (
                <Tag variant="v5" key={tag.id}>
                  {tag.name}
                </Tag>
              ))}
            </Stack>
          )}
          <Stack orientation="horizontal" margin="1 0 0" stretch={true}>
            <Text variant="v5">
              {loadedPost?.poster ? getAccountName(loadedPost?.poster) : "Loading Poster..."}
            </Text>
            <Divider margin="0 1" orientation="vertical" />
            <Text variant="v5">{loadedPost?.category.name}</Text>
          </Stack>
          <Stack orientation="horizontal" margin="1 0 0" align="center" gap="1">
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

      <Text variant="v4" margin="1 1">
        Source code is publicly available at&nbsp;
        <Link to="https://github.com/algeriastartupjobs/algeriastartupjobs.com" variant="v4">
          Github
        </Link>
      </Text>
    </Stack>
  );
};
