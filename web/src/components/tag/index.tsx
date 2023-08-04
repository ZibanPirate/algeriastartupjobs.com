import { FC, forwardRef } from "react";
import "./style.css";
import { Text, TextProps } from "../text";

interface TagProps extends TextProps {}

export const Tag = forwardRef<HTMLDivElement, TagProps>(({ ...textProps }, ref) => {
  const classes = ["tag"];

  return (
    <div className={classes.join(" ")} ref={ref}>
      <Text {...textProps} />
    </div>
  );
});
