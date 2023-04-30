import { FC, PropsWithChildren } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";

export interface TextProps extends PropsWithChildren, StyleProps, FontVariantProps {}

export const Text: FC<TextProps> = ({ children, variant, margin }) => {
  const classes = ["text", `font-variant-${variant}`, ...marginToClasses(margin)];
  let Tag = "span" as keyof JSX.IntrinsicElements;
  return <Tag className={classes.join(" ")}>{children}</Tag>;
};
