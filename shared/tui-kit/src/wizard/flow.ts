import type { WizardStep } from "./steps";
import { stepIndexById } from "./steps";
import { clampNumber, toInt } from "../internal/numbers";

export type WizardFlowState = {
  steps: ReadonlyArray<WizardStep>;
  currentIndex: number;
};

export function getInitialWizardStepId(steps: ReadonlyArray<WizardStep>): string | null {
  if (steps.length === 0) {
    return null;
  }

  const first = steps[0];
  return typeof first.id === "string" && first.id.length > 0 ? first.id : null;
}

export function createWizardFlow(
  steps: ReadonlyArray<WizardStep>,
  startStepId?: string,
): WizardFlowState {
  const index = startStepId !== undefined ? stepIndexById(steps, startStepId) : 0;
  const currentIndex = index >= 0 ? index : 0;

  return {
    steps,
    currentIndex,
  };
}

export function getCurrentStep(state: WizardFlowState): WizardStep | undefined {
  return state.steps[state.currentIndex];
}

export function moveWizardStep(state: WizardFlowState, delta: number): WizardFlowState {
  if (state.steps.length === 0) {
    return {
      steps: state.steps,
      currentIndex: 0,
    };
  }

  const maxIndex = Math.max(0, state.steps.length - 1);
  const baseIndex = toInt(state.currentIndex, 0);
  const stepDelta = toInt(delta, 0);
  const nextIndex = clampNumber(baseIndex + stepDelta, 0, maxIndex);

  return {
    steps: state.steps,
    currentIndex: nextIndex,
  };
}

export async function runWizard(steps: ReadonlyArray<WizardStep>): Promise<string | null> {
  return getInitialWizardStepId(steps);
}
