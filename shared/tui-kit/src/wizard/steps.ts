export type WizardStep = {
  id: string;
  label: string;
  description?: string;
};

export function stepIndexById(steps: ReadonlyArray<WizardStep>, stepId: string): number {
  if (typeof stepId !== "string" || stepId.trim().length === 0) {
    return -1;
  }

  return steps.findIndex((step) => step.id === stepId);
}
