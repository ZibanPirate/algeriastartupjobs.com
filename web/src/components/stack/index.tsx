import { FC, PropsWithChildren } from "react";
import "./style.css";
import { CSSNumber, StyleProps, marginToClasses } from "src/utils/props/style";

interface StackProps extends PropsWithChildren, StyleProps {
  orientation: "horizontal" | "vertical";
  align?: "start" | "center" | "end" | "baseline";
  gap?: CSSNumber;
  stretch?: boolean;
}

export const Stack: FC<StackProps> = ({
  children,
  orientation,
  align = "start",
  margin,
  gap,
  stretch = false,
}) => {
  const classes = [
    "stack",
    stretch ? "width100" : "",
    "height100",
    "flex",
    `flex-${orientation}`,
    `flex-align-${align}`,
    `flex-gap-${gap}`,
    ...marginToClasses(margin),
  ];
  return <div className={classes.join(" ")}>{children}</div>;
};
