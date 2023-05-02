import { CSSProperties, FC, PropsWithChildren } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import { AnimationProps } from "src/utils/props/animation";

export interface TextProps
  extends PropsWithChildren,
    StyleProps,
    FontVariantProps,
    AnimationProps {}

export const Text: FC<TextProps> = ({ children, variant, margin, vtName }) => {
  const classes = ["text", `font-variant-${variant}`, ...marginToClasses(margin)];

  const style: CSSProperties = {};
  if (vtName) style["viewTransitionName"] = vtName;

  let Tag = "span" as keyof JSX.IntrinsicElements;
  return (
    <Tag className={classes.join(" ")} style={style}>
      {children}
    </Tag>
  );
};
