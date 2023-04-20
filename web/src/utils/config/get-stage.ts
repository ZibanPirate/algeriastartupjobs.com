import memoize from "lodash/memoize";

const stages = ["development", "staging", "production"] as const;
type Stage = typeof stages[number];

const providedStage = import.meta.env.MODE as Stage;

console.log("providedStage", providedStage);

const _getStage = (): { stage: Stage; stageIndex: number } => {
  let stageIndex = stages.indexOf(providedStage);
  if (stageIndex === -1)
    return {
      stage: "development",
      stageIndex: 0,
    };

  return {
    stage: providedStage,
    stageIndex,
  };
};
export const getStage = memoize(_getStage);
