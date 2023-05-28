import { FC, useEffect, useState } from "react";
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
import { fetchAccountForCreatePostPage } from "./actions";
import { isLoaded } from "src/utils/loadable";
import { getAccountName } from "src/utils/models/account-name";

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
  } = useSliceSelector("createPostPage");
  const { set } = getStateActions().createPostPage;

  useEffect(() => {
    fetchAccountForCreatePostPage();
  }, [poster_contact]);

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

  return (
    <Stack orientation="vertical" stretch align="start" maxWidth={600} margin="auto">
      {/* Header */}
      <Stack orientation="vertical" margin="1 0 0" stretch={true} align="start">
        <Link variant="v4" back={true} to={"/"} vtName="back">
          <Icon variant="v4" name="back" /> Back
        </Link>
      </Stack>
      {/* Create Post */}
      {/* @TODO-ZM: apply padding to other places where we did workaround it */}
      <Stack orientation="vertical" align="start" stretch gap="1" margin="3 0">
        <Text variant="v4">Looking for</Text>
        <Input
          placeholder="Job title, eg: Sales Manager"
          stretch={true}
          value={title}
          setValue={(value) => set({ title: value })}
          variant="v4"
        />

        <Text variant="v4">Candidate apply by sending email to</Text>
        <DebouncedValueInput
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
                  placeholder="First name"
                  stretch={true}
                  value={poster_first_name}
                  setValue={(value) => set({ poster_first_name: value })}
                  variant="v4"
                  id="first-name"
                />
                <Text variant="v4">Last name</Text>
                <Input
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
        <Stack orientation="horizontal" margin="2 0 0" align="center" gap="1">
          <Button
            variant="v3"
            onClick={() => alert("Stay updated at github.com/algeriastartupjobs")}
            vtName="new-post"
          >
            Post now
          </Button>
          <Text variant="v4">or</Text>
          <Link
            to="#"
            variant="v4"
            onClick={() => alert("Stay updated at github.com/algeriastartupjobs")}
          >
            Add more details
          </Link>
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
