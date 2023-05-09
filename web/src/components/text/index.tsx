import { CSSProperties, FC, PropsWithChildren, forwardRef } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import { AnimationProps } from "src/utils/props/animation";

export interface TextProps
  extends PropsWithChildren,
    StyleProps,
    FontVariantProps,
    AnimationProps {}

export const Text = forwardRef<HTMLSpanElement, TextProps>(
  ({ children, variant, margin, vtName }, ref) => {
    const classes = ["text", `font-variant-${variant}`, ...marginToClasses(margin)];

    const style: CSSProperties = {};
    if (vtName) style["viewTransitionName"] = vtName;

    return (
      <span className={classes.join(" ")} style={style} ref={ref}>
        {children}
      </span>
    );
  }
);
