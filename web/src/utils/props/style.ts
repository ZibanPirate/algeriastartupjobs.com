import memoize from "lodash/memoize";

export type CSSNumber = `${".5" | 0 | 1 | 2 | 3}`;

type MarginOrPadding =
  | "auto"
  | CSSNumber
  | `${CSSNumber} ${CSSNumber}`
  | `${CSSNumber} ${CSSNumber} ${CSSNumber}`
  | `${CSSNumber} ${CSSNumber} ${CSSNumber} ${CSSNumber}`;

export interface StyleProps {
  margin?: MarginOrPadding;
  padding?: MarginOrPadding;
}

const _marginOrPaddingToClasses = <T extends "margin" | "padding">(
  value: MarginOrPadding | "" = "",
  type: T
): Array<`${T}-${"top" | "right" | "bottom" | "left"}-${CSSNumber}` | `${T}-auto`> => {
  if (value === "auto") return [`${type}-auto`];
  const [top, right, bottom, left] = value.split(" ") as CSSNumber[];
  if (left)
    return [
      `${type}-top-${top}`,
      `${type}-left-${left}`,
      `${type}-right-${right}`,
      `${type}-bottom-${bottom}`,
    ];
  else if (bottom)
    return [
      `${type}-top-${top}`,
      `${type}-left-${right}`,
      `${type}-bottom-${bottom}`,
      `${type}-right-${right}`,
    ];
  else if (right)
    return [
      `${type}-top-${top}`,
      `${type}-right-${right}`,
      `${type}-bottom-${top}`,
      `${type}-left-${right}`,
    ];
  else if (top)
    return [
      `${type}-top-${top}`,
      `${type}-right-${top}`,
      `${type}-bottom-${top}`,
      `${type}-left-${top}`,
    ];
  else return [];
};

const _marginToClasses = (margin: MarginOrPadding | "" = "") =>
  _marginOrPaddingToClasses(margin, "margin");
export const marginToClasses = memoize(_marginToClasses);

const _paddingToClasses = (padding: MarginOrPadding | "" = "") =>
  _marginOrPaddingToClasses(padding, "padding");
export const paddingToClasses = memoize(_paddingToClasses);

export interface FontVariantProps {
  variant: "v1" | "v2" | "v3" | "v4" | "v5";
}
