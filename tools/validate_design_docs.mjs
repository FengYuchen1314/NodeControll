import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const errors = [];

function read(rel) {
  return fs.readFileSync(path.join(root, rel), 'utf8');
}

function sourceRows(rel) {
  const re = /^\|\s*((?:MMWX|MMW)-[A-Z0-9]+-\d{3}|(?:PRO|NOLIC)-\d{3})\s*\|/gm;
  return [...read(rel).matchAll(re)].map((m) => m[1]);
}

const sourceFiles = [
  ['docs/02-upstream-features/FEATURE_CATALOG.md', 128],
  ['docs/03-mmwx-gap/X_FEATURE_CATALOG.md', 213],
  ['docs/03-mmwx-gap/PRO_FEATURES.md', 17],
];

const sourceIds = [];
for (const [file, expected] of sourceFiles) {
  const ids = sourceRows(file);
  if (ids.length !== expected) errors.push(`${file}: expected ${expected} rows, got ${ids.length}`);
  sourceIds.push(...ids);
}

const duplicateSources = sourceIds.filter((id, index) => sourceIds.indexOf(id) !== index);
if (duplicateSources.length) errors.push(`duplicate source IDs: ${[...new Set(duplicateSources)].join(', ')}`);

const traceFile = 'docs/04-rebuild/REQUIREMENTS_TRACEABILITY.md';
const traceLines = read(traceFile).split(/\r?\n/).filter((line) => /^\| (?:MMWX|MMW|PRO|NOLIC)-/.test(line));
if (traceLines.length !== 358) errors.push(`${traceFile}: expected 358 rows, got ${traceLines.length}`);

const traceIds = [];
const targetIds = [];
const allowedStatuses = new Set(['planned', 'implemented', 'verified', 'deferred-blocked']);
for (const line of traceLines) {
  const cells = line.split('|').slice(1, -1).map((cell) => cell.trim());
  if (cells.length !== 9) {
    errors.push(`trace row has ${cells.length} cells: ${line.slice(0, 100)}`);
    continue;
  }
  const [sourceId, targetId, title, wp, design, test, implementation, run, status] = cells;
  traceIds.push(sourceId);
  targetIds.push(targetId);
  if (targetId !== `NC-${sourceId}`) errors.push(`${sourceId}: target ID mismatch ${targetId}`);
  if (!title || !/^\[WP-\d{2}\]\(\.\/IMPLEMENTATION_PLAN\.md\)$/.test(wp)) errors.push(`${sourceId}: missing title/WP`);
  if (!design || !test) errors.push(`${sourceId}: missing design/test mapping`);
  if (!allowedStatuses.has(status)) errors.push(`${sourceId}: invalid status ${status}`);
  if (status === 'verified' && (implementation === '—' || run === '—')) errors.push(`${sourceId}: verified without implementation/run`);
}

for (const id of sourceIds) if (!traceIds.includes(id)) errors.push(`source ID missing from trace: ${id}`);
for (const id of traceIds) if (!sourceIds.includes(id)) errors.push(`trace ID missing from source: ${id}`);
for (const [label, ids] of [['trace source', traceIds], ['target', targetIds]]) {
  const duplicates = ids.filter((id, index) => ids.indexOf(id) !== index);
  if (duplicates.length) errors.push(`duplicate ${label} IDs: ${[...new Set(duplicates)].join(', ')}`);
}

const designDir = path.join(root, 'docs/04-rebuild');
const designFiles = fs.readdirSync(designDir).filter((name) => name.endsWith('.md'));
function markdownFiles(directory) {
  const found = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const absolute = path.join(directory, entry.name);
    if (entry.isDirectory()) found.push(...markdownFiles(absolute));
    else if (entry.isFile() && entry.name.endsWith('.md')) found.push(absolute);
  }
  return found;
}

const authoredFiles = [
  path.join(root, 'README.md'),
  ...markdownFiles(path.join(root, 'docs')).filter(
    (file) => !file.includes(path.join('docs', '03-mmwx-gap', 'evidence', 'extracted')),
  ),
];

for (const abs of authoredFiles) {
  const text = fs.readFileSync(abs, 'utf8');
  const linkRe = /\[[^\]]*\]\(([^)]+)\)/g;
  for (const match of text.matchAll(linkRe)) {
    let target = match[1].trim();
    if (target.startsWith('<') && target.endsWith('>')) target = target.slice(1, -1);
    if (/^(?:https?:|mailto:|#)/i.test(target)) continue;
    target = decodeURIComponent(target.split('#', 1)[0]);
    if (!target) continue;
    const resolved = path.resolve(path.dirname(abs), target);
    if (!fs.existsSync(resolved)) errors.push(`${path.relative(root, abs)}: broken link ${match[1]}`);
  }
}

const planned = traceLines.filter((line) => /\| planned \|$/.test(line)).length;
const implemented = traceLines.filter((line) => /\| implemented \|$/.test(line)).length;
const verified = traceLines.filter((line) => /\| verified \|$/.test(line)).length;

if (errors.length) {
  console.error(`design validation failed (${errors.length})`);
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log(JSON.stringify({
  source_requirements: sourceIds.length,
  trace_rows: traceLines.length,
  design_documents: designFiles.length,
  authored_documents_checked: authoredFiles.length,
  statuses: { planned, implemented, verified },
  broken_links: 0,
}, null, 2));
