# 第三方资料与权利边界

NodeControll 是从零实现的新系统，不将研究时克隆的妙妙屋或妙妙屋 X 文件打包进程序、容器或 Actions 制品。

## 妙妙屋

`https://github.com/iluobei/miaomiaowu` 的锁定提交仅作为行为研究输入。`upstream/` 在 Git 中忽略；公开仓库中的源码解剖文档由本项目生成，只保留结构性事实（模块/函数标识、行号、调用/控制流统计、路由和 schema 元数据）及原创说明。生成器不会公开源码签名、常量字面量、完整 SQL 或路由注册语句；原始审计快照不随仓库分发。审阅基线未发现覆盖整个上游仓库的根许可证，因此本项目的 AGPL 许可证不对该上游代码作任何授权表示。

## 妙妙屋 X 文档

`https://miaomiaowux.com/docs/` 的 58 页中文文档用于建立功能与 PRO 差异基线。公开仓库只保留官方 URL、抓取时间、页面哈希、短标题/description 元数据、标题树和本项目的原创分析；完整 HTML 与抽取正文不在仓库分发。页面内容与商标仍归其各自权利人。

## 依赖与编译制品

Rust、JavaScript、容器镜像、sing-box 及其他第三方组件继续适用各自许可证。Actions artifact 是开发期编译证据，不是正式 release，但仍会附带从锁定依赖图生成的 `notices/`：CycloneDX 组件清单、机器可读依赖/许可证索引、依赖包中实际提供的许可证/notice 文件及 checksum。

依赖包归档若没有实际 license/notice 文件，只能使用 `third_party/dependency-license-overrides/overrides.json` 中与 ecosystem、包名、版本和声明表达式精确匹配的审阅记录。每份本地证据固定来源仓库、revision、上游路径、字节数和 SHA-256；缺失、hash 不符、重复、过期或未使用的 entry/file 都使工作流失败。规范许可证文本与上游声明证据会按其实际类型标注，不冒充包归档自带 LICENSE，也不再用自动生成的 declaration notice 通过分发门。

Rust binary 静态链接的标准库不在 Cargo package 图中。收集器另外从固定 Rust 1.98.0 sysroot 复制 Rust README、standard-library copyright 汇编和配套 license texts，作为独立 toolchain runtime component 纳入 inventory、SBOM 与 checksum。glibc、动态 loader 和 `libgcc_s` 由目标系统按 ELF allowlist 动态提供，不随当前 tar 分发；容器/系统包的完整 notices 要在相应 OCI/native 交付真正加入时另行生成。`BUILD-METADATA` 同时记录精确提交和公开对应源码地址。
