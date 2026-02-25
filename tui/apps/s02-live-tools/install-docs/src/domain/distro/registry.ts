import { levitateProfile } from "./levitate-profile";
import type { DistroId, DistroProfile } from "./profile";

const profiles: Record<DistroId, DistroProfile> = {
	levitate: levitateProfile,
	acorn: {
		id: "acorn",
		title: "AcornOS Installation Docs",
		allowedProducts: ["acorn", "shared"],
		allowedScopes: ["install"],
		defaultSlug: "installation",
		allowsMeta: () => false,
	},
	ralph: {
		id: "ralph",
		title: "RalphOS Installation Docs",
		allowedProducts: ["ralph", "shared"],
		allowedScopes: ["install"],
		defaultSlug: "installation",
		allowsMeta: () => false,
	},
};

export function getDistroProfile(id: DistroId = "levitate"): DistroProfile {
	return profiles[id];
}
