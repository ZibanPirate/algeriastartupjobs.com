import { CSSProperties, FC, PropsWithChildren } from "react";
import "./style.css";
import { CSSNumber, StyleProps, marginToClasses } from "src/utils/props/style";
import { AnimationProps } from "src/utils/props/animation";

interface StackProps
  extends PropsWithChildren,
    StyleProps,
    AnimationProps,
    Pick<CSSProperties, "minWidth" | "maxWidth" | "flex"> {
  orientation: "horizontal" | "vertical";
  align?: "start" | "center" | "end" | "baseline" | "stretch";
  gap?: CSSNumber;
  stretch?: boolean;
  wrap?: boolean;
}

export const Stack: FC<StackProps> = ({
  children,
  orientation,
  align = "start",
  margin,
  gap,
  stretch = false,
  wrap = true,
  vtName,
  minWidth,
  maxWidth,
  flex,
}) => {
  const classes = [
    "stack",
    stretch ? "width100" : "",
    "height100",
    "flex",
    `flex-${orientation}`,
    `flex-align-${align}`,
    `flex-gap-${gap}`,
    wrap ? "flex-wrap" : "",
    ...marginToClasses(margin),
  ];

  const style: CSSProperties = {};
  if (vtName) style["viewTransitionName"] = vtName;
  if (maxWidth) style["maxWidth"] = maxWidth;
  if (minWidth) style["minWidth"] = minWidth;
  if (flex) style["flex"] = flex;

  return (
    <div className={classes.join(" ")} style={style}>
      {children}
    </div>
  );
};
