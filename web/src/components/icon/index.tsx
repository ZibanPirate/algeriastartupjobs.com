import { FC, HTMLAttributes, memo } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import * as icons from "./svg";
import { AnimationProps } from "src/utils/props/animation";

interface IconProps
  extends StyleProps,
    FontVariantProps,
    AnimationProps,
    Pick<HTMLAttributes<HTMLSpanElement>, "onClick"> {
  name: keyof typeof icons;
  animation?: "rotate" | "none";
}

// @TODO-ZM: Add ColorVariantProps
const _Icon: FC<IconProps> = ({ variant, margin, name, animation = "none", vtName, ...props }) => {
  const classes = [
    "icon",
    animation === "none" ? "" : `animate-${animation}`,
    `font-variant-${variant}`,
    ...marginToClasses(margin),
    props.onClick ? "icon-clickable" : "",
  ];

  const maskImage = `url("${icons[name].default}"`;

  const style: React.CSSProperties = {
    maskImage,
    WebkitMaskImage: maskImage,
  };

  if (vtName) style["viewTransitionName"] = vtName;

  return <span className={classes.join(" ")} style={style} {...props} />;
};
export const Icon = memo(_Icon);
