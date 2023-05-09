import { FC } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";

export interface InputProps extends StyleProps, FontVariantProps {
  placeholder?: string;
  value: string;
  setValue: (value: string) => void;
}

export const Input: FC<InputProps> = ({ variant, margin, placeholder, value, setValue }) => {
  const classes = ["input", `font-variant-${variant}`, ...marginToClasses(margin)];

  return (
    <input
      className={classes.join(" ")}
      placeholder={placeholder}
      value={value}
      onChange={(e) => setValue(e.target.value)}
    />
  );
};
