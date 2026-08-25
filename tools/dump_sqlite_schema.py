#!/usr/bin/env python3
"""Dump a SQLite schema, columns, indexes and foreign keys as stable JSON."""

from __future__ import annotations

import argparse
import json
import sqlite3
from pathlib import Path


def rows(cursor: sqlite3.Cursor) -> list[dict[str, object]]:
    names = [item[0] for item in cursor.description or []]
    return [dict(zip(names, row, strict=True)) for row in cursor.fetchall()]


def quote_identifier(value: str) -> str:
    return '"' + value.replace('"', '""') + '"'


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("database", type=Path)
    args = parser.parse_args()
    connection = sqlite3.connect(f"file:{args.database.as_posix()}?mode=ro", uri=True)
    connection.row_factory = sqlite3.Row
    objects = rows(connection.execute(
        "SELECT type, name, tbl_name FROM sqlite_master "
        "WHERE name NOT LIKE 'sqlite_%' ORDER BY type, name"
    ))
    tables: list[dict[str, object]] = []
    for item in objects:
        if item["type"] != "table":
            continue
        name = str(item["name"])
        identifier = quote_identifier(name)
        indexes = rows(connection.execute(f"PRAGMA index_list({identifier})"))
        for index in indexes:
            index_name = quote_identifier(str(index["name"]))
            index["columns"] = rows(connection.execute(f"PRAGMA index_info({index_name})"))
        tables.append({
            **item,
            "columns": rows(connection.execute(f"PRAGMA table_info({identifier})")),
            "foreign_keys": rows(connection.execute(f"PRAGMA foreign_key_list({identifier})")),
            "indexes": indexes,
        })
    payload = {
        "database": args.database.name,
        "sqlite_version": sqlite3.sqlite_version,
        "tables": tables,
        "other_objects": [item for item in objects if item["type"] != "table"],
    }
    print(json.dumps(payload, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
