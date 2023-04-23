import { FC, PropsWithChildren } from "react";
import "./style.css";
import { StyleProps, marginToClasses } from "src/utils/props/style";

interface StackProps extends PropsWithChildren, StyleProps {
  orientation: "horizontal" | "vertical";
  align?: "start" | "center" | "end";
}

export const Stack: FC<StackProps> = ({
  children,
  orientation,
  align = "start",
  margin,
}) => {
  const classes = [
    "stack",
    "height100",
    "flex",
    `flex-${orientation}`,
    `flex-align-${align}`,
    ...marginToClasses(margin),
  ];
  return <div className={classes.join(" ")}>{children}</div>;
};
