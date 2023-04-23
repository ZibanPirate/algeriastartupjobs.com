import { FC } from "react";
import { Input } from "src/components/input";
import { Stack } from "src/components/stack";
import { Icon } from "src/components/icon";
import { Button } from "src/components/button";
import { StyleProps } from "src/utils/props/style";

export interface SearchProps extends StyleProps {}

export const Search: FC<SearchProps> = ({ margin }) => {
  return (
    <Stack orientation="horizontal" margin={margin}>
      <Input variant="v3" placeholder="Search 1354 jobs" />
      <Button variant="v3">
        <Icon variant="v3" name="search" />
      </Button>
    </Stack>
  );
};
