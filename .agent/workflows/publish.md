---
description: How to publish packages to npm
---

# Publishing Workflow

This guide explains how to publish packages from the monorepo to npm.

## Prerequisites

1. Ensure you have commit access to the repository
2. NPM_TOKEN secret is configured in GitHub (repository admin)
3. All changes are committed and pushed to main

## Publishing Process

### Step 1: Prepare for Release

```bash
# Pull latest changes
git pull origin main

# Check current versions
node scripts/publish-helper.js versions

# Verify everything builds
pnpm run release:prepare
```

### Step 2: Bump Version

Choose the appropriate version bump type:

```bash
# For bug fixes
pnpm run version:patch

# For new features (backwards compatible)
pnpm run version:minor

# For breaking changes
pnpm run version:major
```

For individual packages:

```bash
node scripts/version-manager.js bump patch core
node scripts/version-manager.js bump minor react
```

### Step 3: Review Changes

```bash
# Check version consistency
pnpm run version:check

# Review package.json changes
git diff

# Verify with dry-run
pnpm run publish:dry-run
```

### Step 4: Commit and Create Tags

```bash
# Commit version bump
git add .
git commit -m "chore: bump version to X.X.X"

# Create git tags
pnpm run release:tag

# Review tags
git tag -l
```

### Step 5: Push to Trigger Publishing

```bash
# Push commits and tags
git push origin main
git push --tags
```

**GitHub Actions will automatically:**

- Run tests and builds
- Publish to npm if version doesn't exist
- Create GitHub releases with changelogs

## Manual Publishing (if needed)

If you need to publish manually:

```bash
# Navigate to package directory
cd packages/core

# Build the package
pnpm run build

# Publish to npm
pnpm publish --access public --no-git-checks
```

## Publishing Pre-releases

For beta, alpha, or RC versions:

```bash
# Bump to pre-release version (manually edit package.json)
# Example: 0.2.0-beta.1

# Create pre-release tag
git tag core-v0.2.0-beta.1
git push --tags
```

GitHub Actions will automatically publish to npm with `--tag next` flag.

## Tag Naming Convention

Tags must follow this pattern:

- Core package: `core-v1.2.3`
- React package: `react-v1.2.3`
- Solid package: `solid-v1.2.3`
- Svelte package: `svelte-v1.2.3`

Pre-release versions:

- `core-v1.2.3-beta.1`
- `react-v1.2.3-alpha.1`
- `solid-v1.2.3-rc.1`

## Troubleshooting

### Package already published error

If you see "Package already published":

- Check npm registry: `npm view @rockerrishabh/rich-text-editor-core`
- Bump version again if needed
- Delete and recreate tags if version was updated

### Build failures

```bash
# Clean everything and rebuild
pnpm run clean
pnpm install
pnpm run build
```

### Tag already exists

```bash
# Delete local tag
git tag -d core-v1.2.3

# Delete remote tag
git push origin :refs/tags/core-v1.2.3

# Recreate tag
git tag core-v1.2.3
git push --tags
```

## Utility Scripts

```bash
# Check version consistency
pnpm run version:check

# View all package versions
node scripts/publish-helper.js versions

# Dry-run publish test
pnpm run publish:dry-run

# Check if packages are ready
pnpm run publish:check
```
