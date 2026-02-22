import type { ScreenHandle } from "../core/screen";
import type { WizardStep } from "../wizard/steps";

export type WidgetBaseOptions = {
  screen: ScreenHandle;
  title: string;
  sidebarTitle?: string;
  sidebarSubtitle?: string;
  steps?: ReadonlyArray<WizardStep>;
  currentStep?: string;
};
