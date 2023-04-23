import { FC } from "react";
import "./style.css";
import {
  FontVariantProps,
  StyleProps,
  marginToClasses,
} from "src/utils/props/style";

interface InputProps extends StyleProps, FontVariantProps {
  placeholder?: string;
}

export const Input: FC<InputProps> = ({ variant, margin, placeholder }) => {
  const classes = [
    "input",
    `font-variant-${variant}`,
    ...marginToClasses(margin),
  ];

  return <input className={classes.join(" ")} placeholder={placeholder} />;
};
