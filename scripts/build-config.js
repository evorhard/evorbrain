#!/usr/bin/env node

/**
 * Build configuration script for EvorBrain
 * Handles environment-specific configuration and build optimization
 */

import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, '..');

// Build modes
const BUILD_MODES = {
  development: {
    minify: false,
    sourcemap: true,
    debug: true,
    optimize: false,
    bundle: false
  },
  staging: {
    minify: true,
    sourcemap: true,
    debug: false,
    optimize: true,
    bundle: false
  },
  production: {
    minify: true,
    sourcemap: false,
    debug: false,
    optimize: true,
    bundle: true
  }
};

// Get build mode from environment or default to development
const buildMode = process.env.BUILD_MODE || 'development';
const config = BUILD_MODES[buildMode];

if (!config) {
  console.error(`❌ Invalid build mode: ${buildMode}`);
  console.error(`   Valid modes: ${Object.keys(BUILD_MODES).join(', ')}`);
  process.exit(1);
}

console.log(`🏗️  Building EvorBrain in ${buildMode} mode...`);
console.log(`   Configuration:`, JSON.stringify(config, null, 2));

// Update Tauri configuration based on build mode
function updateTauriConfig() {
  const tauriConfigPath = join(rootDir, 'apps/desktop/src-tauri/tauri.conf.json');
  const tauriConfig = JSON.parse(readFileSync(tauriConfigPath, 'utf8'));

  // Update build configuration
  if (buildMode === 'production') {
    tauriConfig.build.frontendDist = '../dist';
    tauriConfig.build.devUrl = null;
  } else {
    tauriConfig.build.devUrl = 'http://localhost:5173';
  }

  // Update bundle configuration
  tauriConfig.bundle.active = config.bundle;

  // Update security configuration for production
  if (buildMode === 'production') {
    // Stricter CSP for production
    tauriConfig.app.security.csp['script-src'] = "'self'";
    tauriConfig.app.security.csp['style-src'] = "'self' 'unsafe-inline'";
  }

  writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2));
  console.log(`✅ Updated Tauri configuration for ${buildMode} mode`);
}

// Update Vite configuration
function createViteEnv() {
  const envContent = `# Auto-generated build configuration
VITE_BUILD_MODE=${buildMode}
VITE_ENABLE_SOURCEMAP=${config.sourcemap}
VITE_ENABLE_MINIFY=${config.minify}
VITE_ENABLE_DEBUG=${config.debug}
`;

  const envPath = join(rootDir, 'apps/desktop/.env.build');
  writeFileSync(envPath, envContent);
  console.log(`✅ Created Vite environment configuration`);
}

// Main build configuration
try {
  updateTauriConfig();
  createViteEnv();
  
  console.log(`\n✅ Build configuration complete!`);
  console.log(`   Run 'pnpm build:tauri' to build the application`);
} catch (error) {
  console.error(`❌ Build configuration failed:`, error);
  process.exit(1);
}