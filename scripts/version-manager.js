#!/usr/bin/env node

/**
 * Version Manager for monorepo packages
 * Handles version bumping, tagging, and consistency checks
 */

const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

const PACKAGES_DIR = path.join(__dirname, "..", "packages");
const PUBLISHABLE_PACKAGES = ["core", "react", "solid", "svelte"];

class VersionManager {
  constructor() {
    this.packages = this.loadPackages();
  }

  loadPackages() {
    const packages = {};
    for (const pkg of PUBLISHABLE_PACKAGES) {
      const pkgPath = path.join(PACKAGES_DIR, pkg, "package.json");
      if (fs.existsSync(pkgPath)) {
        packages[pkg] = {
          path: pkgPath,
          data: JSON.parse(fs.readFileSync(pkgPath, "utf8")),
        };
      }
    }
    return packages;
  }

  checkVersionConsistency() {
    console.log("🔍 Checking version consistency across packages...\n");

    const coreVersion = this.packages.core.data.version;
    console.log(`📦 Core version: ${coreVersion}\n`);

    let allConsistent = true;
    for (const [name, pkg] of Object.entries(this.packages)) {
      const version = pkg.data.version;
      const peerDeps = pkg.data.peerDependencies || {};
      const coreDep = peerDeps["@rockerrishabh/rich-text-editor-core"];

      console.log(`📦 ${name}: ${version}`);

      if (name !== "core" && coreDep) {
        const expectedRange = `^${coreVersion}`;
        if (coreDep !== expectedRange) {
          console.log(
            `   ⚠️  WARNING: Core peer dependency is ${coreDep}, expected ${expectedRange}`
          );
          allConsistent = false;
        }
      }
    }

    console.log();
    if (allConsistent) {
      console.log("✅ All versions are consistent!\n");
      return true;
    } else {
      console.log("❌ Version inconsistencies found!\n");
      return false;
    }
  }

  bumpVersion(type = "patch", specific = null) {
    if (specific && !PUBLISHABLE_PACKAGES.includes(specific)) {
      console.error(`❌ Unknown package: ${specific}`);
      process.exit(1);
    }

    const packagesToUpdate = specific ? [specific] : PUBLISHABLE_PACKAGES;

    console.log(
      `🚀 Bumping ${type} version for: ${packagesToUpdate.join(", ")}\n`
    );

    const updatedVersions = {};

    for (const pkgName of packagesToUpdate) {
      const pkg = this.packages[pkgName];
      const currentVersion = pkg.data.version;
      const newVersion = this.calculateNewVersion(currentVersion, type);

      pkg.data.version = newVersion;
      updatedVersions[pkgName] = newVersion;

      console.log(`📦 ${pkgName}: ${currentVersion} → ${newVersion}`);

      // Write updated package.json
      fs.writeFileSync(pkg.path, JSON.stringify(pkg.data, null, 2) + "\n");
    }

    // Update peer dependencies in framework packages
    if (!specific || specific === "core") {
      const newCoreVersion =
        updatedVersions.core || this.packages.core.data.version;
      this.updateCorePeerDependencies(newCoreVersion);
    }

    console.log("\n✅ Versions bumped successfully!\n");
    console.log("📝 Next steps:");
    console.log("   1. Review the changes: git diff");
    console.log(
      '   2. Commit the changes: git add . && git commit -m "chore: bump version"'
    );
    console.log("   3. Create tags: pnpm run release:tag");
    console.log("   4. Push with tags: git push && git push --tags\n");
  }

  calculateNewVersion(version, type) {
    const [major, minor, patch] = version.split(".").map(Number);

    switch (type) {
      case "major":
        return `${major + 1}.0.0`;
      case "minor":
        return `${major}.${minor + 1}.0`;
      case "patch":
      default:
        return `${major}.${minor}.${patch + 1}`;
    }
  }

  updateCorePeerDependencies(coreVersion) {
    const expectedRange = `^${coreVersion}`;

    for (const [name, pkg] of Object.entries(this.packages)) {
      if (
        name !== "core" &&
        pkg.data.peerDependencies &&
        pkg.data.peerDependencies["@rockerrishabh/rich-text-editor-core"]
      ) {
        pkg.data.peerDependencies["@rockerrishabh/rich-text-editor-core"] =
          expectedRange;
        fs.writeFileSync(pkg.path, JSON.stringify(pkg.data, null, 2) + "\n");
        console.log(`   Updated ${name} peer dependency to ${expectedRange}`);
      }
    }
  }

  createTags(specific = null) {
    console.log("🏷️  Creating git tags...\n");

    const packagesToTag = specific ? [specific] : PUBLISHABLE_PACKAGES;

    for (const pkgName of packagesToTag) {
      const pkg = this.packages[pkgName];
      const version = pkg.data.version;
      const tagName = `${pkgName}-v${version}`;

      try {
        // Check if tag already exists
        execSync(`git rev-parse ${tagName}`, { stdio: "ignore" });
        console.log(`⚠️  Tag ${tagName} already exists, skipping...`);
      } catch {
        // Tag doesn't exist, create it
        execSync(`git tag -a ${tagName} -m "Release ${pkgName} v${version}"`, {
          stdio: "inherit",
        });
        console.log(`✅ Created tag: ${tagName}`);
      }
    }

    console.log("\n✅ Tags created successfully!\n");
    console.log("📝 Push tags with: git push --tags\n");
  }
}

// CLI
const args = process.argv.slice(2);
const command = args[0];
const subcommand = args[1];
const packageName = args[2];

const manager = new VersionManager();

switch (command) {
  case "check":
    const consistent = manager.checkVersionConsistency();
    process.exit(consistent ? 0 : 1);
    break;

  case "bump":
    if (!["patch", "minor", "major"].includes(subcommand)) {
      console.error(
        "❌ Invalid version bump type. Use: patch, minor, or major"
      );
      process.exit(1);
    }
    manager.bumpVersion(subcommand, packageName);
    break;

  case "tag":
    manager.createTags(packageName);
    break;

  default:
    console.log(`
Version Manager

Usage:
  node scripts/version-manager.js check                   Check version consistency
  node scripts/version-manager.js bump <type> [package]   Bump version (type: patch|minor|major)
  node scripts/version-manager.js tag [package]           Create git tags for packages

Examples:
  node scripts/version-manager.js check
  node scripts/version-manager.js bump patch              Bump patch version for all packages
  node scripts/version-manager.js bump minor core         Bump minor version for core only
  node scripts/version-manager.js tag                     Create tags for all packages
  node scripts/version-manager.js tag react               Create tag for react package only
    `);
    process.exit(1);
}
