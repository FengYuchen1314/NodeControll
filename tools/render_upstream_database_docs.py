#!/usr/bin/env python3
"""Render the captured 妙妙屋 SQLite schema into a human-readable catalog."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


TABLES = {
    "custom_rule_applications": ("订阅生成", "记录某条自定义规则已向某订阅文件应用的内容与哈希，用于幂等/追踪。"),
    "custom_rules": ("订阅生成", "自定义 DNS、rules、rule-providers 等规则片段及追加/替换模式。"),
    "external_subscriptions": ("订阅来源", "用户导入的外部机场订阅、流量头、过期时间和自动更新策略。"),
    "ip_bans": ("安全", "暴力探测和人工封禁状态；支持临时、永久、释放和重启恢复。"),
    "nodes": ("节点", "规范化节点主表，同时保留原始 URI、解析 JSON、Clash JSON、标签和链式代理关系。"),
    "operation_logs": ("审计", "管理员变更请求的操作者、方法、路径、状态码和来源 IP。"),
    "override_scripts": ("订阅生成", "按用户保存的 JavaScript pre-save/post-fetch 覆写脚本及顺序。"),
    "probe_configs": ("探针", "单例探针数据源类型和地址。"),
    "probe_servers": ("探针", "探针面板内选中的服务器、流量口径、月流量和展示顺序。"),
    "proxy_provider_configs": ("订阅来源", "把外部订阅转成 Clash proxy-provider 时的过滤、健康检查、覆写和处理位置。"),
    "rule_versions": ("订阅生成", "规则 YAML 文件的不可变版本历史。"),
    "security_events": ("安全", "登录失败、短链探测、封禁/解封等追加式安全事件流。"),
    "sessions": ("身份", "管理 UI 登录会话；启动时回填到内存 TokenStore。"),
    "speed_test_results": ("测速", "本地 Mihomo 或远程 tester 的节点下载速度、延迟、出口 IP 和状态。"),
    "speed_testers": ("测速", "远程测速器身份、令牌哈希和最后在线时间。"),
    "subscribe_files": ("订阅产品", "对外发布的订阅文件元数据、短码、模板、选中节点/标签/规则/脚本和流量设置。"),
    "subscription_links": ("订阅产品", "较旧的订阅定义/规则入口模型，保存名称、规则文件、客户端按钮和短链。"),
    "system_config": ("系统", "固定 id=1 的全局产品、安全、通知和输出行为配置。"),
    "system_settings": ("系统", "键值型系统状态；用于初始化标记等不适合强类型单例表的设置。"),
    "task_runs": ("可观测性", "后台任务名称、开始时间、耗时、状态和节流后的详情。"),
    "templates": ("订阅生成", "V2/远程模板定义、规则来源、代理开关和 include-all 行为。"),
    "traffic_records": ("流量", "按日保存聚合流量上限、已用和剩余快照，供 30 日趋势图。"),
    "user_settings": ("身份", "每用户的同步、模板、缓存、探针绑定、节点顺序、调试和短链偏好。"),
    "user_subscriptions": ("授权", "用户与可访问订阅文件的多对多授权表。"),
    "user_tokens": ("订阅授权", "长期订阅 token、系统短码与自定义用户短码；不同于 UI 会话。"),
    "users": ("身份", "用户账户、密码哈希、角色、启用状态、资料、备注和 TOTP 恢复信息。"),
}


SPECIAL_COLUMNS = {
    ("nodes", "raw_url"): "导入时的原始代理 URI。",
    ("nodes", "parsed_config"): "解析器得到的协议 JSON。",
    ("nodes", "clash_config"): "用于订阅生成的 Clash/Mihomo 代理 JSON。",
    ("nodes", "original_server"): "改写服务器地址前的原始地址，用于恢复。",
    ("nodes", "chain_proxy_node_id"): "单节点链式代理引用；未声明数据库外键。",
    ("nodes", "relay_group_node_ids"): "中转/relay 组成员 ID 的 JSON 数组。",
    ("subscribe_files", "selected_tags"): "生成订阅时选择的节点标签 JSON。",
    ("subscribe_files", "selected_node_ids"): "显式选中的节点 ID JSON。",
    ("subscribe_files", "selected_custom_rule_ids"): "绑定的自定义规则 ID JSON。",
    ("subscribe_files", "selected_override_script_ids"): "绑定的覆写脚本 ID JSON。",
    ("subscribe_files", "stats_server_ids"): "用于该订阅流量统计的探针服务器 ID 编码。",
    ("external_subscriptions", "traffic_mode"): "流量计算口径：上传、下载或两者。",
    ("proxy_provider_configs", "process_mode"): "provider 在客户端侧还是妙妙屋服务端预处理。",
    ("system_config", "silent_mode"): "静默模式总开关：正常情况下伪装 404。",
    ("system_config", "silent_mode_timeout"): "订阅访问/启动后临时恢复 UI 的分钟数。",
    ("system_config", "enable_sub_info_nodes"): "是否把到期和剩余流量合成为提示节点。",
    ("system_config", "enable_sub_traffic_header"): "是否输出 Subscription-Userinfo 等流量响应头。",
    ("system_config", "block_unknown_subscription_ua"): "是否拒绝未知订阅客户端 UA。",
    ("user_settings", "node_order"): "用户自定义节点顺序 JSON。",
    ("user_settings", "sync_scope"): "外部订阅同步到已保存节点还是更大范围。",
    ("user_settings", "template_version"): "用户选择的模板系统版本。",
    ("users", "recovery_codes"): "一次性恢复码哈希的 JSON 数组。",
}


def describe_column(table: str, name: str) -> str:
    if (table, name) in SPECIAL_COLUMNS:
        return SPECIAL_COLUMNS[(table, name)]
    common = {
        "id": "行主键。", "username": "用户主键/所有者。", "name": "用户可见名称。",
        "description": "可选说明。", "created_at": "创建时间。", "updated_at": "最后更新时间。",
        "enabled": "启用开关，SQLite INTEGER 布尔值。", "type": "业务类型判别值。",
        "url": "源地址或对外地址。", "filename": "磁盘文件名。", "content": "原始规则、模板或脚本文本。",
        "sort_order": "显式展示/执行顺序。", "token": "令牌明文（按表的用途解释）。",
        "token_hash": "不可逆令牌哈希。", "role": "用户角色。", "is_active": "账户启用状态。",
        "expire_at": "产品配置的到期时间。", "expires_at": "失效时间。", "last_seen": "最后心跳/在线时间。",
        "last_sync_at": "最近成功同步时间。", "address": "远端服务地址。", "status": "执行状态。",
        "error": "失败错误文本。", "actor": "操作发起者。", "ip": "来源或被封禁 IP。",
    }
    if name in common:
        return common[name]
    if name.endswith("_id"):
        return f"关联的 {name.removesuffix('_id')} 标识。"
    if name.endswith("_ids"):
        return "多个关联标识的序列化表示。"
    if name.endswith("_at"):
        return "业务事件时间。"
    if name.startswith("enable_") or name.startswith("notify_") or name.startswith("health_check_"):
        return "功能开关或该功能的参数。"
    if name.startswith("custom_"):
        return "用户自定义值。"
    if name.endswith("_code"):
        return "短码或验证码相关值。"
    if name.endswith("_count"):
        return "计数值。"
    if name.endswith("_bytes"):
        return "字节数。"
    return "对应模型的持久化字段；精确读写行为见 storage 函数索引。"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("schema", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    payload = json.loads(args.schema.read_text(encoding="utf-8"))
    tables = payload["tables"]
    lines = [
        "# 妙妙屋 SQLite 数据库解剖", "",
        "> 基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`。本页不是只抄静态 DDL：在 VPS 隔离实例中执行完整启动迁移后，再从生成数据库只读导出。机器可读证据见 `generated/database-schema.json`。", "",
        "## 数据库运行方式", "",
        "- 驱动为 `modernc.org/sqlite`，默认路径 `data/traffic.db`。", 
        "- 连接池上限强制为 1，因此所有 Repository 查询串行复用单连接；实现简单但会限制高并发写入。",
        "- 启动设置 WAL、`busy_timeout=5000`、`synchronous=NORMAL`、64 MiB journal limit。",
        "- 备份前执行 `wal_checkpoint(TRUNCATE)`；后台任务优先 TRUNCATE，繁忙时回退 PASSIVE。",
        "- 源码未执行 `PRAGMA foreign_keys=ON`。虽然 8 条外键写进 schema，SQLite 默认连接不会强制它们；实际级联主要不能依赖数据库保证，这是重构迁移时必须修正的完整性缺口。",
        "- 迁移没有版本表，而是在单个 `migrate()` 中反复 `CREATE TABLE IF NOT EXISTS`、读取 `PRAGMA table_info`、逐列 `ALTER TABLE`，并对少数历史结构重建表。可重复运行，但难以审计迁移版本和失败恢复点。", "",
        "## 领域总览", "",
        "| 表 | 领域 | 列 | 索引 | 外键 | 作用 |", "|---|---|---:|---:|---:|---|",
    ]
    for table in tables:
        domain, role = TABLES.get(table["name"], ("其他", "未分类上游表。"))
        lines.append(f"| `{table['name']}` | {domain} | {len(table['columns'])} | {len(table['indexes'])} | {len(table['foreign_keys'])} | {role} |")
    lines += ["", "## 关系与完整性", ""]
    foreign_keys = []
    for table in tables:
        for key in table["foreign_keys"]:
            foreign_keys.append((table["name"], key))
    for table_name, key in foreign_keys:
        lines.append(f"- `{table_name}.{key['from']}` → `{key['table']}.{key['to']}`，删除策略 `{key['on_delete']}`。")
    lines += [
        "- `nodes.chain_proxy_node_id` 和 `nodes.relay_group_node_ids` 是逻辑关系，没有数据库外键；删除节点后由 Go 代码遍历并修剪 relay 成员。",
        "- 多个 `subscribe_files.selected_*` 字段把 ID 数组编码进 TEXT，数据库无法保证引用存在，也无法高效反向查询。",
        "- `subscription_links` 与 `subscribe_files` 是并存的两套订阅概念；前者偏规则/按钮入口，后者是当前文件、模板、节点和授权聚合根。重构时需要显式统一或定义边界。", "",
        "## 完整字段目录", "",
    ]
    for table in tables:
        domain, role = TABLES.get(table["name"], ("其他", "未分类上游表。"))
        lines += [f"### `{table['name']}`", "", f"领域：{domain}。{role}", "", "| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |", "|---|---|---|---|---:|---|"]
        for column in table["columns"]:
            default = column["dflt_value"] if column["dflt_value"] is not None else "—"
            lines.append(
                f"| `{column['name']}` | `{column['type'] or 'ANY'}` | {'是' if column['notnull'] else '否'} | "
                f"`{str(default).replace('|', '\\|')}` | {column['pk']} | {describe_column(table['name'], column['name'])} |"
            )
        if table["indexes"]:
            lines += ["", "索引："]
            for index in table["indexes"]:
                columns = ", ".join(item["name"] for item in index["columns"])
                lines.append(f"- `{index['name']}`：`({columns})`，{'唯一' if index['unique'] else '非唯一'}。")
        if table["foreign_keys"]:
            lines += ["", "外键声明（注意运行时未显式开启 SQLite 外键强制）："]
            for key in table["foreign_keys"]:
                lines.append(f"- `{key['from']}` → `{key['table']}.{key['to']}`，ON DELETE `{key['on_delete']}`。")
        lines.append("")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()

