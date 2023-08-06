import { FC, useEffect, useMemo, useState } from "react";
import { Link } from "src/components/link";
import { Stack } from "src/components/stack";
import { Text } from "src/components/text";
import { usePageTitle } from "src/utils/hooks/page-title";

import { Icon } from "src/components/icon";
import { Input } from "src/components/input";
import { Button } from "src/components/button";
import { Select } from "src/components/select";
import { Account } from "src/models/account";
import { useSliceSelector } from "src/utils/state/selector";
import { getStateActions } from "src/state";
import { DebouncedValueInput } from "src/components/input/debounced-value";
import { createPost, fetchAccountForCreatePostPage, fetchTagsForCreatePostPage } from "./actions";
import { isLoaded } from "src/utils/loadable";
import { getAccountName } from "src/utils/models/account-name";
import { POST_PAGE_URL } from "src/utils/urls/common";
import { DebouncedValueRichInput } from "src/components/rich-input/debounced-value";
import { Tag } from "src/components/tag";
import { useMediaQuery } from "src/utils/hooks/use-media-query";

export const Page: FC = () => {
  usePageTitle("Post a job ad for free!");

  const {
    title,
    poster_type,
    poster_name,
    poster_first_name,
    poster_last_name,
    poster_contact,
    poster,
    creation_status,
    compact,
    post_description,
    suggested_tags,
    tags,
  } = useSliceSelector("createPostPage");
  const { query } = useSliceSelector("landingPage");

  const { set } = getStateActions().createPostPage;

  const [uniqueSuggestedTags, setUniqueSuggestedTags] = useState(suggested_tags);
  useEffect(() => {
    if (suggested_tags === "ERROR" || !suggested_tags || !tags)
      setUniqueSuggestedTags(uniqueSuggestedTags);
    else setUniqueSuggestedTags(suggested_tags.filter((tag) => !tags.find((t) => t.id === tag.id)));
  }, [suggested_tags, tags]);

  const [canHideDetails, setCanHideDetails] = useState(false);
  useEffect(() => {
    setCanHideDetails(!(compact || post_description || tags?.length > 0));
  }, [compact, post_description, tags]);

  useEffect(() => {
    fetchAccountForCreatePostPage();
  }, [poster_contact]);

  useEffect(() => {
    if (compact) return;
    fetchTagsForCreatePostPage();
  }, [title, post_description, compact]);

  useEffect(() => {
    if (!title) set({ title: query });
  }, []);

  const [posterName, setPosterName] = useState("");

  useEffect(() => {
    const loadedPoster = isLoaded(poster);
    if (loadedPoster && loadedPoster.email === poster_contact) {
      set({
        poster_type: loadedPoster.type,
        ...(loadedPoster.type === "Company"
          ? { poster_name: loadedPoster.company_name }
          : {
              poster_first_name: loadedPoster.first_name,
              poster_last_name: loadedPoster.last_name,
            }),
      });
      setPosterName(
        (loadedPoster.type === "Company" ? "At " : "By ") + getAccountName(loadedPoster)
      );
    } else {
      setPosterName("");
    }
  }, [poster]);

  const isMediumScreen = useMediaQuery("(max-width: 1300px)");

  const disabledInputs = ["CREATING", "CREATED"].includes(creation_status);

  const Tags = useMemo(
    () => (
      <>
        {tags.length > 0 && (
          <>
            <Text variant="v4">Applied tags</Text>
            <Stack orientation="horizontal" animation={true} gap="1">
              {tags.map((tag) => (
                <Tag
                  variant="v4"
                  padding="0 0 0 .5"
                  key={tag.id}
                  onClick={() => set({ tags: tags.filter((t) => t.id !== tag.id) })}
                >
                  {tag.name}
                  <Icon variant="v4" name="removeTag" />
                </Tag>
              ))}
            </Stack>
          </>
        )}
        {uniqueSuggestedTags === "ERROR" ? (
          <Text variant="v4">Something went wrong while suggesting tags...</Text>
        ) : (
          uniqueSuggestedTags &&
          uniqueSuggestedTags.length > 0 && (
            <>
              <Text variant="v4">Suggested tags</Text>
              <Stack orientation="horizontal" animation={true} gap="1">
                {uniqueSuggestedTags.map((tag) => (
                  <Tag
                    variant="v4"
                    padding="0 0 0 .5"
                    key={tag.id}
                    onClick={() => set({ tags: [...tags, tag] })}
                  >
                    {tag.name}
                    <Icon variant="v4" name="addTag" />
                  </Tag>
                ))}
              </Stack>
            </>
          )
        )}
      </>
    ),
    [tags, uniqueSuggestedTags]
  );

  return (
    <Stack orientation="vertical" fullWidth align="start" maxWidth={600} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" margin="1 0 0" stretch={true} align="start">
        <Link variant="v4" back={POST_PAGE_URL} to={"/"} vtName="back">
          <Icon variant="v4" name="back" /> Back
        </Link>
      </Stack>
      {/* Create Post */}
      <Stack orientation="vertical" align="start" stretch gap="1" padding="3 1" position="relative">
        {!isMediumScreen && (
          <div
            style={{ position: "absolute", right: 0, transform: "translateX(100%)", width: 300 }}
          >
            <Stack orientation="vertical" gap="1">
              {Tags}
            </Stack>
          </div>
        )}
        <Text variant="v4">Looking for</Text>
        <DebouncedValueInput
          vtName="global-search"
          disabled={disabledInputs}
          placeholder="Job title, eg: Sales Manager"
          stretch={true}
          value={title}
          setValue={(value) => set({ title: value })}
          variant="v4"
        />

        <Text variant="v4">Candidate apply by sending email to</Text>
        <DebouncedValueInput
          disabled={disabledInputs}
          placeholder="Contact email"
          stretch={true}
          value={poster_contact}
          setValue={(value) => set({ poster_contact: value })}
          variant="v4"
          id="email"
          inputMode="email"
        />

        {posterName ? (
          <Text variant="v4">{posterName}</Text>
        ) : (
          <>
            <Select<Account["type"]>
              disabled={disabledInputs}
              variant="v4"
              padding="0"
              value={poster_type}
              setValue={(value) => set({ poster_type: value })}
              options={[
                { value: "Company", label: "At (Company)" },
                { value: "Individual", label: "By (Individual)" },
              ]}
            />
            {poster_type === "Company" ? (
              <Input
                disabled={disabledInputs}
                placeholder="Company name"
                stretch={true}
                value={poster_name}
                setValue={(value) => set({ poster_name: value })}
                variant="v4"
              />
            ) : (
              <>
                <Text variant="v4">First name</Text>
                <Input
                  disabled={disabledInputs}
                  placeholder="First name"
                  stretch={true}
                  value={poster_first_name}
                  setValue={(value) => set({ poster_first_name: value })}
                  variant="v4"
                  id="first-name"
                />
                <Text variant="v4">Last name</Text>
                <Input
                  disabled={disabledInputs}
                  placeholder="Last name"
                  stretch={true}
                  value={poster_last_name}
                  setValue={(value) => set({ poster_last_name: value })}
                  variant="v4"
                  id="last-name"
                />
              </>
            )}
          </>
        )}
        {compact ? null : (
          <>
            <Text variant="v4">More details about the job</Text>
            <DebouncedValueRichInput
              disabled={disabledInputs}
              placeholder="What is the job about?"
              stretch={true}
              value={post_description}
              setValue={(value) => set({ post_description: value })}
              autoRows={true}
              variant="v4"
              id="description"
              resize="vertical"
            />
            {isMediumScreen && Tags}
          </>
        )}
        <Stack orientation="vertical" align="center" stretch>
          {["CREATING", "CREATED"].includes(creation_status) ? (
            <Icon
              variant="v3"
              name={creation_status === "CREATING" ? "loadingSpinner" : "success"}
              animation={creation_status === "CREATING" ? "rotate" : undefined}
              margin="3 0"
              vtName="create-post-icon"
            />
          ) : (
            <>
              <Text variant="v4" margin="1 0 2">
                {creation_status === "ERROR" ? "Something went wrong, please try again" : <br />}
              </Text>
              <Stack orientation="horizontal" align="center" gap="1">
                <Button variant="v3" onClick={() => createPost()} vtName="new-post">
                  Post now
                </Button>
                {(compact || canHideDetails) && (
                  <>
                    <Text variant="v4">or</Text>
                    <Link to="#" variant="v4" onClick={() => set({ compact: !compact })}>
                      {compact ? "Add more details" : "Hide details"}
                    </Link>
                  </>
                )}
              </Stack>
            </>
          )}
        </Stack>
      </Stack>
      <Text variant="v4" margin="0 1 1">
        Source code is publicly available at&nbsp;
        <Link to="https://github.com/algeriastartupjobs/algeriastartupjobs.com" variant="v4">
          Github
        </Link>
      </Text>
    </Stack>
  );
};
