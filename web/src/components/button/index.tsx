import { FC, PropsWithChildren } from "react";
import "./style.css";
import {
  FontVariantProps,
  StyleProps,
  marginToClasses,
} from "src/utils/props/style";

interface ButtonProps extends PropsWithChildren, StyleProps, FontVariantProps {
  padding?: "square" | "rectangle";
  onClick?: () => void;
}

export const Button: FC<ButtonProps> = ({
  children,
  variant,
  margin,
  padding = "rectangle",
  onClick,
}) => {
  const classes = [
    "button",
    `button-${padding}`,
    `font-variant-${variant}`,
    ...marginToClasses(margin),
  ];

  return (
    <button className={classes.join(" ")} onClick={onClick}>
      {children}
    </button>
  );
};
