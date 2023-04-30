import { FC, CSSProperties } from "react";
import "./style.css";
import { StyleProps, marginToClasses } from "src/utils/props/style";

interface DividerProps extends StyleProps {
  orientation?: "horizontal" | "vertical";
  length?: CSSProperties["width"];
}

export const Divider: FC<DividerProps> = ({ orientation, margin, length }) => {
  const classes = ["divider", `divider-${orientation}`, ...marginToClasses(margin)];

  return (
    <div
      className={classes.join(" ")}
      style={{ [orientation === "horizontal" ? "width" : "height"]: length }}
    />
  );
};
