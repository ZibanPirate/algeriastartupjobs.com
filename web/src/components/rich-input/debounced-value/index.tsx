import { FC, useCallback, useEffect, useState } from "react";
import { RichInput, RichInputProps } from "..";
import debounce from "lodash/debounce";

interface DebouncedValueRichInputProps extends RichInputProps {
  debounceValue?: number;
}

export const DebouncedValueRichInput: FC<DebouncedValueRichInputProps> = ({
  value,
  setValue,
  debounceValue = 500,
  ...props
}) => {
  const [localValue, setLocalValue] = useState(value);

  useEffect(() => {
    setLocalValue(value);
  }, [value]);

  const debouncedSetValue = useCallback(
    debounce((newLocalValue: string) => setValue(newLocalValue), debounceValue),
    []
  );

  useEffect(() => {
    debouncedSetValue(localValue);
  });

  return <RichInput value={localValue} setValue={setLocalValue} {...props} />;
};
