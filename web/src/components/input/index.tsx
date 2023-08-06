import { CSSProperties, FC, HTMLAttributes } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import { AnimationProps } from "src/utils/props/animation";

export interface InputProps
  extends StyleProps,
    FontVariantProps,
    AnimationProps,
    Pick<HTMLAttributes<HTMLInputElement>, "onKeyDown" | "id" | "inputMode"> {
  placeholder?: string;
  value: string;
  setValue: (value: string) => void;
  stretch?: boolean;
  disabled?: boolean;
  width?: CSSProperties["width"];
}

export const Input: FC<InputProps> = ({
  variant,
  margin,
  placeholder,
  value,
  setValue,
  stretch = false,
  width,
  vtName,
  ...props
}) => {
  const classes = [
    "input",
    `font-variant-${variant}`,
    stretch ? "stretch" : "width100",
    ...marginToClasses(margin),
  ];

  const style: CSSProperties = {};
  if (width) style.width = width;
  if (vtName) style["viewTransitionName"] = vtName;

  return (
    <input
      className={classes.join(" ")}
      placeholder={placeholder}
      value={value}
      style={style}
      onChange={(e) => setValue(e.target.value)}
      {...props}
    />
  );
};
