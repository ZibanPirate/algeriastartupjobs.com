import { FC, useCallback, useEffect, useState } from "react";
import { Input, InputProps } from "src/components/input";
import { Stack, StackProps } from "src/components/stack";
import { Icon } from "src/components/icon";
import { Button } from "src/components/button";
import { StyleProps } from "src/utils/props/style";
import debounce from "lodash/debounce";

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
  const [localValue, setLocalValue] = useState(value);

  const debouncedSetValue = useCallback(
    debounce((newLocalValue: string) => setValue(newLocalValue), debounceValue),
    []
  );

  useEffect(() => {
    debouncedSetValue(localValue), [localValue];
  });

  return (
    <Stack orientation="horizontal" margin={margin} wrap={false} {...stackProps}>
      <Input
        variant="v3"
        placeholder={placeholder}
        value={localValue}
        setValue={setLocalValue}
        onKeyDown={(event) => {
          if (event.key === "Enter") {
            setValue(localValue);
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
