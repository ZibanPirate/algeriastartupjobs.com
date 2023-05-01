import { FC } from "react";
import { Input } from "src/components/input";
import { Stack } from "src/components/stack";
import { Icon } from "src/components/icon";
import { Button } from "src/components/button";
import { StyleProps } from "src/utils/props/style";

export interface SearchProps extends StyleProps {
  placeholder?: string;
}

export const Search: FC<SearchProps> = ({ margin, placeholder }) => {
  return (
    <Stack orientation="horizontal" margin={margin} wrap={false}>
      <Input variant="v3" placeholder={placeholder} />
      <Button variant="v3" padding="square">
        <Icon variant="v3" name="search" />
      </Button>
    </Stack>
  );
};
