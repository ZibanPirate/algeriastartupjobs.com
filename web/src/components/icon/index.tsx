import { FC, memo } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import * as icons from "./svg";

interface IconProps extends StyleProps, FontVariantProps {
  name: keyof typeof icons;
}

// @TODO-ZM: Add ColorVariantProps
const _Icon: FC<IconProps> = ({ variant, margin, name }) => {
  const classes = ["icon", `font-variant-${variant}`, ...marginToClasses(margin)];

  const maskImage = `url(${icons[name].default})`;

  return <span className={classes.join(" ")} style={{ maskImage, WebkitMaskImage: maskImage }} />;
};
export const Icon = memo(_Icon);
