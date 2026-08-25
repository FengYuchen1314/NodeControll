# Dependency license overrides

锁定依赖的安装归档如果没有携带 license、copying、notice 或 copyright 文件，Actions 不能只凭包管理器中的许可证字符串继续打包。此目录保存逐版本人工复核的替代证据，供 `tools/collect_third_party_licenses.mjs` 在构建时严格校验和复制。

`overrides.json` 以 ecosystem、包名和版本为唯一键，同时记录声明的许可证表达式、registry checksum/integrity、来源仓库、revision、版本证据，以及每个本地文件的字节数和 SHA-256。`resolution` 会明确说明证据不是来自精确发布归档的情况，例如后来补入的项目 LICENSE、同 revision 的 monorepo sibling、规范 SPDX 文本或上游文件中的许可证通知。规范文本不会被标成包归档自带文件。

维护规则：

1. 依赖版本变化后重新检查新归档；不得把旧版本 entry 模糊匹配到新版本。
2. 优先使用发布 tag/registry gitHead 对应 revision 中的完整许可证与版权通知；没有时记录可复核的版本证据和局限。
3. 新增或替换文件后更新字节数与 SHA-256。不要保存依赖源码全文；只有许可证、版权通知和最小版本/声明证据可进入本目录。
4. 不同 entry 只有在完整 file spec 完全相同（包括 kind、上游仓库/revision/tag/path、SHA-256、字节数及其他字段）时，才可显式共用同一 `localPath`；任何来源或内容差异都必须使用独立文件。
5. 收集器必须对缺失、hash 不符、重复、过期或未使用的 entry/file fail closed。构建输出不得用自动生成的 declaration notice 冒充许可证正文。
6. 此目录是再分发审计材料，不是法律意见，也不改变任何第三方许可证。
