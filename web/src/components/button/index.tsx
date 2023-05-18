import { CSSProperties, FC, PropsWithChildren } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import { AnimationProps } from "src/utils/props/animation";

export interface ButtonProps
  extends PropsWithChildren,
    StyleProps,
    FontVariantProps,
    AnimationProps {
  padding?: "square" | "rectangle" | "rectangle-end";
  onClick?: () => void;
}

export const Button: FC<ButtonProps> = ({
  children,
  variant,
  margin,
  padding = "rectangle",
  onClick,
  vtName,
}) => {
  const classes = [
    "button",
    `button-${padding}`,
    `font-variant-${variant}`,
    ...marginToClasses(margin),
  ];

  const style: CSSProperties = {};
  if (vtName) style["viewTransitionName"] = vtName;

  return (
    <button className={classes.join(" ")} style={style} onClick={onClick}>
      {children}
    </button>
  );
};
