#!/usr/bin/env python3
"""Extract top-level net/http ServeMux registrations from 妙妙屋 main.go."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


def balanced_statement(lines: list[str], start: int) -> tuple[str, int]:
    text = ""
    depth = 0
    in_string = False
    escaped = False
    seen_open = False
    index = start
    while index < len(lines):
        part = lines[index]
        text += part + "\n"
        for char in part:
            if escaped:
                escaped = False
                continue
            if char == "\\" and in_string:
                escaped = True
                continue
            if char == '"':
                in_string = not in_string
                continue
            if in_string:
                continue
            if char == "(":
                depth += 1
                seen_open = True
            elif char == ")":
                depth -= 1
        if seen_open and depth == 0:
            return text.strip(), index
        index += 1
    return text.strip(), index


def target_name(statement: str) -> str:
    constructors = re.findall(r"handler\.(New[A-Za-z0-9_]+)", statement)
    if constructors:
        return ", ".join(dict.fromkeys(constructors))
    known = [
        name for name in (
            "securityLogHandler", "trafficHandler", "speedTestHandler", "speedTesterWS",
            "tempSubAccessHandler", "shortLinkHandler", "web.Handler", "http.HandlerFunc",
        ) if name in statement
    ]
    if known:
        return ", ".join(known)
    if "func(" in statement:
        return "inline HandlerFunc"
    return "expression"


def access_level(path: str, statement: str) -> str:
    if "auth.RequireAdmin" in statement:
        return "管理员 UI 会话"
    if "auth.RequireToken" in statement:
        return "已登录 UI 会话"
    if path.startswith("/api/clash/subscribe") or path.startswith("/api/proxy-provider/"):
        return "端点内订阅鉴权"
    if path.startswith("/api/speedtest/tester/ws"):
        return "端点内 tester 鉴权"
    if path == "/":
        return "混合：SPA/短链/临时订阅"
    return "公开或 Handler 内校验"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    parser.add_argument("markdown", type=Path)
    parser.add_argument("json_output", type=Path)
    args = parser.parse_args()
    lines = args.source.read_text(encoding="utf-8").splitlines()
    routes = []
    index = 0
    while index < len(lines):
        if "mux.Handle(" not in lines[index] and "mux.HandleFunc(" not in lines[index]:
            index += 1
            continue
        statement, end = balanced_statement(lines, index)
        match = re.search(r"mux\.Handle(?:Func)?\(\s*\"([^\"]+)\"", statement)
        if match:
            path_value = match.group(1)
            routes.append({
                "path": path_value,
                "access": access_level(path_value, statement),
                "handler": target_name(statement),
                "line": index + 1,
                "registration": "HandleFunc" if "mux.HandleFunc(" in statement else "Handle",
            })
        index = end + 1
    args.json_output.parent.mkdir(parents=True, exist_ok=True)
    args.json_output.write_text(json.dumps(routes, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    markdown = [
        "# 妙妙屋顶层 HTTP 路由注册表", "",
        "> 从 `cmd/server/main.go` 的 `http.ServeMux` 注册语句自动提取。前缀路由的子路径和方法由对应 Handler 再分派；详细业务方法见人工 HTTP API 文档。", "",
        f"共 {len(routes)} 条顶层注册。", "",
        "| 路径/前缀 | 访问边界 | Handler/构造器 | 注册方式 | 源码行 |", "|---|---|---|---|---:|",
    ]
    for route in routes:
        markdown.append(
            f"| `{route['path']}` | {route['access']} | `{route['handler']}` | "
            f"`{route['registration']}` | {route['line']} |"
        )
    args.markdown.parent.mkdir(parents=True, exist_ok=True)
    args.markdown.write_text("\n".join(markdown) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
