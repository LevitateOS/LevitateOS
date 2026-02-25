import { statSync } from "node:fs";
import { dirname } from "node:path";

export type PreflightCheck = {
	id: string;
	ok: boolean;
	expectation: string;
	observed: string;
	remediation: string;
};

async function commandExists(command: string): Promise<boolean> {
	const proc = Bun.spawn(["bash", "-lc", `command -v ${command} >/dev/null 2>&1`], {
		stdout: "ignore",
		stderr: "ignore",
	});
	return (await proc.exited) === 0;
}

function checkDiskPath(disk: string): PreflightCheck {
	if (disk.trim().length === 0) {
		return {
			id: "disk-path",
			ok: false,
			expectation: "disk path is specified",
			observed: "disk path is empty",
			remediation: "pass --disk /dev/<target>",
		};
	}

	try {
		statSync(disk);
		return {
			id: "disk-path",
			ok: true,
			expectation: "target disk path exists",
			observed: `${disk} exists`,
			remediation: "none",
		};
	} catch {
		const parent = dirname(disk);
		return {
			id: "disk-path",
			ok: false,
			expectation: "target disk path exists",
			observed: `${disk} missing`,
			remediation: `verify device path under ${parent}`,
		};
	}
}

export async function runPreflightChecks(disk: string): Promise<PreflightCheck[]> {
	const checks: PreflightCheck[] = [];

	const uidProc = Bun.spawn(["id", "-u"], { stdout: "pipe", stderr: "ignore" });
	const uid = (await new Response(uidProc.stdout).text()).trim();
	checks.push({
		id: "root",
		ok: uid === "0",
		expectation: "effective uid is root",
		observed: `uid=${uid || "unknown"}`,
		remediation: "run command with sudo/root",
	});

	for (const tool of ["recpart", "sfdisk", "mkfs.ext4", "mkfs.vfat", "mount", "umount"]) {
		const ok = await commandExists(tool);
		checks.push({
			id: `tool-${tool}`,
			ok,
			expectation: `required tool '${tool}' exists in PATH`,
			observed: ok ? `${tool} found` : `${tool} missing`,
			remediation: `install '${tool}' and retry`,
		});
	}

	checks.push(checkDiskPath(disk));
	return checks;
}
