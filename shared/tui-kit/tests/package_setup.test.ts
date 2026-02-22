import { describe, expect, it } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

type PackageJson = {
  dependencies?: Record<string, string>;
  scripts?: Record<string, string>;
  exports?: Record<string, string>;
};

const PACKAGE_ROOT = resolve(import.meta.dir, "..");

function readPackageJson(): PackageJson {
  const path = resolve(PACKAGE_ROOT, "package.json");
  return JSON.parse(readFileSync(path, "utf8")) as PackageJson;
}

describe("package setup", () => {
  it("keeps blessed as a runtime dependency with core development scripts", () => {
    const pkg = readPackageJson();

    expect(pkg.dependencies?.blessed).toBeTruthy();
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

  it("exports only flat src entrypoints that exist on disk", () => {
    const pkg = readPackageJson();
    const exports = pkg.exports ?? {};

    expect(Object.keys(exports).length).toBeGreaterThan(0);

    for (const target of Object.values(exports)) {
      expect(target.startsWith("./src/")).toBe(true);
      expect(target.includes("packages/")).toBe(false);
      expect(existsSync(resolve(PACKAGE_ROOT, target))).toBe(true);
    }
  });

  it("documents scripts and flattened package intent in README", () => {
    const readme = readFileSync(resolve(PACKAGE_ROOT, "README.md"), "utf8");

    expect(readme.includes("## Scripts")).toBe(true);
    expect(readme.includes("single flattened package")).toBe(true);
    expect(readme.includes("no nested `packages/*`")).toBe(true);
  });
});
