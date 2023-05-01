import { FC, PropsWithChildren } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";
import { NavLink as RL, LinkProps as RLP } from "react-router-dom";
import { isNavigatingBackLeavesWebsite } from "../router-provider";

interface LinkProps extends PropsWithChildren, StyleProps, FontVariantProps, RLP {
  back?: boolean;
}

export const Link: FC<LinkProps> = ({ children, variant, margin, back, ...props }) => {
  const classes = ["link", `font-variant-${variant}`, ...marginToClasses(margin)];

  const willGoBack = back && !isNavigatingBackLeavesWebsite();

  const to = willGoBack ? (-1 as unknown as string) : props.to;

  return (
    <RL className={classes.join(" ")} preventScrollReset={willGoBack} {...props} to={to}>
      {children}
    </RL>
  );
};
