import { FC, PropsWithChildren } from "react";
import "./style.css";
import { FontVariantProps, StyleProps } from "src/utils/props/style";

interface TextProps extends PropsWithChildren, StyleProps, FontVariantProps {}

export const Text: FC<TextProps> = ({ children, variant, margin }) => {
  const classes = [
    "text",
    `font-variant-${variant}`,
    margin && `margin-${margin}`,
  ].filter(Boolean);
  let Tag = "span" as keyof JSX.IntrinsicElements;
  return <Tag className={classes.join(" ")}>{children}</Tag>;
};
