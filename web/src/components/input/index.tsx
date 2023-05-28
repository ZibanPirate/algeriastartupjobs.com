import { FC, HTMLAttributes } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";

export interface InputProps
  extends StyleProps,
    FontVariantProps,
    Pick<HTMLAttributes<HTMLInputElement>, "onKeyDown"> {
  placeholder?: string;
  value: string;
  setValue: (value: string) => void;
  stretch?: boolean;
}

export const Input: FC<InputProps> = ({
  variant,
  margin,
  placeholder,
  value,
  setValue,
  stretch = false,
  ...props
}) => {
  const classes = [
    "input",
    `font-variant-${variant}`,
    stretch ? "stretch" : "width100",
    ...marginToClasses(margin),
  ];

  return (
    <input
      className={classes.join(" ")}
      placeholder={placeholder}
      value={value}
      onChange={(e) => setValue(e.target.value)}
      {...props}
    />
  );
};
