# 数据模型与数据库设计

## 1. 跨数据库约定

NodeControll 同时支持 SQLite 和 PostgreSQL。两套 migration 语义一致，但不追求 SQL 文本相同。

| 概念 | 存储约定 |
|---|---|
| ID | UUIDv7，应用层 newtype；SQLite `TEXT` canonical，PG `uuid` |
| 时间 | UTC epoch milliseconds；SQLite/PG 均 `BIGINT`，避免隐式时区/精度差 |
| revision/sequence/bytes | 非负 `BIGINT`，应用 checked conversion；必要时 CHECK |
| bool | SQLite INTEGER CHECK(0,1)，PG boolean |
| enum | 小写文本 + CHECK；便于 migration 增值，不使用 PG enum |
| JSON | SQLite canonical TEXT + application/schema check，PG `jsonb`；核心可查询字段必须独立列 |
| secret | 业务表只存 `secret_id`；密文在 `secret_records`，绝不明文/通用 JSON |
| 软删除 | `deleted_at_ms`；凭据/会话即时 revoke，账本/audit保留 |
| 乐观并发 | 可编辑聚合根有 `revision BIGINT`，更新 `WHERE id=? AND revision=?` |
| 排序 | `(sort_key BIGINT,id)` 稳定；重排事务并校验集合 |
| 金额/流量 | 无 float；bytes/int倍率用 rational numerator/denominator 或 ppm |

所有显式 FK 在 SQLite connection 执行 `PRAGMA foreign_keys=ON`。SQLite 另设 WAL、`busy_timeout=5000`、`synchronous=NORMAL`，默认 max connections=1；PG 使用 pool、statement/lock timeout。两者 migrations 都通过空库、升级、回滚快照和 repository contract tests。

## 2. 基础、设置与对象存储

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `instances` | `id,name,public_id,created_at_ms,revision` | 单实例当前只有一行；public_id用于联合身份而非许可证 |
| `instance_settings` | `instance_id,key,schema_version,value_json,revision,updated_by` | PK(instance,key)；sensitive key禁止落此表 |
| `instance_assets` | `id,kind,object_id,alt_text,revision` | logo/favicon/background等；kind唯一 current |
| `secret_records` | `id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,deleted_at_ms,revision` | typed AEAD envelope；AAD 绑定 purpose、owner type/id、schema/key version；active(owner,purpose) 唯一，密文不可搜索 |
| `content_objects` | `id,sha256,size,mime,storage_backend,storage_key,created_at,ref_count` | 内容寻址；`sha256` 唯一；storage_key只由 backend生成 |
| `content_references` | `object_id,owner_type,owner_id,purpose,created_at` | 唯一(owner,purpose)；ref_count可重建 |
| `resource_versions` | `id,resource_type,resource_id,revision,object_id,metadata_json,created_by,created_at` | 模板/配置/站点等不可变快照 |
| `system_leases` | `lease_key,owner_id,fencing_token,leased_until_ms,updated_at` | scheduler/migration/singleton；token单调 |

Master encryption root 不在数据库：默认 root-only key file；当前实现加载一枚 current key 和最多 3 枚版本更旧的 key。数据库持久化 system-owned canary，Master 在 HTTP bind 前解密验证；canary 使用旧版本时立即原子 rewrap 到 current key。env/TPM/KMS provider 仍是后续边界。

## 3. 用户、身份、会话与 token

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `users` | `id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms` | username_norm唯一（未删除）；principal_label不可变且全局唯一 |
| `user_auth_state` | `user_id,auth_revision,password_changed_at_ms,updated_at_ms` | 密码/高危凭据变化推进 auth revision；session snapshot 不匹配即失效 |
| `user_profiles` | `user_id,display_name,email,email_norm,avatar_object_id,notes,timezone,locale,revision` | email可选唯一策略；notes仅管理员可见 |
| `auth_sessions` | `id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision` | 两类 token 只存用途隔离 HMAC；`auth_level=password/mfa/phishing_resistant/recovery`；rotation 可继承早于新 row 的证明时间，但不得延长 absolute expiry |
| `totp_credentials` | `user_id,secret_id,status,last_accepted_step,enrolled_at,revision` | 每用户一行；pending/active/disabled |
| `recovery_code_sets` | `user_id,set_version,status,total_count,created_at_ms,replaced_at_ms` | 每用户至多一个 active set；版本单调；整组替换在同一事务失效旧组 |
| `recovery_codes` | `id,user_id,set_version,position,digest_key_version,code_hmac,created_at_ms,consumed_at_ms` | 每组固定 8 个；只存 recovery-code 专用 HMAC；条件 consume 保证并发只成功一次 |
| `api_tokens` | `id,subject_type,subject_id,token_hmac,name,audience,scopes_json,expires_at,last_used_at,revoked_at,created_by` | token唯一；MCP/Cert webhook/automation使用明确audience |
| `subscription_credentials` | `id,user_id,package_instance_id,file_id,kind,token_hmac,short_code,status,expires_at,max_uses,use_count,grace_until,revision` | token_hmac/short_code分别唯一；绑定 audience/subject |
| `login_security_events` | `id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash` | 账号/IP 只存用途隔离的不可逆摘要，无明文账号、地址或错误密码；限时保留 |
| `idempotency_records` | `actor_key,route_key,idempotency_key,request_hash,response_status,response_headers_json,response_object_id,expires_at` | 组合 PK；相同 key不同 request hash冲突 |
| `turnstile_configs` | `instance_id,site_key,secret_id,enabled,fail_mode,revision` | secret独立加密；仅一配置 |

活动会话列表是时间相关查询：除 `status='active'` 外，还要检查 idle/absolute deadline 和当前 `user_auth_state.auth_revision`。`expired` 是撤销原因，不是 session status；清理任务可异步把惰性过期的 active row 转成 `status='revoked', revoked_reason='expired'`，但 API 在清理前也不能把它继续称为活动会话。重新认证只替换当前会话；改密、logout-all 与用户主动撤销 session 都要在事务内复核各自的稳定安全快照。普通 touch 会推进 session revision，因此全量退出和 actor 有效性复核不能把易变 revision 当作授权 CAS。

## 4. 服务器、设备、连接与 capability

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `servers` | `id,name,region,provider_name,provider_url,renew_at,status,connection_mode,callback_url,traffic_source,revision,deleted_at` | 名称可重复；callback需验证；status reason另表condition |
| `server_conditions` | `server_id,type,status,reason,message,since_ms,observed_at` | PK(server,type)；message安全脱敏 |
| `enrollment_grants` | `id,server_id,token_hmac,expires_at,max_uses,use_count,consumed_at,revoked_at,created_by` | 默认一次；原子 consume |
| `agent_devices` | `id,server_id,public_key,status,protocol_major,protocol_minor,agent_version,target_triple,first_seen,last_seen,revision` | 默认server一个active device；换机显式 revoke/adopt |
| `agent_certificates` | `id,device_id,serial,not_before,not_after,status,issued_at,revoked_at` | serial唯一；不保存device private key |
| `agent_sessions` | `id,device_id,transport,transport_epoch,owner_instance,owner_fencing,connected_at,last_heartbeat,disconnected_at,remote_addr,boot_id` | 每device最多一个 active owner（partial unique/事务） |
| `agent_capability_snapshots` | `id,device_id,observed_at,agent_version,core_version,core_hash,kernel_version,capabilities_json,snapshot_hash` | 不可变；最新通过索引查 |
| `server_system_snapshots` | `id,server_id,device_id,boot_id,sampled_at,cpu_json,memory_json,disk_json,load_json,uptime_sec` | sampled_at倒序索引；缺失字段null |
| `server_interfaces` | `id,server_id,boot_id,name,kind,is_physical,rx_bytes,tx_bytes,sampled_at` | 唯一(server,boot,name,sampled_at) |
| `server_port_observations` | `server_id,protocol,listen_addr,port,owner_process,observed_at` | 短期发现数据；不当配置事实源 |
| `discovered_services` | `id,server_id,kind,name,version,config_path,status,snapshot_hash,observed_at,claimed_at` | discovery/claim分离 |
| `managed_resources` | `id,server_id,kind,logical_id,path,ownership_marker,current_hash,created_at,deleted_at` | 安全删除/漂移；unique(server,kind,path) |

## 5. Durable jobs、workflows、outbox 与 audit

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `jobs` | `id,type,schema_version,target_type,target_id,state,priority,idempotency_key,desired_revision,payload_object_id,payload_hash,not_before,deadline,attempt,max_attempts,lease_owner,lease_token,lease_until,requested_by,reason,created_at,updated_at` | unique(type,target,idempotency)活跃/terminal策略；lease fencing |
| `job_attempts` | `id,job_id,attempt,device_id,session_id,lease_token,started_at,finished_at,result_code,error_code,result_object_id,log_object_id` | unique(job,attempt)；历史不覆盖 |
| `job_progress` | `job_id,attempt,sequence,percent,phase,message,occurred_at` | unique(job,attempt,sequence)；percent单调由应用校验 |
| `workflows` | `id,type,resource_type,resource_id,state,current_step,input_hash,created_by,created_at,updated_at` | 迁移/ACME/升级/分享清理等多步状态 |
| `workflow_steps` | `workflow_id,step_no,name,state,attempt,input_hash,result_object_id,started_at,finished_at` | unique(workflow,step_no)；补偿步骤显式 |
| `schedules` | `id,type,subject_type,subject_id,enabled,timezone,schedule_json,next_run_at,last_run_at,revision` | unique(type,subject)；时区显式 |
| `domain_outbox` | `id,aggregate_type,aggregate_id,aggregate_revision,event_type,schema_version,payload_json,occurred_at,available_at,attempt,locked_by,locked_until,dispatched_at` | unique(aggregate,event,revision)按事件定义 |
| `event_receipts` | `consumer,event_id,handled_at,result_hash` | PK(consumer,event)；consumer幂等 |
| `audit_logs` | `id,occurred_at,actor_type,actor_id,action,resource_type,resource_id,request_id,trace_id,ip_prefix,before_hash,after_hash,diff_json,reason,result` | append-only；DB权限禁止UPDATE/DELETE（保留策略单独归档） |
| `security_alerts` | `id,severity,type,subject_type,subject_id,status,first_seen,last_seen,count,details_json,ack_by,ack_at` | fingerprint去重；secret必须redact |

## 6. sing-box 制品、配置与 reported state

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `core_artifacts` | `id,core_type,version,channel,target_triple,sha256,size,source_url,signature_object_id,source_commit,status,created_at` | unique(type,version,target,hash)；source_url可空/离线 |
| `server_core_instances` | `id,server_id,name,mode,desired_artifact_id,reported_version,status,config_path,service_name,revision` | 默认 unique(server,name)；mode managed/external |
| `core_config_revisions` | `id,core_instance_id,revision,ir_object_id,compiled_object_id,compiled_sha256,target_version,created_by,created_at,apply_state,applied_at` | unique(instance,revision/hash)；不可变 |
| `core_reported_states` | `core_instance_id,observed_revision,observed_hash,process_state,health,api_version,started_at,observed_at,reason` | 每instance一行当前 projection |
| `core_config_diagnostics` | `id,config_revision_id,severity,path,code,message,source` | path指向IR字段；source compiler/core_check/health |
| `core_epochs` | `id,core_instance_id,device_boot_id,config_revision,core_started_at,first_sample_at,last_sample_at,closed_at` | 计数 reset 边界；组合唯一 |

## 7. 入站、用户凭据、节点、出站与路由

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `inbounds` | `id,server_id,core_instance_id,tag,protocol,listen_addr,listen_port,transport,security,settings_json,enabled,origin,owner_peer_id,revision,deleted_at` | unique(core,tag)和 active port策略；settings过schema |
| `inbound_principals` | `id,inbound_id,principal_id,credential_id,display_label,enabled,revision` | unique(inbound,principal)；label映射稳定 |
| `principal_credentials` | `id,principal_type,principal_id,protocol,public_id,secret_id,status,created_at,rotated_at,revoked_at` | public_id/UUID等与secret分离；按协议唯一策略 |
| `nodes` | `id,name,protocol,origin_kind,inbound_id,external_config_json,server_addr,server_port,enabled,sort_key,address_state_json,fingerprint_hmac,revision,deleted_at` | managed node inbound_id唯一；external无inbound |
| `tags` | `id,name,name_norm,color,created_at` | name_norm唯一 |
| `node_tags` | `node_id,tag_id` | 组合PK，cascade |
| `outbounds` | `id,server_id,core_instance_id,tag,type,settings_json,managed_kind,enabled,revision,deleted_at` | unique(core,tag)；system direct/block受保护 |
| `route_rules` | `id,server_id,core_instance_id,name,sort_key,conditions_json,action_json,scope,inbound_id,is_system,enabled,revision` | first-match稳定；系统API规则锁定 |
| `route_rule_refs` | `rule_id,ref_type,ref_id` | 编译前引用图，防悬空 |
| `balancers` | `id,server_id,tag,strategy,probe_url,interval_ms,tolerance_ms,enabled,revision` | tag唯一；strategy enum |
| `balancer_candidates` | `balancer_id,outbound_id,sort_key,weight` | 组合PK；至少一个active候选 |
| `balancer_runtime` | `balancer_id,selected_outbound_id,selection_seq,reason,observed_at,metrics_json` | reported projection，不当desired |
| `tunnels` | `id,server_id,name,listen_addr,listen_port,target_type,target_id,target_host,target_port,paired_node_id,enabled,revision` | target union校验；拓扑无环 |
| `routed_outbounds` | `id,owner_type,owner_id,parent_node_id,target_outbound_id,child_node_id,inbound_principal_id,status,status_reason,created_at,revision` | user/admin owner；child/credential lifecycle绑定 |
| `routed_outbound_quotas` | `user_id,max_active,daily_mutations,timezone,enabled,revision` | max不能魔法0；daily按timezone ledger |
| `routed_outbound_mutations` | `id,user_id,routed_id,action,occurred_at` | append-only配额计数 |
| `warp_accounts` | `id,server_id,credential_secret_id,account_type,status,config_object_id,revision` | 每server独立；WARP+ key secret分离 |

## 8. 套餐、entitlement 与策略

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `package_templates` | `id,name,description,status,current_revision,created_at,deleted_at` | name_norm可唯一；模板壳 |
| `package_template_revisions` | `id,template_id,revision,traffic_limit_bytes,period_kind,period_value,billing_direction,default_multiplier_num,default_multiplier_den,default_speed_mbps,default_max_connections,policy_json,created_by,created_at` | unique(template,revision)，不可变 |
| `package_template_nodes` | `template_revision_id,node_id,sort_key,multiplier_num,multiplier_den,speed_mbps,max_connections` | 逐节点覆盖 |
| `package_template_tags` | `template_revision_id,tag_id` | 动态选择条件 |
| `package_instances` | `id,user_id,template_id,template_revision_id,name,status,starts_at,expires_at,period_start,period_end,traffic_limit_bytes,baseline_id,revision,deleted_at` | 同用户多实例；独立周期/状态 |
| `package_instance_nodes` | `instance_id,node_id,source,sort_key,multiplier_num,multiplier_den,speed_mbps,max_connections` | effective snapshot；source explicit/tag/manual |
| `package_principals` | `instance_id,principal_id,subscription_credential_id` | 通常一实例一个principal，保留扩展 |
| `user_policy_overrides` | `id,user_id,node_id nullable,speed_mbps,max_connections,valid_from,valid_until,revision` | unique(user,node/null active)；node null为全局 |
| `effective_policy_snapshots` | `id,user_id,package_instance_id,node_id,policy_revision,speed_mbps,max_connections,sources_json,computed_at` | 调试/下发快照，可重建 |
| `enforcement_reports` | `id,server_id,device_id,policy_snapshot_id,status,executor,reason,applied_at,removed_at` | 不把desired当applied |
| `entitlement_snapshots` | `id,user_id,revision,nodes_hash,files_hash,packages_hash,computed_at` | 订阅cache/reconcile key，可重建 |

## 9. 原始计量、账本、连接与聚合

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `metric_batches` | `id,agent_device_id,boot_id,core_epoch_id,first_seq,last_seq,sampled_from,sampled_to,object_id,sha256,received_at` | unique(device,boot,first,last)防重复 |
| `counter_samples` | `id,source,server_id,core_epoch_id,dimension_type,dimension_key,direction,counter_value,sampled_at,sequence` | unique(source,epoch,dimension,direction,sequence) |
| `traffic_deltas` | `id,source_sample_id,server_id,principal_id,inbound_id,outbound_id,node_id,direction,bytes,interval_start,interval_end,attribution_status` | source sample + dimension唯一；未知principal可后补 |
| `traffic_ledger` | `id,source_delta_id,user_id,package_instance_id,node_id,billing_point,direction,raw_bytes,billed_bytes,multiplier_num,multiplier_den,policy_revision,occurred_at` | append-only；unique(source_delta,package,billing_point) |
| `traffic_adjustments` | `id,user_id,package_instance_id,signed_bytes,reason,created_by,occurred_at` | append-only，不改ledger |
| `traffic_baselines` | `id,subject_type,subject_id,source,epoch_or_counter_ref,baseline_bytes,reason,created_by,created_at` | reset事件；subject当前引用最新 |
| `traffic_periods` | `id,package_instance_id,starts_at,ends_at,status,baseline_id,raw_bytes,billed_bytes,closed_at` | 不重叠；历史周期不可改边界 |
| `traffic_hourly` | `bucket_start,timezone_key,user_id,package_id,node_id,source,direction,raw_bytes,billed_bytes,aggregate_version` | 查询projection，可重建 |
| `traffic_daily` | 同 hourly + `bucket_date` | 日账本/趋势；unique维度组合 |
| `connection_events` | `id,server_id,core_epoch_id,connection_id,event_seq,event_type,principal_id,inbound_id,outbound_id,source_addr,destination,protocol,uplink_delta,downlink_delta,occurred_at` | unique(epoch,connection,event_seq)；IP按保留策略 |
| `active_connection_projection` | `server_id,connection_id,principal_id,node_id,opened_at,last_update,uplink,downlink` | 可重建/TTL，close即删 |
| `enforcement_incidents` | `id,policy_snapshot_id,connection_id,type,action,reason,occurred_at` | 超连接/限速degraded等审计 |

## 10. 订阅文件、外部源、provider、模板与规则

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `subscription_files` | `id,name,description,kind,current_object_id,public,short_code,sort_key,traffic_limit_bytes,revision,deleted_at` | short_code唯一；kind normal/aggregate/generated |
| `subscription_file_grants` | `file_id,user_id,granted_by,created_at` | 组合PK |
| `subscription_aggregate_members` | `aggregate_id,member_file_id,sort_key` | 组合PK；DAG无环 |
| `external_sources` | `id,owner_user_id,name,url,enabled,interval_sec,next_sync_at,etag,last_modified,include_regex,exclude_regex,rename_policy,match_policy,sync_scope,traffic_sync,revision` | URL加密凭据独立；due索引 |
| `external_source_secrets` | `source_id,auth_secret_id` | 独立表避免列表误取secret |
| `external_sync_runs` | `id,source_id,trigger,state,started_at,finished_at,http_status,content_hash,candidate_count,created_count,updated_count,error_code,diagnostics_object_id` | 每次历史；last-good不被失败覆盖 |
| `node_selection_sessions` | `id,actor_id,source_type,source_id,candidates_object_id,expires_at,committed_at` | token HMAC或session owner；持久/过期 |
| `proxy_providers` | `id,name,source_type,source_id,url,interval_sec,cache_object_id,health_url,filters_json,overrides_json,revision,deleted_at` | name/发布路径唯一 |
| `provider_snapshots` | `id,provider_id,revision,object_id,node_count,source_hash,created_at,status` | last-good不可变 |
| `templates` | `id,name,version_kind,status,current_revision,owner_user_id,is_public,is_default,deleted_at` | default每kind最多一项 |
| `template_revisions` | `id,template_id,revision,schema_version,definition_json,definition_hash,created_by,created_at` | immutable；unique(template,revision/hash) |
| `rule_sources` | `id,name,source_type,url,current_object_id,format,enabled,interval_sec,next_sync_at,revision` | safe fetch/local object |
| `rule_source_snapshots` | `id,source_id,revision,object_id,rule_count,source_hash,status,created_at` | last-good |
| `rule_templates` | `id,name,definition_json,sort_key,enabled,revision` | 结构化规则选择器 |
| `publish_snapshots` | `id,subject_type,subject_id,file_id,format,input_revision_hash,output_object_id,etag,warnings_json,created_at,expires_at` | unique(subject,file,format,input hash) cache |
| `temporary_subscriptions` | `id,credential_id,node_selection_object_id,format,expires_at,max_uses,created_by,created_at` | credential控制访问；不在内存 |
| `transform_scripts` | `id,name,runtime,current_object_id,enabled,fuel_limit,memory_limit,time_limit_ms,network_allowed,revision` | 默认禁网；runtime allowlist |

## 11. 证书与站点

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `dns_provider_credentials` | `id,provider,name,secret_id,scope_json,created_by,revision` | 不在普通设置；test result另记 |
| `acme_accounts` | `id,directory_url,email,key_secret_id,status,terms_accepted_at,created_at` | directory+email唯一策略 |
| `certificate_orders` | `id,account_id,domains_json,dns_credential_id,state,current_step,not_before,not_after,last_error,next_retry,workflow_id,created_at` | domains canonical/hash去重 |
| `certificates` | `id,name,domains_json,cert_object_id,chain_object_id,key_secret_id,serial,not_before,not_after,status,source,revision,deleted_at` | key与cert match；serial/issuer追踪 |
| `certificate_deployments` | `id,certificate_id,server_id,target_kind,target_id,cert_path,key_path,mode,reload_action,desired_revision,status,reported_hash,reason` | unique(cert,target)；path ownership |
| `sites` | `id,server_id,name,kind,domains_json,listen_json,tls_certificate_id,static_artifact_id,upstream_json,status,revision,deleted_at` | typed static/reverse；端口/域冲突 |
| `site_artifacts` | `id,site_id,object_id,file_count,unpacked_bytes,created_by,created_at` | archive预算/内容寻址 |
| `site_deployments` | `id,site_id,desired_revision,rendered_object_id,rendered_hash,status,reported_hash,reason,applied_at` | nginx -t/apply/rollback状态 |

## 12. 测速、探针与公开 projection

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `tester_devices` | `id,name,agent_device_id,status,capabilities_json,last_seen,revision` | device role=tester；不能转服务器Agent |
| `speed_test_runs` | `id,requested_by,source_type,source_id,test_types_json,thread_count,state,created_at,started_at,finished_at,canceled_at` | parent workflow；预算/并发 |
| `speed_test_tasks` | `id,run_id,node_id,sort_key,state,job_id,started_at,finished_at,error_code` | unique(run,node,test type按payload) |
| `speed_test_results` | `id,task_id,node_id,source_id,latency_samples_json,latency_ms,download_bps,bytes,duration_ms,exit_ip,executor,executor_version,created_at` | immutable历史；IP保留策略 |
| `public_probe_configs` | `instance_id,enabled,origin_token_secret_id,refresh_sec,fields_json,branding_json,revision` | 每字段allowlist；license badge不存在 |
| `public_probe_servers` | `server_id,public_name,sort_key,visible,field_overrides_json,revision` | 只引用server，不复制secret |
| `probe_series` | `server_id,metric,bucket_start,bucket_width,value_num,value_den,sample_count,missing_count` | 公共预聚合，range/max points |

## 13. Telegram、通知与 MCP

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `telegram_configs` | `instance_id,enabled,bot_token_secret_id,bot_username,admin_ids_json,mini_app_url,revision` | 单实例；token加密 |
| `telegram_bindings` | `telegram_user_id,user_id,status,bound_at,revoked_at,last_auth_at` | TG ID/user active唯一 |
| `telegram_binding_codes` | `id,user_id,code_hmac,expires_at,used_at` | 一次性 |
| `invitation_codes` | `id,code_hmac,name,max_uses,use_count,expires_at,default_package_template_id,status,created_by` | 原子 consume，默认user role |
| `telegram_updates` | `update_id,received_at,handled_at,result_code` | Telegram重试去重 |
| `notification_preferences` | `user_id,event_type,channel,enabled,quiet_hours_json,revision` | 组合PK |
| `notification_deliveries` | `id,event_id,recipient_type,recipient_id,channel,state,dedupe_key,attempt,next_attempt,last_error,created_at,sent_at` | dedupe唯一；dead-letter |
| `mcp_intents` | `id,token_hmac,api_token_id,tool_name,args_hash,expires_at,consumed_at,created_at` | 高危两阶段，一次性 |
| `mcp_calls` | `id,api_token_id,session_id,tool_name,args_hash,result_code,audit_id,started_at,finished_at` | 不存敏感完整参数，必要时encrypted object |

## 14. 实例联合/分享服务器

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `instance_identities` | `instance_id,public_key,cert_secret_id,ca_pin,revision` | 无许可证；私钥secret |
| `federation_peers` | `id,remote_instance_id,name,base_url,ca_pin,status,protocol_version,last_seen,revision` | remote public ID唯一；URL/钉扎变更高危 |
| `share_grants` | `id,server_id,token_hmac,name,consumer_instance_id,scopes_json,tag_prefix,quota_json,expires_at,status,created_by,revision` | token hash；origin owner only；多个grant可撤销 |
| `accepted_shares` | `id,peer_id,remote_grant_id,remote_server_id,local_name,tag_prefix,status,projection_cursor,revision` | consumer projection；不可再分享 |
| `shared_resources` | `id,grant_id,consumer_instance_id,resource_type,origin_resource_id,consumer_resource_id,tag,status,created_at,deleted_at` | ownership边界；unique(grant,consumer resource) |
| `federation_requests` | `id,peer_id,request_id,method,resource_type,resource_id,args_hash,result_code,audit_id,occurred_at` | request ID去重/审计 |

## 15. 迁移与导入

| 表 | 关键列 | 约束/用途 |
|---|---|---|
| `import_sessions` | `id,kind,source_fingerprint,state,phase,backup_object_id,plan_object_id,started_by,started_at,finished_at,rollback_state` | 同 source active唯一；空库规则由precheck |
| `import_items` | `session_id,source_type,source_id,target_type,target_id,action,confidence,status,diagnostics_json` | 组合PK；每项映射/冲突可审计 |
| `import_checkpoints` | `session_id,phase,cursor,checksum,completed_at` | crash-resume；phase单调 |
| `legacy_id_mappings` | `source_system,source_type,source_id,target_type,target_id,created_at` | unique source identity；避免重复导入 |

## 16. 关键关系与删除策略

- `users → package_instances → package_principals → inbound_principals` 是服务端身份链；用户 email 改变不影响流量归属。
- `inbound ↔ managed node` 为一对一显式 FK，不以 tag/server:port猜测；external node没有 inbound。
- `server/core_config_revision` 是 desired history；`core_reported_state` 是当前观测，二者永不互相覆盖。
- package/user/node删除先进入 disabled/deleted状态并 reconcile 数据面；traffic/audit/import mapping不 cascade删除。
- certificate/site/core artifacts 使用 content references；业务软删后异步 GC，必须确认 `ref_count=0` 且超 retention。
- share resource ownership 始终指向 origin grant；accepted share 不能成为新的 `share_grants.server_id` 来源。

## 17. 索引、分区和保留

最低索引：所有 FK；`status/next_due` jobs/schedules/sources/certs；user/package/node traffic维度+time；public credential HMAC；list pages `(sort_key,id)`；audit `(resource_type,resource_id,occurred_at)` 和 `(actor,occurred_at)`。

PG 大规模模式按月分区 `counter_samples/traffic_deltas/traffic_ledger/connection_events/audit_logs`；SQLite 使用时间索引和批量归档。默认保留建议：

| 数据 | 默认 |
|---|---|
| raw connection source IP | 7 天（可缩至0）；聚合保留 |
| raw counter samples | 30 天 |
| traffic ledger/daily | ledger 24个月，daily长期/管理员配置 |
| speed history | 90 天 |
| system metrics | raw 7天、5分钟聚合30天、日聚合1年 |
| job progress/log object | 30 天；terminal summary长期 |
| audit/security | 1 年，支持归档签名；不能普通UI清空 |
| revoked sessions/tokens | 90 天或到原absolute expiry后30天 |

清理任务每批有上限、按 owned IDs 删除、记录 audit/metrics；从不接收宽泛路径或用户输入 glob。

## 18. 数据库验收

1. SQLite/PG 从零 migration 后 schema contract 等价；所有 FK/unique/check实测。
2. 每个 repository 同一 contract suite，包含并发 revision、lease fencing、idempotency、pagination稳定性。
3. 10k users、50k nodes、1亿 ledger rows（PG）和小型 SQLite基准有 query plan/延迟预算。
4. crash injection 验证 outbox、workflow、config apply、metric ingestion 不产生半状态/重复计费。
5. backup→损坏/缺文件/hash错→dry-run拒绝；正常恢复后 domain checksum、对象引用和secret canary一致。
6. 数据匿名/删除流程不破坏财务/流量/audit引用，并输出可验证报告。
