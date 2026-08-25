const npmPackages = [
  'vue',
  'vuetify',
  'vite',
  'typescript',
  'pnpm',
  '@vitejs/plugin-vue',
  'vue-router',
  'pinia',
  '@tanstack/vue-query',
  'vue-i18n',
  'vee-validate',
  'zod',
  'vitest',
  '@playwright/test',
  'eslint',
  'prettier',
  '@vue/compiler-sfc',
  'vite-plugin-vuetify',
  'vue-tsc',
  '@types/node',
  'typescript-eslint',
  'eslint-plugin-vue',
  'eslint-config-prettier',
  'sass',
  'jsdom',
  '@testing-library/vue',
  '@vitest/coverage-v8',
];

const crates = [
  'axum',
  'tokio',
  'serde',
  'serde_json',
  'sqlx',
  'utoipa',
  'tracing',
  'tracing-subscriber',
  'uuid',
  'time',
  'thiserror',
  'anyhow',
  'tower',
  'tower-http',
  'argon2',
  'secrecy',
  'zeroize',
  'blake3',
  'prost',
  'tonic',
  'opentelemetry',
];

async function json(url) {
  const response = await fetch(url, { headers: { 'user-agent': 'nodecontroll-version-audit/1' } });
  if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);
  return response.json();
}

const rustManifestUrl = 'https://static.rust-lang.org/dist/channel-rust-stable.toml';
const rustResponse = await fetch(rustManifestUrl);
if (!rustResponse.ok) throw new Error(`${rustManifestUrl}: HTTP ${rustResponse.status}`);
const rustManifest = await rustResponse.text();
const rustPackageSection = rustManifest.match(/\[pkg\.rust\]\r?\n([\s\S]*?)(?=\r?\n\[|$)/)?.[1];
const rustVersion = rustPackageSection?.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
const rustDate = rustManifest.match(/^date\s*=\s*"([^"]+)"/m)?.[1];
if (!rustVersion || !rustDate) throw new Error('could not parse Rust stable manifest');

const nodeIndexUrl = 'https://nodejs.org/dist/index.json';
const nodeIndex = await json(nodeIndexUrl);
const currentLts = nodeIndex.find((release) => release.lts);
if (!currentLts) throw new Error('could not find current Node LTS');

const npm = Object.fromEntries(await Promise.all(npmPackages.map(async (name) => {
  const url = `https://registry.npmjs.org/${encodeURIComponent(name)}/latest`;
  const metadata = await json(url);
  return [name, {
    version: metadata.version,
    engines: metadata.engines ?? null,
    peer_dependencies: metadata.peerDependencies ?? null,
    source: url,
  }];
})));

const cargo = Object.fromEntries(await Promise.all(crates.map(async (name) => {
  const url = `https://crates.io/api/v1/crates/${name}`;
  const metadata = await json(url);
  return [name, {
    version: metadata.crate.max_stable_version ?? metadata.crate.max_version,
    source: url,
  }];
})));

console.log(JSON.stringify({
  queried_at: new Date().toISOString(),
  rust: { version: rustVersion, manifest_date: rustDate, source: rustManifestUrl },
  node_lts: {
    version: currentLts.version,
    lts: currentLts.lts,
    release_date: currentLts.date,
    npm: currentLts.npm,
    source: nodeIndexUrl,
  },
  npm,
  cargo,
}, null, 2));
