# TypeScript/TSX 源码符号总览

> 使用 TypeScript 5.9 AST 自动生成。公开版只保留符号、行号、原创作用说明、调用和控制流证据，不公开源码签名、常量字面量或表达式正文。

- 文件：135
- 函数/方法/闭包：3263
- 其他顶层声明：403
- 检测为 React 组件的函数：511
- 自定义 Hook：177
- TanStack 路由：24
- 静态可识别的 /api 调用：225

| 分区 | 文件数 | 函数数 | 作用 | 详细索引 |
|---|---:|---:|---|---|
| `components` | 20 | 424 | 跨页面复用的业务组件和交互对话框。 | [components](areas/components.md) |
| `components-layout` | 3 | 30 | 导航栏、顶栏、用户菜单和应用外壳。 | [components-layout](areas/components-layout.md) |
| `components-template-v3` | 6 | 103 | V3 模板编辑、预览、筛选与代理组控件。 | [components-template-v3](areas/components-template-v3.md) |
| `components-ui` | 33 | 198 | 基于 Radix UI/Tailwind 的无业务基础 UI 封装。 | [components-ui](areas/components-ui.md) |
| `config` | 3 | 0 | 前端预设、字体和覆写脚本模板。 | [config](areas/config.md) |
| `context` | 3 | 26 | 主题、字体、方向等 React Context。 | [context](areas/context.md) |
| `hooks` | 7 | 44 | 可复用 React Hook、响应式状态和拖拽/媒体查询行为。 | [hooks](areas/hooks.md) |
| `lib` | 32 | 289 | API 客户端、Clash/订阅构建、校验、格式化和通用工具。 | [lib](areas/lib.md) |
| `root` | 3 | 26 | 应用入口、生成路由树和顶层类型。 | [root](areas/root.md) |
| `routes` | 24 | 2118 | TanStack Router 页面、加载/重定向守卫和页面内业务交互。 | [routes](areas/routes.md) |
| `stores` | 1 | 5 | Zustand 全局状态，主要承载认证会话。 | [stores](areas/stores.md) |
