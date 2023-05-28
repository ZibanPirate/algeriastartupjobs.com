import { CSSProperties, PropsWithChildren, forwardRef } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import { NavLink as RL, LinkProps as RLP } from "react-router-dom";
import { isNavigatingBackLeavesWebsite } from "../router-provider";
import { AnimationProps } from "src/utils/props/animation";

interface LinkProps extends PropsWithChildren, StyleProps, FontVariantProps, RLP, AnimationProps {
  back?: boolean;
  className?: string;
}

export const Link = forwardRef<HTMLAnchorElement, LinkProps>(
  ({ children, variant, margin, back, className, vtName, ...props }, ref) => {
    const classes = [
      className,
      "link",
      `font-variant-${variant}`,
      ...marginToClasses(margin),
    ].filter(Boolean);

    const style: CSSProperties = {};
    if (vtName) style["viewTransitionName"] = vtName;

    const willGoBack = back && !isNavigatingBackLeavesWebsite();

    const to = willGoBack ? (-1 as unknown as string) : props.to;

    return (
      <RL
        className={classes.join(" ")}
        preventScrollReset={willGoBack}
        style={style}
        {...props}
        to={to}
        ref={ref}
      >
        {children}
      </RL>
    );
  }
);
