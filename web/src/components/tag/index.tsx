import { FC } from "react";
import "./style.css";
import { Text, TextProps } from "../text";

interface TagProps extends TextProps {}

export const Tag: FC<TagProps> = ({ ...textProps }) => {
  const classes = ["tag"];

  return (
    <div className={classes.join(" ")}>
      <Text {...textProps} />
    </div>
  );
};
