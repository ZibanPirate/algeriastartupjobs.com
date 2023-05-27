import { FC } from "react";
import { InputProps } from "src/components/input";
import { Stack, StackProps } from "src/components/stack";
import { Icon } from "src/components/icon";
import { Button } from "src/components/button";
import { StyleProps } from "src/utils/props/style";
import { DebouncedValueInput } from "../input/debounced-value";

export interface SearchProps
  extends StyleProps,
    Pick<InputProps, "value" | "setValue">,
    Pick<StackProps, "vtName"> {
  placeholder?: string;
  debounceValue?: number;
  onClick?: () => void;
}

export const Search: FC<SearchProps> = ({
  margin,
  placeholder,
  value,
  setValue,
  debounceValue = 500,
  onClick,
  ...stackProps
}) => {
  return (
    <Stack orientation="horizontal" margin={margin} wrap={false} {...stackProps}>
      <DebouncedValueInput
        variant="v3"
        placeholder={placeholder}
        value={value}
        setValue={setValue}
        onKeyDown={(event) => {
          if (event.key === "Enter") {
            setValue(event.currentTarget.value);
            onClick?.();
          }
        }}
      />
      <Button variant="v3" paddingPreset="square" onClick={onClick}>
        <Icon variant="v3" name="search" />
      </Button>
    </Stack>
  );
};
