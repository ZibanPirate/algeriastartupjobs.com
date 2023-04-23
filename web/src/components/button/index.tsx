import { FC, PropsWithChildren } from "react";
import "./style.css";
import {
  FontVariantProps,
  StyleProps,
  marginToClasses,
} from "src/utils/props/style";

interface ButtonProps extends PropsWithChildren, StyleProps, FontVariantProps {}

export const Button: FC<ButtonProps> = ({ children, variant, margin }) => {
  const classes = [
    "button",
    `font-variant-${variant}`,
    ...marginToClasses(margin),
  ];

  return <button className={classes.join(" ")}>{children}</button>;
};
