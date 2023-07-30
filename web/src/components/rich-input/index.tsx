import { CSSProperties, FC, HTMLAttributes } from "react";
import "./style.css";
import { FontVariantProps, StyleProps, marginToClasses } from "src/utils/props/style";

export interface RichInputProps
  extends StyleProps,
    FontVariantProps,
    Pick<HTMLAttributes<HTMLTextAreaElement>, "id"> {
  placeholder?: string;
  value: string;
  setValue: (value: string) => void;
  stretch?: boolean;
  disabled?: boolean;
  width?: CSSProperties["width"];
  autoRows?: boolean;
}

export const RichInput: FC<RichInputProps> = ({
  variant,
  margin,
  placeholder,
  value,
  setValue,
  stretch = false,
  width,
  autoRows = false,
  ...props
}) => {
  const classes = [
    "rich-input",
    `font-variant-${variant}`,
    stretch ? "stretch" : "width100",
    ...marginToClasses(margin),
  ];

  const style: CSSProperties = {};
  if (width) style.width = width;

  return (
    <textarea
      className={classes.join(" ")}
      placeholder={placeholder}
      value={value}
      style={style}
      onChange={(e) => setValue(e.target.value)}
      rows={autoRows ? (value?.split("\n").length || 1) + 2 : undefined}
      {...props}
    />
  );
};
