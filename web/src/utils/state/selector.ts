import { useSelector } from "react-redux";
import { SliceName, State } from "src/state";

export const useSliceSelector = <K extends SliceName>(
  sliceKey: K,
  equalityFn?: Parameters<typeof useSelector>["1"]
) => useSelector<State, State[K]>((state) => state[sliceKey], equalityFn);
