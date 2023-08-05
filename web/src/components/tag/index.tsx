import { FC, forwardRef } from "react";
import "./style.css";
import { Text, TextProps } from "../text";

interface TagProps extends TextProps {
  onClick?: () => void;
}

export const Tag = forwardRef<HTMLDivElement, TagProps>(({ onClick, ...textProps }, ref) => {
  const classes = ["tag", onClick && "tag-clickable"].filter(Boolean);

  return (
    <div className={classes.join(" ")} ref={ref} onClick={onClick}>
      <Text {...textProps} />
    </div>
  );
});
