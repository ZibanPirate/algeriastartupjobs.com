import { CSSProperties, DOMAttributes, FC, PropsWithChildren, useRef } from "react";
import "./style.css";
import {
  FontVariantProps,
  StyleProps,
  marginToClasses,
  paddingToClasses,
} from "src/utils/props/style";
import { AnimationProps } from "src/utils/props/animation";
import { FunctionComponent } from "react";
import { Icon } from "../icon";

export interface SelectProps<O> extends StyleProps, FontVariantProps, AnimationProps {
  paddingPreset?: "square" | "rectangle" | "rectangle-end";
  options: Array<{ value: O; label: string }>;
  value: O;
  setValue: (value: O) => void;
}

export const Select = <O extends string>({
  variant,
  margin,
  padding,
  paddingPreset = "rectangle",
  vtName,
  options,
  value,
  setValue,
}: SelectProps<O>): ReturnType<FunctionComponent<SelectProps<O>>> => {
  const selectRef = useRef<HTMLSelectElement>(null);

  const classes = ["select", `font-variant-${variant}`];

  const containerClasses = [
    "select-container",
    `select-${paddingPreset}`,
    ...marginToClasses(margin),
    ...paddingToClasses(padding),
  ];

  const style: CSSProperties = {};
  if (vtName) style["viewTransitionName"] = vtName;

  return (
    <div className={containerClasses.join(" ")}>
      <select
        className={classes.join(" ")}
        style={style}
        ref={selectRef}
        value={value}
        onChange={(event) => setValue(event.target.value as O)}
      >
        {options.map(({ value, label }) => (
          <option value={value}>{label}</option>
        ))}
      </select>
      <Icon variant={variant} name="dropdown" />
    </div>
  );
};
