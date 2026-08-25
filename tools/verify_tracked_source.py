#!/usr/bin/env python3
"""Verify a worktree byte-for-byte against a pinned Git commit tree.

The check intentionally ignores the Git index so assume-unchanged and
skip-worktree flags cannot hide mutations. Extra untracked/ignored files are
outside this function's contract and must be checked separately where needed.
"""

from __future__ import annotations

import os
import pathlib
import re
import stat
import subprocess
import sys


def fail(message: str) -> "NoReturn":
    raise SystemExit(message)


def git(repository: pathlib.Path, *arguments: str) -> bytes:
    environment = os.environ.copy()
    environment["GIT_NO_REPLACE_OBJECTS"] = "1"
    try:
        return subprocess.run(
            ["git", "--no-replace-objects", "-C", str(repository), *arguments],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=environment,
        ).stdout
    except subprocess.CalledProcessError as error:
        detail = error.stderr.decode("utf-8", errors="replace").strip()
        fail(f"git {' '.join(arguments)} failed: {detail}")


def canonical_root(value: str, label: str) -> pathlib.Path:
    raw = pathlib.Path(value)
    try:
        metadata = raw.lstat()
    except OSError as error:
        fail(f"{label} is missing or unreadable: {error}")
    if not stat.S_ISDIR(metadata.st_mode) or stat.S_ISLNK(metadata.st_mode):
        fail(f"{label} must be a real, non-symlink directory")
    return raw.resolve(strict=True)


def read_candidate(candidate: pathlib.Path, relative: pathlib.PurePosixPath, mode: str) -> bytes:
    target = candidate.joinpath(*relative.parts)
    current = candidate
    for parent_part in relative.parts[:-1]:
        current /= parent_part
        try:
            parent_metadata = current.lstat()
        except OSError as error:
            fail(f"tracked parent {relative.as_posix()!r} is missing or unreadable: {error}")
        if not stat.S_ISDIR(parent_metadata.st_mode) or stat.S_ISLNK(parent_metadata.st_mode):
            fail(f"tracked parent is not a real directory: {current.relative_to(candidate).as_posix()}")

    try:
        metadata = target.lstat()
    except OSError as error:
        fail(f"tracked path {relative.as_posix()!r} is missing or unreadable: {error}")

    if mode == "120000":
        if not stat.S_ISLNK(metadata.st_mode):
            fail(f"tracked path must be a symlink: {relative.as_posix()}")
        return os.fsencode(os.readlink(target))

    if mode not in {"100644", "100755"}:
        fail(f"unsupported tracked Git mode {mode} for {relative.as_posix()}")
    if not stat.S_ISREG(metadata.st_mode) or stat.S_ISLNK(metadata.st_mode):
        fail(f"tracked path must be a regular non-symlink file: {relative.as_posix()}")
    executable = bool(metadata.st_mode & 0o111)
    if executable != (mode == "100755"):
        fail(f"tracked executable mode changed: {relative.as_posix()}")
    try:
        return target.read_bytes()
    except OSError as error:
        fail(f"cannot read tracked path {relative.as_posix()!r}: {error}")


def main() -> None:
    if len(sys.argv) != 4:
        fail("usage: verify_tracked_source.py <40-hex-revision> <git-repository> <candidate-worktree>")
    revision, repository_argument, candidate_argument = sys.argv[1:]
    if re.fullmatch(r"[0-9a-f]{40}", revision) is None:
        fail(f"revision is not a lowercase 40-hex commit: {revision!r}")

    repository = canonical_root(repository_argument, "Git repository worktree")
    candidate = canonical_root(candidate_argument, "candidate worktree")
    object_format = git(repository, "rev-parse", "--show-object-format").decode("ascii").strip()
    if object_format != "sha1":
        fail(f"only a SHA-1 Git object repository is currently supported, found {object_format!r}")
    head = git(repository, "rev-parse", "--verify", "HEAD").decode("ascii").strip()
    if head != revision:
        fail(f"repository HEAD changed: expected {revision}, found {head}")
    object_type = git(repository, "cat-file", "-t", revision).decode("ascii").strip()
    if object_type != "commit":
        fail(f"pinned revision is not a commit: {object_type!r}")

    tree = git(repository, "ls-tree", "-r", "-z", "--full-tree", revision)
    entries = tree.split(b"\0")
    if entries and entries[-1] == b"":
        entries.pop()
    if not entries:
        fail("pinned commit tree is empty")

    seen: set[str] = set()
    verified = 0
    for entry in entries:
        header, separator, raw_path = entry.partition(b"\t")
        if separator != b"\t":
            fail(f"cannot parse Git tree entry: {entry!r}")
        header_parts = header.split(b" ")
        if len(header_parts) != 3:
            fail(f"cannot parse Git tree header: {header!r}")
        mode_bytes, object_type_bytes, object_id_bytes = header_parts
        try:
            mode = mode_bytes.decode("ascii")
            entry_type = object_type_bytes.decode("ascii")
            object_id = object_id_bytes.decode("ascii")
            path_text = raw_path.decode("utf-8")
        except UnicodeDecodeError as error:
            fail(f"Git tree entry is not canonical ASCII/UTF-8: {error}")
        if entry_type != "blob" or re.fullmatch(r"[0-9a-f]{40}", object_id) is None:
            fail(f"unsupported non-blob Git tree entry: {entry!r}")
        if (
            not path_text
            or "\\" in path_text
            or any(ord(character) < 0x20 or ord(character) == 0x7F for character in path_text)
        ):
            fail(f"tracked path is not canonical UTF-8 POSIX text: {path_text!r}")
        relative = pathlib.PurePosixPath(path_text)
        if (
            relative.is_absolute()
            or relative.as_posix() != path_text
            or any(part in {"", ".", ".."} for part in relative.parts)
            or relative.parts[0] == ".git"
        ):
            fail(f"tracked path is unsafe or non-canonical: {path_text!r}")
        if path_text in seen:
            fail(f"pinned Git tree repeats tracked path: {path_text}")
        seen.add(path_text)

        payload = read_candidate(candidate, relative, mode)
        expected_payload = git(repository, "cat-file", "blob", object_id)
        if payload != expected_payload:
            fail(f"tracked blob bytes changed: {path_text}; expected Git blob {object_id}")
        verified += 1

    print(f"verified {verified} tracked blobs and modes against commit {revision}")


if __name__ == "__main__":
    main()
