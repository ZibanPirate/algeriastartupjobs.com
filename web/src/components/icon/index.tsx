import { FC, HTMLAttributes, memo } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import * as icons from "./svg";

interface IconProps
  extends StyleProps,
    FontVariantProps,
    Pick<HTMLAttributes<HTMLSpanElement>, "onClick"> {
  name: keyof typeof icons;
  animation?: "rotate" | "none";
}

// @TODO-ZM: Add ColorVariantProps
const _Icon: FC<IconProps> = ({ variant, margin, name, animation = "none", ...props }) => {
  const classes = [
    "icon",
    animation === "none" ? "" : `animate-${animation}`,
    `font-variant-${variant}`,
    ...marginToClasses(margin),
  ];

  const maskImage = `url(${icons[name].default})`;

  return (
    <span
      className={classes.join(" ")}
      style={{ maskImage, WebkitMaskImage: maskImage }}
      {...props}
    />
  );
};
export const Icon = memo(_Icon);
