import { CSSProperties, FC, PropsWithChildren } from "react";
import "./style.css";
import { CSSNumber, StyleProps, marginToClasses, paddingToClasses } from "src/utils/props/style";
import { AnimationProps } from "src/utils/props/animation";
import FlipMove from "react-flip-move";
import { shouldAnimate } from "src/utils/animation/should-animate";

export interface StackProps
  extends PropsWithChildren,
    StyleProps,
    AnimationProps,
    Pick<
      CSSProperties,
      "minWidth" | "minHeight" | "maxWidth" | "flex" | "justifyContent" | "position"
    > {
  orientation: "horizontal" | "vertical";
  align?: "start" | "center" | "end" | "baseline" | "stretch" | "space-between";
  gap?: CSSNumber;
  stretch?: boolean;
  fullWidth?: boolean;
  wrap?: boolean;
  animation?: boolean;
}

export const Stack: FC<StackProps> = ({
  children,
  orientation,
  align = "start",
  margin,
  padding,
  gap,
  stretch = false,
  wrap = true,
  vtName,
  minWidth,
  minHeight,
  maxWidth,
  flex,
  animation = false,
  fullWidth = false,
  justifyContent,
  position,
}) => {
  const classes = [
    "stack",
    stretch ? "stretch" : "",
    "height100",
    fullWidth ? "width100" : "",
    "flex",
    `flex-${orientation}`,
    `flex-align-${align}`,
    `flex-gap-${gap}`,
    wrap ? "flex-wrap" : "",
    ...marginToClasses(margin),
    ...paddingToClasses(padding),
  ];

  const style: CSSProperties = {};
  if (vtName) style["viewTransitionName"] = vtName;
  if (maxWidth) style["maxWidth"] = maxWidth;
  if (minWidth) style["minWidth"] = minWidth;
  if (minHeight) style["minHeight"] = minHeight;
  if (flex) style["flex"] = flex;
  if (justifyContent) style["justifyContent"] = justifyContent;
  if (position) style["position"] = position;
  if (animation)
    return (
      <div style={style}>
        <FlipMove
          className={classes.join(" ")}
          appearAnimation="fade"
          leaveAnimation="none"
          enterAnimation="fade"
          disableAllAnimations={!shouldAnimate}
        >
          {children}
        </FlipMove>
      </div>
    );
  else
    return (
      <div className={classes.join(" ")} style={style}>
        {children}
      </div>
    );
};
