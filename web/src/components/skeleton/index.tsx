import { CSSProperties, FC } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";

export interface SkeletonProps
  extends StyleProps,
    FontVariantProps,
    Pick<CSSProperties, "width" | "maxWidth" | "height"> {}

export const Skeleton: FC<SkeletonProps> = ({ variant, margin, ...cssProps }) => {
  const classes = ["skeleton", `font-variant-${variant}`, ...marginToClasses(margin)];

  return <span className={classes.join(" ")} style={cssProps} />;
};
