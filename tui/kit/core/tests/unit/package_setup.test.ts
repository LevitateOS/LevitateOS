import { describe, expect, it } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

type PackageJson = {
	dependencies?: Record<string, string>;
	scripts?: Record<string, string>;
	exports?: Record<string, string>;
};

const PACKAGE_ROOT = resolve(import.meta.dir, "../..");

function readPackageJson(): PackageJson {
	const path = resolve(PACKAGE_ROOT, "package.json");
	return JSON.parse(readFileSync(path, "utf8")) as PackageJson;
}

describe("package setup", () => {
	it("keeps runtime deps for React/Ink with core development scripts", () => {
		const pkg = readPackageJson();

		expect(pkg.dependencies?.ink).toBeTruthy();
		expect(pkg.dependencies?.react).toBeTruthy();
		expect(pkg.scripts?.build).toContain("tsc");
		expect(pkg.scripts?.lint).toContain("oxlint");
		expect(pkg.scripts?.format).toContain("oxfmt");
		expect(pkg.scripts?.typecheck).toContain("--noEmit");
		expect(pkg.scripts?.test).toContain("bun test");
		expect(pkg.scripts?.check).toContain("bun run typecheck");
		expect(pkg.scripts?.precommit).toContain("format:check");
		expect(pkg.scripts?.precommit).toContain("bun run lint");
	});

	it("ships a package pre-commit hook script", () => {
		expect(existsSync(resolve(PACKAGE_ROOT, "scripts/pre-commit.sh"))).toBe(true);
	});

	it("exports strict stable entrypoints", () => {
		const pkg = readPackageJson();
		const exports = pkg.exports ?? {};

		expect(exports["."]).toBe("./src/index.ts");
		expect(exports["./theme"]).toBe("./src/theme.ts");
		expect(Object.keys(exports).sort()).toEqual([".", "./theme"]);

		for (const target of Object.values(exports)) {
			expect(target.startsWith("./src/")).toBe(true);
			expect(existsSync(resolve(PACKAGE_ROOT, target))).toBe(true);
		}
	});

	it("documents React+Ink architecture and docs boundary in README", () => {
		const readme = readFileSync(resolve(PACKAGE_ROOT, "README.md"), "utf8");

		expect(readme.includes("React + Ink")).toBe(true);
		expect(readme.includes("Docs-domain rendering/navigation no longer lives in `tui-kit`")).toBe(
			true,
		);
	});

	it("has no docs subpath export", () => {
		const pkg = readPackageJson();
		expect(pkg.exports?.["./docs"]).toBeUndefined();
	});
});
