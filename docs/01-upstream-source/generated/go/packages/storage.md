# Go 包 `storage`

SQLite 建表、迁移、Repository 方法和持久化数据模型。

## `internal/storage/logs_tables.go`

依赖：`context`、`errors`、`fmt`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–23 | type | `SecurityEvent` | 定义 'SecurityEvent' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 26–35 | type | `IPBan` | 定义 'IPBan' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 38–45 | type | `TaskRun` | 定义 'TaskRun' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 47–55 | type | `OperationLog` | 定义 'OperationLog' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 57–112 | function | `(*TrafficRepository).migrateLogTables` | *TrafficRepository 的方法，执行与 'migrate log tables' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'fmt.Errorf'、'r.db.Exec' |
| 114–117 | function | `(*TrafficRepository).InsertOperationLog` | *TrafficRepository 的方法，写入与 'insert operation log' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext'、'time.Now' |
| 119–137 | function | `(*TrafficRepository).ListOperationLogs` | *TrafficRepository 的方法，列举与 'list operation logs' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 3；goroutine 0；调用 'append'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 139–145 | function | `(*TrafficRepository).DeleteOldOperationLogs` | *TrafficRepository 的方法，删除与 'delete old operation logs' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext'、'result.RowsAffected' |
| 150–159 | function | `(*TrafficRepository).InsertSecurityEvent` | *TrafficRepository 的方法，写入与 'insert security event' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext'、'time.Now' |
| 162–194 | function | `(*TrafficRepository).ListSecurityEvents` | *TrafficRepository 的方法，列举与 'list security events' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 3；goroutine 0；调用 'append'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 197–204 | function | `(*TrafficRepository).DeleteOldSecurityEvents` | *TrafficRepository 的方法，删除与 'delete old security events' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext'、'res.RowsAffected' |
| 209–221 | function | `(*TrafficRepository).UpsertIPBan` | *TrafficRepository 的方法，执行与 'upsert ip ban' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'boolToInt'、'r.db.ExecContext' |
| 224–229 | function | `(*TrafficRepository).ReleaseIPBan` | *TrafficRepository 的方法，执行与 'release ip ban' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext'、'time.Now' |
| 232–243 | function | `(*TrafficRepository).ListActiveIPBans` | *TrafficRepository 的方法，列举与 'list active ip bans' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.QueryContext'、'rows.Close'、'scanIPBans'、'time.Now' |
| 247–249 | function | `(*TrafficRepository).ListRestorableIPBans` | *TrafficRepository 的方法，列举与 'list restorable ip bans' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.ListActiveIPBans' |
| 251–267 | function | `scanIPBans` | 执行与 'scan ip bans' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'append'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 272–280 | function | `(*TrafficRepository).InsertTaskRun` | *TrafficRepository 的方法，写入与 'insert task run' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext' |
| 283–294 | function | `(*TrafficRepository).StartTaskRun` | *TrafficRepository 的方法，启动与 'start task run' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'errors.New'、'r.db.ExecContext'、'res.LastInsertId'、'time.Now' |
| 297–305 | function | `(*TrafficRepository).FinishTaskRun` | *TrafficRepository 的方法，执行与 'finish task run' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext' |
| 308–340 | function | `(*TrafficRepository).ListTaskRuns` | *TrafficRepository 的方法，列举与 'list task runs' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 3；goroutine 0；调用 'append'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 343–350 | function | `(*TrafficRepository).DeleteOldTaskRuns` | *TrafficRepository 的方法，删除与 'delete old task runs' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext'、'res.RowsAffected' |

## `internal/storage/nodes.go`

依赖：`context`、`database/sql`、`encoding/json`、`errors`、`fmt`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–25 | function | `scanNodeTags` | 执行与 'scan node tags' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'json.Unmarshal'、'len' |
| 28–43 | function | `serializeNodeTags` | 执行与 'serialize node tags' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 3；goroutine 0；调用 'json.Marshal'、'len'、'string' |
| 45–49 | function | `scanRelayGroupNodeIDs` | 执行与 'scan relay group node i ds' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'json.Unmarshal' |
| 51–57 | function | `serializeRelayGroupNodeIDs` | 执行与 'serialize relay group node i ds' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'json.Marshal'、'len'、'string' |
| 60–67 | function | `(Node).HasAnyTag` | Node 的方法，判断是否具有与 'has any tag' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0 |
| 70–96 | function | `(*TrafficRepository).CheckNodeNameExists` | *TrafficRepository 的方法，检查与 'check node name exists' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Scan'、'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 99–134 | function | `(*TrafficRepository).ListNodes` | *TrafficRepository 的方法，列举与 'list nodes' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 6；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan'、'scanNodeTags'、'scanRelayGroupNodeIDs'、'strings.TrimSpace' |
| 137–166 | function | `(*TrafficRepository).GetNode` | *TrafficRepository 的方法，查询或读取与 'get node' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'row.Scan'、'scanNodeTags'、'scanRelayGroupNodeIDs'、'strings.TrimSpace' |
| 169–225 | function | `(*TrafficRepository).CreateNode` | *TrafficRepository 的方法，创建与 'create node' 对应的业务或基础设施操作。 | 分支 11；循环 0；返回 8；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'len'、'r.GetNode'、'r.db.ExecContext'、'res.LastInsertId'、'serializeNodeTags'、'serializeRelayGroupNodeIDs'、'strings.ToLower'、'strings.TrimSpace' |
| 228–291 | function | `(*TrafficRepository).UpdateNode` | *TrafficRepository 的方法，更新与 'update node' 对应的业务或基础设施操作。 | 分支 13；循环 0；返回 10；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'len'、'r.GetNode'、'r.db.ExecContext'、'res.RowsAffected'、'serializeNodeTags'、'serializeRelayGroupNodeIDs'、'strings.ToLower'、'strings.TrimSpace' |
| 294–366 | function | `(*TrafficRepository).DeleteNode` | *TrafficRepository 的方法，删除与 'delete node' 对应的业务或基础设施操作。 | 分支 13；循环 0；返回 10；goroutine 0；调用 'Scan'、'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'r.db.QueryRowContext'、'r.pruneRelayGroupMember'、'res.RowsAffected'、'strings.TrimSpace' |
| 371–414 | function | `(*TrafficRepository).pruneRelayGroupMember` | *TrafficRepository 的方法，执行与 'prune relay group member' 对应的业务或基础设施操作。 | 分支 6；循环 3；返回 1；goroutine 0；调用 'append'、'json.Unmarshal'、'len'、'r.db.ExecContext'、'r.db.QueryContext'、'rows.Close'、'rows.Next'、'rows.Scan'、'serializeRelayGroupNodeIDs' |
| 418–448 | function | `(*TrafficRepository).DeleteNodeForSync` | *TrafficRepository 的方法，删除与 'delete node for sync' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 7；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'res.RowsAffected'、'strings.TrimSpace' |
| 451–569 | function | `(*TrafficRepository).BatchCreateNodes` | *TrafficRepository 的方法，执行与 'batch create nodes' 对应的业务或基础设施操作。 | 分支 19；循环 4；返回 16；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'len'、'make'、'r.db.BeginTx'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan'、'stmt.Close'、'strings.ToLower'、'strings.TrimSpace'、'tx.PrepareContext'、'tx.QueryContext'、'tx.Rollback'、'uniqueImportedNodeName' |
| 571–581 | function | `uniqueImportedNodeName` | 执行与 'unique imported node name' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0；调用 'fmt.Sprintf' |
| 583–597 | function | `replaceJSONNodeName` | 执行与 'replace json node name' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'json.Marshal'、'json.Unmarshal'、'string'、'strings.TrimSpace' |
| 600–616 | function | `(*TrafficRepository).DeleteAllUserNodes` | *TrafficRepository 的方法，删除与 'delete all user nodes' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.TrimSpace' |
| 619–649 | function | `(*TrafficRepository).UpdateNodeProbeServer` | *TrafficRepository 的方法，更新与 'update node probe server' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 7；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'res.RowsAffected'、'strings.TrimSpace' |
| 653–704 | function | `(*TrafficRepository).migrateChainProxyNodes` | *TrafficRepository 的方法，执行与 'migrate chain proxy nodes' 对应的业务或基础设施操作。 | 分支 6；循环 2；返回 1；goroutine 0；调用 'Scan'、'append'、'delete'、'json.Marshal'、'json.Unmarshal'、'r.db.Exec'、'r.db.Query'、'r.db.QueryRow'、'rows.Close'、'rows.Next'、'rows.Scan'、'string' |

## `internal/storage/nodes_batch_test.go`

依赖：`context`、`strings`、`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–30 | function | `TestBatchCreateNodesAddsSuffixForDuplicateNames` | 执行与 'test batch create nodes adds suffix for duplicate names' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 0；goroutine 0；调用 'context.Background'、'mustCreateNode'、'newTestRepo'、'repo.BatchCreateNodes'、'strings.Contains'、't.Fatal'、't.Fatalf' |

## `internal/storage/nodes_relay_test.go`

依赖：`context`、`path/filepath`、`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 10–17 | function | `newTestRepo` | 创建并初始化与 'new test repo' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'NewTrafficRepository'、'filepath.Join'、't.Fatalf'、't.Helper'、't.TempDir' |
| 19–26 | function | `mustCreateNode` | 执行与 'must create node' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'context.Background'、'repo.CreateNode'、't.Fatalf'、't.Helper' |
| 31–83 | function | `TestDeleteNode_PrunesRelayGroupMember` | 执行与 'test delete node_ prunes relay group member' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 0；goroutine 0；调用 'context.Background'、'len'、'mustCreateNode'、'newTestRepo'、'repo.DeleteNode'、'repo.GetNode'、't.Errorf'、't.Fatalf' |

## `internal/storage/rule_template_owners.go`

依赖：`context`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–21 | function | `(*TrafficRepository).ensureRuleTemplateOwnersTable` | *TrafficRepository 的方法，执行与 'ensure rule template owners table' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext' |
| 23–33 | function | `(*TrafficRepository).SetRuleTemplatePublic` | *TrafficRepository 的方法，设置与 'set rule template public' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext'、'r.ensureRuleTemplateOwnersTable' |
| 35–41 | function | `(*TrafficRepository).IsRuleTemplatePublic` | *TrafficRepository 的方法，判断与 'is rule template public' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'Scan'、'r.db.QueryRowContext'、'r.ensureRuleTemplateOwnersTable' |
| 44–56 | function | `(*TrafficRepository).SetRuleTemplateOwner` | *TrafficRepository 的方法，设置与 'set rule template owner' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'r.db.ExecContext'、'r.ensureRuleTemplateOwnersTable' |
| 59–72 | function | `(*TrafficRepository).GetRuleTemplateOwner` | *TrafficRepository 的方法，查询或读取与 'get rule template owner' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'Scan'、'r.db.QueryRowContext'、'r.ensureRuleTemplateOwnersTable' |
| 75–84 | function | `(*TrafficRepository).RenameRuleTemplateOwner` | *TrafficRepository 的方法，执行与 'rename rule template owner' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'r.db.ExecContext'、'r.ensureRuleTemplateOwnersTable' |
| 87–96 | function | `(*TrafficRepository).DeleteRuleTemplateOwner` | *TrafficRepository 的方法，删除与 'delete rule template owner' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'r.db.ExecContext'、'r.ensureRuleTemplateOwnersTable' |
| 99–109 | function | `(*TrafficRepository).CountUserRuleTemplates` | *TrafficRepository 的方法，执行与 'count user rule templates' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Scan'、'r.db.QueryRowContext'、'r.ensureRuleTemplateOwnersTable' |
| 112–133 | function | `(*TrafficRepository).ListRuleTemplateOwners` | *TrafficRepository 的方法，列举与 'list rule template owners' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'make'、'r.db.QueryContext'、'r.ensureRuleTemplateOwnersTable'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |

## `internal/storage/speedtest.go`

依赖：`context`、`database/sql`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 10–16 | type | `SpeedTester` | 定义 'SpeedTester' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 18–26 | function | `(*TrafficRepository).CreateSpeedTester` | *TrafficRepository 的方法，创建与 'create speed tester' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext'、'res.LastInsertId' |
| 28–41 | function | `(*TrafficRepository).GetSpeedTesterByTokenHash` | *TrafficRepository 的方法，查询或读取与 'get speed tester by token hash' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Scan'、'r.db.QueryRowContext' |
| 43–63 | function | `(*TrafficRepository).ListSpeedTesters` | *TrafficRepository 的方法，列举与 'list speed testers' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 3；goroutine 0；调用 'append'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 65–68 | function | `(*TrafficRepository).DeleteSpeedTester` | *TrafficRepository 的方法，删除与 'delete speed tester' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 72–75 | function | `(*TrafficRepository).UpdateSpeedTesterToken` | *TrafficRepository 的方法，更新与 'update speed tester token' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 77–80 | function | `(*TrafficRepository).TouchSpeedTester` | *TrafficRepository 的方法，执行与 'touch speed tester' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 83–96 | type | `SpeedTestResult` | 定义 'SpeedTestResult' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 98–107 | function | `(*TrafficRepository).InsertSpeedTestResult` | *TrafficRepository 的方法，写入与 'insert speed test result' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext'、'result.LastInsertId' |
| 109–114 | function | `(*TrafficRepository).UpdateSpeedTestResult` | *TrafficRepository 的方法，更新与 'update speed test result' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 116–135 | function | `(*TrafficRepository).ListLatestSpeedTestResults` | *TrafficRepository 的方法，列举与 'list latest speed test results' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 3；goroutine 0；调用 'append'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 137–167 | function | `(*TrafficRepository).ListSpeedTestResults` | *TrafficRepository 的方法，列举与 'list speed test results' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 3；goroutine 0；调用 'append'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 170–188 | function | `(*TrafficRepository).GetNodeByID` | *TrafficRepository 的方法，查询或读取与 'get node by id' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Scan'、'r.db.QueryRowContext'、'scanNodeTags'、'scanRelayGroupNodeIDs' |

## `internal/storage/subscribe_files.go`

依赖：`context`、`database/sql`、`encoding/json`、`errors`、`fmt`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–13 | const | `SubscribeTypeCreate` | 定义 'SubscribeTypeCreate' 的不可变协议值、默认值或枚举成员。 |  |
| 14–14 | const | `SubscribeTypeImport` | 定义 'SubscribeTypeImport' 的不可变协议值、默认值或枚举成员。 |  |
| 15–15 | const | `SubscribeTypeUpload` | 定义 'SubscribeTypeUpload' 的不可变协议值、默认值或枚举成员。 |  |
| 18–31 | function | `parseSubscribeFileJSONFields` | 解析与 'parse subscribe file json fields' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'json.Unmarshal' |
| 33–42 | function | `serializeInt64Slice` | 执行与 'serialize int64 slice' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'json.Marshal'、'len'、'string' |
| 45–85 | function | `(*TrafficRepository).ListSubscribeFiles` | *TrafficRepository 的方法，列举与 'list subscribe files' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'parseSubscribeFileJSONFields'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 88–122 | function | `(*TrafficRepository).GetSubscribeFileByID` | *TrafficRepository 的方法，查询或读取与 'get subscribe file by id' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'parseSubscribeFileJSONFields'、'r.db.QueryRowContext'、'row.Scan' |
| 125–160 | function | `(*TrafficRepository).GetSubscribeFileByName` | *TrafficRepository 的方法，查询或读取与 'get subscribe file by name' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'parseSubscribeFileJSONFields'、'r.db.QueryRowContext'、'row.Scan'、'strings.TrimSpace' |
| 163–198 | function | `(*TrafficRepository).GetSubscribeFileByFilename` | *TrafficRepository 的方法，查询或读取与 'get subscribe file by filename' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'parseSubscribeFileJSONFields'、'r.db.QueryRowContext'、'row.Scan'、'strings.TrimSpace' |
| 201–276 | function | `(*TrafficRepository).CreateSubscribeFile` | *TrafficRepository 的方法，创建与 'create subscribe file' 对应的业务或基础设施操作。 | 分支 14；循环 1；返回 11；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'generateFileShortCode'、'json.Marshal'、'len'、'r.GetSubscribeFileByID'、'r.db.ExecContext'、'res.LastInsertId'、'serializeInt64Slice'、'string'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 279–348 | function | `(*TrafficRepository).UpdateSubscribeFile` | *TrafficRepository 的方法，更新与 'update subscribe file' 对应的业务或基础设施操作。 | 分支 15；循环 0；返回 11；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'json.Marshal'、'len'、'r.GetSubscribeFileByID'、'r.db.ExecContext'、'res.RowsAffected'、'serializeInt64Slice'、'string'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 351–374 | function | `(*TrafficRepository).ReorderSubscribeFiles` | *TrafficRepository 的方法，执行与 'reorder subscribe files' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.BeginTx'、'stmt.Close'、'stmt.ExecContext'、'tx.Commit'、'tx.PrepareContext'、'tx.Rollback' |
| 377–418 | function | `(*TrafficRepository).DeleteSubscribeFile` | *TrafficRepository 的方法，删除与 'delete subscribe file' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 9；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.BeginTx'、'res.RowsAffected'、'tx.Commit'、'tx.ExecContext'、'tx.Rollback' |
| 422–472 | function | `(*TrafficRepository).GetSubscribeFilesByTemplate` | *TrafficRepository 的方法，查询或读取与 'get subscribe files by template' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 6；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'parseSubscribeFileJSONFields'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan'、'strings.TrimSpace' |
| 475–520 | function | `(*TrafficRepository).GetSubscribeFilesWithTemplate` | *TrafficRepository 的方法，查询或读取与 'get subscribe files with template' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'parseSubscribeFileJSONFields'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 523–573 | function | `(*TrafficRepository).CleanupStatsServerIDs` | *TrafficRepository 的方法，清理与 'cleanup stats server i ds' 对应的业务或基础设施操作。 | 分支 7；循环 4；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'len'、'make'、'r.db.ExecContext'、'r.db.QueryContext'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.Join'、'strings.Split'、'strings.TrimSpace' |
| 576–585 | function | `(*TrafficRepository).ClearAllStatsServerIDs` | *TrafficRepository 的方法，执行与 'clear all stats server i ds' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext' |

## `internal/storage/template.go`

依赖：`context`、`database/sql`、`errors`、`fmt`、`strings`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–23 | type | `Template` | 定义 'Template' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 26–26 | var | `ErrTemplateNotFound` | 保存 'ErrTemplateNotFound' 的包级共享状态、配置或预计算值。 |  |
| 27–27 | var | `ErrTemplateExists` | 保存 'ErrTemplateExists' 的包级共享状态、配置或预计算值。 |  |
| 31–60 | function | `(*TrafficRepository).ListTemplates` | *TrafficRepository 的方法，列举与 'list templates' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'scanTemplate' |
| 63–84 | function | `(*TrafficRepository).GetTemplateByID` | *TrafficRepository 的方法，查询或读取与 'get template by id' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'scanTemplate' |
| 87–113 | function | `(*TrafficRepository).GetTemplateByName` | *TrafficRepository 的方法，查询或读取与 'get template by name' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'scanTemplate'、'strings.TrimSpace' |
| 116–153 | function | `(*TrafficRepository).CreateTemplate` | *TrafficRepository 的方法，创建与 'create template' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 7；goroutine 0；调用 'boolToInt'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.GetTemplateByName'、'r.db.ExecContext'、'result.LastInsertId'、'strings.TrimSpace' |
| 156–201 | function | `(*TrafficRepository).UpdateTemplate` | *TrafficRepository 的方法，更新与 'update template' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 8；goroutine 0；调用 'boolToInt'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.GetTemplateByID'、'r.GetTemplateByName'、'r.db.ExecContext'、'strings.TrimSpace' |
| 204–228 | function | `(*TrafficRepository).DeleteTemplate` | *TrafficRepository 的方法，删除与 'delete template' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.RowsAffected' |
| 230–242 | function | `scanTemplate` | 执行与 'scan template' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'scanner.Scan' |
| 244–249 | function | `boolToInt` | 执行与 'bool to int' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0 |

## `internal/storage/traffic.go`

依赖：`context`、`crypto/rand`、`database/sql`、`encoding/json`、`errors`、`fmt`、`os`、`path/filepath`、`strings`、`time`、`github.com/google/uuid`、`modernc.org/sqlite`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 20–20 | const | `pragmaJournalMode` | 定义 'pragmaJournalMode' 的不可变协议值、默认值或枚举成员。 |  |
| 24–24 | const | `RoleAdmin` | 定义 'RoleAdmin' 的不可变协议值、默认值或枚举成员。 |  |
| 25–25 | const | `RoleUser` | 定义 'RoleUser' 的不可变协议值、默认值或枚举成员。 |  |
| 29–29 | const | `SubscriptionButtonQR` | 定义 'SubscriptionButtonQR' 的不可变协议值、默认值或枚举成员。 |  |
| 30–30 | const | `SubscriptionButtonCopy` | 定义 'SubscriptionButtonCopy' 的不可变协议值、默认值或枚举成员。 |  |
| 31–31 | const | `SubscriptionButtonImport` | 定义 'SubscriptionButtonImport' 的不可变协议值、默认值或枚举成员。 |  |
| 35–40 | type | `TrafficRecord` | 定义 'TrafficRecord' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 43–45 | type | `TrafficRepository` | 定义 'TrafficRepository' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 48–58 | type | `SubscriptionLink` | 定义 'SubscriptionLink' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 60–86 | function | `normalizeSubscriptionButtons` | 规范化与 'normalize subscription buttons' 对应的业务或基础设施操作。 | 分支 4；循环 2；返回 3；goroutine 0；调用 'append'、'len'、'make'、'strings.ToLower'、'strings.TrimSpace' |
| 88–95 | function | `encodeSubscriptionButtons` | 执行与 'encode subscription buttons' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'json.Marshal'、'normalizeSubscriptionButtons'、'string' |
| 97–108 | function | `decodeSubscriptionButtons` | 执行与 'decode subscription buttons' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'append'、'json.Unmarshal'、'normalizeSubscriptionButtons'、'strings.TrimSpace' |
| 110–112 | type | `rowScanner` | 定义 'rowScanner' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 114–127 | function | `scanSubscriptionLink` | 执行与 'scan subscription link' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'decodeSubscriptionButtons'、'scanner.Scan' |
| 129–135 | function | `scanProbeConfig` | 执行与 'scan probe config' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'scanner.Scan' |
| 137–143 | function | `scanProbeServer` | 执行与 'scan probe server' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'scanner.Scan' |
| 146–146 | var | `ErrTokenNotFound` | 保存 'ErrTokenNotFound' 的包级共享状态、配置或预计算值。 |  |
| 147–147 | var | `ErrUserNotFound` | 保存 'ErrUserNotFound' 的包级共享状态、配置或预计算值。 |  |
| 148–148 | var | `ErrUserExists` | 保存 'ErrUserExists' 的包级共享状态、配置或预计算值。 |  |
| 149–149 | var | `ErrRuleVersionNotFound` | 保存 'ErrRuleVersionNotFound' 的包级共享状态、配置或预计算值。 |  |
| 150–150 | var | `ErrSubscriptionNotFound` | 保存 'ErrSubscriptionNotFound' 的包级共享状态、配置或预计算值。 |  |
| 151–151 | var | `ErrSubscriptionExists` | 保存 'ErrSubscriptionExists' 的包级共享状态、配置或预计算值。 |  |
| 152–152 | var | `ErrProbeConfigNotFound` | 保存 'ErrProbeConfigNotFound' 的包级共享状态、配置或预计算值。 |  |
| 153–153 | var | `ErrNodeNotFound` | 保存 'ErrNodeNotFound' 的包级共享状态、配置或预计算值。 |  |
| 154–154 | var | `ErrSubscribeFileNotFound` | 保存 'ErrSubscribeFileNotFound' 的包级共享状态、配置或预计算值。 |  |
| 155–155 | var | `ErrSubscribeFileExists` | 保存 'ErrSubscribeFileExists' 的包级共享状态、配置或预计算值。 |  |
| 156–156 | var | `ErrUserSettingsNotFound` | 保存 'ErrUserSettingsNotFound' 的包级共享状态、配置或预计算值。 |  |
| 157–157 | var | `ErrExternalSubscriptionNotFound` | 保存 'ErrExternalSubscriptionNotFound' 的包级共享状态、配置或预计算值。 |  |
| 158–158 | var | `ErrExternalSubscriptionExists` | 保存 'ErrExternalSubscriptionExists' 的包级共享状态、配置或预计算值。 |  |
| 162–166 | var | `allowedSubscriptionButtons` | 保存 'allowedSubscriptionButtons' 的包级共享状态、配置或预计算值。 |  |
| 167–171 | var | `defaultSubscriptionButtons` | 保存 'defaultSubscriptionButtons' 的包级共享状态、配置或预计算值。 |  |
| 175–175 | const | `ProbeTypeNezha` | 定义 'ProbeTypeNezha' 的不可变协议值、默认值或枚举成员。 |  |
| 176–176 | const | `ProbeTypeNezhaV0` | 定义 'ProbeTypeNezhaV0' 的不可变协议值、默认值或枚举成员。 |  |
| 177–177 | const | `ProbeTypeDstatus` | 定义 'ProbeTypeDstatus' 的不可变协议值、默认值或枚举成员。 |  |
| 178–178 | const | `ProbeTypeKomari` | 定义 'ProbeTypeKomari' 的不可变协议值、默认值或枚举成员。 |  |
| 180–180 | const | `TrafficMethodUp` | 定义 'TrafficMethodUp' 的不可变协议值、默认值或枚举成员。 |  |
| 181–181 | const | `TrafficMethodDown` | 定义 'TrafficMethodDown' 的不可变协议值、默认值或枚举成员。 |  |
| 182–182 | const | `TrafficMethodBoth` | 定义 'TrafficMethodBoth' 的不可变协议值、默认值或枚举成员。 |  |
| 185–192 | type | `ProbeConfig` | 定义 'ProbeConfig' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 194–204 | type | `ProbeServer` | 定义 'ProbeServer' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 207–225 | type | `Node` | 定义 'Node' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 228–250 | type | `SubscribeFile` | 定义 'SubscribeFile' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 253–275 | type | `UserSettings` | 定义 'UserSettings' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 278–315 | type | `SystemConfig` | 定义 'SystemConfig' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 318–335 | type | `ExternalSubscription` | 定义 'ExternalSubscription' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 338–348 | type | `OverrideScript` | 定义 'OverrideScript' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 351–360 | type | `CustomRule` | 定义 'CustomRule' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 363–387 | type | `ProxyProviderConfig` | 定义 'ProxyProviderConfig' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 390–395 | var | `allowedProbeTypes` | 保存 'allowedProbeTypes' 的包级共享状态、配置或预计算值。 |  |
| 396–400 | var | `allowedTrafficMethods` | 保存 'allowedTrafficMethods' 的包级共享状态、配置或预计算值。 |  |
| 404–446 | function | `NewTrafficRepository` | 创建并初始化与 'new traffic repository' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 9；goroutine 0；调用 'db.Close'、'db.Exec'、'db.SetMaxOpenConns'、'errors.New'、'filepath.Dir'、'fmt.Errorf'、'os.MkdirAll'、'repo.migrate'、'sql.Open'、'strings.HasPrefix' |
| 449–454 | function | `(*TrafficRepository).Close` | *TrafficRepository 的方法，执行与 'close' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.Close' |
| 458–470 | function | `(*TrafficRepository).Checkpoint` | *TrafficRepository 的方法，检查与 'checkpoint' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'Scan'、'fmt.Errorf'、'r.db.QueryRow' |
| 474–490 | function | `(*TrafficRepository).CheckpointBestEffort` | *TrafficRepository 的方法，检查与 'checkpoint best effort' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'fmt.Errorf'、'r.db.QueryRow' |
| 492–1333 | function | `(*TrafficRepository).migrate` | *TrafficRepository 的方法，执行与 'migrate' 对应的业务或基础设施操作。 | 分支 119；循环 1；返回 120；goroutine 0；调用 'fmt.Errorf'、'r.db.Exec'、'r.ensureDefaultProbeConfig'、'r.ensureExternalSubscriptionColumn'、'r.ensureNodeColumn'、'r.ensureSubscribeFileColumn'、'r.ensureSubscriptionLinkColumn'、'r.ensureUserColumn'、'r.ensureUserSettingsColumn'、'r.ensureUserTokenColumn'、'r.generateMissingFileShortCodes'、'r.generateMissingUserShortCodes'、'r.mig… |
| 1335–1345 | function | `(*TrafficRepository).GetSystemSetting` | *TrafficRepository 的方法，查询或读取与 'get system setting' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'r.db.QueryRowContext' |
| 1347–1355 | function | `(*TrafficRepository).SetSystemSetting` | *TrafficRepository 的方法，设置与 'set system setting' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'errors.New'、'r.db.ExecContext' |
| 1358–1383 | function | `(*TrafficRepository).ListSubscriptionLinks` | *TrafficRepository 的方法，列举与 'list subscription links' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'scanSubscriptionLink' |
| 1386–1407 | function | `(*TrafficRepository).GetSubscriptionByName` | *TrafficRepository 的方法，查询或读取与 'get subscription by name' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'scanSubscriptionLink'、'strings.TrimSpace' |
| 1410–1430 | function | `(*TrafficRepository).GetSubscriptionByID` | *TrafficRepository 的方法，查询或读取与 'get subscription by id' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'scanSubscriptionLink' |
| 1433–1449 | function | `(*TrafficRepository).GetFirstSubscriptionLink` | *TrafficRepository 的方法，查询或读取与 'get first subscription link' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'scanSubscriptionLink' |
| 1452–1495 | function | `(*TrafficRepository).CreateSubscriptionLink` | *TrafficRepository 的方法，创建与 'create subscription link' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 8；goroutine 0；调用 'encodeSubscriptionButtons'、'err.Error'、'errors.New'、'fmt.Errorf'、'r.GetSubscriptionByID'、'r.db.ExecContext'、'res.LastInsertId'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 1498–1545 | function | `(*TrafficRepository).UpdateSubscriptionLink` | *TrafficRepository 的方法，更新与 'update subscription link' 对应的业务或基础设施操作。 | 分支 10；循环 0；返回 10；goroutine 0；调用 'encodeSubscriptionButtons'、'err.Error'、'errors.New'、'fmt.Errorf'、'r.GetSubscriptionByID'、'r.db.ExecContext'、'res.RowsAffected'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 1548–1570 | function | `(*TrafficRepository).DeleteSubscriptionLink` | *TrafficRepository 的方法，删除与 'delete subscription link' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'res.RowsAffected' |
| 1573–1589 | function | `(*TrafficRepository).CountSubscriptionsByFilename` | *TrafficRepository 的方法，执行与 'count subscriptions by filename' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'Scan'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 1592–1632 | function | `(*TrafficRepository).GetProbeConfig` | *TrafficRepository 的方法，查询或读取与 'get probe config' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 7；goroutine 0；调用 'append'、'context.Background'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'r.db.QueryRowContext'、'rows.Close'、'rows.Err'、'rows.Next'、'scanProbeConfig'、'scanProbeServer' |
| 1635–1724 | function | `(*TrafficRepository).UpsertProbeConfig` | *TrafficRepository 的方法，执行与 'upsert probe config' 对应的业务或基础设施操作。 | 分支 15；循环 2；返回 15；goroutine 0；调用 'append'、'context.Background'、'errors.New'、'fmt.Errorf'、'len'、'make'、'r.GetProbeConfig'、'r.db.BeginTx'、'stmt.Close'、'stmt.ExecContext'、'strings.ToLower'、'strings.TrimSpace'、'tx.Commit'、'tx.ExecContext'、'tx.PrepareContext'、'tx.Rollback' |
| 1727–1754 | function | `(*TrafficRepository).DeleteProbeConfig` | *TrafficRepository 的方法，删除与 'delete probe config' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'fmt.Errorf'、'r.db.BeginTx'、'tx.Commit'、'tx.ExecContext'、'tx.Rollback' |
| 1756–1760 | function | `(*TrafficRepository).ensureDefaultProbeConfig` | *TrafficRepository 的方法，执行与 'ensure default probe config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 1762–1834 | function | `(*TrafficRepository).migrateProbeConfigsForNezhaV0` | *TrafficRepository 的方法，执行与 'migrate probe configs for nezha v0' 对应的业务或基础设施操作。 | 分支 11；循环 0；返回 12；goroutine 0；调用 'fmt.Errorf'、'r.db.Begin'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.Contains'、'tx.Commit'、'tx.Exec'、'tx.Rollback' |
| 1836–1866 | function | `(*TrafficRepository).ensureUserColumn` | *TrafficRepository 的方法，执行与 'ensure user column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 1868–1898 | function | `(*TrafficRepository).ensureUserTokenColumn` | *TrafficRepository 的方法，执行与 'ensure user token column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 1900–1930 | function | `(*TrafficRepository).ensureSubscriptionLinkColumn` | *TrafficRepository 的方法，执行与 'ensure subscription link column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 1932–1962 | function | `(*TrafficRepository).ensureNodeColumn` | *TrafficRepository 的方法，执行与 'ensure node column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 1964–1994 | function | `(*TrafficRepository).ensureUserSettingsColumn` | *TrafficRepository 的方法，执行与 'ensure user settings column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 1998–2014 | function | `(*TrafficRepository).migrateTemplateVersionFromBool` | *TrafficRepository 的方法，执行与 'migrate template version from bool' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Scan'、'fmt.Errorf'、'r.db.Exec'、'r.db.QueryRow' |
| 2016–2079 | function | `(*TrafficRepository).migrateCustomRulesAppendMode` | *TrafficRepository 的方法，执行与 'migrate custom rules append mode' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 8；goroutine 0；调用 'fmt.Errorf'、'r.db.Begin'、'tx.Commit'、'tx.Exec'、'tx.Rollback' |
| 2081–2111 | function | `(*TrafficRepository).ensureSubscribeFileColumn` | *TrafficRepository 的方法，执行与 'ensure subscribe file column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 2113–2143 | function | `(*TrafficRepository).ensureExternalSubscriptionColumn` | *TrafficRepository 的方法，执行与 'ensure external subscription column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 2145–2175 | function | `(*TrafficRepository).ensureProxyProviderConfigColumn` | *TrafficRepository 的方法，执行与 'ensure proxy provider config column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 2177–2207 | function | `(*TrafficRepository).ensureSystemConfigColumn` | *TrafficRepository 的方法，执行与 'ensure system config column' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.EqualFold' |
| 2209–2219 | function | `(*TrafficRepository).syncNicknames` | *TrafficRepository 的方法，同步与 'sync nicknames' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.Exec' |
| 2222–2244 | function | `(*TrafficRepository).RecordDaily` | *TrafficRepository 的方法，执行与 'record daily' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Format'、'date.UTC'、'errors.New'、'fmt.Errorf'、'r.db.ExecContext' |
| 2247–2298 | function | `(*TrafficRepository).ListRecent` | *TrafficRepository 的方法，列举与 'list recent' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 6；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan'、'time.Parse' |
| 2301–2341 | function | `(*TrafficRepository).GetOrCreateUserToken` | *TrafficRepository 的方法，查询或读取与 'get or create user token' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 6；goroutine 0；调用 'Scan'、'err.Error'、'errors.Is'、'errors.New'、'fmt.Errorf'、'generateUserShortCode'、'r.db.ExecContext'、'r.db.QueryRowContext'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace'、'uuid.NewString' |
| 2344–2385 | function | `(*TrafficRepository).ResetUserToken` | *TrafficRepository 的方法，重置与 'reset user token' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 6；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'generateUserShortCode'、'r.db.ExecContext'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace'、'uuid.NewString' |
| 2388–2408 | function | `(*TrafficRepository).ValidateUserToken` | *TrafficRepository 的方法，校验与 'validate user token' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 2411–2425 | function | `generateFileShortCode` | 生成与 'generate file short code' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'fmt.Errorf'、'int'、'len'、'make'、'rand.Read'、'string' |
| 2428–2448 | function | `generateUserShortCode` | 生成与 'generate user short code' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 3；goroutine 0；调用 'fmt.Errorf'、'int'、'len'、'make'、'rand.Read'、'string' |
| 2451–2489 | function | `(*TrafficRepository).generateMissingFileShortCodes` | *TrafficRepository 的方法，生成与 'generate missing file short codes' 对应的业务或基础设施操作。 | 分支 5；循环 3；返回 5；goroutine 0；调用 'append'、'err.Error'、'fmt.Errorf'、'generateFileShortCode'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.Contains'、'strings.ToLower' |
| 2492–2530 | function | `(*TrafficRepository).generateMissingUserShortCodes` | *TrafficRepository 的方法，生成与 'generate missing user short codes' 对应的业务或基础设施操作。 | 分支 5；循环 3；返回 5；goroutine 0；调用 'append'、'err.Error'、'fmt.Errorf'、'generateUserShortCode'、'r.db.Exec'、'r.db.Query'、'rows.Close'、'rows.Next'、'rows.Scan'、'strings.Contains'、'strings.ToLower' |
| 2534–2563 | function | `(*TrafficRepository).ResetAllSubscriptionShortURLs` | *TrafficRepository 的方法，重置与 'reset all subscription short ur ls' 对应的业务或基础设施操作。 | 分支 4；循环 2；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'r.resetFileShortCode'、'rows.Close'、'rows.Next'、'rows.Scan' |
| 2566–2588 | function | `(*TrafficRepository).resetFileShortCode` | *TrafficRepository 的方法，重置与 'reset file short code' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 4；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'generateFileShortCode'、'r.db.ExecContext'、'strings.Contains'、'strings.ToLower' |
| 2591–2610 | function | `(*TrafficRepository).GetSubscriptionByShortURL` | *TrafficRepository 的方法，查询或读取与 'get subscription by short url' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 2613–2632 | function | `(*TrafficRepository).GetFilenameByFileShortCode` | *TrafficRepository 的方法，查询或读取与 'get filename by file short code' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 2635–2651 | function | `(*TrafficRepository).GetFilenameByCustomShortCode` | *TrafficRepository 的方法，查询或读取与 'get filename by custom short code' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 2654–2673 | function | `(*TrafficRepository).GetUsernameByUserShortCode` | *TrafficRepository 的方法，查询或读取与 'get username by user short code' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 2676–2695 | function | `(*TrafficRepository).GetUserShortCode` | *TrafficRepository 的方法，查询或读取与 'get user short code' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 2698–2718 | function | `(*TrafficRepository).GetEffectiveUserShortCode` | *TrafficRepository 的方法，查询或读取与 'get effective user short code' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 2722–2746 | function | `(*TrafficRepository).GetAllFileShortCodes` | *TrafficRepository 的方法，查询或读取与 'get all file short codes' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'make'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 2750–2774 | function | `(*TrafficRepository).GetAllUserShortCodes` | *TrafficRepository 的方法，查询或读取与 'get all user short codes' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'make'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 2777–2807 | function | `(*TrafficRepository).UpdateUserCustomShortCode` | *TrafficRepository 的方法，更新与 'update user custom short code' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 8；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'r.GetOrCreateUserToken'、'r.db.ExecContext'、'res.RowsAffected'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 2810–2827 | function | `(*TrafficRepository).GetUserCustomShortCode` | *TrafficRepository 的方法，查询或读取与 'get user custom short code' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 2830–2871 | function | `(*TrafficRepository).SaveRuleVersion` | *TrafficRepository 的方法，持久化与 'save rule version' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 7；goroutine 0；调用 'Scan'、'errors.New'、'fmt.Errorf'、'int64'、'r.db.BeginTx'、'strings.TrimSpace'、'tx.Commit'、'tx.ExecContext'、'tx.QueryRowContext'、'tx.Rollback' |
| 2848–2854 | closure | `SaveRuleVersion.closure#1` | 供 SaveRuleVersion 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'tx.Commit'、'tx.Rollback' |
| 2874–2909 | function | `(*TrafficRepository).ListRuleVersions` | *TrafficRepository 的方法，列举与 'list rule versions' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 6；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan'、'strings.TrimSpace' |
| 2912–2921 | function | `(*TrafficRepository).LatestRuleVersion` | *TrafficRepository 的方法，执行与 'latest rule version' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'len'、'r.ListRuleVersions' |
| 2924–2930 | type | `RuleVersion` | 定义 'RuleVersion' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 2933–2947 | type | `User` | 定义 'User' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 2950–2954 | type | `UserProfileUpdate` | 定义 'UserProfileUpdate' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 2957–2976 | function | `(*TrafficRepository).EnsureUser` | *TrafficRepository 的方法，执行与 'ensure user' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.TrimSpace' |
| 2979–3016 | function | `(*TrafficRepository).CreateUser` | *TrafficRepository 的方法，创建与 'create user' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 6；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 3019–3048 | function | `(*TrafficRepository).GetUser` | *TrafficRepository 的方法，查询或读取与 'get user' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'row.Scan'、'strings.TrimSpace' |
| 3051–3061 | function | `(*TrafficRepository).GetAdminUsername` | *TrafficRepository 的方法，查询或读取与 'get admin username' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Scan'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext' |
| 3064–3101 | function | `(*TrafficRepository).ListUsers` | *TrafficRepository 的方法，列举与 'list users' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 3104–3120 | function | `(*TrafficRepository).UpdateUserRemark` | *TrafficRepository 的方法，更新与 'update user remark' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.TrimSpace' |
| 3123–3150 | function | `(*TrafficRepository).UpdateUserPassword` | *TrafficRepository 的方法，更新与 'update user password' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 7；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'res.RowsAffected'、'strings.TrimSpace' |
| 3153–3185 | function | `(*TrafficRepository).UpdateUserRole` | *TrafficRepository 的方法，更新与 'update user role' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 6；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'res.RowsAffected'、'strings.ToLower'、'strings.TrimSpace' |
| 3188–3217 | function | `(*TrafficRepository).UpdateUserStatus` | *TrafficRepository 的方法，更新与 'update user status' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'res.RowsAffected'、'strings.TrimSpace' |
| 3220–3292 | function | `(*TrafficRepository).DeleteUser` | *TrafficRepository 的方法，删除与 'delete user' 对应的业务或基础设施操作。 | 分支 13；循环 0；返回 14；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.BeginTx'、'res.RowsAffected'、'strings.TrimSpace'、'tx.Commit'、'tx.ExecContext'、'tx.Rollback' |
| 3295–3324 | function | `(*TrafficRepository).UpdateUserNickname` | *TrafficRepository 的方法，更新与 'update user nickname' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'res.RowsAffected'、'strings.TrimSpace' |
| 3327–3359 | function | `(*TrafficRepository).UpdateUserProfile` | *TrafficRepository 的方法，更新与 'update user profile' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'res.RowsAffected'、'strings.TrimSpace' |
| 3361–3364 | function | `(*TrafficRepository).SetUserTOTPSecret` | *TrafficRepository 的方法，设置与 'set user totp secret' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 3366–3369 | function | `(*TrafficRepository).EnableUserTOTP` | *TrafficRepository 的方法，执行与 'enable user totp' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 3371–3374 | function | `(*TrafficRepository).DisableUserTOTP` | *TrafficRepository 的方法，执行与 'disable user totp' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 3376–3379 | function | `(*TrafficRepository).UpdateUserRecoveryCodes` | *TrafficRepository 的方法，更新与 'update user recovery codes' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 3382–3424 | function | `(*TrafficRepository).RenameUser` | *TrafficRepository 的方法，执行与 'rename user' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 8；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.BeginTx'、'res.RowsAffected'、'strings.TrimSpace'、'tx.Commit'、'tx.ExecContext'、'tx.Rollback' |
| 3398–3404 | closure | `RenameUser.closure#1` | 供 RenameUser 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'tx.Commit'、'tx.Rollback' |
| 3427–3432 | type | `Session` | 定义 'Session' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 3435–3455 | function | `(*TrafficRepository).CreateSession` | *TrafficRepository 的方法，创建与 'create session' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.TrimSpace' |
| 3458–3474 | function | `(*TrafficRepository).DeleteSession` | *TrafficRepository 的方法，删除与 'delete session' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.TrimSpace' |
| 3477–3493 | function | `(*TrafficRepository).DeleteUserSessions` | *TrafficRepository 的方法，删除与 'delete user sessions' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.TrimSpace' |
| 3496–3522 | function | `(*TrafficRepository).LoadSessions` | *TrafficRepository 的方法，加载与 'load sessions' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 3525–3536 | function | `(*TrafficRepository).CleanupExpiredSessions` | *TrafficRepository 的方法，清理与 'cleanup expired sessions' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext' |
| 3539–3558 | function | `(*TrafficRepository).AssignSubscriptionToUser` | *TrafficRepository 的方法，执行与 'assign subscription to user' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.TrimSpace' |
| 3561–3580 | function | `(*TrafficRepository).RemoveSubscriptionFromUser` | *TrafficRepository 的方法，移除与 'remove subscription from user' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'strings.TrimSpace' |
| 3584–3622 | function | `(*TrafficRepository).GetUserSubscriptionIDs` | *TrafficRepository 的方法，查询或读取与 'get user subscription i ds' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 6；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan'、'strings.TrimSpace' |
| 3625–3672 | function | `(*TrafficRepository).SetUserSubscriptions` | *TrafficRepository 的方法，设置与 'set user subscriptions' 对应的业务或基础设施操作。 | 分支 9；循环 1；返回 8；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'len'、'r.db.BeginTx'、'stmt.Close'、'stmt.ExecContext'、'strings.TrimSpace'、'tx.Commit'、'tx.ExecContext'、'tx.PrepareContext'、'tx.Rollback' |
| 3676–3695 | function | `(*TrafficRepository).UserHasAccessToSubscribeFile` | *TrafficRepository 的方法，执行与 'user has access to subscribe file' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'Scan'、'errors.New'、'fmt.Errorf'、'r.GetUser'、'r.db.QueryRowContext' |
| 3698–3702 | type | `UserShortCodeInfo` | 定义 'UserShortCodeInfo' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 3705–3732 | function | `(*TrafficRepository).GetUsersBySubscriptionID` | *TrafficRepository 的方法，查询或读取与 'get users by subscription id' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 4；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 3735–3778 | function | `(*TrafficRepository).GetUserSubscriptions` | *TrafficRepository 的方法，查询或读取与 'get user subscriptions' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 6；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan'、'strings.TrimSpace' |
| 3781–3829 | function | `(*TrafficRepository).GetUserSettings` | *TrafficRepository 的方法，查询或读取与 'get user settings' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'json.Unmarshal'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 3832–3948 | function | `(*TrafficRepository).UpsertUserSettings` | *TrafficRepository 的方法，执行与 'upsert user settings' 对应的业务或基础设施操作。 | 分支 18；循环 0；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'json.Marshal'、'len'、'r.db.ExecContext'、'string'、'strings.TrimSpace' |
| 3951–3991 | function | `(*TrafficRepository).ListExternalSubscriptions` | *TrafficRepository 的方法，列举与 'list external subscriptions' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 6；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan'、'strings.TrimSpace' |
| 3994–4029 | function | `(*TrafficRepository).GetExternalSubscription` | *TrafficRepository 的方法，查询或读取与 'get external subscription' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 6；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 4032–4065 | function | `(*TrafficRepository).GetExternalSubscriptionByURL` | *TrafficRepository 的方法，查询或读取与 'get external subscription by url' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 6；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'strings.TrimSpace' |
| 4068–4122 | function | `(*TrafficRepository).CreateExternalSubscription` | *TrafficRepository 的方法，创建与 'create external subscription' 对应的业务或基础设施操作。 | 分支 11；循环 0；返回 8；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.LastInsertId'、'strings.Contains'、'strings.TrimSpace' |
| 4125–4184 | function | `(*TrafficRepository).UpdateExternalSubscription` | *TrafficRepository 的方法，更新与 'update external subscription' 对应的业务或基础设施操作。 | 分支 12；循环 0；返回 9；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.RowsAffected'、'strings.TrimSpace' |
| 4187–4223 | function | `(*TrafficRepository).DeleteExternalSubscription` | *TrafficRepository 的方法，删除与 'delete external subscription' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 8；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.RowsAffected'、'strings.TrimSpace' |
| 4228–4228 | var | `ErrCustomRuleNotFound` | 保存 'ErrCustomRuleNotFound' 的包级共享状态、配置或预计算值。 |  |
| 4232–4269 | function | `(*TrafficRepository).ListCustomRules` | *TrafficRepository 的方法，列举与 'list custom rules' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 4272–4295 | function | `(*TrafficRepository).GetCustomRule` | *TrafficRepository 的方法，查询或读取与 'get custom rule' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Scan'、'errors.Is'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext' |
| 4298–4356 | function | `(*TrafficRepository).CreateCustomRule` | *TrafficRepository 的方法，创建与 'create custom rule' 对应的业务或基础设施操作。 | 分支 13；循环 0；返回 11；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.LastInsertId'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 4359–4424 | function | `(*TrafficRepository).UpdateCustomRule` | *TrafficRepository 的方法，更新与 'update custom rule' 对应的业务或基础设施操作。 | 分支 15；循环 0；返回 13；goroutine 0；调用 'err.Error'、'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.RowsAffected'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 4427–4452 | function | `(*TrafficRepository).DeleteCustomRule` | *TrafficRepository 的方法，删除与 'delete custom rule' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.RowsAffected' |
| 4455–4492 | function | `(*TrafficRepository).ListEnabledCustomRules` | *TrafficRepository 的方法，列举与 'list enabled custom rules' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows4.Close'、'rows4.Err'、'rows4.Next'、'rows4.Scan' |
| 4495–4508 | function | `(*TrafficRepository).IsSyncTrafficEnabled` | *TrafficRepository 的方法，判断与 'is sync traffic enabled' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Scan'、'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext' |
| 4511–4547 | function | `(*TrafficRepository).ListAllExternalSubscriptions` | *TrafficRepository 的方法，列举与 'list all external subscriptions' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 4550–4586 | function | `(*TrafficRepository).GetSubscribeFilesWithAutoSync` | *TrafficRepository 的方法，查询或读取与 'get subscribe files with auto sync' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 4591–4624 | function | `(*TrafficRepository).CreateProxyProviderConfig` | *TrafficRepository 的方法，创建与 'create proxy provider config' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 3；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.LastInsertId' |
| 4627–4662 | function | `(*TrafficRepository).GetProxyProviderConfig` | *TrafficRepository 的方法，查询或读取与 'get proxy provider config' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'row.Scan' |
| 4665–4700 | function | `(*TrafficRepository).GetProxyProviderConfigByName` | *TrafficRepository 的方法，查询或读取与 'get proxy provider config by name' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.QueryRowContext'、'row.Scan' |
| 4703–4746 | function | `(*TrafficRepository).ListProxyProviderConfigs` | *TrafficRepository 的方法，列举与 'list proxy provider configs' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 4749–4792 | function | `(*TrafficRepository).ListProxyProviderConfigsBySubscription` | *TrafficRepository 的方法，列举与 'list proxy provider configs by subscription' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 4796–4841 | function | `(*TrafficRepository).ListMMWProxyProviderConfigs` | *TrafficRepository 的方法，列举与 'list mmw proxy provider configs' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 5；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 4844–4886 | function | `(*TrafficRepository).UpdateProxyProviderConfig` | *TrafficRepository 的方法，更新与 'update proxy provider config' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.RowsAffected' |
| 4889–4908 | function | `(*TrafficRepository).DeleteProxyProviderConfig` | *TrafficRepository 的方法，删除与 'delete proxy provider config' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'r.db.ExecContext'、'result.RowsAffected' |
| 4912–5034 | function | `(*TrafficRepository).GetSystemConfig` | *TrafficRepository 的方法，查询或读取与 'get system config' 对应的业务或基础设施操作。 | 分支 15；循环 0；返回 3；goroutine 0；调用 'Scan'、'errors.Is'、'fmt.Errorf'、'r.db.QueryRowContext' |
| 5038–5148 | function | `(*TrafficRepository).UpdateSystemConfig` | *TrafficRepository 的方法，更新与 'update system config' 对应的业务或基础设施操作。 | 分支 10；循环 0；返回 6；goroutine 0；调用 'boolToInt'、'fmt.Errorf'、'r.db.ExecContext'、'result.RowsAffected' |
| 5078–5083 | closure | `UpdateSystemConfig.closure#1` | 供 UpdateSystemConfig 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 2；goroutine 0 |
| 5152–5178 | function | `(*TrafficRepository).ListOverrideScripts` | *TrafficRepository 的方法，列举与 'list override scripts' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 3；goroutine 0；调用 'append'、'r.db.QueryContext'、'rows.Close'、'rows.Err'、'rows.Next'、'rows.Scan' |
| 5180–5190 | function | `(*TrafficRepository).GetOverrideScript` | *TrafficRepository 的方法，查询或读取与 'get override script' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'Scan'、'r.db.QueryRowContext' |
| 5192–5200 | function | `(*TrafficRepository).CreateOverrideScript` | *TrafficRepository 的方法，创建与 'create override script' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'r.db.ExecContext'、'result.LastInsertId' |
| 5202–5208 | function | `(*TrafficRepository).UpdateOverrideScript` | *TrafficRepository 的方法，更新与 'update override script' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |
| 5210–5214 | function | `(*TrafficRepository).DeleteOverrideScript` | *TrafficRepository 的方法，删除与 'delete override script' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'r.db.ExecContext' |

