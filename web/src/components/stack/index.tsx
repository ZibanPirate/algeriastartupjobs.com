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
    Pick<CSSProperties, "minWidth" | "maxWidth" | "flex"> {
  orientation: "horizontal" | "vertical";
  align?: "start" | "center" | "end" | "baseline" | "stretch" | "space-between";
  gap?: CSSNumber;
  stretch?: boolean;
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
  maxWidth,
  flex,
  animation = false,
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
    ...paddingToClasses(padding),
  ];

  const style: CSSProperties = {};
  if (vtName) style["viewTransitionName"] = vtName;
  if (maxWidth) style["maxWidth"] = maxWidth;
  if (minWidth) style["minWidth"] = minWidth;
  if (flex) style["flex"] = flex;
  if (animation)
    return (
      <div style={style}>
        <FlipMove
          className={classes.join(" ")}
          appearAnimation="none"
          leaveAnimation="none"
          enterAnimation="none"
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
