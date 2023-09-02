import { PropsWithChildren, useEffect, useRef, useState } from "react";
import "./style.css";
import { StyleProps, paddingToClasses } from "src/utils/props/style";

export type DialogName = `${Capitalize<string>}`;

export type UseDialogReturnType<N extends string> = { [key in `is${N}DialogOpen`]: boolean } & {
  [key in `toggle${N}Dialog`]: (isOpen: boolean) => void;
} & { dialogName: N };

export const useDialogProps = <N extends DialogName>(
  dialogName: N
): { [key in `propsFor${N}Dialog`]: UseDialogReturnType<N> } => {
  const [isOpen, setIsOpen] = useState(false);
  return {
    [`propsFor${dialogName}Dialog`]: {
      [`is${dialogName}DialogOpen`]: isOpen,
      [`toggle${dialogName}Dialog`]: setIsOpen,
      dialogName,
    },
  } as { [key in `propsFor${N}Dialog`]: UseDialogReturnType<N> };
};

export type DialogProps<N extends DialogName> = PropsWithChildren &
  Omit<StyleProps, "margin"> &
  UseDialogReturnType<N>;

export const Dialog = <N extends DialogName>({
  children,
  padding,
  dialogName,
  ...props
}: DialogProps<N>) => {
  const classes = ["dialog", ...paddingToClasses(padding)];

  const ref = useRef<HTMLDialogElement>(null);
  // @ts-ignore
  const open = props[`is${dialogName}DialogOpen`] as boolean;
  // @ts-ignore
  const toggle = props[`toggle${dialogName}Dialog`] as (isOpen: boolean) => void;

  useEffect(() => {
    if (!ref.current) return;

    if (open) ref.current.showModal();
    else ref.current.close();

    if (open) {
      const close = (event: MouseEvent) => {
        if (!ref.current) return;

        const { clientX, clientY } = event;
        const { left, right, top, bottom } = ref.current.getBoundingClientRect();
        if (clientX < left || clientX > right || clientY < top || clientY > bottom) toggle(false);
      };
      ref.current.addEventListener("click", close);
      return () => ref.current?.removeEventListener("click", close);
    }
  }, [open, ref.current]);

  return (
    <dialog ref={ref} className={classes.join(" ")} onClose={() => toggle(false)}>
      {children}
    </dialog>
  );
};
