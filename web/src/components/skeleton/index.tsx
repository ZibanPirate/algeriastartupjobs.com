import { CSSProperties, FC, forwardRef } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import { AnimationProps } from "src/utils/props/animation";

export interface SkeletonProps
  extends StyleProps,
    FontVariantProps,
    Pick<CSSProperties, "width" | "maxWidth" | "height">,
    AnimationProps {}

export const Skeleton = forwardRef<HTMLSpanElement, SkeletonProps>(
  ({ variant, margin, vtName, ...cssProps }, ref) => {
    const classes = ["skeleton", `font-variant-${variant}`, ...marginToClasses(margin)];

    const style: CSSProperties = cssProps;
    if (vtName) style["viewTransitionName"] = vtName;

    return <span className={classes.join(" ")} style={style} ref={ref} />;
  }
);
