import type { WizardStep } from "./steps";

export type SidebarOptions = {
  title?: string;
  subtitle?: string;
};

export function renderWizardSidebar(
  steps: ReadonlyArray<WizardStep>,
  currentStepId: string,
  options: SidebarOptions = {},
): string {
  const lines: string[] = [];
  const safeTitle = typeof options.title === "string" ? options.title.trim() : "";
  const safeSubtitle = typeof options.subtitle === "string" ? options.subtitle.trim() : "";

  if (safeTitle.length > 0) {
    lines.push(safeTitle);
  }

  if (safeSubtitle.length > 0) {
    lines.push(safeSubtitle);
  }

  if (lines.length > 0) {
    lines.push("");
  }

  if (steps.length === 0) {
    lines.push("(no steps)");
    return lines.join("\n");
  }

  const selectedStepId = steps.some((step) => step.id === currentStepId)
    ? currentStepId
    : steps[0].id;

  for (const [index, step] of steps.entries()) {
    const active = step.id === selectedStepId;
    const marker = active ? ">" : " ";
    const label = typeof step.label === "string" ? step.label.trim() : "";
    lines.push(`${marker} ${index + 1}. ${label.length > 0 ? label : step.id}`);
  }

  return lines.join("\n");
}
