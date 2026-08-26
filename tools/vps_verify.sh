#!/usr/bin/env bash
set -Eeuo pipefail
export GIT_NO_REPLACE_OBJECTS=1

for git_override in \
  GIT_DIR \
  GIT_WORK_TREE \
  GIT_COMMON_DIR \
  GIT_OBJECT_DIRECTORY \
  GIT_ALTERNATE_OBJECT_DIRECTORIES \
  GIT_INDEX_FILE \
  GIT_NAMESPACE \
  GIT_REPLACE_REF_BASE \
  GIT_SHALLOW_FILE \
  GIT_GRAFT_FILE \
  GIT_QUARANTINE_PATH \
  GIT_CONFIG \
  GIT_CONFIG_GLOBAL \
  GIT_CONFIG_SYSTEM \
  GIT_CONFIG_NOSYSTEM \
  GIT_CONFIG_COUNT \
  GIT_CONFIG_PARAMETERS; do
  if [[ -v "${git_override}" ]]; then
    echo "formal verification refuses Git environment override ${git_override}" >&2
    exit 2
  fi
done
unset git_override

readonly TASK_ROOT="/opt/nodecontroll"
readonly WORKTREE_INPUT="${NODECONTROLL_WORKTREE:-}"
if [[ -z "${WORKTREE_INPUT}" ]]; then
  echo "NODECONTROLL_WORKTREE is required" >&2
  exit 2
fi
readonly WORKTREE="$(readlink -f "${WORKTREE_INPUT}")"
readonly RUNS_ROOT="${TASK_ROOT}/artifacts/test-runs"
readonly RUST_IMAGE="nodecontroll-builder-rust:1.98.0"
readonly RUST_IMAGE_ID="sha256:6ab6185f9998fe126309ed033570b3828808212bb3c4f7edbf88f98892881613"
readonly NODE_IMAGE="nodecontroll-builder-node:24.19.0-pnpm11.24.0"
readonly NODE_IMAGE_ID="sha256:06628671caed76e73560464d4ce47cacb202fcf28d090c0d24f2ead1cc23afcb"
readonly POSTGRES_IMAGE="postgres@sha256:1c59e2c3c818eaa0f0628f695b36e7c9e362d6b219b36a54a32df645cbd7e1af"
readonly POSTGRES_IMAGE_ID="sha256:1c59e2c3c818eaa0f0628f695b36e7c9e362d6b219b36a54a32df645cbd7e1af"
readonly PLAYWRIGHT_IMAGE="mcr.microsoft.com/playwright@sha256:baed2032d533817f3dbe6425de795788430ba345e819a1201337009ba17c9d07"
readonly PLAYWRIGHT_IMAGE_ID="sha256:baed2032d533817f3dbe6425de795788430ba345e819a1201337009ba17c9d07"
readonly PLAYWRIGHT_PACKAGE_VERSION="1.62.0"
readonly PLAYWRIGHT_NODE_VERSION="v24.19.0"
readonly PLAYWRIGHT_BROWSER_VERSION="151.0.7922.34"
readonly PLAYWRIGHT_EXECUTABLE_SHA256="0b20b130e7edd9dd51873be867761295fe0cfad490c2b9a64f95bd3cfc08fa71"
readonly RUN_ID="$(date -u +%Y%m%dT%H%M%S%NZ)-p5"
readonly RUN_DIR="${RUNS_ROOT}/${RUN_ID}"
readonly ACTIONS_ARTIFACT_SNAPSHOT="${RUN_DIR}/provenance/nodecontroll-linux-x86_64-glibc2.36.tar.gz"
readonly SMOKE_CONTAINER="nc-verify-${RUN_ID,,}"
readonly E2E_MASTER_CONTAINER="nc-e2e-master-${RUN_ID,,}"
readonly E2E_BROWSER_CONTAINER="nc-e2e-browser-${RUN_ID,,}"
readonly POSTGRES_CONTAINER="nc-postgres-${RUN_ID,,}"
readonly TEST_NETWORK="nc-test-${RUN_ID,,}"
readonly TEST_SECRET_FILE="${TASK_ROOT}/tmp/${RUN_ID}.root-key"
readonly TEST_SETUP_TOKEN_FILE="${TASK_ROOT}/tmp/${RUN_ID}.setup-token"
readonly E2E_NODE_RUNTIME="${TASK_ROOT}/tmp/${RUN_ID}.e2e-node"
readonly NODE_WORKSPACE="${TASK_ROOT}/tmp/${RUN_ID}.node-workspace"
readonly NODE_PNPM_STORE="${TASK_ROOT}/tmp/${RUN_ID}.pnpm-store"
readonly PRIVATE_CARGO_HOME="${TASK_ROOT}/tmp/${RUN_ID}.cargo-home"
readonly PRIVATE_CARGO_TEST_TARGET="${TASK_ROOT}/tmp/${RUN_ID}.cargo-test-target"
readonly SOURCE_VERIFIER="${RUN_DIR}/provenance/verify_tracked_source.py"
readonly CARGO_INPUT_CLOSURE="${RUN_DIR}/provenance/cargo-home-inputs.json"
readonly PNPM_INPUT_CLOSURE="${RUN_DIR}/provenance/pnpm-store-inputs.json"
readonly AUTH_E2E_ROOT="${RUN_DIR}/browser"
readonly AUTH_E2E_DATABASE_FILE="${AUTH_E2E_ROOT}/database"
readonly AUTH_E2E_DATABASE_TEMP="${AUTH_E2E_ROOT}/.database.temporary"
readonly AUTH_E2E_DATABASE_DUMP_DIR="${AUTH_E2E_ROOT}/database-dump"
readonly AUTH_E2E_DATABASE_DUMP="${AUTH_E2E_DATABASE_DUMP_DIR}/control.sql"
readonly AUTH_E2E_DATABASE_DUMP_TEMP="${AUTH_E2E_DATABASE_DUMP_DIR}/.control.sql.temporary"
readonly AUTH_E2E_RUNTIME_LOGS="${AUTH_E2E_ROOT}/runtime-logs"
readonly AUTH_E2E_MASTER_LOG="${AUTH_E2E_RUNTIME_LOGS}/master-runtime.log"
readonly AUTH_E2E_TEST_ARTIFACTS="${AUTH_E2E_ROOT}/test-artifacts"
readonly AUTH_E2E_TLS_CERTIFICATE="${AUTH_E2E_TEST_ARTIFACTS}/tls-certificate.pem"
readonly AUTH_E2E_GATE_ATTESTATION="${AUTH_E2E_TEST_ARTIFACTS}/gate-attestation.json"
readonly AUTH_E2E_GATE_ATTESTATION_TEMP="${AUTH_E2E_TEST_ARTIFACTS}/.gate-attestation.json.temporary"
readonly AUTH_E2E_HANDSHAKE="${AUTH_E2E_ROOT}/handshake"
readonly AUTH_E2E_BEHAVIOR_MARKER="${AUTH_E2E_HANDSHAKE}/behavior-ready"
readonly AUTH_E2E_SCAN_MARKER="${AUTH_E2E_HANDSHAKE}/scan-ready"
readonly AUTH_E2E_SCAN_MARKER_TEMP="${AUTH_E2E_HANDSHAKE}/.scan-ready.verifier-temporary"
readonly AUTH_E2E_EVIDENCE="${AUTH_E2E_HANDSHAKE}/evidence.json"
readonly AUTH_E2E_BROWSER_LOG="${AUTH_E2E_ROOT}/gate-runtime.log"
readonly AUTH_E2E_CLOSURE="${RUN_DIR}/provenance/browser-e2e-closure.json"
readonly AUTH_E2E_TEMP_ROOT="${TASK_ROOT}/tmp/${RUN_ID}.auth-e2e"
readonly AUTH_E2E_LIVE_DATABASE_DIR="${AUTH_E2E_TEMP_ROOT}/live-database"
readonly AUTH_E2E_LIVE_DATABASE_FILE="${AUTH_E2E_LIVE_DATABASE_DIR}/control.db"
readonly AUTH_E2E_ROOT_KEY_FILE="${AUTH_E2E_TEMP_ROOT}/root-key"
readonly AUTH_E2E_SETUP_TOKEN_FILE="${AUTH_E2E_TEMP_ROOT}/setup-token"
readonly AUTH_E2E_PASSWORD_FILE="${AUTH_E2E_TEMP_ROOT}/password"
readonly AUTH_E2E_TLS_KEY_FILE="${AUTH_E2E_TEMP_ROOT}/tls-key.pem"
readonly AUTH_E2E_TLS_CERTIFICATE_TEMP="${AUTH_E2E_TEMP_ROOT}/tls-certificate.pem"
readonly CYCLONEDX_CLI_VERSION="0.33.1"
readonly CYCLONEDX_CLI_SHA256="bfc8b2538da86fe239bc53658bbb63c1c8c510a293c1e6891aa5bea5d3c58746"
readonly CYCLONEDX_CLI_URL="https://github.com/CycloneDX/cyclonedx-cli/releases/download/v0.33.1/cyclonedx-linux-x64"
readonly CYCLONEDX_CLI_FILE="${TASK_ROOT}/tmp/${RUN_ID}.cyclonedx-cli"
readonly ACTIONS_ARTIFACT_INPUT="${NODECONTROLL_ACTIONS_ARTIFACT:-}"
readonly GITHUB_RUN_ID="${NODECONTROLL_GITHUB_RUN_ID:-}"
readonly GITHUB_ARTIFACT_ID="${NODECONTROLL_GITHUB_ARTIFACT_ID:-}"

if [[ "$(pwd -P)" != "${WORKTREE}" ]]; then
  echo "run this script from ${WORKTREE}" >&2
  exit 2
fi
case "${WORKTREE}" in
  /opt/nodecontroll/checkouts/*) ;;
  *)
    echo "formal verification requires a fresh checkout under /opt/nodecontroll/checkouts: ${WORKTREE}" >&2
    exit 2
    ;;
esac

if [[ -z "${ACTIONS_ARTIFACT_INPUT}" ]]; then
  echo "NODECONTROLL_ACTIONS_ARTIFACT is required" >&2
  exit 2
fi
if [[ ! "${GITHUB_RUN_ID}" =~ ^[1-9][0-9]*$ || ! "${GITHUB_ARTIFACT_ID}" =~ ^[1-9][0-9]*$ ]]; then
  echo "NODECONTROLL_GITHUB_RUN_ID and NODECONTROLL_GITHUB_ARTIFACT_ID must be positive integers" >&2
  exit 2
fi
readonly ACTIONS_ARTIFACT="$(readlink -f "${ACTIONS_ARTIFACT_INPUT}")"
case "${ACTIONS_ARTIFACT}" in
  /opt/nodecontroll/artifacts/github-actions/*/nodecontroll-linux-x86_64-glibc2.36.tar.gz) ;;
  *)
    echo "refusing unexpected Actions artifact path: ${ACTIONS_ARTIFACT}" >&2
    exit 2
    ;;
esac
if [[ ! -f "${ACTIONS_ARTIFACT}" || -L "${ACTIONS_ARTIFACT}" ]]; then
  echo "Actions artifact must be a regular, non-symlink file" >&2
  exit 2
fi

case "${RUN_DIR}" in
  /opt/nodecontroll/artifacts/test-runs/*) ;;
  *)
    echo "refusing unexpected artifact path: ${RUN_DIR}" >&2
    exit 2
    ;;
esac

mkdir -p "${RUNS_ROOT}" "${TASK_ROOT}/tmp"
if [[ ! -d "${RUNS_ROOT}" || -L "${RUNS_ROOT}" || ! -d "${TASK_ROOT}/tmp" || -L "${TASK_ROOT}/tmp" ]]; then
  echo "verifier parent directories must be real directories" >&2
  exit 2
fi
if [[ -e "${RUN_DIR}" || -L "${RUN_DIR}" ]]; then
  echo "refusing pre-existing verifier run directory: ${RUN_DIR}" >&2
  exit 2
fi
umask 077
mkdir --mode=0700 -- "${RUN_DIR}"
mkdir --mode=0700 -- "${RUN_DIR}/logs" "${RUN_DIR}/provenance" "${RUN_DIR}/compiled"
exec 9>"${TASK_ROOT}/verify.lock"
if ! flock --nonblock 9; then
  echo "another NodeControll verifier is already running" >&2
  exit 2
fi

case "${TEST_SECRET_FILE}" in
  /opt/nodecontroll/tmp/*.root-key) ;;
  *)
    echo "refusing unexpected temporary secret path: ${TEST_SECRET_FILE}" >&2
    exit 2
    ;;
esac
case "${TEST_SETUP_TOKEN_FILE}" in
  /opt/nodecontroll/tmp/*.setup-token) ;;
  *)
    echo "refusing unexpected temporary setup-token path: ${TEST_SETUP_TOKEN_FILE}" >&2
    exit 2
    ;;
esac
case "${E2E_NODE_RUNTIME}" in
  /opt/nodecontroll/tmp/*.e2e-node) ;;
  *)
    echo "refusing unexpected temporary E2E Node runtime path: ${E2E_NODE_RUNTIME}" >&2
    exit 2
    ;;
esac
if [[ -e "${E2E_NODE_RUNTIME}" || -L "${E2E_NODE_RUNTIME}" ]]; then
  echo "refusing pre-existing temporary E2E Node runtime path: ${E2E_NODE_RUNTIME}" >&2
  exit 2
fi
for isolated_path in "${NODE_WORKSPACE}" "${NODE_PNPM_STORE}" "${PRIVATE_CARGO_HOME}" \
  "${PRIVATE_CARGO_TEST_TARGET}"; do
  case "${isolated_path}" in
    /opt/nodecontroll/tmp/*.node-workspace | /opt/nodecontroll/tmp/*.pnpm-store | \
    /opt/nodecontroll/tmp/*.cargo-home | /opt/nodecontroll/tmp/*.cargo-test-target) ;;
    *)
      echo "refusing unexpected isolated build path: ${isolated_path}" >&2
      exit 2
      ;;
  esac
  if [[ -e "${isolated_path}" || -L "${isolated_path}" ]]; then
    echo "refusing pre-existing isolated build path: ${isolated_path}" >&2
    exit 2
  fi
done
case "${AUTH_E2E_TEMP_ROOT}" in
  /opt/nodecontroll/tmp/*.auth-e2e) ;;
  *)
    echo "refusing unexpected auth E2E temporary root: ${AUTH_E2E_TEMP_ROOT}" >&2
    exit 2
    ;;
esac
if [[ -e "${AUTH_E2E_TEMP_ROOT}" || -L "${AUTH_E2E_TEMP_ROOT}" ]]; then
  echo "refusing pre-existing auth E2E temporary root: ${AUTH_E2E_TEMP_ROOT}" >&2
  exit 2
fi
case "${CYCLONEDX_CLI_FILE}" in
  /opt/nodecontroll/tmp/*.cyclonedx-cli) ;;
  *)
    echo "refusing unexpected temporary CycloneDX CLI path: ${CYCLONEDX_CLI_FILE}" >&2
    exit 2
    ;;
esac
if [[ -e "${CYCLONEDX_CLI_FILE}" || -L "${CYCLONEDX_CLI_FILE}" ]]; then
  echo "refusing pre-existing temporary CycloneDX CLI path: ${CYCLONEDX_CLI_FILE}" >&2
  exit 2
fi

capture_container_log_new() {
  local container="$1"
  local destination="$2"
  local temporary="${destination}.capturing"
  if [[ -e "${destination}" || -L "${destination}" || -e "${temporary}" || -L "${temporary}" ]]; then
    echo "refusing to overwrite container log: ${destination}" >&2
    return 2
  fi
  umask 077
  docker logs "${container}" > "${temporary}" 2>&1 || {
    rm -f -- "${temporary}"
    return 1
  }
  if ! python3 - "${temporary}" <<'PY'
import os
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
with path.open("rb+") as stream:
    os.fsync(stream.fileno())
PY
  then
    rm -f -- "${temporary}"
    return 1
  fi
  if ! ln -- "${temporary}" "${destination}"; then
    rm -f -- "${temporary}"
    return 1
  fi
  if ! rm -f -- "${temporary}"; then
    rm -f -- "${destination}" "${temporary}"
    remove_failed_secret_scan_target "${destination}" "auth E2E log publication" || true
    return 1
  fi
  if ! python3 - "$(dirname "${destination}")" <<'PY'
import os
import pathlib
import sys

directory = os.open(pathlib.Path(sys.argv[1]), os.O_RDONLY | os.O_DIRECTORY)
try:
    os.fsync(directory)
finally:
    os.close(directory)
PY
  then
    remove_failed_secret_scan_target "${destination}" "auth E2E log directory fsync" || true
    return 1
  fi
}

capture_container_log_precreated() {
  local container="$1"
  local destination="$2"
  [[ -f "${destination}" && ! -L "${destination}" \
    && "$(stat -c '%a' "${destination}")" == "600" \
    && "$(stat -c '%s' "${destination}")" -eq 0 \
    && "$(readlink -f "${destination}")" == "${destination}" ]] || {
    echo "gate log must be a canonical empty private pre-created file" >&2
    return 2
  }
  if ! docker logs "${container}" > "${destination}" 2>&1; then
    remove_failed_secret_scan_target "${destination}" "auth E2E browser log capture" || true
    return 1
  fi
  if ! python3 - "${destination}" <<'PY'
import os
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
with path.open("rb+") as stream:
    os.fsync(stream.fileno())
directory = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
try:
    os.fsync(directory)
finally:
    os.close(directory)
PY
  then
    remove_failed_secret_scan_target "${destination}" "auth E2E browser log fsync" || true
    return 1
  fi
}

scan_auth_e2e_fixture_secrets() {
  local target="$1"
  [[ -f "${target}" && -r "${target}" && ! -L "${target}" \
    && -f "${AUTH_E2E_ROOT_KEY_FILE}" && -r "${AUTH_E2E_ROOT_KEY_FILE}" \
    && -f "${AUTH_E2E_SETUP_TOKEN_FILE}" && -r "${AUTH_E2E_SETUP_TOKEN_FILE}" \
    && -f "${AUTH_E2E_PASSWORD_FILE}" && -r "${AUTH_E2E_PASSWORD_FILE}" \
    && -f "${AUTH_E2E_TLS_KEY_FILE}" && -r "${AUTH_E2E_TLS_KEY_FILE}" ]] || return 2
  python3 - \
    "${target}" \
    "${AUTH_E2E_ROOT_KEY_FILE}" \
    "${AUTH_E2E_SETUP_TOKEN_FILE}" \
    "${AUTH_E2E_PASSWORD_FILE}" \
    "${AUTH_E2E_TLS_KEY_FILE}" <<'PY'
import base64
import json
import pathlib
import sys
import urllib.parse

target, root_key_file, setup_token_file, password_file, tls_key_file = map(
    pathlib.Path, sys.argv[1:]
)
body = target.read_bytes()
single_line_secrets = [
    root_key_file.read_bytes().strip(),
    setup_token_file.read_bytes().strip(),
    password_file.read_bytes().strip(),
]
tls_pem = tls_key_file.read_bytes()
if any(not value for value in single_line_secrets) or not tls_pem:
    raise SystemExit("auth E2E secret fixture is empty")

canaries = set(single_line_secrets)
canaries.add(tls_pem)
pem_body_lines = [
    line.strip()
    for line in tls_pem.splitlines()
    if line and not line.startswith(b"-----") and len(line.strip()) >= 16
]
canaries.update(pem_body_lines)
joined_pem_body = b"".join(pem_body_lines)
if joined_pem_body:
    canaries.add(joined_pem_body)
    try:
        canaries.add(base64.b64decode(joined_pem_body, validate=True))
    except ValueError as error:
        raise SystemExit("TLS private key PEM body is invalid") from error

for secret in list(canaries):
    if len(secret) < 16:
        continue
    try:
        text = secret.decode("utf-8")
    except UnicodeDecodeError:
        text = None
    if text is not None:
        canaries.add(json.dumps(text, ensure_ascii=False).encode("utf-8"))
        canaries.add(urllib.parse.quote(text, safe="").encode("ascii"))
    canaries.add(base64.b64encode(secret))
    canaries.add(base64.urlsafe_b64encode(secret).rstrip(b"="))

if any(len(canary) >= 16 and canary in body for canary in canaries):
    raise SystemExit(f"auth E2E fixture secret found in {target}")
PY
}

remove_failed_secret_scan_target() {
  local target="$1"
  local label="$2"
  case "${target}" in
    "${RUN_DIR}/logs/"* | "${AUTH_E2E_ROOT}/"*) ;;
    *)
      echo "refusing to remove unexpected failed secret-scan target: ${target}" >&2
      return 2
      ;;
  esac
  if ! rm -f -- "${target}"; then
    printf '%s\n' "${label} secret scan failed; contaminated evidence could not be removed" \
      > "${RUN_DIR}/SECRET_SCAN_FAILED"
    echo "could not remove failed secret-scan target: ${target}" >&2
    return 2
  fi
  printf '%s\n' "${label} secret scan failed; contaminated evidence was removed" \
    > "${RUN_DIR}/SECRET_SCAN_FAILED"
}

scan_auth_e2e_fixture_secrets_or_remove() {
  local target="$1"
  local status
  if scan_auth_e2e_fixture_secrets "${target}"; then
    return 0
  else
    status="$?"
  fi
  remove_failed_secret_scan_target "${target}" "auth E2E" || return 2
  return "${status}"
}

cleanup_auth_e2e() {
  local cleanup_log
  if docker inspect "${E2E_BROWSER_CONTAINER}" >/dev/null 2>&1; then
    docker stop --time 2 "${E2E_BROWSER_CONTAINER}" >/dev/null 2>&1 || true
    cleanup_log="${RUN_DIR}/logs/auth-e2e-browser-cleanup.log"
    if [[ ! -e "${cleanup_log}" && ! -L "${cleanup_log}" ]]; then
      if ! capture_container_log_new "${E2E_BROWSER_CONTAINER}" "${cleanup_log}"; then
        printf '%s\n' "auth E2E browser cleanup log capture failed" \
          > "${RUN_DIR}/SECRET_SCAN_FAILED" || true
      elif [[ -f "${cleanup_log}" ]]; then
        scan_auth_e2e_fixture_secrets_or_remove "${cleanup_log}" || true
      fi
    fi
    if [[ -f "${AUTH_E2E_BROWSER_LOG}" || -L "${AUTH_E2E_BROWSER_LOG}" ]]; then
      scan_auth_e2e_fixture_secrets_or_remove "${AUTH_E2E_BROWSER_LOG}" || true
    fi
  fi
  docker rm --force "${E2E_BROWSER_CONTAINER}" >/dev/null 2>&1 || true

  if docker inspect "${E2E_MASTER_CONTAINER}" >/dev/null 2>&1; then
    docker stop --time 5 "${E2E_MASTER_CONTAINER}" >/dev/null 2>&1 || true
    cleanup_log="${RUN_DIR}/logs/auth-e2e-master-cleanup.log"
    if [[ ! -e "${cleanup_log}" && ! -L "${cleanup_log}" ]]; then
      if ! capture_container_log_new "${E2E_MASTER_CONTAINER}" "${cleanup_log}"; then
        printf '%s\n' "auth E2E Master cleanup log capture failed" \
          > "${RUN_DIR}/SECRET_SCAN_FAILED" || true
      elif [[ -f "${cleanup_log}" ]]; then
        scan_auth_e2e_fixture_secrets_or_remove "${cleanup_log}" || true
      fi
    fi
    if [[ -f "${AUTH_E2E_MASTER_LOG}" || -L "${AUTH_E2E_MASTER_LOG}" ]]; then
      scan_auth_e2e_fixture_secrets_or_remove "${AUTH_E2E_MASTER_LOG}" || true
    fi
  fi
  docker rm --force "${E2E_MASTER_CONTAINER}" >/dev/null 2>&1 || true

  rm -f -- \
    "${AUTH_E2E_DATABASE_TEMP}" \
    "${AUTH_E2E_DATABASE_TEMP}-wal" \
    "${AUTH_E2E_DATABASE_TEMP}-shm" \
    "${AUTH_E2E_DATABASE_TEMP}-journal" \
    "${AUTH_E2E_DATABASE_DUMP_TEMP}" \
    "${AUTH_E2E_GATE_ATTESTATION_TEMP}" \
    "${AUTH_E2E_SCAN_MARKER_TEMP}"

  case "${AUTH_E2E_TEMP_ROOT}" in
    /opt/nodecontroll/tmp/*.auth-e2e)
      rm -f -- \
        "${AUTH_E2E_ROOT_KEY_FILE}" \
        "${AUTH_E2E_SETUP_TOKEN_FILE}" \
        "${AUTH_E2E_PASSWORD_FILE}" \
        "${AUTH_E2E_TLS_KEY_FILE}" \
        "${AUTH_E2E_TLS_CERTIFICATE_TEMP}" \
        "${AUTH_E2E_LIVE_DATABASE_FILE}" \
        "${AUTH_E2E_LIVE_DATABASE_FILE}-wal" \
        "${AUTH_E2E_LIVE_DATABASE_FILE}-shm" \
        "${AUTH_E2E_LIVE_DATABASE_FILE}-journal"
      rmdir -- "${AUTH_E2E_LIVE_DATABASE_DIR}" >/dev/null 2>&1 || true
      rmdir -- "${AUTH_E2E_TEMP_ROOT}" >/dev/null 2>&1 || true
      ;;
  esac
}

cleanup() {
  local runtime_log="${RUN_DIR}/logs/master-runtime.log"
  local runtime_log_tmp="${RUN_DIR}/logs/master-runtime.log.capturing"
  cleanup_auth_e2e
  if docker inspect "${SMOKE_CONTAINER}" >/dev/null 2>&1; then
    docker stop "${SMOKE_CONTAINER}" >/dev/null 2>&1 || true
    if docker logs "${SMOKE_CONTAINER}" > "${runtime_log_tmp}" 2>&1; then
      if mv -f -- "${runtime_log_tmp}" "${runtime_log}"; then
        scan_runtime_secrets_or_remove "${runtime_log}" || true
      else
        printf '%s\n' "runtime log secret scan failed during cleanup" \
          > "${RUN_DIR}/SECRET_SCAN_FAILED" || true
      fi
    else
      rm -f -- "${runtime_log_tmp}"
      printf '%s\n' "runtime log capture failed during cleanup" \
        > "${RUN_DIR}/SECRET_SCAN_FAILED" || true
    fi
  fi
  docker rm --force "${SMOKE_CONTAINER}" >/dev/null 2>&1 || true
  docker rm --force --volumes "${POSTGRES_CONTAINER}" >/dev/null 2>&1 || true
  docker network rm "${TEST_NETWORK}" >/dev/null 2>&1 || true
  rm -f "${TEST_SECRET_FILE}" "${TEST_SETUP_TOKEN_FILE}"
  rm -f -- "${CYCLONEDX_CLI_FILE}"
  case "${E2E_NODE_RUNTIME}" in
    /opt/nodecontroll/tmp/*.e2e-node)
      rm -rf -- "${E2E_NODE_RUNTIME}"
      ;;
  esac
  for isolated_path in "${NODE_WORKSPACE}" "${NODE_PNPM_STORE}" "${PRIVATE_CARGO_HOME}" \
    "${PRIVATE_CARGO_TEST_TARGET}"; do
    case "${isolated_path}" in
      /opt/nodecontroll/tmp/*.node-workspace | /opt/nodecontroll/tmp/*.pnpm-store | \
      /opt/nodecontroll/tmp/*.cargo-home | /opt/nodecontroll/tmp/*.cargo-test-target)
        rm -rf -- "${isolated_path}"
        ;;
    esac
  done
}

verify_cleanup_complete() {
  local container path
  if [[ -e "${RUN_DIR}/SECRET_SCAN_FAILED" || -L "${RUN_DIR}/SECRET_SCAN_FAILED" ]]; then
    echo "verifier cleanup recorded a failed secret scan" >&2
    return 1
  fi
  for container in \
    "${SMOKE_CONTAINER}" \
    "${E2E_MASTER_CONTAINER}" \
    "${E2E_BROWSER_CONTAINER}" \
    "${POSTGRES_CONTAINER}"; do
    if docker inspect "${container}" >/dev/null 2>&1; then
      echo "verifier cleanup left container ${container}" >&2
      return 1
    fi
  done
  if docker network inspect "${TEST_NETWORK}" >/dev/null 2>&1; then
    echo "verifier cleanup left network ${TEST_NETWORK}" >&2
    return 1
  fi
  for path in \
    "${TEST_SECRET_FILE}" \
    "${TEST_SETUP_TOKEN_FILE}" \
    "${CYCLONEDX_CLI_FILE}" \
    "${E2E_NODE_RUNTIME}" \
    "${NODE_WORKSPACE}" \
    "${NODE_PNPM_STORE}" \
    "${PRIVATE_CARGO_HOME}" \
    "${PRIVATE_CARGO_TEST_TARGET}" \
    "${AUTH_E2E_TEMP_ROOT}" \
    "${AUTH_E2E_DATABASE_TEMP}" \
    "${AUTH_E2E_DATABASE_TEMP}-wal" \
    "${AUTH_E2E_DATABASE_TEMP}-shm" \
    "${AUTH_E2E_DATABASE_TEMP}-journal" \
    "${AUTH_E2E_DATABASE_DUMP_TEMP}" \
    "${AUTH_E2E_GATE_ATTESTATION_TEMP}" \
    "${AUTH_E2E_SCAN_MARKER_TEMP}" \
    "${AUTH_E2E_MASTER_LOG}.capturing" \
    "${RUN_DIR}/logs/auth-e2e-browser-cleanup.log.capturing" \
    "${RUN_DIR}/logs/auth-e2e-master-cleanup.log.capturing" \
    "${RUN_DIR}/logs/master-runtime.log.capturing"; do
    if [[ -e "${path}" || -L "${path}" ]]; then
      echo "verifier cleanup left temporary path ${path}" >&2
      return 1
    fi
  done
}

finalize_manifest() {
  local status="$1"
  local finished_at failed_stage=""
  [[ -f "${RUN_DIR}/manifest.json" ]] || return 0
  finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  if [[ -f "${RUN_DIR}/FAILED_STAGE" ]]; then
    failed_stage="$(<"${RUN_DIR}/FAILED_STAGE")"
  fi
  python3 - "${RUN_DIR}/manifest.json" "${status}" "${finished_at}" "${failed_stage}" <<'PY'
import json
import pathlib
import sys

manifest, status, finished_at, failed_stage = sys.argv[1:]
path = pathlib.Path(manifest)
payload = json.loads(path.read_text(encoding="utf-8"))
payload["status"] = status
payload["finished_at"] = finished_at
if status == "completed":
    payload["source_checkout_clean_after_tests"] = True
else:
    payload.pop("source_checkout_clean_after_tests", None)
if failed_stage:
    payload["failed_stage"] = failed_stage
else:
    payload.pop("failed_stage", None)
temporary = path.with_suffix(".json.tmp")
temporary.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
temporary.replace(path)
PY
}

on_exit() {
  local exit_status="$?"
  set +e
  cleanup
  finalize_manifest failed
  trap - EXIT INT TERM
  exit "${exit_status}"
}
trap on_exit EXIT
trap 'exit 130' INT TERM

assert_image() {
  local image="$1"
  local expected="$2"
  local actual
  actual="$(docker image inspect "${image}" --format '{{.Id}}')"
  if [[ "${actual}" != "${expected}" ]]; then
    echo "builder image mismatch for ${image}: expected ${expected}, got ${actual}" >&2
    exit 2
  fi
}

assert_repo_digest() {
  local image="$1"
  local expected="$2"
  local found=false digest repo_digests
  repo_digests="$(docker image inspect "${image}" --format '{{range .RepoDigests}}{{println .}}{{end}}')" || return
  while IFS= read -r digest; do
    if [[ "${digest}" == "${expected}" ]]; then
      found=true
      break
    fi
  done <<< "${repo_digests}"
  if [[ "${found}" != true ]]; then
    echo "image ${image} does not expose the expected repository digest ${expected}" >&2
    exit 2
  fi
}

verify_rust_builder_toolchain() {
  docker run --rm \
    --network none \
    --read-only \
    --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
    --tmpfs /cargo-home:rw,nosuid,nodev,mode=0755 \
    -e HOME=/tmp/rust-home \
    -e CARGO_HOME=/cargo-home \
    -e RUSTUP_HOME=/usr/local/rustup \
    -e RUSTUP_TOOLCHAIN=1.98.0 \
    "${RUST_IMAGE_ID}" sh -euc '
      test "$(rustc --version)" = \
        "rustc 1.98.0 (88d9e12ae 2026-08-18)"
      test "$(cargo --version)" = \
        "cargo 1.98.0 (797e8a9bc 2026-08-05)"
      test "$(cargo fmt --version)" = \
        "rustfmt 1.9.0-stable (88d9e12ae1 2026-08-18)"
      test "$(cargo clippy --version)" = \
        "clippy 0.1.98 (88d9e12ae1 2026-08-18)"
    '
}

scan_runtime_secrets() {
  local log_file="$1"
  [[ -f "${log_file}" && -r "${log_file}" \
    && -f "${TEST_SETUP_TOKEN_FILE}" && -r "${TEST_SETUP_TOKEN_FILE}" \
    && -f "${TEST_SECRET_FILE}" && -r "${TEST_SECRET_FILE}" ]] || return 2
  python3 - \
    "${log_file}" \
    "${TEST_SETUP_TOKEN_FILE}" \
    "${TEST_SECRET_FILE}" <<'PY'
from pathlib import Path
import sys

log_path, setup_token_path, root_key_path = map(Path, sys.argv[1:])
try:
    log = log_path.read_bytes()
    setup_token = setup_token_path.read_bytes().strip()
    root_key = root_key_path.read_bytes().strip()
except OSError:
    print("could not scan Master runtime log", file=sys.stderr)
    raise SystemExit(2)

if not setup_token or not root_key:
    print("runtime secret fixture is empty", file=sys.stderr)
    raise SystemExit(2)

forbidden_values = (
    setup_token,
    root_key,
    b"VPS smoke bootstrap passphrase",
    b"Another smoke bootstrap passphrase",
    b"Rejected contract passphrase",
    b"Incorrect smoke login passphrase",
    b"$argon2id$",
)
if any(value in log for value in forbidden_values):
    print(
        "Master runtime log contains setup-token, password, or PHC material",
        file=sys.stderr,
    )
    raise SystemExit(1)
PY
}

scan_runtime_secrets_or_remove() {
  local target="$1"
  local status
  if scan_runtime_secrets "${target}"; then
    return 0
  else
    status="$?"
  fi
  remove_failed_secret_scan_target "${target}" "Master runtime" || return 2
  return "${status}"
}

stop_and_capture_master() {
  local log_file="$1"
  docker stop "${SMOKE_CONTAINER}" >/dev/null || return
  if ! docker logs "${SMOKE_CONTAINER}" > "${log_file}" 2>&1; then
    rm -f -- "${log_file}"
    return 1
  fi
  docker rm "${SMOKE_CONTAINER}" >/dev/null || return
}

prepare_auth_e2e_fixtures() {
  if [[ -e "${AUTH_E2E_ROOT}" || -L "${AUTH_E2E_ROOT}" \
    || -e "${AUTH_E2E_TEMP_ROOT}" || -L "${AUTH_E2E_TEMP_ROOT}" ]]; then
    echo "auth E2E output and temporary roots must not pre-exist" >&2
    return 2
  fi
  umask 077
  mkdir --mode=0700 -- \
    "${AUTH_E2E_ROOT}" \
    "${AUTH_E2E_DATABASE_DUMP_DIR}" \
    "${AUTH_E2E_RUNTIME_LOGS}" \
    "${AUTH_E2E_TEST_ARTIFACTS}" \
    "${AUTH_E2E_HANDSHAKE}" \
    "${AUTH_E2E_TEMP_ROOT}" \
    "${AUTH_E2E_LIVE_DATABASE_DIR}"
  : > "${AUTH_E2E_BROWSER_LOG}"
  chmod 0600 -- "${AUTH_E2E_BROWSER_LOG}"

  for secret_file in \
    "${AUTH_E2E_ROOT_KEY_FILE}" \
    "${AUTH_E2E_SETUP_TOKEN_FILE}" \
    "${AUTH_E2E_PASSWORD_FILE}"; do
    head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n' > "${secret_file}"
    [[ -f "${secret_file}" && ! -L "${secret_file}" \
      && "$(stat -c '%a' "${secret_file}")" == "600" \
      && "$(wc -c < "${secret_file}")" -eq 64 ]] || return 2
  done

  command -v openssl >/dev/null
  openssl req \
    -x509 \
    -newkey rsa:3072 \
    -sha256 \
    -nodes \
    -days 1 \
    -subj '/CN=127.0.0.1' \
    -addext 'subjectAltName=IP:127.0.0.1' \
    -keyout "${AUTH_E2E_TLS_KEY_FILE}" \
    -out "${AUTH_E2E_TLS_CERTIFICATE_TEMP}"
  chmod 0600 -- "${AUTH_E2E_TLS_KEY_FILE}" "${AUTH_E2E_TLS_CERTIFICATE_TEMP}"
  [[ -f "${AUTH_E2E_TLS_KEY_FILE}" && ! -L "${AUTH_E2E_TLS_KEY_FILE}" \
    && -f "${AUTH_E2E_TLS_CERTIFICATE_TEMP}" && ! -L "${AUTH_E2E_TLS_CERTIFICATE_TEMP}" ]] || return 2
  openssl pkey -in "${AUTH_E2E_TLS_KEY_FILE}" -check -noout
  openssl x509 -in "${AUTH_E2E_TLS_CERTIFICATE_TEMP}" -checkend 0 -noout
  openssl x509 -in "${AUTH_E2E_TLS_CERTIFICATE_TEMP}" -noout -ext subjectAltName \
    | grep -F 'IP Address:127.0.0.1' >/dev/null
  cmp \
    <(openssl pkey -in "${AUTH_E2E_TLS_KEY_FILE}" -pubout 2>/dev/null) \
    <(openssl x509 -in "${AUTH_E2E_TLS_CERTIFICATE_TEMP}" -pubkey -noout)
  install -m 0400 -- "${AUTH_E2E_TLS_CERTIFICATE_TEMP}" "${AUTH_E2E_TLS_CERTIFICATE}"

  python3 - \
    "${AUTH_E2E_GATE_ATTESTATION}" \
    "${SOURCE_REVISION}" \
    "${PLAYWRIGHT_IMAGE_ID}" \
    "${RUN_ID}" <<'PY'
import json
import os
import pathlib
import sys

destination, revision, image_id, run_id = sys.argv[1:]
path = pathlib.Path(destination)
temporary = path.with_name(f".{path.name}.temporary")
payload = {
    "browser_image_digest": image_id,
    "run_id": run_id,
    "source_revision": revision,
}
descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
    stream.write(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    stream.flush()
    os.fsync(stream.fileno())
os.link(temporary, path)
temporary.unlink()
PY
  chmod 0600 -- "${AUTH_E2E_GATE_ATTESTATION}"
}

start_auth_e2e_master() {
  [[ -d "${AUTH_E2E_LIVE_DATABASE_DIR}" && ! -L "${AUTH_E2E_LIVE_DATABASE_DIR}" \
    && -z "$(find "${AUTH_E2E_LIVE_DATABASE_DIR}" -mindepth 1 -print -quit)" ]] || return 2
  docker run --detach \
    --name "${E2E_MASTER_CONTAINER}" \
    --network host \
    --read-only \
    --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
    --stop-timeout 10 \
    -e NODECONTROLL__HTTP__LISTEN=127.0.0.1:18081 \
    -e NODECONTROLL__HTTP__PUBLIC_ORIGIN=https://127.0.0.1:18443 \
    -e NODECONTROLL__DATABASE__URL='sqlite:///var/lib/nodecontroll/control.db?mode=rwc' \
    -e NODECONTROLL__SECRETS__ROOT_KEY_FILE=/run/secrets/nodecontroll-root-key \
    -e NODECONTROLL__SECRETS__SETUP_TOKEN_FILE=/run/secrets/nodecontroll-setup-token \
    -v "${AUTH_E2E_ROOT_KEY_FILE}:/run/secrets/nodecontroll-root-key:ro" \
    -v "${AUTH_E2E_SETUP_TOKEN_FILE}:/run/secrets/nodecontroll-setup-token:ro" \
    -v "${AUTH_E2E_LIVE_DATABASE_DIR}:/var/lib/nodecontroll" \
    -v "${ACTIONS_ARTIFACT_ROOT}:/compiled:ro" \
    -w /compiled \
    "${RUST_IMAGE_ID}" \
    /compiled/bin/nodecontroll-master
}

start_auth_e2e_browser() {
  [[ -d "${E2E_NODE_RUNTIME}/usr/local" && ! -L "${E2E_NODE_RUNTIME}/usr/local" ]] || return 2
  docker run --detach \
    --name "${E2E_BROWSER_CONTAINER}" \
    --network host \
    --ipc host \
    --init \
    --read-only \
    --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
    -e CI=true \
    -e HOME=/tmp/browser-home \
    -e XDG_CACHE_HOME=/tmp/browser-cache \
    -e PLAYWRIGHT_BROWSERS_PATH=/ms-playwright \
    -e NODECONTROLL_E2E_BASE_URL=https://127.0.0.1:18443/ \
    -e NODECONTROLL_E2E_UPSTREAM_URL=http://127.0.0.1:18081/ \
    -e NODECONTROLL_E2E_DIST_DIR=/evidence/compiled/web \
    -e NODECONTROLL_E2E_RUN_ROOT=/evidence \
    -e NODECONTROLL_E2E_TLS_KEY_FILE=/run/auth-e2e/tls-key.pem \
    -e NODECONTROLL_E2E_TLS_CERT_FILE=/run/auth-e2e/tls-certificate.pem \
    -e NODECONTROLL_E2E_SETUP_TOKEN_FILE=/run/auth-e2e/setup-token \
    -e NODECONTROLL_E2E_PASSWORD_FILE=/run/auth-e2e/password \
    -e NODECONTROLL_E2E_ROOT_KEY_FILE=/run/auth-e2e/root-key \
    -e NODECONTROLL_E2E_RUN_ID="${RUN_ID}" \
    -e NODECONTROLL_E2E_SOURCE_REVISION="${SOURCE_REVISION}" \
    -e NODECONTROLL_E2E_BROWSER_IMAGE_DIGEST="${PLAYWRIGHT_IMAGE_ID}" \
    -e NODECONTROLL_E2E_EVIDENCE_FILE=/evidence/browser/handshake/evidence.json \
    -e NODECONTROLL_E2E_BEHAVIOR_READY_FILE=/evidence/browser/handshake/behavior-ready \
    -e NODECONTROLL_E2E_SCAN_READY_FILE=/evidence/browser/handshake/scan-ready \
    -e NODECONTROLL_E2E_GATE_LOG_FILE=/evidence/browser/gate-runtime.log \
    -v "${NODE_WORKSPACE}:/workspace:ro" \
    -v "${RUN_DIR}:/evidence:ro" \
    -v "${AUTH_E2E_HANDSHAKE}:/evidence/browser/handshake:rw" \
    -v "${E2E_NODE_RUNTIME}/usr/local:/node-runtime:ro" \
    -v "${AUTH_E2E_TLS_KEY_FILE}:/run/auth-e2e/tls-key.pem:ro" \
    -v "${AUTH_E2E_TLS_CERTIFICATE}:/run/auth-e2e/tls-certificate.pem:ro" \
    -v "${AUTH_E2E_SETUP_TOKEN_FILE}:/run/auth-e2e/setup-token:ro" \
    -v "${AUTH_E2E_PASSWORD_FILE}:/run/auth-e2e/password:ro" \
    -v "${AUTH_E2E_ROOT_KEY_FILE}:/run/auth-e2e/root-key:ro" \
    -w /workspace/apps/web \
    "${PLAYWRIGHT_IMAGE}" \
    /node-runtime/bin/node /workspace/apps/web/e2e/auth-rotation.mjs
}

wait_auth_e2e_behavior_ready() {
  local deadline=$((SECONDS + 120)) state
  while (( SECONDS < deadline )); do
    state="$(docker inspect "${E2E_BROWSER_CONTAINER}" --format '{{.State.Running}} {{.State.ExitCode}}' 2>/dev/null)" || return 1
    if [[ -e "${AUTH_E2E_BEHAVIOR_MARKER}" || -L "${AUTH_E2E_BEHAVIOR_MARKER}" ]]; then
      [[ "${state}" == true\ * \
        && -f "${AUTH_E2E_BEHAVIOR_MARKER}" \
        && ! -L "${AUTH_E2E_BEHAVIOR_MARKER}" \
        && "$(stat -c '%a' "${AUTH_E2E_BEHAVIOR_MARKER}")" == "600" ]] || return 2
      cmp -s -- "${AUTH_E2E_BEHAVIOR_MARKER}" <(printf '%s\n' "${RUN_ID}") || return 2
      return 0
    fi
    [[ "${state}" == true\ * ]] || {
      echo "auth E2E browser exited before behavior-ready: ${state}" >&2
      return 1
    }
    sleep 0.1
  done
  echo "timed out waiting for auth E2E behavior-ready" >&2
  return 1
}

freeze_auth_e2e_artifacts() {
  local state
  docker stop --time 10 "${E2E_MASTER_CONTAINER}" >/dev/null
  state="$(docker inspect "${E2E_MASTER_CONTAINER}" --format '{{.State.Running}} {{.State.ExitCode}}')"
  [[ "${state}" == "false 0" ]] || {
    echo "auth E2E Master did not stop cleanly: ${state}" >&2
    return 1
  }
  capture_container_log_new "${E2E_MASTER_CONTAINER}" "${AUTH_E2E_MASTER_LOG}"
  docker rm "${E2E_MASTER_CONTAINER}" >/dev/null
  scan_auth_e2e_fixture_secrets_or_remove "${AUTH_E2E_MASTER_LOG}"

  python3 - \
    "${AUTH_E2E_LIVE_DATABASE_FILE}" \
    "${AUTH_E2E_DATABASE_FILE}" \
    "${AUTH_E2E_DATABASE_DUMP}" <<'PY'
import os
import pathlib
import sqlite3
import stat
import sys

source_path, snapshot_path, dump_path = map(pathlib.Path, sys.argv[1:])
source_metadata = source_path.lstat()
if not stat.S_ISREG(source_metadata.st_mode) or stat.S_ISLNK(source_metadata.st_mode):
    raise SystemExit("live auth E2E database is not a regular file")
for output in (snapshot_path, dump_path):
    if output.exists() or output.is_symlink():
        raise SystemExit(f"refusing to overwrite auth E2E database evidence: {output}")
    parent_metadata = output.parent.lstat()
    if not stat.S_ISDIR(parent_metadata.st_mode) or stat.S_ISLNK(parent_metadata.st_mode):
        raise SystemExit(f"database evidence parent is not a real directory: {output.parent}")

source = sqlite3.connect(f"file:{source_path}?mode=rw", uri=True)
try:
    checkpoint = source.execute("PRAGMA wal_checkpoint(TRUNCATE)").fetchone()
    if checkpoint is None or checkpoint[0] != 0:
        raise SystemExit(f"auth E2E WAL checkpoint remained busy: {checkpoint!r}")
    if source.execute("PRAGMA integrity_check").fetchone() != ("ok",):
        raise SystemExit("live auth E2E database failed integrity_check")
    if source.execute("PRAGMA foreign_key_check").fetchone() is not None:
        raise SystemExit("live auth E2E database failed foreign_key_check")
    snapshot_temporary = snapshot_path.with_name(f".{snapshot_path.name}.temporary")
    if snapshot_temporary.exists() or snapshot_temporary.is_symlink():
        raise SystemExit("auth E2E snapshot temporary path already exists")
    destination = sqlite3.connect(snapshot_temporary)
    try:
        source.backup(destination)
        journal_mode = destination.execute("PRAGMA journal_mode=DELETE").fetchone()
        if journal_mode is None or str(journal_mode[0]).lower() != "delete":
            raise SystemExit(
                f"auth E2E database snapshot could not leave WAL mode: {journal_mode!r}"
            )
        if destination.execute("PRAGMA integrity_check").fetchone() != ("ok",):
            raise SystemExit("auth E2E database snapshot failed integrity_check")
        if destination.execute("PRAGMA foreign_key_check").fetchone() is not None:
            raise SystemExit("auth E2E database snapshot failed foreign_key_check")
    finally:
        destination.close()
finally:
    source.close()

for suffix in ("-wal", "-shm", "-journal"):
    sidecar = pathlib.Path(str(snapshot_temporary) + suffix)
    if sidecar.exists() or sidecar.is_symlink():
        raise SystemExit(f"auth E2E temporary snapshot contains a forbidden sidecar: {sidecar}")

with snapshot_temporary.open("rb+") as stream:
    os.fsync(stream.fileno())
os.link(snapshot_temporary, snapshot_path)
snapshot_temporary.unlink()
for suffix in ("-wal", "-shm", "-journal"):
    sidecar = pathlib.Path(str(snapshot_path) + suffix)
    if sidecar.exists() or sidecar.is_symlink():
        raise SystemExit(f"auth E2E snapshot contains a forbidden sidecar: {sidecar}")

dump_temporary = dump_path.with_name(f".{dump_path.name}.temporary")
descriptor = os.open(dump_temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
snapshot = sqlite3.connect(f"file:{snapshot_path}?mode=ro&immutable=1", uri=True)
try:
    with os.fdopen(descriptor, "w", encoding="utf-8", newline="\n") as stream:
        for line in snapshot.iterdump():
            stream.write(line)
            stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())
finally:
    snapshot.close()
if dump_temporary.stat().st_size == 0:
    raise SystemExit("auth E2E database dump is empty")
os.link(dump_temporary, dump_path)
dump_temporary.unlink()
for suffix in ("-wal", "-shm", "-journal"):
    sidecar = pathlib.Path(str(snapshot_path) + suffix)
    if sidecar.exists() or sidecar.is_symlink():
        raise SystemExit(f"auth E2E dump created a forbidden sidecar: {sidecar}")
for directory in (snapshot_path.parent, dump_path.parent):
    descriptor = os.open(directory, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)
PY
  python3 - \
    "${ACTIONS_ARTIFACT_ROOT}/bin" \
    "${AUTH_E2E_DATABASE_FILE}" \
    "${AUTH_E2E_DATABASE_DUMP}" \
    "${ACTIONS_ARTIFACT_ROOT}/openapi/nodecontroll-v1.json" \
    "${AUTH_E2E_RUNTIME_LOGS}" \
    "${AUTH_E2E_TEST_ARTIFACTS}" \
    "${ACTIONS_ARTIFACT_ROOT}/web" <<'PY'
import os
import pathlib
import stat
import sys

def freeze(path):
    metadata = path.lstat()
    if stat.S_ISLNK(metadata.st_mode):
        raise SystemExit(f"auth E2E scan target contains a symlink: {path}")
    if stat.S_ISREG(metadata.st_mode):
        os.chmod(path, stat.S_IMODE(metadata.st_mode) & ~0o222, follow_symlinks=False)
        return
    if not stat.S_ISDIR(metadata.st_mode):
        raise SystemExit(f"auth E2E scan target contains a special root: {path}")
    directories = []
    for current_root, directory_names, file_names in os.walk(path, topdown=True, followlinks=False):
        directory_names.sort()
        file_names.sort()
        current = pathlib.Path(current_root)
        directories.append(current)
        for name in directory_names:
            child = current / name
            child_metadata = child.lstat()
            if not stat.S_ISDIR(child_metadata.st_mode) or stat.S_ISLNK(child_metadata.st_mode):
                raise SystemExit(f"auth E2E scan target contains a non-directory entry: {child}")
        for name in file_names:
            child = current / name
            child_metadata = child.lstat()
            if not stat.S_ISREG(child_metadata.st_mode) or stat.S_ISLNK(child_metadata.st_mode):
                raise SystemExit(f"auth E2E scan target contains a special file: {child}")
            os.chmod(child, stat.S_IMODE(child_metadata.st_mode) & ~0o222, follow_symlinks=False)
    for directory in reversed(directories):
        directory_metadata = directory.lstat()
        os.chmod(
            directory,
            stat.S_IMODE(directory_metadata.st_mode) & ~0o222,
            follow_symlinks=False,
        )

for raw_path in sys.argv[1:]:
    freeze(pathlib.Path(raw_path))
PY
}

publish_auth_e2e_scan_ready() {
  python3 - "${AUTH_E2E_SCAN_MARKER}" "${RUN_ID}" <<'PY'
import os
import pathlib
import stat
import sys

path = pathlib.Path(sys.argv[1])
run_id = sys.argv[2]
parent_metadata = path.parent.lstat()
if not stat.S_ISDIR(parent_metadata.st_mode) or stat.S_ISLNK(parent_metadata.st_mode):
    raise SystemExit("scan-ready parent must be a real directory")
if path.exists() or path.is_symlink():
    raise SystemExit("scan-ready marker must not pre-exist")
temporary = path.with_name(f".{path.name}.verifier-temporary")
descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
    stream.write(run_id + "\n")
    stream.flush()
    os.fsync(stream.fileno())
os.link(temporary, path)
temporary.unlink()
directory = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
try:
    os.fsync(directory)
finally:
    os.close(directory)
PY
}

wait_auth_e2e_browser_finish() {
  local deadline=$((SECONDS + 180)) state
  while (( SECONDS < deadline )); do
    state="$(docker inspect "${E2E_BROWSER_CONTAINER}" --format '{{.State.Running}} {{.State.ExitCode}}' 2>/dev/null)" || return 1
    [[ "${state}" == true\ * ]] || break
    sleep 0.1
  done
  [[ "${state:-}" == "false 0" ]] || {
    echo "auth E2E browser did not finish successfully: ${state:-missing}" >&2
    return 1
  }
  capture_container_log_precreated "${E2E_BROWSER_CONTAINER}" "${AUTH_E2E_BROWSER_LOG}"
  docker rm "${E2E_BROWSER_CONTAINER}" >/dev/null
  scan_auth_e2e_fixture_secrets_or_remove "${AUTH_E2E_BROWSER_LOG}"
  python3 - "${AUTH_E2E_BROWSER_LOG}" <<'PY'
import pathlib
import re
import sys

body = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
if re.fullmatch(
    r"HTTPS browser rotation passed: evidence\.json \([1-9][0-9]* scanned files\)\n",
    body,
) is None:
    raise SystemExit("auth E2E browser success log contains unexpected output")
PY
  chmod 0400 -- "${AUTH_E2E_BROWSER_LOG}"
}

verify_auth_e2e_evidence() {
  local certificate_sha
  certificate_sha="$(sha256sum "${AUTH_E2E_TLS_CERTIFICATE}" | cut -d' ' -f1)"
  docker run --rm \
    --network none \
    --read-only \
    --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
    -e HOME=/tmp/node-home \
    -v "${NODE_WORKSPACE}:/workspace:ro" \
    -v "${RUN_DIR}:/evidence:ro" \
    -w /workspace \
    "${NODE_IMAGE_ID}" \
    node tools/verify_auth_e2e_evidence.mjs \
      /evidence/browser/handshake/evidence.json \
      /evidence \
      /evidence/browser/handshake/behavior-ready \
      /evidence/browser/handshake/scan-ready \
      "${RUN_ID}" \
      "${SOURCE_REVISION}" \
      "${PLAYWRIGHT_IMAGE_ID}" \
      "${PLAYWRIGHT_PACKAGE_VERSION}" \
      "${PLAYWRIGHT_NODE_VERSION}" \
      "${PLAYWRIGHT_BROWSER_VERSION}" \
      "${PLAYWRIGHT_EXECUTABLE_SHA256}" \
      "${AUTH_E2E_GATE_SHA256}" \
      "${certificate_sha}"
}

seal_auth_e2e_artifacts() {
  python3 - "${AUTH_E2E_ROOT}" <<'PY'
import os
import pathlib
import stat
import sys

root = pathlib.Path(sys.argv[1])
root_metadata = root.lstat()
if not stat.S_ISDIR(root_metadata.st_mode) or stat.S_ISLNK(root_metadata.st_mode):
    raise SystemExit("auth E2E evidence root must be a real directory")
directories = []
for current_root, directory_names, file_names in os.walk(root, topdown=True, followlinks=False):
    directory_names.sort()
    file_names.sort()
    current = pathlib.Path(current_root)
    directories.append(current)
    for name in directory_names:
        metadata = (current / name).lstat()
        if not stat.S_ISDIR(metadata.st_mode) or stat.S_ISLNK(metadata.st_mode):
            raise SystemExit(f"auth E2E evidence contains non-directory entry: {current / name}")
    for name in file_names:
        path = current / name
        metadata = path.lstat()
        if not stat.S_ISREG(metadata.st_mode) or stat.S_ISLNK(metadata.st_mode):
            raise SystemExit(f"auth E2E evidence contains special file: {path}")
        os.chmod(path, 0o400, follow_symlinks=False)
for directory in reversed(directories):
    os.chmod(directory, 0o500, follow_symlinks=False)
PY
}

record_auth_e2e_manifest() {
  python3 - \
    "${RUN_DIR}/manifest.json" \
    "${AUTH_E2E_EVIDENCE}" \
    "${AUTH_E2E_CLOSURE}" \
    "${AUTH_E2E_TLS_CERTIFICATE}" \
    "${AUTH_E2E_EVIDENCE#${RUN_DIR}/}" \
    "${AUTH_E2E_CLOSURE#${RUN_DIR}/}" <<'PY'
import hashlib
import json
import pathlib
import sys

manifest_file, evidence_file, closure_file, certificate_file, evidence_relative, closure_relative = sys.argv[1:]
manifest_path = pathlib.Path(manifest_file)
evidence_path = pathlib.Path(evidence_file)
closure_path = pathlib.Path(closure_file)
evidence = json.loads(evidence_path.read_text(encoding="utf-8"))
payload = json.loads(manifest_path.read_text(encoding="utf-8"))
digest = lambda path: hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
payload["browser_e2e"] = {
    "closure_file": closure_relative,
    "closure_sha256": digest(closure_path),
    "evidence_file": evidence_relative,
    "evidence_sha256": digest(evidence_path),
    "scanned_artifact_bytes": evidence["scannedArtifactBytes"],
    "scanned_artifact_files": evidence["scannedArtifactFiles"],
    "test": evidence["test"],
    "tls_certificate_sha256": digest(certificate_file),
}
temporary = manifest_path.with_suffix(".json.tmp")
temporary.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
temporary.replace(manifest_path)
PY
}

record_checksums_manifest() {
  python3 - "${RUN_DIR}/manifest.json" "${RUN_DIR}/checksums.txt" <<'PY'
import hashlib
import json
import pathlib
import sys

manifest_path, checksums_path = map(pathlib.Path, sys.argv[1:])
payload = json.loads(manifest_path.read_text(encoding="utf-8"))
payload["checksums_sha256"] = hashlib.sha256(checksums_path.read_bytes()).hexdigest()
temporary = manifest_path.with_suffix(".json.tmp")
temporary.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
temporary.replace(manifest_path)
PY
}

run_stage() {
  local stage="$1"
  shift
  local started status finished
  started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf '%s\t%s\t%s\n' "${started}" "${stage}" "$*" >> "${RUN_DIR}/commands.tsv"
  set +e
  (trap - EXIT INT TERM; set -Eeuo pipefail; "$@") > "${RUN_DIR}/logs/${stage}.log" 2>&1
  status="$?"
  set -e
  cat "${RUN_DIR}/logs/${stage}.log" || true
  finished="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf '%s\t%s\t%s\n' "${finished}" "${stage}" "exit=${status}" >> "${RUN_DIR}/commands.tsv"
  if [[ "${status}" -ne 0 ]]; then
    printf '%s\n' "${stage}" > "${RUN_DIR}/FAILED_STAGE"
    return "${status}"
  fi
}

verify_checkout_provenance() {
  local branch common_directory config_includes object_directory origin_main promisor_config shallow
  local -a local_origin_urls=() origin_urls=() remotes=()

  mapfile -t remotes < <(git --no-replace-objects remote)
  if [[ "${#remotes[@]}" -ne 1 || "${remotes[0]:-}" != "origin" ]]; then
    echo "formal checkout must have exactly one remote named origin" >&2
    printf 'found remote: %s\n' "${remotes[@]}" >&2
    return 2
  fi
  mapfile -t local_origin_urls < <(
    git --no-replace-objects config --file "${GIT_DIRECTORY}/config" --get-all remote.origin.url
  )
  mapfile -t origin_urls < <(git --no-replace-objects config --get-all remote.origin.url)
  if [[ "${#local_origin_urls[@]}" -ne 1 || "${#origin_urls[@]}" -ne 1 || \
        "${local_origin_urls[0]:-}" != "https://github.com/FengYuchen1314/NodeControll.git" || \
        "${origin_urls[0]:-}" != "${local_origin_urls[0]:-}" ]]; then
    echo "formal checkout origin must be the public NodeControll repository" >&2
    return 2
  fi
  config_includes="$(
    git --no-replace-objects config --file "${GIT_DIRECTORY}/config" --name-only \
      --get-regexp '^include' 2>/dev/null || true
  )"
  if [[ -n "${config_includes}" ]]; then
    echo "formal checkout local config must not include external config files" >&2
    printf '%s\n' "${config_includes}" >&2
    return 2
  fi
  common_directory="$(readlink -f "$(git --no-replace-objects rev-parse --git-common-dir)")"
  object_directory="$(readlink -f "$(git --no-replace-objects rev-parse --git-path objects)")"
  if [[ "${common_directory}" != "${GIT_DIRECTORY}" || \
        "${object_directory}" != "${GIT_DIRECTORY}/objects" || \
        -e "${GIT_DIRECTORY}/commondir" || -L "${GIT_DIRECTORY}/commondir" ]]; then
    echo "formal checkout must keep its common and object directories inside its standalone .git" >&2
    return 2
  fi
  branch="$(git --no-replace-objects symbolic-ref --quiet --short HEAD || true)"
  if [[ "${branch}" != "main" ]]; then
    echo "formal checkout must have main checked out, found ${branch:-detached HEAD}" >&2
    return 2
  fi
  origin_main="$(git --no-replace-objects rev-parse --verify refs/remotes/origin/main^{commit} 2>/dev/null || true)"
  if [[ "${origin_main}" != "${SOURCE_REVISION}" ]]; then
    echo "origin/main does not identify the verified source revision" >&2
    return 2
  fi
  shallow="$(git --no-replace-objects rev-parse --is-shallow-repository)"
  if [[ "${shallow}" != "false" || -e "${GIT_DIRECTORY}/shallow" || -L "${GIT_DIRECTORY}/shallow" ]]; then
    echo "formal checkout must not be shallow" >&2
    return 2
  fi
  for forbidden_input in \
    "${GIT_DIRECTORY}/info/grafts" \
    "${GIT_DIRECTORY}/objects/info/alternates"; do
    if [[ -e "${forbidden_input}" || -L "${forbidden_input}" ]]; then
      echo "formal checkout contains forbidden object indirection: ${forbidden_input}" >&2
      return 2
    fi
  done
  promisor_config="$(git --no-replace-objects config --get-regexp '^remote\..*\.promisor$' || true)"
  if [[ -n "${promisor_config}" ]]; then
    echo "formal checkout must not use a partial-clone promisor remote" >&2
    printf '%s\n' "${promisor_config}" >&2
    return 2
  fi
  printf 'verified standalone full checkout of origin/main at %s\n' "${SOURCE_REVISION}"
}

verify_source_revision_integrity() {
  local current_head current_blob current_sha replacement_refs
  current_head="$(git --no-replace-objects rev-parse --verify HEAD)"
  if [[ "${current_head}" != "${SOURCE_REVISION}" ]]; then
    echo "source checkout HEAD changed during verification: ${current_head}" >&2
    return 2
  fi
  replacement_refs="$(git --no-replace-objects for-each-ref --format='%(refname)' refs/replace/)"
  if [[ -n "${replacement_refs}" ]]; then
    echo "source repository contains forbidden Git replacement refs" >&2
    printf '%s\n' "${replacement_refs}" >&2
    return 2
  fi
  python3 "${SOURCE_VERIFIER}" "${SOURCE_REVISION}" "${WORKTREE}" "${WORKTREE}"
  if [[ ! -f tools/collect_third_party_licenses.mjs || -L tools/collect_third_party_licenses.mjs ]]; then
    echo "license collector must remain a regular non-symlink source file" >&2
    return 2
  fi
  current_blob="$(git --no-replace-objects hash-object -- tools/collect_third_party_licenses.mjs)"
  current_sha="$(sha256sum tools/collect_third_party_licenses.mjs | cut -d' ' -f1)"
  if [[ "${current_blob}" != "${LICENSE_COLLECTOR_BLOB}" || "${current_sha}" != "${LICENSE_COLLECTOR_SHA256}" ]]; then
    echo "license collector changed during verification" >&2
    return 2
  fi
}

verify_source_clean_after_tests() {
  local ignored status
  verify_source_revision_integrity
  status="$(git status --porcelain=v1 --untracked-files=all)"
  if [[ -n "${status}" ]]; then
    echo "source checkout has tracked or untracked changes after tests" >&2
    printf '%s\n' "${status}" >&2
    return 2
  fi
  ignored="$(git status --porcelain=v1 --ignored=matching --untracked-files=all | sed -n 's/^!! //p')"
  if [[ -n "${ignored}" ]]; then
    echo "source checkout has ignored inputs after tests" >&2
    printf '%s\n' "${ignored}" >&2
    return 2
  fi
}

install_source_verifier() {
  if [[ -e "${SOURCE_VERIFIER}" || -L "${SOURCE_VERIFIER}" ]]; then
    echo "source verifier destination must not pre-exist" >&2
    return 2
  fi
  git --no-replace-objects cat-file blob "${SOURCE_REVISION}:tools/verify_tracked_source.py" > "${SOURCE_VERIFIER}"
  chmod 0500 -- "${SOURCE_VERIFIER}"
  if [[ "$(git --no-replace-objects hash-object -- "${SOURCE_VERIFIER}")" != "${SOURCE_VERIFIER_BLOB}" ]]; then
    echo "extracted source verifier does not match its pinned Git blob" >&2
    return 2
  fi
  python3 "${SOURCE_VERIFIER}" "${SOURCE_REVISION}" "${WORKTREE}" "${WORKTREE}"
}

prepare_isolated_build_directories() {
  umask 077
  for directory in "${NODE_WORKSPACE}" "${NODE_PNPM_STORE}" "${PRIVATE_CARGO_HOME}" \
    "${PRIVATE_CARGO_TEST_TARGET}"; do
    if [[ -e "${directory}" || -L "${directory}" ]]; then
      echo "isolated build directory must not pre-exist: ${directory}" >&2
      return 2
    fi
    mkdir --mode=0700 -- "${directory}"
  done
  mkdir --mode=0700 -- "${PRIVATE_CARGO_HOME}/registry" "${PRIVATE_CARGO_HOME}/git"
}

export_node_workspace() {
  if [[ -n "$(find "${NODE_WORKSPACE}" -mindepth 1 -print -quit)" ]]; then
    echo "isolated Node workspace must be empty before export" >&2
    return 2
  fi
  git --no-replace-objects archive --format=tar "${SOURCE_REVISION}" \
    | tar --extract --file=- --directory="${NODE_WORKSPACE}" --no-same-owner
  python3 "${SOURCE_VERIFIER}" "${SOURCE_REVISION}" "${WORKTREE}" "${NODE_WORKSPACE}"
}

verify_node_workspace_integrity() {
  python3 "${SOURCE_VERIFIER}" "${SOURCE_REVISION}" "${WORKTREE}" "${NODE_WORKSPACE}"
  python3 - "${SOURCE_REVISION}" "${WORKTREE}" "${NODE_WORKSPACE}" <<'PY'
import os
import pathlib
import stat
import subprocess
import sys

revision, repository_raw, candidate_raw = sys.argv[1:]
repository = pathlib.Path(repository_raw).resolve(strict=True)
candidate = pathlib.Path(candidate_raw).resolve(strict=True)
environment = os.environ.copy()
environment["GIT_NO_REPLACE_OBJECTS"] = "1"
tree = subprocess.run(
    ["git", "--no-replace-objects", "-C", str(repository),
     "ls-tree", "-r", "-z", "--name-only", revision],
    check=True,
    stdout=subprocess.PIPE,
    env=environment,
).stdout.split(b"\0")
tracked_files = {item.decode("utf-8") for item in tree if item}
tracked_directories = set()
for item in tracked_files:
    pure = pathlib.PurePosixPath(item)
    tracked_directories.update(parent.as_posix() for parent in pure.parents if parent.as_posix() != ".")

allowed_roots = [pathlib.PurePosixPath("node_modules"), pathlib.PurePosixPath("apps/web/node_modules")]

for allowed_root in allowed_roots:
    allowed_path = candidate.joinpath(*allowed_root.parts)
    try:
        allowed_metadata = allowed_path.lstat()
    except FileNotFoundError:
        continue
    if not stat.S_ISDIR(allowed_metadata.st_mode) or stat.S_ISLNK(allowed_metadata.st_mode):
        raise SystemExit(f"allowed generated root must be a real directory: {allowed_root.as_posix()}")

def allowed_extra(relative: pathlib.PurePosixPath) -> bool:
    return any(relative == root or root in relative.parents for root in allowed_roots)

stack = [(candidate, pathlib.PurePosixPath())]
extras = []
while stack:
    directory, relative_directory = stack.pop()
    with os.scandir(directory) as entries:
        current_entries = sorted(entries, key=lambda item: item.name)
    for entry in current_entries:
        relative = relative_directory / entry.name
        text = relative.as_posix()
        metadata = entry.stat(follow_symlinks=False)
        is_directory = stat.S_ISDIR(metadata.st_mode) and not stat.S_ISLNK(metadata.st_mode)
        if text not in tracked_files and text not in tracked_directories and not allowed_extra(relative):
            extras.append(text + ("/" if is_directory else ""))
            continue
        if is_directory:
            stack.append((pathlib.Path(entry.path), relative))

if extras:
    preview = "\n".join(extras[:100])
    suffix = "" if len(extras) <= 100 else f"\n... and {len(extras) - 100} more"
    raise SystemExit(f"isolated Node workspace has forbidden extra paths:\n{preview}{suffix}")
print("verified isolated Node extra-path closure")
PY
}

write_directory_closure() {
  local input_root="$1"
  local output_file="$2"
  local label="$3"
  python3 - "${input_root}" "${output_file}" "${label}" <<'PY'
import hashlib
import json
import os
import pathlib
import stat
import sys

root = pathlib.Path(sys.argv[1])
output = pathlib.Path(sys.argv[2])
label = sys.argv[3]
metadata = root.lstat()
if not stat.S_ISDIR(metadata.st_mode) or stat.S_ISLNK(metadata.st_mode):
    raise SystemExit(f"{label} root must be a real directory")
root = root.resolve(strict=True)
entries = []
for current_root, directory_names, file_names in os.walk(root, topdown=True, followlinks=False):
    directory_names.sort()
    file_names.sort()
    current = pathlib.Path(current_root)
    for name in list(directory_names):
        path = current / name
        item = path.lstat()
        relative = path.relative_to(root).as_posix()
        if stat.S_ISLNK(item.st_mode):
            directory_names.remove(name)
            entries.append({"kind": "symlink", "mode": stat.S_IMODE(item.st_mode), "path": relative,
                            "target": os.readlink(path)})
        elif stat.S_ISDIR(item.st_mode):
            entries.append({"kind": "directory", "mode": stat.S_IMODE(item.st_mode), "path": relative})
        else:
            raise SystemExit(f"{label} contains a special directory entry: {relative}")
    for name in file_names:
        path = current / name
        item = path.lstat()
        relative = path.relative_to(root).as_posix()
        if stat.S_ISLNK(item.st_mode):
            entries.append({"kind": "symlink", "mode": stat.S_IMODE(item.st_mode), "path": relative,
                            "target": os.readlink(path)})
            continue
        if not stat.S_ISREG(item.st_mode):
            raise SystemExit(f"{label} contains a special file: {relative}")
        digest = hashlib.sha256()
        with path.open("rb") as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(chunk)
        entries.append({"bytes": item.st_size, "kind": "file", "mode": stat.S_IMODE(item.st_mode),
                        "path": relative, "sha256": digest.hexdigest()})
payload = {"entries": sorted(entries, key=lambda item: item["path"]), "label": label, "schema_version": 1}
temporary = output.with_suffix(output.suffix + ".tmp")
temporary.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
temporary.replace(output)
print(f"recorded {len(entries)} {label} closure entries")
PY
}

verify_directory_closure() {
  local input_root="$1"
  local expected_file="$2"
  local label="$3"
  local current_file="${expected_file}.current"
  if [[ -e "${current_file}" || -L "${current_file}" ]]; then
    echo "directory closure temporary path already exists: ${current_file}" >&2
    return 2
  fi
  write_directory_closure "${input_root}" "${current_file}" "${label}"
  cmp -- "${expected_file}" "${current_file}"
  rm -f -- "${current_file}"
}

record_private_input_closures() {
  python3 - "${RUN_DIR}/manifest.json" "${CARGO_INPUT_CLOSURE}" "${PNPM_INPUT_CLOSURE}" <<'PY'
import hashlib
import json
import pathlib
import sys

manifest_path = pathlib.Path(sys.argv[1])
payload = json.loads(manifest_path.read_text(encoding="utf-8"))
closures = {}
for label, raw_path in (("cargo_home", sys.argv[2]), ("pnpm_store", sys.argv[3])):
    path = pathlib.Path(raw_path)
    data = path.read_bytes()
    closures[label] = {
        "path": path.relative_to(manifest_path.parent).as_posix(),
        "sha256": hashlib.sha256(data).hexdigest(),
    }
payload["private_input_closures"] = closures
temporary = manifest_path.with_suffix(".json.tmp")
temporary.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
temporary.replace(manifest_path)
PY
}

verify_artifact_snapshot_integrity() {
  local actual_sha
  if [[ ! -f "${ACTIONS_ARTIFACT_SNAPSHOT}" || -L "${ACTIONS_ARTIFACT_SNAPSHOT}" ]]; then
    echo "Actions artifact snapshot must remain a regular non-symlink file" >&2
    return 2
  fi
  actual_sha="$(sha256sum "${ACTIONS_ARTIFACT_SNAPSHOT}" | cut -d' ' -f1)"
  if [[ "${actual_sha}" != "${ACTIONS_ARTIFACT_SHA}" ]]; then
    echo "Actions artifact snapshot changed during verification: ${actual_sha}" >&2
    return 2
  fi
}

extract_pinned_node_runtime() {
  umask 077
  mkdir --mode=0700 -- "${E2E_NODE_RUNTIME}"
  docker run --rm --entrypoint tar "${NODE_IMAGE_ID}" \
    -C / -cf - \
    usr/local/bin/node \
    | tar --extract --file - --directory "${E2E_NODE_RUNTIME}"

  local node_path="${E2E_NODE_RUNTIME}/usr/local/bin/node"
  if [[ ! -f "${node_path}" || -L "${node_path}" || ! -x "${node_path}" ]]; then
    echo "pinned Node image did not yield an executable regular Node binary" >&2
    return 2
  fi
  printf '%s\n' "extracted pinned Node.js runtime from ${NODE_IMAGE_ID}"
}

download_pinned_cyclonedx_cli() {
  if [[ -e "${CYCLONEDX_CLI_FILE}" || -L "${CYCLONEDX_CLI_FILE}" ]]; then
    echo "CycloneDX CLI destination must not pre-exist" >&2
    return 2
  fi
  umask 077
  curl \
    --proto '=https' \
    --tlsv1.2 \
    --fail \
    --location \
    --silent \
    --show-error \
    --output "${CYCLONEDX_CLI_FILE}" \
    "${CYCLONEDX_CLI_URL}"
  if [[ ! -f "${CYCLONEDX_CLI_FILE}" || -L "${CYCLONEDX_CLI_FILE}" ]]; then
    echo "downloaded CycloneDX CLI is not a regular non-symlink file" >&2
    return 2
  fi
  local actual_sha version_output
  actual_sha="$(sha256sum "${CYCLONEDX_CLI_FILE}" | cut -d' ' -f1)"
  if [[ "${actual_sha}" != "${CYCLONEDX_CLI_SHA256}" ]]; then
    echo "CycloneDX CLI SHA-256 mismatch: ${actual_sha}" >&2
    return 2
  fi
  chmod 0500 -- "${CYCLONEDX_CLI_FILE}"
  version_output="$("${CYCLONEDX_CLI_FILE}" --version)"
  case "${version_output}" in
    "${CYCLONEDX_CLI_VERSION}"+*) ;;
    *)
      echo "unexpected CycloneDX CLI version output: ${version_output}" >&2
      return 2
      ;;
  esac
  printf '%s\n' "verified CycloneDX CLI ${version_output} sha256:${actual_sha}"
}

validate_archive_members() {
  local archive="$1"
  python3 - "${archive}" <<'PY'
import pathlib
import re
import stat
import sys
import tarfile

archive = pathlib.Path(sys.argv[1])
metadata = archive.lstat()
if not stat.S_ISREG(metadata.st_mode) or stat.S_ISLNK(metadata.st_mode):
    raise SystemExit("Actions artifact archive must be a regular non-symlink file")
if metadata.st_size > 256 * 1024 * 1024:
    raise SystemExit("Actions artifact compressed size exceeds 256 MiB")

maximum_members = 50_000
maximum_file_size = 256 * 1024 * 1024
maximum_total_size = 1024 * 1024 * 1024
seen = {}
members_by_normalized = {}
total_size = 0

try:
    bundle = tarfile.open(archive, mode="r:gz", errorlevel=2)
except (OSError, tarfile.TarError) as error:
    raise SystemExit(f"cannot parse Actions artifact tar archive: {error}") from error

with bundle:
    if bundle.pax_headers:
        raise SystemExit("Actions artifact must not contain global PAX headers")
    try:
        members = bundle.getmembers()
    except (OSError, tarfile.TarError) as error:
        raise SystemExit(f"cannot enumerate Actions artifact tar members: {error}") from error
    if not members or len(members) > maximum_members:
        raise SystemExit(f"Actions artifact member count is outside 1..{maximum_members}")
    for member in members:
        raw_name = member.name
        try:
            raw_name.encode("ascii")
        except UnicodeEncodeError as error:
            raise SystemExit(f"Actions artifact member name must be ASCII: {raw_name!r}") from error
        if re.search(r"[\x00-\x1f\x7f]", raw_name) or "\\" in raw_name:
            raise SystemExit(f"Actions artifact member name contains forbidden characters: {raw_name!r}")
        if raw_name == ".":
            normalized = ""
        elif raw_name.startswith("./"):
            normalized = raw_name[2:]
        else:
            raise SystemExit(f"Actions artifact member lacks canonical ./ prefix: {raw_name!r}")
        if normalized:
            pure = pathlib.PurePosixPath(normalized)
            if (normalized != pure.as_posix() or normalized.startswith("/")
                    or any(part in ("", ".", "..") for part in pure.parts)):
                raise SystemExit(f"Actions artifact member path is not canonical: {raw_name!r}")
        if normalized in seen:
            raise SystemExit(
                f"Actions artifact contains duplicate/aliased member {raw_name!r}; "
                f"first occurrence was {seen[normalized]!r}"
            )
        seen[normalized] = raw_name
        members_by_normalized[normalized] = member
        if member.mode & 0o7022:
            raise SystemExit(f"Actions artifact member has unsafe permission bits: {raw_name!r}")
        if member.pax_headers:
            raise SystemExit(f"Actions artifact member uses PAX extensions: {raw_name!r}")
        if getattr(member, "sparse", None) is not None:
            raise SystemExit(f"Actions artifact member is sparse: {raw_name!r}")
        if member.isdir():
            if member.size != 0:
                raise SystemExit(f"Actions artifact directory has non-zero payload: {raw_name!r}")
        elif member.isreg():
            if normalized == "":
                raise SystemExit("Actions artifact root member must be a directory")
            if member.size < 0 or member.size > maximum_file_size:
                raise SystemExit(f"Actions artifact member exceeds the per-file size limit: {raw_name!r}")
            total_size += member.size
            if total_size > maximum_total_size:
                raise SystemExit("Actions artifact total uncompressed size exceeds 1 GiB")
        else:
            raise SystemExit(f"Actions artifact contains a non-file/non-directory member: {raw_name!r}")

for normalized, raw_name in seen.items():
    if normalized == "":
        continue
    pure = pathlib.PurePosixPath(normalized)
    for parent in pure.parents:
        parent_name = "" if parent.as_posix() == "." else parent.as_posix()
        if parent_name not in seen:
            raise SystemExit(f"Actions artifact member has an undeclared parent directory: {raw_name!r}")
        parent_member = members_by_normalized[parent_name]
        if not parent_member.isdir():
            raise SystemExit(f"Actions artifact member parent is not a directory: {raw_name!r}")

print(f"validated {len(members)} canonical archive members and {total_size} uncompressed file bytes")
PY
}

verify_package_contents() {
  local package_root="$1"
  local expected_revision="$2"
  python3 - "${package_root}" "${expected_revision}" <<'PY'
from collections import Counter
import datetime
import hashlib
import json
import os
import pathlib
import re
import stat
import subprocess
import sys
import tomllib
import urllib.parse

root = pathlib.Path(sys.argv[1]).resolve(strict=True)
expected_revision = sys.argv[2]
source_root = pathlib.Path.cwd().resolve(strict=True)
expected_repository = "FengYuchen1314/NodeControll"
catalog_relative = "third_party/dependency-license-overrides/overrides.json"
hex40 = re.compile(r"[0-9a-f]{40}")
hex64 = re.compile(r"[0-9a-f]{64}")
sri = re.compile(r"sha(?:256|384|512)-[A-Za-z0-9+/]+={0,2}")
semver = re.compile(
    r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)"
    r"(?:-(?:[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?"
    r"(?:\+(?:[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?"
)
license_pointer_stub = re.compile(
    rb"(?:\.\.[\\/])+(?:licen[cs]e|copying|notice|copyright|unlicense)(?:[-._][A-Za-z0-9.-]+)?",
    re.IGNORECASE,
)

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

def require_object(value, label: str) -> dict:
    require(isinstance(value, dict), f"{label} must be an object")
    return value

def require_array(value, label: str) -> list:
    require(isinstance(value, list), f"{label} must be an array")
    return value

def require_string(value, label: str) -> str:
    require(isinstance(value, str) and value != "", f"{label} must be a non-empty string")
    return value

def require_integer(value, label: str, minimum: int = 0) -> int:
    require(isinstance(value, int) and not isinstance(value, bool) and value >= minimum,
            f"{label} must be an integer >= {minimum}")
    return value

def hash_file(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()

def regular_file(path: pathlib.Path, label: str) -> os.stat_result:
    try:
        metadata = path.lstat()
    except OSError as error:
        raise SystemExit(f"{label} is missing or unreadable: {error}") from error
    require(stat.S_ISREG(metadata.st_mode) and not stat.S_ISLNK(metadata.st_mode),
            f"{label} must be a regular non-symlink file")
    return metadata

def canonical_relative(value, label: str, required_prefix: str | None = None) -> pathlib.PurePosixPath:
    require_string(value, label)
    require("\\" not in value and not value.startswith("/") and not value.endswith("/"),
            f"{label} must be a canonical relative POSIX path")
    pure = pathlib.PurePosixPath(value)
    require(value == pure.as_posix() and pure.parts and all(part not in ("", ".", "..") for part in pure.parts),
            f"{label} must be a canonical relative POSIX path")
    if required_prefix is not None:
        require(value.startswith(required_prefix), f"{label} must start with {required_prefix!r}")
    return pure

def file_beneath(base: pathlib.Path, relative: str, label: str) -> pathlib.Path:
    pure = canonical_relative(relative, label)
    target = base.joinpath(*pure.parts)
    regular_file(target, label)
    resolved = target.resolve(strict=True)
    resolved_base = base.resolve(strict=True)
    require(resolved_base in resolved.parents, f"{label} escapes its expected directory")
    return target

def load_json(path: pathlib.Path, label: str):
    regular_file(path, label)
    def reject_constant(value: str):
        raise ValueError(f"non-finite JSON number {value!r}")
    def unique_object(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"duplicate JSON object key {key!r}")
            result[key] = value
        return result
    try:
        return json.loads(
            path.read_text(encoding="utf-8"),
            parse_constant=reject_constant,
            object_pairs_hook=unique_object,
        )
    except (OSError, UnicodeError, json.JSONDecodeError, ValueError) as error:
        raise SystemExit(f"cannot parse {label}: {error}") from error

def js_encode(value: str) -> str:
    return urllib.parse.quote(value, safe="-._~")

def purl_qualifier_encode(value: str) -> str:
    return urllib.parse.quote(value, safe="-._~:")

def require_repository_url(value: str, label: str) -> None:
    require(not any(character.isspace() or ord(character) < 0x20 or ord(character) == 0x7f
                    for character in value) and "\\" not in value,
            f"{label} contains whitespace, controls, or a backslash")
    try:
        parsed = urllib.parse.urlsplit(value)
        _ = parsed.port
    except ValueError as error:
        raise SystemExit(f"{label} is not a valid absolute repository URI: {error}") from error
    require(parsed.scheme.lower() in {"http", "https", "ssh", "git"} and parsed.netloc != ""
            and parsed.hostname is not None,
            f"{label} must be an absolute http(s), ssh, or git repository URI")
    if parsed.scheme.lower() in {"http", "https"}:
        require(parsed.username is None and parsed.password is None,
                f"{label} must not put credentials in an HTTP(S) repository URI")

def normalized_properties(value, label: str) -> Counter:
    properties = require_array(value, label)
    rows = []
    for index, item in enumerate(properties):
        item = require_object(item, f"{label}[{index}]")
        require(set(item) == {"name", "value"}, f"{label}[{index}] has an invalid shape")
        rows.append((require_string(item.get("name"), f"{label}[{index}].name"),
                     require_string(item.get("value"), f"{label}[{index}].value")))
    require(len(rows) == len(set(rows)), f"{label} contains duplicate name/value pairs")
    return Counter(rows)

require(hex40.fullmatch(expected_revision) is not None,
        f"expected package revision is not a lowercase full commit ID: {expected_revision!r}")
require(root != source_root, "compiled package root must be separate from the source checkout")
for legal_name in ("LICENSE", "THIRD_PARTY_NOTICES.md"):
    source_legal = source_root / legal_name
    packaged_legal = root / legal_name
    regular_file(source_legal, f"source {legal_name}")
    regular_file(packaged_legal, f"packaged {legal_name}")
    require(hash_file(source_legal) == hash_file(packaged_legal)
            and source_legal.read_bytes() == packaged_legal.read_bytes(),
            f"packaged {legal_name} does not exactly match the checked-out source")

manifest = root / "CONTENTS.sha256"
regular_file(manifest, "CONTENTS.sha256")
manifest_text = manifest.read_text(encoding="utf-8")
require(manifest_text.endswith("\n") and "\r" not in manifest_text,
        "CONTENTS.sha256 must be LF-terminated")
listed: dict[str, str] = {}
for line in manifest_text.splitlines():
    digest, separator, relative = line.partition("  ")
    if separator != "  " or hex64.fullmatch(digest) is None or not relative.startswith("./"):
        raise SystemExit(f"invalid CONTENTS.sha256 row: {line!r}")
    canonical_relative(relative.removeprefix("./"), "CONTENTS.sha256 path")
    if relative in listed:
        raise SystemExit(f"duplicate CONTENTS.sha256 path: {relative}")
    listed[relative] = digest

actual = set()
actual_directories = set()
for path in root.rglob("*"):
    metadata = path.lstat()
    require(not stat.S_ISLNK(metadata.st_mode),
            f"compiled package must not contain symlinks: {path.relative_to(root).as_posix()}")
    require(metadata.st_mode & 0o7022 == 0,
            f"compiled package entry has unsafe permission bits: {path.relative_to(root).as_posix()}")
    if stat.S_ISDIR(metadata.st_mode):
        actual_directories.add(path.relative_to(root).as_posix())
        continue
    require(stat.S_ISREG(metadata.st_mode),
            f"compiled package contains a non-regular entry: {path.relative_to(root).as_posix()}")
    if path != manifest:
        actual.add("./" + path.relative_to(root).as_posix())
if actual != set(listed):
    missing = sorted(actual - set(listed))
    extra = sorted(set(listed) - actual)
    raise SystemExit(f"package content manifest mismatch: unlisted={missing!r} absent={extra!r}")
expected_directories = set()
for relative in actual:
    pure = pathlib.PurePosixPath(relative.removeprefix("./"))
    expected_directories.update(
        parent.as_posix()
        for parent in pure.parents
        if parent.as_posix() != "."
    )
require(actual_directories == expected_directories,
        f"package directory closure mismatch: empty/extra={sorted(actual_directories - expected_directories)!r} "
        f"missing={sorted(expected_directories - actual_directories)!r}")
for relative, expected in sorted(listed.items()):
    target = (root / relative.removeprefix("./")).resolve(strict=True)
    if root not in target.parents:
        raise SystemExit(f"content path escapes package root: {relative}")
    actual_digest = hash_file(target)
    if actual_digest != expected:
        raise SystemExit(f"content checksum mismatch: {relative}")

dependencies_path = root / "notices" / "DEPENDENCIES.json"
bom_path = root / "notices" / "bom.cdx.json"
license_manifest_path = root / "notices" / "LICENSES.sha256"
inventory = require_object(load_json(dependencies_path, "notices/DEPENDENCIES.json"),
                           "notices/DEPENDENCIES.json")
require(set(inventory) == {
    "schema_version", "source_revision", "source_repository", "generated_from",
    "override_catalog", "components", "issues", "warnings",
}, "DEPENDENCIES.json has an invalid schema-v2 top-level shape")
require(inventory.get("schema_version") == 2, "DEPENDENCIES.json schema_version must be 2")
require(inventory.get("source_revision") == expected_revision,
        "DEPENDENCIES.json source_revision does not match the checked-out commit")
require(inventory.get("source_repository") == expected_repository,
        "DEPENDENCIES.json source_repository is not FengYuchen1314/NodeControll")
require(inventory.get("generated_from") == [
    "Cargo.lock",
    "pnpm-lock.yaml",
    catalog_relative,
    "rustc --print sysroot:share/doc/rust",
], "DEPENDENCIES.json generated_from is not the complete schema-v2 input contract")
require(inventory.get("issues") == [], "DEPENDENCIES.json issues must be exactly []")
require(inventory.get("warnings") == [], "DEPENDENCIES.json warnings must be exactly []")

override_summary = require_object(inventory.get("override_catalog"),
                                  "DEPENDENCIES.json override_catalog")
require(set(override_summary) == {
    "path", "sha256", "schema_version", "source_audit_date", "declared_entries", "used_entries",
}, "DEPENDENCIES.json override_catalog has an invalid shape")
require(override_summary.get("path") == catalog_relative,
        "DEPENDENCIES.json override catalog path is not the repository catalog")
require(require_integer(override_summary.get("schema_version"), "override_catalog.schema_version", 1) == 1,
        "DEPENDENCIES.json override catalog schema_version must be 1")
require(override_summary.get("declared_entries") == 20 and override_summary.get("used_entries") == 20,
        "DEPENDENCIES.json override catalog must close exactly 20 declared and 20 used entries")
catalog_digest = override_summary.get("sha256")
require(isinstance(catalog_digest, str) and hex64.fullmatch(catalog_digest) is not None,
        "DEPENDENCIES.json override catalog sha256 is invalid")
audit_date = override_summary.get("source_audit_date")
try:
    datetime.date.fromisoformat(require_string(audit_date, "override_catalog.source_audit_date"))
except ValueError as error:
    raise SystemExit("override_catalog.source_audit_date is not an ISO calendar date") from error

catalog_path = file_beneath(source_root, catalog_relative, "source override catalog")
require(hash_file(catalog_path) == catalog_digest,
        "DEPENDENCIES.json override catalog hash does not match the checked-out source catalog")
catalog = require_object(load_json(catalog_path, "source override catalog"), "source override catalog")
require(require_integer(catalog.get("schemaVersion"), "source override catalog schemaVersion", 1) == 1,
        "source override catalog schemaVersion must be 1")
require(catalog.get("sourceAuditDate") == audit_date,
        "source override catalog audit date does not match DEPENDENCIES.json")
catalog_entries = require_array(catalog.get("entries"), "source override catalog entries")
require(len(catalog_entries) == 20, "source override catalog must contain exactly 20 entries")
override_root = catalog_path.parent
catalog_identities = {}
catalog_files = set()
catalog_file_signatures = {}

for offset, raw_entry in enumerate(catalog_entries, start=1):
    label = f"source override catalog entries[{offset - 1}]"
    entry = require_object(raw_entry, label)
    ecosystem = entry.get("ecosystem")
    require(ecosystem in ("cargo", "npm"), f"{label}.ecosystem must be cargo or npm")
    name = require_string(entry.get("name"), f"{label}.name")
    version = require_string(entry.get("version"), f"{label}.version")
    declared_license = require_string(entry.get("declaredLicense"), f"{label}.declaredLicense")
    repository = require_string(entry.get("repository"), f"{label}.repository")
    revision = require_string(entry.get("revision"), f"{label}.revision")
    require(hex40.fullmatch(revision) is not None, f"{label}.revision must be lowercase 40-hex")
    require_string(entry.get("resolution"), f"{label}.resolution")
    integrity_field = "registryChecksum" if ecosystem == "cargo" else "registryIntegrity"
    integrity = require_string(entry.get(integrity_field), f"{label}.{integrity_field}")
    if ecosystem == "cargo":
        require(hex64.fullmatch(integrity) is not None, f"{label}.registryChecksum is invalid")
    else:
        require(sri.fullmatch(integrity) is not None, f"{label}.registryIntegrity is invalid")

    identity = (ecosystem, name, version, integrity)
    require(identity not in catalog_identities, f"duplicate source override catalog identity: {identity!r}")
    catalog_identities[identity] = (offset, entry)

    version_evidence = require_object(entry.get("versionEvidence"), f"{label}.versionEvidence")
    canonical_relative(require_string(version_evidence.get("upstreamPath"),
                                      f"{label}.versionEvidence.upstreamPath"),
                       f"{label}.versionEvidence.upstreamPath")
    version_evidence_hash = require_string(version_evidence.get("sha256"),
                                           f"{label}.versionEvidence.sha256")
    require(hex64.fullmatch(version_evidence_hash) is not None,
            f"{label}.versionEvidence.sha256 is invalid")
    require_string(version_evidence.get("expectedName"), f"{label}.versionEvidence.expectedName")
    require(version_evidence.get("expectedVersion") == version,
            f"{label}.versionEvidence expectedVersion does not match the override component")

    entry_files = require_array(entry.get("files"), f"{label}.files")
    require(entry_files, f"{label}.files must not be empty")
    entry_local_paths = set()
    for file_offset, raw_spec in enumerate(entry_files, start=1):
        spec_label = f"{label}.files[{file_offset - 1}]"
        spec = require_object(raw_spec, spec_label)
        require_string(spec.get("kind"), f"{spec_label}.kind")
        require_string(spec.get("upstreamRepository"), f"{spec_label}.upstreamRepository")
        if "upstreamTag" in spec:
            require_string(spec.get("upstreamTag"), f"{spec_label}.upstreamTag")
        upstream_revision = require_string(spec.get("upstreamRevision"), f"{spec_label}.upstreamRevision")
        require(hex40.fullmatch(upstream_revision) is not None,
                f"{spec_label}.upstreamRevision must be lowercase 40-hex")
        canonical_relative(require_string(spec.get("upstreamPath"), f"{spec_label}.upstreamPath"),
                           f"{spec_label}.upstreamPath")
        has_line_start = "upstreamLineStart" in spec
        has_line_end = "upstreamLineEnd" in spec
        require(has_line_start == has_line_end,
                f"{spec_label} must provide upstreamLineStart and upstreamLineEnd together")
        if has_line_start:
            line_start = require_integer(spec.get("upstreamLineStart"),
                                         f"{spec_label}.upstreamLineStart", 1)
            line_end = require_integer(spec.get("upstreamLineEnd"),
                                       f"{spec_label}.upstreamLineEnd", line_start)
            require(line_end >= line_start, f"{spec_label} line range is reversed")
            require_string(spec.get("extraction"), f"{spec_label}.extraction")
        elif "extraction" in spec:
            require_string(spec.get("extraction"), f"{spec_label}.extraction")
        local_path = require_string(spec.get("localPath"), f"{spec_label}.localPath")
        canonical_relative(local_path, f"{spec_label}.localPath")
        require(local_path not in entry_local_paths,
                f"override evidence path is duplicated within one entry: {local_path}")
        entry_local_paths.add(local_path)
        shared_signature = (
            spec.get("kind"), spec.get("upstreamRepository"), spec.get("upstreamRevision"),
            spec.get("upstreamTag"), spec.get("upstreamPath"),
            spec.get("upstreamLineStart"), spec.get("upstreamLineEnd"), spec.get("extraction"),
            spec.get("sha256"), spec.get("bytes"),
        )
        prior_signature = catalog_file_signatures.get(local_path)
        require(prior_signature is None or prior_signature == shared_signature,
                f"shared override evidence has conflicting provenance: {local_path}")
        catalog_file_signatures.setdefault(local_path, shared_signature)
        catalog_files.add(local_path)
        evidence_hash = require_string(spec.get("sha256"), f"{spec_label}.sha256")
        require(hex64.fullmatch(evidence_hash) is not None, f"{spec_label}.sha256 is invalid")
        evidence_bytes = require_integer(spec.get("bytes"), f"{spec_label}.bytes", 1)
        source_evidence = file_beneath(override_root, local_path, f"source override evidence {local_path}")
        require(source_evidence.stat().st_size == evidence_bytes,
                f"source override evidence byte length changed: {local_path}")
        require(hash_file(source_evidence) == evidence_hash,
                f"source override evidence hash changed: {local_path}")
        require(source_evidence.read_bytes().strip() != b"",
                f"source override evidence is empty or whitespace-only: {local_path}")
    version_local_path = version_evidence.get("localPath")
    if version_local_path is not None:
        require(version_local_path in entry_local_paths,
                f"{label}.versionEvidence.localPath is not one of the entry evidence files")
        matching_spec = next(spec for spec in entry_files if spec.get("localPath") == version_local_path)
        require(matching_spec.get("sha256") == version_evidence_hash,
                f"{label}.versionEvidence hash does not match its evidence file")

actual_catalog_files = set()
for path in override_root.rglob("*"):
    metadata = path.lstat()
    require(not stat.S_ISLNK(metadata.st_mode),
            f"source override directory contains a symlink: {path.relative_to(override_root).as_posix()}")
    if stat.S_ISDIR(metadata.st_mode):
        continue
    require(stat.S_ISREG(metadata.st_mode),
            f"source override directory contains a non-regular entry: {path.relative_to(override_root).as_posix()}")
    relative = path.relative_to(override_root).as_posix()
    if relative not in ("overrides.json", "README.md"):
        actual_catalog_files.add(relative)
require(actual_catalog_files == catalog_files,
        f"source override evidence closure mismatch: unreferenced={sorted(actual_catalog_files - catalog_files)!r} "
        f"missing={sorted(catalog_files - actual_catalog_files)!r}")

with (source_root / "Cargo.lock").open("rb") as stream:
    cargo_lock = tomllib.load(stream)
with (source_root / "Cargo.toml").open("rb") as stream:
    workspace_manifest = tomllib.load(stream)
workspace = require_object(workspace_manifest.get("workspace"), "Cargo.toml workspace")
workspace_version = require_string(require_object(workspace.get("package"), "Cargo.toml workspace.package").get("version"),
                                   "Cargo.toml workspace.package.version")
workspace_packages = set()
for member in require_array(workspace.get("members"), "Cargo.toml workspace.members"):
    member_path = canonical_relative(require_string(member, "Cargo.toml workspace member"),
                                     "Cargo.toml workspace member")
    member_manifest_path = source_root.joinpath(*member_path.parts, "Cargo.toml")
    regular_file(member_manifest_path, f"workspace member manifest {member}")
    with member_manifest_path.open("rb") as stream:
        member_manifest = tomllib.load(stream)
    member_package = require_object(member_manifest.get("package"), f"{member}/Cargo.toml package")
    member_name = require_string(member_package.get("name"), f"{member}/Cargo.toml package.name")
    member_version_value = member_package.get("version")
    if isinstance(member_version_value, str):
        member_version = member_version_value
    else:
        require(member_version_value == {"workspace": True},
                f"{member}/Cargo.toml package.version must be a string or inherit workspace version")
        member_version = workspace_version
    require((member_name, member_version) not in workspace_packages,
            f"duplicate Cargo workspace package identity: {member_name}@{member_version}")
    workspace_packages.add((member_name, member_version))

expected_cargo = Counter()
for index, package in enumerate(require_array(cargo_lock.get("package"), "Cargo.lock package")):
    package = require_object(package, f"Cargo.lock package[{index}]")
    name = require_string(package.get("name"), f"Cargo.lock package[{index}].name")
    version = require_string(package.get("version"), f"Cargo.lock package[{index}].version")
    source = package.get("source")
    checksum = package.get("checksum")
    if source is None and checksum is None:
        require((name, version) in workspace_packages,
                f"Cargo.lock contains an unproven local package: {name}@{version}")
        continue
    require(isinstance(source, str) and source != "" and isinstance(checksum, str)
            and hex64.fullmatch(checksum) is not None,
            f"Cargo.lock dependency lacks a supported locked SHA-256: {name}@{version}")
    expected_cargo[(name, version, source, checksum)] += 1

def parse_yaml_scalar(value: str) -> str:
    value = value.strip()
    if value.startswith("'") and value.endswith("'"):
        return value[1:-1].replace("''", "'")
    if value.startswith('"') and value.endswith('"'):
        parsed = json.loads(value)
        require(isinstance(parsed, str), "pnpm lock scalar must decode to a string")
        return parsed
    return value

pnpm_lock_path = source_root / "pnpm-lock.yaml"
regular_file(pnpm_lock_path, "pnpm-lock.yaml")
pnpm_lines = pnpm_lock_path.read_text(encoding="utf-8").splitlines()
top_level_sections = {}
for line_index, line in enumerate(pnpm_lines):
    require("\t" not in line,
            f"pnpm-lock.yaml contains a tab on line {line_index + 1}")
    if line.strip() == "" or line.lstrip(" ").startswith("#") or line.startswith(" "):
        continue
    top_level_match = re.fullmatch(r"([A-Za-z][A-Za-z0-9_-]*):(.*)", line)
    require(top_level_match is not None,
            f"pnpm-lock.yaml contains a non-canonical top-level node on line {line_index + 1}")
    section_name = top_level_match.group(1)
    require(section_name not in top_level_sections,
            f"pnpm-lock.yaml contains duplicate top-level key {section_name!r}")
    top_level_sections[section_name] = (line_index, top_level_match.group(2).strip())
require(list(top_level_sections) == ["lockfileVersion", "settings", "importers", "packages", "snapshots"],
        "pnpm-lock.yaml top-level section order/closure differs from the audited v9 format")
require(top_level_sections["lockfileVersion"][1] == "'9.0'",
        "pnpm-lock.yaml must contain exactly one audited lockfileVersion '9.0'")
for section_name in ("settings", "importers", "packages", "snapshots"):
    require(top_level_sections[section_name][1] == "",
            f"pnpm-lock.yaml section {section_name!r} must use block mapping form")
pnpm_integrities = {}
pnpm_package_keys = set()
current_key = None
packages_start = top_level_sections["packages"][0] + 1
packages_end = top_level_sections["snapshots"][0]
require(packages_start < packages_end, "pnpm-lock.yaml packages section is empty or misordered")
for line in pnpm_lines[packages_start:packages_end]:
    package_match = re.fullmatch(r"  (\S.*):\s*", line)
    if package_match:
        current_key = parse_yaml_scalar(package_match.group(1))
        require(current_key not in pnpm_package_keys, f"duplicate pnpm lock package key: {current_key}")
        pnpm_package_keys.add(current_key)
        continue
    if current_key is None:
        continue
    resolution_match = re.fullmatch(r"    resolution:\s*\{(.*)\}\s*", line)
    if resolution_match is None:
        continue
    integrity_match = re.search(r"(?:^|,\s*)integrity:\s*([^,}]+)", resolution_match.group(1))
    if integrity_match is None:
        continue
    integrity = parse_yaml_scalar(integrity_match.group(1))
    require(sri.fullmatch(integrity) is not None,
            f"pnpm lock package {current_key} has an invalid registry integrity")
    require(current_key not in pnpm_integrities,
            f"pnpm lock package {current_key} has more than one integrity")
    pnpm_integrities[current_key] = integrity
require(pnpm_package_keys and pnpm_integrities,
        "pnpm-lock.yaml package integrity inventory is empty")

components = require_array(inventory.get("components"), "DEPENDENCIES.json components")
require(components, "DEPENDENCIES.json components must not be empty")
component_order = []
component_identities = set()
component_purls = set()
actual_cargo = Counter()
npm_identities = set()
evidence_paths = set()
evidence_hashes = {}
override_components = {}
rust_components = []

base_component_keys = {
    "ecosystem", "name", "version", "declared_license", "repository", "source", "purl",
    "locked_integrity", "locked_integrity_kind", "license_files",
}
for component_index, raw_component in enumerate(components):
    label = f"DEPENDENCIES.json components[{component_index}]"
    component = require_object(raw_component, label)
    require(base_component_keys <= set(component) <= base_component_keys | {"license_override", "toolchain_provenance"},
            f"{label} has an invalid schema-v2 shape")
    ecosystem = component.get("ecosystem")
    require(ecosystem in ("cargo", "npm", "rust-toolchain"), f"{label}.ecosystem is invalid")
    name = require_string(component.get("name"), f"{label}.name")
    version = require_string(component.get("version"), f"{label}.version")
    declared_license = require_string(component.get("declared_license"), f"{label}.declared_license")
    source = require_string(component.get("source"), f"{label}.source")
    purl = require_string(component.get("purl"), f"{label}.purl")
    repository = component.get("repository")
    require(repository is None or (isinstance(repository, str) and repository != ""),
            f"{label}.repository must be null or a non-empty string")
    if repository is not None:
        require_repository_url(repository, f"{label}.repository")
    locked_integrity = component.get("locked_integrity")
    locked_kind = component.get("locked_integrity_kind")
    identity = ((ecosystem, name, version, source, locked_integrity)
                if ecosystem == "cargo" else (ecosystem, name, version))
    require(identity not in component_identities, f"duplicate component identity: {identity!r}")
    require(purl not in component_purls, f"duplicate component purl: {purl}")
    component_identities.add(identity)
    component_purls.add(purl)
    component_order.append(identity)

    if ecosystem == "cargo":
        require(component.keys() == base_component_keys or set(component) == base_component_keys | {"license_override"},
                f"{label} has fields that do not belong to a Cargo component")
        require(isinstance(locked_integrity, str) and hex64.fullmatch(locked_integrity) is not None
                and locked_kind == "cargo-registry-sha256",
                f"{label} lacks Cargo lockfile SHA-256 provenance")
        require(source.startswith("registry+") or source.startswith("git+"),
                f"{label}.source is not a supported Cargo source URL")
        cargo_source_url = source.removeprefix("registry+") if source.startswith("registry+") else source.removeprefix("git+")
        expected_cargo_purl = (
            f"pkg:cargo/{js_encode(name)}@{js_encode(version)}"
            f"?repository_url={purl_qualifier_encode(cargo_source_url)}"
        )
        require(component.get("purl") == expected_cargo_purl,
                f"{label}.purl does not identify its exact Cargo package")
        actual_cargo[(name, version, source, locked_integrity)] += 1
    elif ecosystem == "npm":
        require(component.keys() == base_component_keys or set(component) == base_component_keys | {"license_override"},
                f"{label} has fields that do not belong to an npm component")
        npm_purl_name = "/".join(js_encode(part) for part in name.split("/"))
        require(component.get("purl") == f"pkg:npm/{npm_purl_name}@{js_encode(version)}",
                f"{label}.purl does not identify its exact npm package")
        require(source == "https://registry.npmjs.org/", f"{label}.source is not the npm registry")
        require(isinstance(locked_integrity, str) and sri.fullmatch(locked_integrity) is not None
                and locked_kind == "pnpm-registry-integrity",
                f"{label} lacks pnpm registry-integrity provenance")
        lock_key = f"{name}@{version}"
        require(pnpm_integrities.get(lock_key) == locked_integrity,
                f"{label} does not match pnpm-lock.yaml registry integrity")
        npm_identities.add((name, version))
    else:
        rust_components.append(component)

    license_files = require_array(component.get("license_files"), f"{label}.license_files")
    require(license_files, f"{label}.license_files must contain actual evidence")
    for evidence_index, raw_evidence in enumerate(license_files):
        evidence_label = f"{label}.license_files[{evidence_index}]"
        evidence = require_object(raw_evidence, evidence_label)
        require({"kind", "path", "sha256", "bytes", "source"} <= set(evidence)
                <= {"kind", "evidence_kind", "path", "sha256", "bytes", "source"},
                f"{evidence_label} has an invalid shape")
        evidence_kind = require_string(evidence.get("kind"), f"{evidence_label}.kind")
        evidence_path = require_string(evidence.get("path"), f"{evidence_label}.path")
        canonical_relative(evidence_path, f"{evidence_label}.path", "licenses/")
        require(evidence_path not in evidence_paths, f"license evidence path is reused: {evidence_path}")
        evidence_paths.add(evidence_path)
        evidence_hash = require_string(evidence.get("sha256"), f"{evidence_label}.sha256")
        require(hex64.fullmatch(evidence_hash) is not None, f"{evidence_label}.sha256 is invalid")
        evidence_bytes = require_integer(evidence.get("bytes"), f"{evidence_label}.bytes", 1)
        evidence_file = file_beneath(root / "notices", evidence_path, f"packaged evidence {evidence_path}")
        require(evidence_file.stat().st_size == evidence_bytes,
                f"packaged evidence byte length mismatch: {evidence_path}")
        require(hash_file(evidence_file) == evidence_hash,
                f"packaged evidence SHA-256 mismatch: {evidence_path}")
        payload = evidence_file.read_bytes()
        require(payload.strip() != b"", f"{evidence_label} is empty or whitespace-only")
        evidence_hashes[evidence_path] = evidence_hash
        evidence_source = require_object(evidence.get("source"), f"{evidence_label}.source")
        if evidence_kind == "package-license-or-notice":
            require(set(evidence) == {"kind", "path", "sha256", "bytes", "source"},
                    f"{evidence_label} package evidence has unexpected fields")
            require(set(evidence_source) == {"kind", "package_relative_path"}
                    and evidence_source.get("kind") == "package-archive",
                    f"{evidence_label} package evidence provenance is invalid")
            canonical_relative(require_string(evidence_source.get("package_relative_path"),
                                              f"{evidence_label}.source.package_relative_path"),
                               f"{evidence_label}.source.package_relative_path")
            require(not (len(payload) <= 256 and license_pointer_stub.fullmatch(payload.strip()) is not None),
                    f"{evidence_label} is only a relative license pointer, not substantive evidence")
        elif evidence_kind == "vetted-source-license-or-notice":
            require_string(evidence.get("evidence_kind"), f"{evidence_label}.evidence_kind")
            require(evidence_source.get("kind") == "vetted-source-override",
                    f"{evidence_label} override evidence provenance is invalid")
        elif ecosystem == "rust-toolchain" and evidence_kind in {
            "rust-toolchain-license-readme", "rust-standard-library-copyright", "rust-toolchain-license-text",
        }:
            require(set(evidence) == {"kind", "path", "sha256", "bytes", "source"},
                    f"{evidence_label} Rust evidence has unexpected fields")
            require(set(evidence_source) == {
                "kind", "sysroot_relative_path", "rustc_release", "rustc_commit_hash",
            } and evidence_source.get("kind") == "installed-rustc-sysroot",
                    f"{evidence_label} Rust sysroot provenance is invalid")
            canonical_relative(require_string(evidence_source.get("sysroot_relative_path"),
                                              f"{evidence_label}.source.sysroot_relative_path"),
                               f"{evidence_label}.source.sysroot_relative_path", "share/doc/rust/")
        else:
            raise SystemExit(f"{evidence_label}.kind is unsupported: {evidence_kind!r}")

    override = component.get("license_override")
    if override is not None:
        require(ecosystem in ("cargo", "npm"), f"{label} Rust component must not use an override")
        override_identity = (ecosystem, name, version, locked_integrity)
        require(override_identity in catalog_identities, f"{label} has no exact entry in the source override catalog")
        canonical_override_source = (
            "registry+https://github.com/rust-lang/crates.io-index"
            if ecosystem == "cargo" else "https://registry.npmjs.org/"
        )
        require(source == canonical_override_source,
                f"{label} override source is not the canonical locked registry")
        require(override_identity not in override_components,
                f"duplicate overridden component identity: {override_identity!r}")
        override_components[override_identity] = component
    else:
        require(all(item.get("kind") != "vetted-source-license-or-notice" for item in license_files),
                f"{label} has override evidence without license_override provenance")

require(component_order == sorted(component_order),
        "DEPENDENCIES.json components are not deterministically sorted by ecosystem/name/version")
require(actual_cargo == expected_cargo,
        f"Cargo component closure differs from Cargo.lock: missing={list((expected_cargo - actual_cargo).elements())!r} "
        f"extra={list((actual_cargo - expected_cargo).elements())!r}")
require(npm_identities, "DEPENDENCIES.json contains no npm component records")

tracked_manifests = subprocess.run(
    ["git", "--no-replace-objects", "-C", str(source_root),
     "ls-tree", "-r", "-z", "--name-only", expected_revision],
    check=True,
    stdout=subprocess.PIPE,
).stdout.split(b"\0")
tracked_manifest_names = []
for item in tracked_manifests:
    if not item:
        continue
    try:
        name = item.decode("utf-8")
    except UnicodeDecodeError as error:
        raise SystemExit(f"commit tree path is not UTF-8: {error}") from error
    if name == "package.json" or name.endswith("/package.json"):
        tracked_manifest_names.append(name)
require(len(tracked_manifest_names) == len(set(tracked_manifest_names)),
        "commit tree contains duplicate package.json paths")
tracked_manifest_paths = sorted(pathlib.PurePosixPath(name) for name in tracked_manifest_names)
require(pathlib.PurePosixPath("package.json") in tracked_manifest_paths,
        "tracked workspace root package.json is missing")
require(pathlib.PurePosixPath("apps/web/package.json") in tracked_manifest_paths,
        "tracked Web workspace package.json is missing")
for package_manifest_relative in tracked_manifest_paths:
    canonical_relative(package_manifest_relative.as_posix(), "tracked package.json path")
    package_manifest_path = source_root.joinpath(*package_manifest_relative.parts)
    require(not package_manifest_path.is_symlink(),
            f"source package manifest must not be a symlink: {package_manifest_relative}")
    package_manifest = require_object(load_json(package_manifest_path, "source package.json"), "source package.json")
    for dependency_group in ("dependencies", "devDependencies", "optionalDependencies"):
        declarations = package_manifest.get(dependency_group, {})
        require(isinstance(declarations, dict),
                f"{package_manifest_path.relative_to(source_root)} {dependency_group} must be an object")
        for name, specifier in declarations.items():
            if not isinstance(specifier, str) or specifier.startswith(("workspace:", "link:", "file:")):
                continue
            exact_version = specifier.removeprefix("=")
            require(semver.fullmatch(exact_version) is not None,
                    f"direct npm dependency must use a supported exact SemVer: {name}@{specifier}")
            require((name, exact_version) in npm_identities,
                    f"direct npm dependency is absent from the component inventory: {name}@{exact_version}")

require(len(rust_components) == 1,
        "DEPENDENCIES.json must contain exactly one Rust toolchain/runtime component")
rust_component = rust_components[0]
require((rust_component.get("name"), rust_component.get("version")) == ("rust-std-runtime", "1.98.0"),
        "Rust toolchain/runtime component must be rust-std-runtime@1.98.0")
require(set(rust_component) == base_component_keys | {"toolchain_provenance"},
        "Rust toolchain/runtime component has an invalid shape")
require(rust_component.get("declared_license") == "MIT OR Apache-2.0"
        and rust_component.get("repository") == "https://github.com/rust-lang/rust"
        and rust_component.get("source") == "rustc-sysroot:share/doc/rust"
        and rust_component.get("purl") == "pkg:generic/rust-std-runtime@1.98.0",
        "Rust toolchain/runtime identity or source provenance is invalid")
rust_commit = rust_component.get("locked_integrity")
require(rust_commit == "88d9e12ae178fab0fb5cc050a94da85685d449ea"
        and rust_component.get("locked_integrity_kind") == "rustc-commit",
        "Rust toolchain/runtime component does not match the pinned rustc commit")
toolchain = require_object(rust_component.get("toolchain_provenance"), "Rust toolchain provenance")
require(set(toolchain) == {
    "sysroot_query", "evidence_root", "rustc_release", "rustc_commit_hash", "rustc_commit_date",
    "rustc_host", "llvm_version",
}, "Rust toolchain provenance has an invalid shape")
require(toolchain == {
    "sysroot_query": "rustc --print sysroot",
    "evidence_root": "share/doc/rust",
    "rustc_release": "1.98.0",
    "rustc_commit_hash": "88d9e12ae178fab0fb5cc050a94da85685d449ea",
    "rustc_commit_date": "2026-08-18",
    "rustc_host": "x86_64-unknown-linux-gnu",
    "llvm_version": "22.1.8",
}, "Rust toolchain provenance does not match the pinned Rust 1.98.0 image")
expected_rust_evidence = {
    "share/doc/rust/README.md": "rust-toolchain-license-readme",
    "share/doc/rust/COPYRIGHT-library.html": "rust-standard-library-copyright",
    "share/doc/rust/licenses/Apache-2.0.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/BSD-2-Clause.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/CC-BY-SA-4.0.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/GCC-exception-3.1.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/GPL-2.0-only.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/GPL-3.0-or-later.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/ISC.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/LLVM-exception.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/MIT.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/NCSA.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/OFL-1.1.txt": "rust-toolchain-license-text",
    "share/doc/rust/licenses/Unicode-3.0.txt": "rust-toolchain-license-text",
}
actual_rust_evidence = {
    (
        item["source"].get("sysroot_relative_path"),
        item.get("path"),
    ): item.get("kind")
    for item in rust_component["license_files"]
}
expected_rust_evidence_with_destinations = {
    (
        source_path,
        "licenses/rust-toolchain/rust-std-runtime-1.98.0/"
        + source_path.removeprefix("share/doc/rust/"),
    ): evidence_kind
    for source_path, evidence_kind in expected_rust_evidence.items()
}
require(actual_rust_evidence == expected_rust_evidence_with_destinations,
        "Rust toolchain evidence paths/kinds do not match the pinned sysroot subset")
for item in rust_component["license_files"]:
    require(item["path"].startswith("licenses/rust-toolchain/rust-std-runtime-1.98.0/"),
            "Rust toolchain evidence is outside its versioned package directory")
    require(item["source"].get("rustc_release") == "1.98.0"
            and item["source"].get("rustc_commit_hash") == rust_commit,
            "Rust toolchain evidence provenance does not match the runtime component")

require(set(override_components) == set(catalog_identities),
        f"override component closure mismatch: missing={sorted(set(catalog_identities) - set(override_components))!r} "
        f"extra={sorted(set(override_components) - set(catalog_identities))!r}")
require(len(override_components) == 20, "component inventory must use exactly 20 vetted overrides")
for identity, component in override_components.items():
    entry_index, entry = catalog_identities[identity]
    integrity_field = "registryChecksum" if identity[0] == "cargo" else "registryIntegrity"
    version_evidence = entry["versionEvidence"]
    normalized_version_evidence = {
        "upstream_path": version_evidence["upstreamPath"],
        "sha256": version_evidence["sha256"],
        "expected_name": version_evidence["expectedName"],
        "expected_version": version_evidence["expectedVersion"],
    }
    if "localPath" in version_evidence:
        normalized_version_evidence["local_path"] = version_evidence["localPath"]
    expected_override = {
        "catalog_path": catalog_relative,
        "catalog_sha256": catalog_digest,
        "catalog_schema_version": 1,
        "catalog_entry_index": entry_index,
        "source_audit_date": audit_date,
        "resolution": entry["resolution"],
        "repository": entry["repository"],
        "revision": entry["revision"],
        "version_tag": entry.get("versionTag"),
        "registry_git_head": entry.get("registryGitHead"),
        "locked_integrity_field": integrity_field,
        "locked_integrity": entry[integrity_field],
        "version_evidence": normalized_version_evidence,
        "upstream_paths": sorted(
            f"{spec['upstreamRepository']}@{spec['upstreamRevision']}:{spec['upstreamPath']}"
            for spec in entry["files"]
        ),
    }
    require(component.get("license_override") == expected_override,
            f"override provenance differs from the source catalog for {identity!r}")
    require(component.get("declared_license") == entry["declaredLicense"]
            and component.get("locked_integrity") == entry[integrity_field],
            f"override license/integrity differs from the exact component for {identity!r}")
    component_evidence = component["license_files"]
    require(len(component_evidence) == len(entry["files"])
            and all(item.get("kind") == "vetted-source-license-or-notice" for item in component_evidence),
            f"override evidence count/type differs from the catalog for {identity!r}")
    evidence_by_local_path = {}
    for item in component_evidence:
        source = require_object(item.get("source"), f"override evidence source for {identity!r}")
        local_path = source.get("local_path")
        require(isinstance(local_path, str) and local_path not in evidence_by_local_path,
                f"override evidence has a missing/duplicate local_path for {identity!r}")
        evidence_by_local_path[local_path] = item
    require(set(evidence_by_local_path) == {spec["localPath"] for spec in entry["files"]},
            f"override evidence file closure differs from the catalog for {identity!r}")
    for spec in entry["files"]:
        item = evidence_by_local_path[spec["localPath"]]
        expected_source = {
            "kind": "vetted-source-override",
            "catalog_path": catalog_relative,
            "local_path": spec["localPath"],
            "upstream_repository": spec["upstreamRepository"],
            "upstream_revision": spec["upstreamRevision"],
            "upstream_path": spec["upstreamPath"],
        }
        for catalog_key, output_key in (
            ("upstreamTag", "upstream_tag"),
            ("upstreamLineStart", "upstream_line_start"),
            ("upstreamLineEnd", "upstream_line_end"),
            ("extraction", "extraction"),
        ):
            if catalog_key in spec:
                expected_source[output_key] = spec[catalog_key]
        require(item.get("evidence_kind") == spec["kind"]
                and item.get("sha256") == spec["sha256"]
                and item.get("bytes") == spec["bytes"]
                and item.get("source") == expected_source,
                f"override evidence provenance differs from the catalog for {identity!r}/{spec['localPath']}")

license_directory = root / "notices" / "licenses"
require(license_directory.is_dir() and not license_directory.is_symlink(),
        "notices/licenses must be a regular directory")
actual_license_files = set()
for path in license_directory.rglob("*"):
    metadata = path.lstat()
    require(not stat.S_ISLNK(metadata.st_mode),
            f"notices/licenses contains a symlink: {path.relative_to(root / 'notices').as_posix()}")
    if stat.S_ISDIR(metadata.st_mode):
        continue
    require(stat.S_ISREG(metadata.st_mode),
            f"notices/licenses contains a non-regular entry: {path.relative_to(root / 'notices').as_posix()}")
    actual_license_files.add(path.relative_to(root / "notices").as_posix())
require(actual_license_files == evidence_paths,
        f"component evidence closure mismatch: unclaimed={sorted(actual_license_files - evidence_paths)!r} "
        f"absent={sorted(evidence_paths - actual_license_files)!r}")

regular_file(license_manifest_path, "notices/LICENSES.sha256")
license_manifest_text = license_manifest_path.read_text(encoding="utf-8")
require(license_manifest_text.endswith("\n") and "\r" not in license_manifest_text,
        "notices/LICENSES.sha256 must be LF-terminated")
license_checksums = {}
for line in license_manifest_text.splitlines():
    digest, separator, relative = line.partition("  ")
    require(separator == "  " and hex64.fullmatch(digest) is not None,
            f"invalid LICENSES.sha256 row: {line!r}")
    canonical_relative(relative, "LICENSES.sha256 path", "licenses/")
    require(relative not in license_checksums, f"duplicate LICENSES.sha256 path: {relative}")
    license_checksums[relative] = digest
require(set(license_checksums) == actual_license_files,
        f"LICENSES.sha256 closure mismatch: unlisted={sorted(actual_license_files - set(license_checksums))!r} "
        f"absent={sorted(set(license_checksums) - actual_license_files)!r}")
require(license_checksums == evidence_hashes,
        "LICENSES.sha256 checksums do not exactly match the component evidence inventory")
for relative, expected_digest in sorted(license_checksums.items()):
    target = file_beneath(root / "notices", relative, f"LICENSES.sha256 target {relative}")
    require(hash_file(target) == expected_digest, f"LICENSES.sha256 digest mismatch: {relative}")

bom = require_object(load_json(bom_path, "notices/bom.cdx.json"), "notices/bom.cdx.json")
require(set(bom) == {"bomFormat", "specVersion", "version", "metadata", "components"},
        "CycloneDX document has an invalid top-level shape")
require(bom.get("bomFormat") == "CycloneDX" and bom.get("specVersion") == "1.6"
        and require_integer(bom.get("version"), "CycloneDX version", 1) == 1,
        "CycloneDX document must be version 1 of the 1.6 schema")
metadata = require_object(bom.get("metadata"), "CycloneDX metadata")
require(set(metadata) == {"component", "tools", "properties"},
        "CycloneDX metadata has an invalid shape")
application = require_object(metadata.get("component"), "CycloneDX metadata.component")
expected_application_ref = (
    "pkg:github/"
    + "/".join(js_encode(part.lower()) for part in expected_repository.split("/"))
    + f"@{js_encode(expected_revision)}"
)
require(application == {
    "type": "application",
    "name": "NodeControll",
    "version": workspace_version,
    "bom-ref": expected_application_ref,
    "externalReferences": [{
        "type": "vcs",
        "url": f"https://github.com/{expected_repository}/tree/{expected_revision}",
    }],
}, "CycloneDX application revision/repository does not match the checked-out commit")
require(metadata.get("tools") == {"components": [{
    "type": "application", "name": "nodecontroll-license-collector", "version": "2",
}]}, "CycloneDX metadata does not identify the schema-v2 collector")
require(normalized_properties(metadata.get("properties"), "CycloneDX metadata.properties") == Counter({
    ("nodecontroll:source-revision", expected_revision): 1,
    ("nodecontroll:override-catalog-sha256", catalog_digest): 1,
}), "CycloneDX metadata revision/catalog provenance is incomplete")

bom_components = require_array(bom.get("components"), "CycloneDX components")
require(len(bom_components) == len(components),
        "CycloneDX and DEPENDENCIES.json component counts differ")
require([item.get("purl") for item in bom_components] == [item.get("purl") for item in components],
        "CycloneDX component order/identity differs from DEPENDENCIES.json")
for index, (component, bom_component_raw) in enumerate(zip(components, bom_components, strict=True)):
    label = f"CycloneDX components[{index}]"
    bom_component = require_object(bom_component_raw, label)
    expected_bom_keys = {
        "type", "name", "version", "purl", "bom-ref", "licenses", "externalReferences", "properties",
    }
    if component["ecosystem"] == "cargo":
        expected_bom_keys.add("hashes")
    require(set(bom_component) == expected_bom_keys, f"{label} has an invalid shape")
    expected_external_references = [] if component["repository"] is None else [{
        "type": "vcs", "url": component["repository"],
    }]
    require(bom_component.get("type") == "library"
            and bom_component.get("name") == component["name"]
            and bom_component.get("version") == component["version"]
            and bom_component.get("purl") == component["purl"]
            and bom_component.get("bom-ref") == component["purl"]
            and bom_component.get("licenses") == [{
                "license": {"name": component["declared_license"]},
            }]
            and bom_component.get("externalReferences") == expected_external_references,
            f"{label} identity/license/repository differs from DEPENDENCIES.json")
    if component["ecosystem"] == "cargo":
        require(bom_component.get("hashes") == [{
            "alg": "SHA-256", "content": component["locked_integrity"],
        }], f"{label} lacks its exact Cargo.lock SHA-256 hash")
    expected_properties = [
        ("nodecontroll:ecosystem", component["ecosystem"]),
        ("nodecontroll:source", component["source"] or "unknown"),
        (f"nodecontroll:locked-integrity:{component['locked_integrity_kind'] or 'unknown'}",
         component["locked_integrity"]),
    ]
    override = component.get("license_override")
    if override is not None:
        expected_properties.extend([
            ("nodecontroll:license-override-catalog", override["catalog_path"]),
            ("nodecontroll:license-override-catalog-sha256", override["catalog_sha256"]),
            ("nodecontroll:license-override-revision", override["revision"]),
        ])
        expected_properties.extend(
            (f"nodecontroll:license-override-upstream-path:{offset}", value)
            for offset, value in enumerate(override["upstream_paths"], start=1)
        )
    toolchain_provenance = component.get("toolchain_provenance")
    if toolchain_provenance is not None:
        expected_properties.extend(
            (f"nodecontroll:rust-toolchain:{name}", str(value))
            for name, value in toolchain_provenance.items() if value is not None
        )
    for evidence_index, evidence in enumerate(component["license_files"], start=1):
        evidence_source = evidence["source"]
        source_path = (evidence_source.get("sysroot_relative_path")
                       or evidence_source.get("upstream_path")
                       or evidence_source.get("package_relative_path")
                       or "unknown")
        expected_properties.append((
            f"nodecontroll:license-evidence:{evidence_index}",
            f"{evidence['kind']}|{source_path}|sha256:{evidence['sha256']}|bytes:{evidence['bytes']}",
        ))
    require(normalized_properties(bom_component.get("properties"), f"{label}.properties")
            == Counter(expected_properties),
            f"{label} lock/evidence/runtime/override provenance is incomplete")

print(
    f"verified {len(listed)} packaged files, {len(components)} locked components, "
    f"{len(evidence_paths)} evidence files, 20/20 overrides, and Rust 1.98 runtime provenance"
)
PY
}

verify_elf_binaries() {
  local package_root="$1"
  docker run --rm --network none --read-only \
    -v "${package_root}:/compiled:ro" "${RUST_IMAGE_ID}" bash -Eeuo pipefail -c '
    export LC_ALL=C
    for binary in /compiled/bin/nodecontroll-master /compiled/bin/nodecontroll-agent; do
      file "${binary}" | grep -Eq "ELF 64-bit LSB (pie )?executable, x86-64"
      readelf --file-header "${binary}" | grep -Eq "Machine:[[:space:]]+Advanced Micro Devices X86-64"
      readelf --program-headers "${binary}" | grep -F "/lib64/ld-linux-x86-64.so.2"
      ldd_output="$(ldd "${binary}")"
      if grep -Fq "not found" <<< "${ldd_output}"; then
        echo "${binary} has an unresolved shared library" >&2
        exit 2
      fi
      if ! dynamic_output="$(readelf --dynamic "${binary}")"; then
        echo "failed to read dynamic entries for ${binary}" >&2
        exit 2
      fi
      if ! needed_libraries="$(
        sed -n "s/.*Shared library: \[\([^]]*\)\].*/\1/p" <<< "${dynamic_output}"
      )"; then
        echo "failed to parse DT_NEEDED entries for ${binary}" >&2
        exit 2
      fi
      if [[ -z "${needed_libraries}" ]]; then
        echo "${binary} has no auditable DT_NEEDED entries" >&2
        exit 2
      fi
      while IFS= read -r library; do
        case "${library}" in
          ld-linux-x86-64.so.2 | libc.so.6 | libdl.so.2 | libgcc_s.so.1 | libm.so.6 | libpthread.so.0 | librt.so.1) ;;
          *) echo "unexpected shared library for ${binary}: ${library}" >&2; exit 2 ;;
        esac
      done <<< "${needed_libraries}"
      maximum_glibc="$(readelf --version-info "${binary}" | sed -n "s/.*Name: GLIBC_\([0-9.]*\).*/\1/p" | sort -Vu | tail -n1)"
      if [[ -n "${maximum_glibc}" && "$(printf "%s\n%s\n" "${maximum_glibc}" 2.36 | sort -V | tail -n1)" != "2.36" ]]; then
        echo "${binary} requires GLIBC_${maximum_glibc}, newer than 2.36" >&2
        exit 2
      fi
    done
  '
}

verify_agent_artifact_smoke() {
  local output
  output="$(docker run --rm \
    --network none \
    --read-only \
    -v "${ACTIONS_ARTIFACT_ROOT}:/compiled:ro" \
    -w /compiled \
    "${RUST_IMAGE_ID}" \
    /compiled/bin/nodecontroll-agent)"
  python3 - "${output}" "${APPLICATION_VERSION}" <<'PY'
import json
import sys

try:
    payload = json.loads(sys.argv[1])
except json.JSONDecodeError as error:
    raise SystemExit(f"Agent artifact did not emit JSON: {error}") from error
expected = {
    "product": "NodeControll Agent",
    "protocol_status": "skeleton-not-enrolled",
    "version": sys.argv[2],
}
if payload != expected:
    raise SystemExit(f"Agent artifact identity differs: expected {expected!r}, found {payload!r}")
print(f"verified Agent artifact identity at application version {sys.argv[2]}")
PY
}

verify_build_metadata() {
  local metadata_file="$1"
  local expected_run_id="$2"
  local expected_revision="$3"
  python3 - "${metadata_file}" "${expected_run_id}" "${expected_revision}" <<'PY'
import pathlib
import sys

metadata_file, run_id, revision = sys.argv[1:]
path = pathlib.Path(metadata_file)
metadata = path.lstat()
if not path.is_file() or path.is_symlink():
    raise SystemExit("BUILD-METADATA must be a regular non-symlink file")
raw = path.read_bytes()
if not raw.endswith(b"\n") or b"\r" in raw or b"\0" in raw:
    raise SystemExit("BUILD-METADATA must be LF-terminated and contain no CR or NUL")
try:
    lines = raw.decode("utf-8").splitlines()
except UnicodeDecodeError as error:
    raise SystemExit(f"BUILD-METADATA must be UTF-8: {error}") from error
expected = [
    f"run_id={run_id}",
    "run_attempt=1",
    f"commit={revision}",
    "target=x86_64-unknown-linux-gnu",
    "glibc=2.36",
    "builder=rust@sha256:e536cf316987faedfe8ae120f83b70c7df0068fdb4fc9efcce55c71a625001d5",
    f"source=https://github.com/FengYuchen1314/NodeControll/tree/{revision}",
]
if lines != expected:
    raise SystemExit(f"BUILD-METADATA does not match the exact seven-line contract: {lines!r}")
print("verified exact seven-line BUILD-METADATA contract")
PY
}

verify_github_provenance() {
  local run_id="$1"
  local artifact_id="$2"
  local revision="$3"
  local artifact_sha="$4"
  local artifact_size="$5"
  local output_directory="$6"
  python3 - "${run_id}" "${artifact_id}" "${revision}" "${artifact_sha}" "${artifact_size}" "${output_directory}" <<'PY'
import json
import pathlib
import sys
import urllib.request

repository = "FengYuchen1314/NodeControll"
run_id, artifact_id, revision, artifact_sha, artifact_size, output_directory = sys.argv[1:]

def fetch(url: str) -> dict:
    request = urllib.request.Request(url, headers={
        "Accept": "application/vnd.github+json",
        "User-Agent": "NodeControll-VPS-Verifier/1",
        "X-GitHub-Api-Version": "2022-11-28",
    })
    with urllib.request.urlopen(request, timeout=20) as response:
        return json.load(response)

run = fetch(f"https://api.github.com/repos/{repository}/actions/runs/{run_id}")
artifact = fetch(f"https://api.github.com/repos/{repository}/actions/artifacts/{artifact_id}")
expected = {
    "repository": repository,
    "event": "push",
    "head_branch": "main",
    "head_sha": revision,
    "run_attempt": 1,
    "status": "completed",
    "conclusion": "success",
    "path": ".github/workflows/build.yml",
}
actual = {
    "repository": run.get("repository", {}).get("full_name"),
    "event": run.get("event"),
    "head_branch": run.get("head_branch"),
    "head_sha": run.get("head_sha"),
    "run_attempt": run.get("run_attempt"),
    "status": run.get("status"),
    "conclusion": run.get("conclusion"),
    "path": run.get("path"),
}
if actual != expected:
    raise SystemExit(f"untrusted GitHub Actions run metadata: expected={expected!r} actual={actual!r}")
if int(run.get("id", 0)) != int(run_id):
    raise SystemExit("GitHub run ID mismatch")
if int(artifact.get("id", 0)) != int(artifact_id):
    raise SystemExit("GitHub artifact ID mismatch")
if int(artifact.get("size_in_bytes", -1)) != int(artifact_size):
    raise SystemExit("GitHub artifact size does not match the immutable VPS snapshot")
if artifact.get("expired") is not False:
    raise SystemExit("GitHub artifact is expired")
if artifact.get("name") != "nodecontroll-linux-x86_64-glibc2.36.tar.gz":
    raise SystemExit(f"unexpected GitHub artifact name: {artifact.get('name')!r}")
workflow_run = artifact.get("workflow_run") or {}
if int(workflow_run.get("id", 0)) != int(run_id):
    raise SystemExit("artifact does not belong to the requested workflow run")
if workflow_run.get("head_sha") != revision or workflow_run.get("head_branch") != "main":
    raise SystemExit("artifact workflow source does not match the checked-out main commit")
api_digest = artifact.get("digest")
if api_digest != f"sha256:{artifact_sha}":
    raise SystemExit(f"GitHub artifact digest mismatch: API={api_digest!r} file=sha256:{artifact_sha}")

destination = pathlib.Path(output_directory)
destination.mkdir(parents=True, exist_ok=True)
(destination / "github-run.json").write_text(json.dumps(run, indent=2, sort_keys=True) + "\n", encoding="utf-8")
(destination / "github-artifact.json").write_text(json.dumps(artifact, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(f"verified trusted main push run {run_id}, attempt {run.get('run_attempt')}, artifact {artifact_id}")
PY
}

assert_image "${RUST_IMAGE}" "${RUST_IMAGE_ID}"
assert_image "${NODE_IMAGE}" "${NODE_IMAGE_ID}"
assert_image "${POSTGRES_IMAGE}" "${POSTGRES_IMAGE_ID}"
assert_repo_digest "${POSTGRES_IMAGE}" "${POSTGRES_IMAGE}"
assert_image "${PLAYWRIGHT_IMAGE}" "${PLAYWRIGHT_IMAGE_ID}"
assert_repo_digest "${PLAYWRIGHT_IMAGE}" "${PLAYWRIGHT_IMAGE}"

readonly SOURCE_REVISION="$(git --no-replace-objects rev-parse --verify HEAD)"
if [[ ! "${SOURCE_REVISION}" =~ ^[0-9a-f]{40}$ ]]; then
  echo "HEAD is not a full Git commit ID: ${SOURCE_REVISION}" >&2
  exit 2
fi
if [[ "${WORKTREE}" != "/opt/nodecontroll/checkouts/${SOURCE_REVISION}" ]]; then
  echo "checkout path must be /opt/nodecontroll/checkouts/${SOURCE_REVISION}" >&2
  exit 2
fi
if [[ "${ACTIONS_ARTIFACT}" != "/opt/nodecontroll/artifacts/github-actions/${SOURCE_REVISION}/nodecontroll-linux-x86_64-glibc2.36.tar.gz" ]]; then
  echo "artifact path must be commit-scoped under /opt/nodecontroll/artifacts/github-actions/${SOURCE_REVISION}" >&2
  exit 2
fi
readonly STARTED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

git_directory="$(git --no-replace-objects rev-parse --absolute-git-dir)"
git_directory="$(readlink -f "${git_directory}")"
readonly GIT_DIRECTORY="${git_directory}"
if [[ "${GIT_DIRECTORY}" != "${WORKTREE}/.git" || ! -d "${GIT_DIRECTORY}" || -L "${GIT_DIRECTORY}" ]]; then
  echo "formal verification requires a standalone clone with ${WORKTREE}/.git" >&2
  exit 2
fi
verify_checkout_provenance
readonly SOURCE_VERIFIER_BLOB="$(git --no-replace-objects rev-parse --verify "${SOURCE_REVISION}:tools/verify_tracked_source.py")"
if [[ ! "${SOURCE_VERIFIER_BLOB}" =~ ^[0-9a-f]{40}$ ]]; then
  echo "could not pin the source verifier Git blob" >&2
  exit 2
fi
install_source_verifier
readonly SOURCE_VERIFIER_SHA256="$(sha256sum "${SOURCE_VERIFIER}" | cut -d' ' -f1)"

readonly LICENSE_COLLECTOR_BLOB="$(git --no-replace-objects rev-parse --verify "${SOURCE_REVISION}:tools/collect_third_party_licenses.mjs")"
readonly LICENSE_COLLECTOR_SHA256="$(git --no-replace-objects cat-file blob "${LICENSE_COLLECTOR_BLOB}" | sha256sum | cut -d' ' -f1)"
if [[ ! "${LICENSE_COLLECTOR_BLOB}" =~ ^[0-9a-f]{40}$ || ! "${LICENSE_COLLECTOR_SHA256}" =~ ^[0-9a-f]{64}$ ]]; then
  echo "could not pin the license collector source identity" >&2
  exit 2
fi

readonly AUTH_E2E_GATE_BLOB="$(git --no-replace-objects rev-parse --verify "${SOURCE_REVISION}:apps/web/e2e/auth-rotation.mjs")"
readonly AUTH_E2E_GATE_SHA256="$(git --no-replace-objects cat-file blob "${AUTH_E2E_GATE_BLOB}" | sha256sum | cut -d' ' -f1)"
readonly AUTH_E2E_VALIDATOR_BLOB="$(git --no-replace-objects rev-parse --verify "${SOURCE_REVISION}:tools/verify_auth_e2e_evidence.mjs")"
readonly AUTH_E2E_VALIDATOR_SHA256="$(git --no-replace-objects cat-file blob "${AUTH_E2E_VALIDATOR_BLOB}" | sha256sum | cut -d' ' -f1)"
if [[ ! "${AUTH_E2E_GATE_BLOB}" =~ ^[0-9a-f]{40}$ \
  || ! "${AUTH_E2E_GATE_SHA256}" =~ ^[0-9a-f]{64}$ \
  || ! "${AUTH_E2E_VALIDATOR_BLOB}" =~ ^[0-9a-f]{40}$ \
  || ! "${AUTH_E2E_VALIDATOR_SHA256}" =~ ^[0-9a-f]{64}$ ]]; then
  echo "could not pin the auth E2E gate and evidence validator source identity" >&2
  exit 2
fi

application_version="$(python3 - <<'PY'
import pathlib
import re
import tomllib

version = tomllib.loads(pathlib.Path("Cargo.toml").read_text(encoding="utf-8"))["workspace"]["package"]["version"]
semver = re.compile(
    r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)"
    r"(?:-(?:[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?"
    r"(?:\+(?:[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?"
)
if not isinstance(version, str) or semver.fullmatch(version) is None:
    raise SystemExit("workspace.package.version must be an exact SemVer")
print(version)
PY
)"
readonly APPLICATION_VERSION="${application_version}"
source_date_epoch="$(git --no-replace-objects show -s --format=%ct "${SOURCE_REVISION}")"
if [[ ! "${source_date_epoch}" =~ ^[1-9][0-9]*$ ]]; then
  echo "source commit timestamp is invalid" >&2
  exit 2
fi
readonly SOURCE_DATE_EPOCH="${source_date_epoch}"

verify_source_revision_integrity

initial_status="$(git status --porcelain=v1 --untracked-files=all)"
readonly INITIAL_STATUS="${initial_status}"
if [[ -n "${INITIAL_STATUS}" ]]; then
  echo "VPS verification requires a clean source checkout" >&2
  printf '%s\n' "${INITIAL_STATUS}" >&2
  exit 2
fi

initial_ignored="$(git status --porcelain=v1 --ignored=matching --untracked-files=all | sed -n 's/^!! //p')"
readonly INITIAL_IGNORED="${initial_ignored}"
if [[ -n "${INITIAL_IGNORED}" ]]; then
  echo "VPS verification requires a fresh checkout with no ignored files" >&2
  printf '%s\n' "${INITIAL_IGNORED}" >&2
  exit 2
fi

readonly CHECKOUT_CLAIM="${GIT_DIRECTORY}/nodecontroll-verifier.claim"
if ! (
  set -o noclobber
  printf '%s\n' \
    "run_id=${RUN_ID}" \
    "source_revision=${SOURCE_REVISION}" \
    "started_at=${STARTED_AT}" \
    > "${CHECKOUT_CLAIM}"
) 2>/dev/null; then
  echo "formal verification refuses a checkout that has already been claimed; create a new clone at ${WORKTREE}" >&2
  exit 2
fi

if [[ -e "${ACTIONS_ARTIFACT_SNAPSHOT}" || -L "${ACTIONS_ARTIFACT_SNAPSHOT}" ]]; then
  echo "refusing to overwrite Actions artifact snapshot: ${ACTIONS_ARTIFACT_SNAPSHOT}" >&2
  exit 2
fi
install -m 0444 -- "${ACTIONS_ARTIFACT}" "${ACTIONS_ARTIFACT_SNAPSHOT}"

readonly CARGO_LOCK_SHA="$(sha256sum Cargo.lock | cut -d' ' -f1)"
readonly PNPM_LOCK_SHA="$(sha256sum pnpm-lock.yaml | cut -d' ' -f1)"
readonly ACTIONS_ARTIFACT_SHA="$(sha256sum "${ACTIONS_ARTIFACT_SNAPSHOT}" | cut -d' ' -f1)"
readonly ACTIONS_ARTIFACT_SIZE="$(stat -c '%s' "${ACTIONS_ARTIFACT_SNAPSHOT}")"

python3 - "${RUN_DIR}/manifest.json" "${RUN_ID}" "${STARTED_AT}" "${SOURCE_REVISION}" \
  "${RUST_IMAGE_ID}" "${NODE_IMAGE_ID}" "${POSTGRES_IMAGE_ID}" "${ACTIONS_ARTIFACT}" "${ACTIONS_ARTIFACT_SNAPSHOT}" \
  "${ACTIONS_ARTIFACT_SHA}" "${CARGO_LOCK_SHA}" "${PNPM_LOCK_SHA}" "${GITHUB_RUN_ID}" "${GITHUB_ARTIFACT_ID}" \
  "${LICENSE_COLLECTOR_BLOB}" "${LICENSE_COLLECTOR_SHA256}" "${CYCLONEDX_CLI_VERSION}" "${CYCLONEDX_CLI_SHA256}" \
  "${SOURCE_VERIFIER_BLOB}" "${SOURCE_VERIFIER_SHA256}" "${APPLICATION_VERSION}" "${SOURCE_DATE_EPOCH}" \
  "${PLAYWRIGHT_IMAGE}" "${PLAYWRIGHT_IMAGE_ID}" "${PLAYWRIGHT_PACKAGE_VERSION}" "${PLAYWRIGHT_NODE_VERSION}" \
  "${PLAYWRIGHT_BROWSER_VERSION}" "${PLAYWRIGHT_EXECUTABLE_SHA256}" \
  "${AUTH_E2E_GATE_BLOB}" "${AUTH_E2E_GATE_SHA256}" \
  "${AUTH_E2E_VALIDATOR_BLOB}" "${AUTH_E2E_VALIDATOR_SHA256}" <<'PY'
import json
import pathlib
import platform
import sys

(manifest, run_id, started_at, revision, rust_builder, node_builder, postgres_image,
 artifact_source, artifact_snapshot, artifact_sha, cargo_lock, pnpm_lock,
 github_run_id, github_artifact_id, collector_blob, collector_sha,
 cyclonedx_version, cyclonedx_sha, verifier_blob, verifier_sha,
 application_version, source_date_epoch, playwright_image, playwright_image_id,
 playwright_version, playwright_node_version, browser_version, browser_executable_sha,
 auth_e2e_gate_blob, auth_e2e_gate_sha, auth_e2e_validator_blob,
 auth_e2e_validator_sha) = sys.argv[1:]
payload = {
    "schema_version": 3,
    "run_id": run_id,
    "status": "running",
    "started_at": started_at,
    "source_revision": revision,
    "source_date_epoch": int(source_date_epoch),
    "application_version": application_version,
    "source_checkout_clean": True,
    "source_checkout_one_time_claimed": True,
    "source_ignored_inputs_absent": True,
    "source_checkout_origin": "https://github.com/FengYuchen1314/NodeControll.git",
    "source_checkout_branch": "main",
    "source_checkout_full_clone": True,
    "github": {
        "repository": "FengYuchen1314/NodeControll",
        "run_id": int(github_run_id),
        "run_attempt": 1,
        "artifact_id": int(github_artifact_id),
        "artifact_sha256": artifact_sha,
    },
    "verification_images": {
        "rust": rust_builder,
        "node": node_builder,
        "postgres": postgres_image,
        "playwright": {
            "browser_executable_sha256": browser_executable_sha,
            "browser_version": browser_version,
            "image": playwright_image,
            "image_id": playwright_image_id,
            "node_runtime_image_id": node_builder,
            "node_version": playwright_node_version,
            "package_version": playwright_version,
        },
    },
    "artifact_source_file": artifact_source,
    "artifact_file": artifact_snapshot,
    "cargo_lock_sha256": cargo_lock,
    "pnpm_lock_sha256": pnpm_lock,
    "license_collector": {
        "git_blob": collector_blob,
        "sha256": collector_sha,
    },
    "source_verifier": {
        "git_blob": verifier_blob,
        "sha256": verifier_sha,
    },
    "validators": {
        "auth_e2e_evidence": {
            "git_blob": auth_e2e_validator_blob,
            "sha256": auth_e2e_validator_sha,
        },
        "cyclonedx_cli": {
            "version": cyclonedx_version,
            "sha256": cyclonedx_sha,
        },
    },
    "auth_e2e_gate": {
        "git_blob": auth_e2e_gate_blob,
        "sha256": auth_e2e_gate_sha,
    },
    "host": {
        "system": platform.system(),
        "release": platform.release(),
        "machine": platform.machine(),
    },
    "commands_log": "commands.tsv",
}
pathlib.Path(manifest).write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY

run_stage rust-builder-toolchain verify_rust_builder_toolchain
run_stage github-provenance verify_github_provenance \
  "${GITHUB_RUN_ID}" "${GITHUB_ARTIFACT_ID}" "${SOURCE_REVISION}" "${ACTIONS_ARTIFACT_SHA}" \
  "${ACTIONS_ARTIFACT_SIZE}" "${RUN_DIR}/provenance"
run_stage source-integrity-initial verify_source_revision_integrity
run_stage isolated-build-directories prepare_isolated_build_directories
run_stage node-workspace-export export_node_workspace
run_stage cyclonedx-cli-download download_pinned_cyclonedx_cli
run_stage actions-snapshot-integrity-before-archive verify_artifact_snapshot_integrity
run_stage actions-archive-members validate_archive_members "${ACTIONS_ARTIFACT_SNAPSHOT}"
run_stage actions-snapshot-integrity-after-archive verify_artifact_snapshot_integrity
run_stage actions-archive-extract tar --extract --gzip --file "${ACTIONS_ARTIFACT_SNAPSHOT}" \
  --directory "${RUN_DIR}/compiled" --no-same-owner
run_stage actions-snapshot-integrity-after-extract verify_artifact_snapshot_integrity

readonly ACTIONS_ARTIFACT_ROOT="${RUN_DIR}/compiled"
if [[ -n "$(find "${ACTIONS_ARTIFACT_ROOT}" -type l -print -quit)" ]]; then
  echo "compiled artifact must not contain symlinks" >&2
  exit 2
fi
for required_artifact in \
  LICENSE \
  THIRD_PARTY_NOTICES.md \
  BUILD-METADATA \
  CONTENTS.sha256 \
  bin/nodecontroll-master \
  bin/nodecontroll-agent \
  openapi/nodecontroll-v1.json \
  web/index.html \
  notices/README.md \
  notices/DEPENDENCIES.json \
  notices/bom.cdx.json \
  notices/LICENSES.sha256; do
  if [[ ! -f "${ACTIONS_ARTIFACT_ROOT}/${required_artifact}" || -L "${ACTIONS_ARTIFACT_ROOT}/${required_artifact}" ]]; then
    echo "compiled artifact is missing regular non-symlink file ${required_artifact}" >&2
    exit 2
  fi
done
if [[ ! -x "${ACTIONS_ARTIFACT_ROOT}/bin/nodecontroll-master" || ! -x "${ACTIONS_ARTIFACT_ROOT}/bin/nodecontroll-agent" ]]; then
  echo "compiled Master and Agent must be executable" >&2
  exit 2
fi
if [[ -z "$(find "${ACTIONS_ARTIFACT_ROOT}/notices/licenses" -type f -print -quit)" ]]; then
  echo "compiled artifact has no dependency license-evidence files" >&2
  exit 2
fi
run_stage actions-content-check verify_package_contents "${ACTIONS_ARTIFACT_ROOT}" "${SOURCE_REVISION}"
run_stage actions-cyclonedx-schema "${CYCLONEDX_CLI_FILE}" validate \
  --input-file "${ACTIONS_ARTIFACT_ROOT}/notices/bom.cdx.json" \
  --input-format json \
  --input-version v1_6 \
  --fail-on-errors
run_stage actions-elf-check verify_elf_binaries "${ACTIONS_ARTIFACT_ROOT}"
run_stage actions-build-metadata verify_build_metadata \
  "${ACTIONS_ARTIFACT_ROOT}/BUILD-METADATA" "${GITHUB_RUN_ID}" "${SOURCE_REVISION}"

run_stage test-network docker network create --internal "${TEST_NETWORK}"
run_stage postgres-start docker run --detach \
  --name "${POSTGRES_CONTAINER}" \
  --network "${TEST_NETWORK}" \
  --network-alias postgres \
  -e POSTGRES_USER=nodecontroll_test \
  -e POSTGRES_PASSWORD=nodecontroll_test \
  -e POSTGRES_DB=nodecontroll_test \
  "${POSTGRES_IMAGE}"
run_stage postgres-ready bash -c \
  'for attempt in $(seq 1 30); do docker exec "$1" pg_isready -U nodecontroll_test -d nodecontroll_test && exit 0; sleep 1; done; exit 1' \
  _ "${POSTGRES_CONTAINER}"

readonly CARGO_FETCH_RUN=(
  docker run --rm
  --network bridge
  --read-only
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777
  -e HOME=/tmp/cargo-fetch-home
  -e CARGO_HOME=/private-cargo-home
  -e RUSTUP_HOME=/usr/local/rustup
  -e RUSTUP_TOOLCHAIN=1.98.0
  -e CARGO_INCREMENTAL=0
  -e SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH}"
  -e TZ=UTC
  -e LANG=C.UTF-8
  -e LC_ALL=C.UTF-8
  -v "${WORKTREE}:/workspace:ro"
  -v "${PRIVATE_CARGO_HOME}:/private-cargo-home"
  -w /workspace
  "${RUST_IMAGE_ID}"
)

readonly RUST_RUN=(
  docker run --rm
  --network "${TEST_NETWORK}"
  --read-only
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777
  --tmpfs /cargo-home:rw,nosuid,nodev,mode=0755
  -e HOME=/tmp/rust-home
  -e CARGO_HOME=/cargo-home
  -e RUSTUP_HOME=/usr/local/rustup
  -e RUSTUP_TOOLCHAIN=1.98.0
  -e CARGO_TARGET_DIR=/cargo-target
  -e CARGO_NET_OFFLINE=true
  -e CARGO_INCREMENTAL=0
  -e SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH}"
  -e TZ=UTC
  -e LANG=C.UTF-8
  -e LC_ALL=C.UTF-8
  -e NODECONTROLL_TEST_POSTGRES_URL=postgres://nodecontroll_test:nodecontroll_test@postgres:5432/nodecontroll_test
  -v "${WORKTREE}:/workspace:ro"
  -v "${PRIVATE_CARGO_HOME}/registry:/cargo-home/registry:ro"
  -v "${PRIVATE_CARGO_HOME}/git:/cargo-home/git:ro"
  -v "${PRIVATE_CARGO_TEST_TARGET}:/cargo-target"
  -w /workspace
  "${RUST_IMAGE_ID}"
)

readonly NODE_INSTALL_RUN=(
  docker run --rm
  --network bridge
  --read-only
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777
  -e CI=true
  -e HOME=/tmp/node-home
  -e XDG_CACHE_HOME=/tmp/cache
  -e npm_config_enable_global_virtual_store=false
  -e SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH}"
  -e TZ=UTC
  -e LANG=C.UTF-8
  -v "${NODE_WORKSPACE}:/workspace"
  -v "${NODE_PNPM_STORE}:/pnpm/store"
  -w /workspace
  "${NODE_IMAGE_ID}"
)

readonly NODE_RUN=(
  docker run --rm
  --network none
  --read-only
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777
  --tmpfs /pnpm:rw,nosuid,nodev,mode=0755
  -e CI=true
  -e HOME=/tmp/node-home
  -e XDG_CACHE_HOME=/tmp/cache
  -e npm_config_enable_global_virtual_store=false
  -e SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH}"
  -e TZ=UTC
  -e LANG=C.UTF-8
  -v "${NODE_WORKSPACE}:/workspace"
  -w /workspace
  "${NODE_IMAGE_ID}"
)

run_stage cargo-fetch-private "${CARGO_FETCH_RUN[@]}" cargo fetch --locked
run_stage cargo-fetch-source-integrity verify_source_revision_integrity
run_stage cargo-input-closure write_directory_closure \
  "${PRIVATE_CARGO_HOME}" "${CARGO_INPUT_CLOSURE}" "private Cargo home"
run_stage pnpm-install-private "${NODE_INSTALL_RUN[@]}" pnpm \
  --config.enable-global-virtual-store=false install \
  --frozen-lockfile --ignore-scripts --package-import-method=copy --store-dir /pnpm/store
run_stage pnpm-install-source-integrity verify_node_workspace_integrity
run_stage pnpm-input-closure write_directory_closure \
  "${NODE_PNPM_STORE}" "${PNPM_INPUT_CLOSURE}" "private pnpm store"
run_stage private-input-manifest record_private_input_closures

run_stage cargo-fmt "${RUST_RUN[@]}" cargo fmt --all -- --check
run_stage cargo-test "${RUST_RUN[@]}" cargo test --locked --workspace --all-targets
run_stage cargo-clippy "${RUST_RUN[@]}" cargo clippy --locked --workspace --all-targets -- -D warnings
run_stage cargo-post-test-source-integrity verify_source_revision_integrity
run_stage cargo-input-closure-after-tests verify_directory_closure \
  "${PRIVATE_CARGO_HOME}" "${CARGO_INPUT_CLOSURE}" "private Cargo home"
run_stage actions-openapi-match cmp \
  "${ACTIONS_ARTIFACT_ROOT}/openapi/nodecontroll-v1.json" \
  openapi/nodecontroll-v1.json
run_stage master-config-check docker run --rm \
  --network none \
  --read-only \
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
  -e NODECONTROLL__DATABASE__URL=sqlite://must-not-be-created.db?mode=rwc \
  -v "${ACTIONS_ARTIFACT_ROOT}:/compiled:ro" \
  -w /compiled \
  "${RUST_IMAGE_ID}" \
  /compiled/bin/nodecontroll-master --check-config
run_stage agent-artifact-smoke verify_agent_artifact_smoke
run_stage openapi-validate "${NODE_RUN[@]}" node tools/validate_openapi.mjs
run_stage docs-validate "${NODE_RUN[@]}" node tools/validate_design_docs.mjs
run_stage upstream-publication-boundary "${NODE_RUN[@]}" node tools/sanitize_upstream_generated.mjs
run_stage node-tool-source-integrity verify_node_workspace_integrity
run_stage web-artifact-check docker run --rm \
  --network none \
  --read-only \
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
  -v "${NODE_WORKSPACE}:/workspace:ro" \
  -v "${ACTIONS_ARTIFACT_ROOT}:/compiled:ro" \
  -w /workspace \
  "${NODE_IMAGE_ID}" \
  node tools/verify_web_artifact.mjs /compiled/web
run_stage node-toolchain-versions "${NODE_RUN[@]}" sh -euc \
  'test "$(node --version)" = v24.19.0
   test "$(pnpm --version)" = 11.24.0
   test "$(pnpm config get enable-global-virtual-store)" = false'
run_stage license-source-integrity verify_source_revision_integrity
run_stage npm-installed-license-inventory docker run --rm \
  --network none \
  --read-only \
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
  -v "${NODE_WORKSPACE}:/workspace:ro" \
  -v "${ACTIONS_ARTIFACT_ROOT}/notices/DEPENDENCIES.json:/inventory/DEPENDENCIES.json:ro" \
  -w /workspace \
  "${NODE_IMAGE_ID}" \
  node tools/verify_installed_npm_license_inventory.mjs /inventory/DEPENDENCIES.json
run_stage web-generate "${NODE_RUN[@]}" pnpm \
  --config.enable-global-virtual-store=false --filter @nodecontroll/web generate:api
run_stage generated-contract-drift verify_node_workspace_integrity
run_stage web-typecheck "${NODE_RUN[@]}" pnpm \
  --config.enable-global-virtual-store=false --filter @nodecontroll/web typecheck
run_stage web-lint "${NODE_RUN[@]}" pnpm \
  --config.enable-global-virtual-store=false --filter @nodecontroll/web lint
run_stage web-test "${NODE_RUN[@]}" pnpm \
  --config.enable-global-virtual-store=false --filter @nodecontroll/web test
run_stage web-post-test-source-integrity verify_node_workspace_integrity
run_stage pnpm-input-closure-after-tests verify_directory_closure \
  "${NODE_PNPM_STORE}" "${PNPM_INPUT_CLOSURE}" "private pnpm store"

run_stage secret-key-create bash -c \
  'umask 077; head -c 32 /dev/urandom | od -An -tx1 | tr -d " \n" > "$1"; test "$(wc -c < "$1")" -eq 64' \
  _ "${TEST_SECRET_FILE}"
run_stage setup-token-create bash -c \
  'umask 077; head -c 32 /dev/urandom | od -An -tx1 | tr -d " \n" > "$1"; test "$(wc -c < "$1")" -eq 64' \
  _ "${TEST_SETUP_TOKEN_FILE}"

run_stage master-start docker run --detach \
  --name "${SMOKE_CONTAINER}" \
  --network host \
  -e NODECONTROLL__HTTP__LISTEN=127.0.0.1:18080 \
  -e NODECONTROLL__HTTP__PUBLIC_ORIGIN=http://127.0.0.1:18080 \
  -e NODECONTROLL__DATABASE__URL=sqlite::memory: \
  -e NODECONTROLL__SECRETS__ROOT_KEY_FILE=/run/secrets/nodecontroll-root-key \
  -e NODECONTROLL__SECRETS__SETUP_TOKEN_FILE=/run/secrets/nodecontroll-setup-token \
  -v "${TEST_SECRET_FILE}:/run/secrets/nodecontroll-root-key:ro" \
  -v "${TEST_SETUP_TOKEN_FILE}:/run/secrets/nodecontroll-setup-token:ro" \
  -v "${ACTIONS_ARTIFACT_ROOT}:/compiled:ro" \
  -w /compiled \
  "${RUST_IMAGE_ID}" \
  /compiled/bin/nodecontroll-master

run_stage runtime-openapi-match docker run --rm \
  --network host \
  --read-only \
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
  -v "${NODE_WORKSPACE}:/workspace:ro" \
  -v "${ACTIONS_ARTIFACT_ROOT}:/compiled:ro" \
  -w /workspace \
  "${NODE_IMAGE_ID}" \
  node tools/compare_runtime_openapi.mjs \
  http://127.0.0.1:18080/api-docs/openapi.json \
  /compiled/openapi/nodecontroll-v1.json
run_stage master-smoke docker run --rm \
  --network host \
  --read-only \
  --tmpfs /tmp:rw,nosuid,nodev,mode=1777 \
  -e NODECONTROLL_TEST_SETUP_TOKEN_FILE=/run/secrets/nodecontroll-setup-token \
  -v "${TEST_SETUP_TOKEN_FILE}:/run/secrets/nodecontroll-setup-token:ro" \
  -v "${NODE_WORKSPACE}:/workspace:ro" \
  -w /workspace \
  "${NODE_IMAGE_ID}" \
  node tools/smoke_master.mjs http://127.0.0.1:18080
run_stage master-stop-capture stop_and_capture_master "${RUN_DIR}/logs/master-runtime.log"
run_stage runtime-log-secret-scan scan_runtime_secrets_or_remove "${RUN_DIR}/logs/master-runtime.log"

run_stage e2e-node-runtime-extract extract_pinned_node_runtime
run_stage auth-e2e-fixtures prepare_auth_e2e_fixtures
run_stage auth-e2e-master-start start_auth_e2e_master
run_stage auth-e2e-browser-start start_auth_e2e_browser
run_stage auth-e2e-behavior-ready wait_auth_e2e_behavior_ready
run_stage auth-e2e-freeze freeze_auth_e2e_artifacts
run_stage auth-e2e-scan-release publish_auth_e2e_scan_ready
run_stage auth-e2e-browser-finish wait_auth_e2e_browser_finish
run_stage auth-e2e-evidence-validate verify_auth_e2e_evidence
run_stage auth-e2e-evidence-fixture-scan \
  scan_auth_e2e_fixture_secrets_or_remove "${AUTH_E2E_EVIDENCE}"
run_stage auth-e2e-seal seal_auth_e2e_artifacts
run_stage auth-e2e-artifact-closure write_directory_closure \
  "${AUTH_E2E_ROOT}" "${AUTH_E2E_CLOSURE}" "browser auth E2E evidence"
run_stage auth-e2e-manifest-record record_auth_e2e_manifest

run_stage node-final-source-integrity verify_node_workspace_integrity
run_stage cargo-input-closure-final verify_directory_closure \
  "${PRIVATE_CARGO_HOME}" "${CARGO_INPUT_CLOSURE}" "private Cargo home"
run_stage pnpm-input-closure-final verify_directory_closure \
  "${NODE_PNPM_STORE}" "${PNPM_INPUT_CLOSURE}" "private pnpm store"
run_stage auth-e2e-artifact-closure-final verify_directory_closure \
  "${AUTH_E2E_ROOT}" "${AUTH_E2E_CLOSURE}" "browser auth E2E evidence"
run_stage source-clean-after-tests verify_source_clean_after_tests

run_stage verifier-cleanup cleanup
run_stage verifier-cleanup-closure verify_cleanup_complete

readonly CHECKSUMS_TEMP="${RUN_DIR}/checksums.txt.temporary"
if [[ -e "${RUN_DIR}/checksums.txt" || -L "${RUN_DIR}/checksums.txt" \
  || -e "${CHECKSUMS_TEMP}" || -L "${CHECKSUMS_TEMP}" ]]; then
  echo "refusing to overwrite formal checksum evidence" >&2
  exit 2
fi
umask 077
{
  sha256sum Cargo.lock pnpm-lock.yaml openapi/nodecontroll-v1.json
  sha256sum \
    "${SOURCE_VERIFIER}" \
    "${CARGO_INPUT_CLOSURE}" \
    "${PNPM_INPUT_CLOSURE}" \
    "${AUTH_E2E_CLOSURE}"
  printf '%s  %s\n' "${ACTIONS_ARTIFACT_SHA}" "github-actions/nodecontroll-linux-x86_64-glibc2.36.tar.gz"
  (
    cd "${ACTIONS_ARTIFACT_ROOT}"
    sha256sum \
      BUILD-METADATA \
      CONTENTS.sha256 \
      bin/nodecontroll-master \
      bin/nodecontroll-agent \
      openapi/nodecontroll-v1.json \
      notices/DEPENDENCIES.json \
      notices/LICENSES.sha256 \
      notices/bom.cdx.json
    find web -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum
  )
  (
    cd "${RUN_DIR}"
    find browser -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum
  )
} > "${CHECKSUMS_TEMP}"
python3 - "${CHECKSUMS_TEMP}" <<'PY'
import os
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
with path.open("rb+") as stream:
    os.fsync(stream.fileno())
PY
ln -- "${CHECKSUMS_TEMP}" "${RUN_DIR}/checksums.txt"
rm -f -- "${CHECKSUMS_TEMP}"
python3 - "${RUN_DIR}" <<'PY'
import os
import pathlib
import sys

directory = os.open(pathlib.Path(sys.argv[1]), os.O_RDONLY | os.O_DIRECTORY)
try:
    os.fsync(directory)
finally:
    os.close(directory)
PY
record_checksums_manifest

readonly FINISHED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
finalize_manifest completed
printf '%s\n' "${FINISHED_AT}" > "${RUN_DIR}/COMPLETED_AT"
trap - EXIT INT TERM
printf '%s\n' "${RUN_ID}"
