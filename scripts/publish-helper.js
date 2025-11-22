#!/usr/bin/env node

/**
 * Publish Helper for monorepo packages
 * Validates packages before publishing and performs dry-run testing
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const PACKAGES_DIR = path.join(__dirname, '..', 'packages');
const PUBLISHABLE_PACKAGES = ['core', 'react', 'solid', 'svelte'];

class PublishHelper {
  constructor() {
    this.packages = this.loadPackages();
  }

  loadPackages() {
    const packages = {};
    for (const pkg of PUBLISHABLE_PACKAGES) {
      const pkgPath = path.join(PACKAGES_DIR, pkg);
      const pkgJsonPath = path.join(pkgPath, 'package.json');
      if (fs.existsSync(pkgJsonPath)) {
        packages[pkg] = {
          dir: pkgPath,
          path: pkgJsonPath,
          data: JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'))
        };
      }
    }
    return packages;
  }

  checkPackage(pkgName) {
    console.log(`\n🔍 Checking package: ${pkgName}\n`);
    
    const pkg = this.packages[pkgName];
    if (!pkg) {
      console.error(`❌ Package ${pkgName} not found`);
      return false;
    }

    let allGood = true;

    // Check required fields
    const requiredFields = ['name', 'version', 'description', 'main', 'types', 'license'];
    for (const field of requiredFields) {
      if (!pkg.data[field]) {
        console.log(`❌ Missing required field: ${field}`);
        allGood = false;
      } else {
        console.log(`✅ ${field}: ${pkg.data[field]}`);
      }
    }

    // Check files array
    if (!pkg.data.files || pkg.data.files.length === 0) {
      console.log('❌ No files array specified');
      allGood = false;
    } else {
      console.log(`✅ Files to publish: ${pkg.data.files.join(', ')}`);
      
      // Check if dist directory exists
      const distPath = path.join(pkg.dir, 'dist');
      if (pkg.data.files.includes('dist') && !fs.existsSync(distPath)) {
        console.log('❌ dist directory not found - run build first!');
        allGood = false;
      }
    }

    // Check repository info
    if (!pkg.data.repository || !pkg.data.repository.url) {
      console.log('⚠️  No repository URL specified');
    } else {
      console.log(`✅ Repository: ${pkg.data.repository.url}`);
    }

    // Check if package is already published
    const packageName = pkg.data.name;
    const version = pkg.data.version;
    console.log(`\n📦 Checking if ${packageName}@${version} is already published...`);
    
    try {
      execSync(`npm view ${packageName}@${version}`, { stdio: 'ignore' });
      console.log(`⚠️  WARNING: ${packageName}@${version} is already published!`);
      allGood = false;
    } catch {
      console.log(`✅ Version ${version} is available for publishing`);
    }

    return allGood;
  }

  checkAll() {
    console.log('🔍 Checking all packages for publishing...\n');
    console.log('=' .repeat(60));

    let overallResult = true;
    for (const pkgName of PUBLISHABLE_PACKAGES) {
      const result = this.checkPackage(pkgName);
      if (!result) {
        overallResult = false;
      }
      console.log('=' .repeat(60));
    }

    if (overallResult) {
      console.log('\n✅ All packages are ready for publishing!\n');
    } else {
      console.log('\n❌ Some packages have issues that need to be fixed.\n');
    }

    return overallResult;
  }

  dryRun(pkgName) {
    console.log(`\n🧪 Running dry-run publish for: ${pkgName || 'all packages'}\n`);

    const packagesToTest = pkgName 
      ? [pkgName] 
      : PUBLISHABLE_PACKAGES;

    for (const pkg of packagesToTest) {
      const pkgInfo = this.packages[pkg];
      if (!pkgInfo) {
        console.error(`❌ Package ${pkg} not found`);
        continue;
      }

      console.log(`\n📦 Testing ${pkg}...`);
      console.log('-'.repeat(60));

      try {
        const output = execSync('pnpm publish --dry-run --no-git-checks', {
          cwd: pkgInfo.dir,
          encoding: 'utf8',
          stdio: 'pipe'
        });
        
        console.log(output);
        console.log(`✅ Dry-run successful for ${pkg}`);
      } catch (error) {
        console.error(`❌ Dry-run failed for ${pkg}:`);
        console.error(error.message);
      }
    }

    console.log('\n📝 Next steps:');
    console.log('   1. Review the dry-run output above');
    console.log('   2. Fix any issues if needed');
    console.log('   3. Create and push tags: pnpm run release:tag && git push --tags');
    console.log('   4. GitHub Actions will automatically publish when tags are pushed\n');
  }

  listVersions() {
    console.log('\n📦 Package Versions:\n');
    
    for (const [name, pkg] of Object.entries(this.packages)) {
      const version = pkg.data.version;
      const npmName = pkg.data.name;
      
      console.log(`${name.padEnd(10)} v${version.padEnd(10)} (${npmName})`);
      
      // Check latest published version
      try {
        const latest = execSync(`npm view ${npmName} version`, { 
          encoding: 'utf8',
          stdio: 'pipe' 
        }).trim();
        console.log(`           Published: v${latest}`);
      } catch {
        console.log(`           Published: Not yet published`);
      }
    }
    console.log();
  }
}

// CLI
const args = process.argv.slice(2);
const command = args[0];
const packageName = args[1];

const helper = new PublishHelper();

switch (command) {
  case 'check':
    if (packageName) {
      const result = helper.checkPackage(packageName);
      process.exit(result ? 0 : 1);
    } else {
      const result = helper.checkAll();
      process.exit(result ? 0 : 1);
    }
    break;

  case 'dry-run':
    helper.dryRun(packageName);
    break;

  case 'versions':
    helper.listVersions();
    break;

  default:
    console.log(`
Publish Helper

Usage:
  node scripts/publish-helper.js check [package]      Check if package(s) ready for publishing
  node scripts/publish-helper.js dry-run [package]    Run dry-run publish test
  node scripts/publish-helper.js versions             List all package versions

Examples:
  node scripts/publish-helper.js check                Check all packages
  node scripts/publish-helper.js check core           Check core package only
  node scripts/publish-helper.js dry-run              Dry-run all packages
  node scripts/publish-helper.js dry-run react        Dry-run react package only
  node scripts/publish-helper.js versions             Show all versions
    `);
    process.exit(1);
}
