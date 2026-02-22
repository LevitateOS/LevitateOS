import { describe, expect, it } from "bun:test";
import {
  createWizardFlow,
  getInitialWizardStepId,
  getCurrentStep,
  moveWizardStep,
  runWizard,
} from "../src/wizard/flow";
import { renderWizardSidebar } from "../src/wizard/sidebar";
import { stepIndexById } from "../src/wizard/steps";

describe("wizard helpers", () => {
  const steps = [
    { id: "disk", label: "Disk" },
    { id: "mode", label: "Mode" },
    { id: "apply", label: "Apply" },
  ];

  it("sidebar marks active step", () => {
    const sidebar = renderWizardSidebar(steps, "mode", {
      title: "recpart",
      subtitle: "wizard",
    });

    expect(sidebar.includes("> 2. Mode")).toBe(true);
  });

  it("flow movement clamps at bounds", () => {
    const state = createWizardFlow(steps, "mode");
    expect(getCurrentStep(state)?.id).toBe("mode");

    const next = moveWizardStep(state, 10);
    expect(getCurrentStep(next)?.id).toBe("apply");

    const prev = moveWizardStep(next, -10);
    expect(getCurrentStep(prev)?.id).toBe("disk");
  });

  it("runWizard returns first step id", async () => {
    await expect(runWizard(steps)).resolves.toBe("disk");
  });

  it("initial step helper returns null for missing or blank ids", () => {
    expect(getInitialWizardStepId([])).toBeNull();
    expect(getInitialWizardStepId([{ id: "", label: "Blank" }])).toBeNull();
  });

  it("flow remains stable with non-finite delta/index", () => {
    const weird = {
      steps,
      currentIndex: Number.NaN,
    };

    const moved = moveWizardStep(weird, Number.NaN);
    expect(moved.currentIndex).toBe(0);
  });

  it("sidebar handles empty steps", () => {
    const sidebar = renderWizardSidebar([], "anything", {
      title: "recpart",
    });

    expect(sidebar.includes("(no steps)")).toBe(true);
  });

  it("step index rejects empty step id", () => {
    expect(stepIndexById(steps, "")).toBe(-1);
  });
});
