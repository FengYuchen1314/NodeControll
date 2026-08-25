# TypeScript 分区 `lib`

API 客户端、Clash/订阅构建、校验、格式化和通用工具。

## `lib/api.ts`

依赖：`axios`、`@/stores/auth-store`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–4 | const | `AUTH_HEADER` | 保存 'AUTH_HEADER' 的模块级常量、配置、路由或预计算值。 |  |
| 5–5 | const | `rawConfiguredBaseURL` | 保存 'rawConfiguredBaseURL' 的模块级常量、配置、路由或预计算值。 |  |
| 6–9 | const | `configuredBaseURL` | 保存 'configuredBaseURL' 的模块级常量、配置、路由或预计算值。 |  |
| 11–14 | const | `api` | 保存 'api' 的模块级常量、配置、路由或预计算值。 |  |
| 25–32 | function | `api.interceptors.request.use.callback#1` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'useAuthStore.getState' |
| 35–35 | function | `api.interceptors.response.use.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 36–52 | function | `api.interceptors.response.use.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 5；循环 0；返回 1；await 0；调用 'Promise.reject'、'useAuthStore.getState'、'useAuthStore.getState.auth.reset' |

## `lib/clash-validator.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–6 | type | `ValidationLevel` | 定义 'ValidationLevel' 的数据契约、联合类型或组件属性。 |  |
| 8–14 | interface | `ValidationIssue` | 定义 'ValidationIssue' 的数据契约、联合类型或组件属性。 |  |
| 16–20 | interface | `ValidationResult` | 定义 'ValidationResult' 的数据契约、联合类型或组件属性。 |  |
| 25–58 | function | `validateClashConfig` | 校验与 'validateClashConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 'JSON.parse'、'JSON.stringify'、'detectCircularReferences'、'issues.push'、'issues.some'、'validateProxies'、'validateProxyGroups' |
| 51–51 | function | `validateClashConfig > issues.some.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 56–56 | function | `validateClashConfig > issues.some.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 63–130 | function | `validateProxies` | 校验与 'validateProxies' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 1；返回 1；await 0；调用 'Object.keys'、'fixed.push'、'issues.push'、'proxy.name.trim'、'reorderProxyFields'、'seenNames.add'、'seenNames.has' |
| 135–293 | function | `validateProxyGroups` | 校验与 'validateProxyGroups' 对应的前端业务、状态或数据转换逻辑。 | 分支 12；循环 2；返回 1；await 0；调用 'Array.isArray'、'Object.keys'、'fixed.push'、'group.filter.trim'、'group.name.trim'、'groupNames.has'、'groups.map'、'groups.map.filter'、'issues.push'、'proxies.map'、'proxyNames.has'、'reorderGroupFields'、'seenNames.add'、'seenNames.has'、'specialNodes.has'、'uniqueProxies.add'、'uniqueProxies.has'、'validProxies.push' |
| 142–142 | function | `validateProxyGroups > proxies.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 143–143 | function | `validateProxyGroups > groups.map.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 298–349 | function | `detectCircularReferences` | 执行与 'detectCircularReferences' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 2；返回 1；await 0；调用 '<BinaryExpression>.filter'、'<BinaryExpression>.filter.filter'、'groupMap.set'、'hasCycle'、'visited.has' |
| 306–306 | function | `detectCircularReferences > <BinaryExpression>.filter.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 307–307 | function | `detectCircularReferences > <BinaryExpression>.filter.filter.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'groups.some' |
| 307–307 | function | `detectCircularReferences > <BinaryExpression>.filter.filter.callback#10 > groups.some.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 312–339 | function | `detectCircularReferences > hasCycle` | 判断是否具有与 'hasCycle' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 1；返回 3；await 0；调用 '<ArrayLiteralExpression>.join'、'groupMap.get'、'hasCycle'、'issues.push'、'path.indexOf'、'path.pop'、'path.push'、'path.slice'、'recStack.add'、'recStack.delete'、'recStack.has'、'visited.add'、'visited.has' |
| 354–373 | function | `reorderProxyFields` | 执行与 'reorderProxyFields' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 2；返回 1；await 0；调用 'Object.entries'、'priorityKeys.includes' |
| 378–397 | function | `reorderGroupFields` | 执行与 'reorderGroupFields' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 2；返回 1；await 0；调用 'Object.entries'、'priorityKeys.includes' |
| 402–499 | function | `formatValidationIssues` | 格式化与 'formatValidationIssues' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 0；返回 2；await 0；调用 'formatGroupedIssues'、'issues.filter' |
| 407–407 | function | `formatValidationIssues > issues.filter.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 408–408 | function | `formatValidationIssues > issues.filter.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 409–409 | function | `formatValidationIssues > issues.filter.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 414–416 | function | `formatValidationIssues > extractPattern` | 执行与 'extractPattern' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'msg.replace' |
| 419–422 | function | `formatValidationIssues > extractName` | 执行与 'extractName' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'msg.match' |
| 425–480 | function | `formatValidationIssues > formatGroupedIssues` | 格式化与 'formatGroupedIssues' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'grouped.forEach'、'issueList.forEach' |
| 429–435 | function | `formatValidationIssues > formatGroupedIssues > issueList.forEach.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 '<NonNullExpression>.push'、'extractPattern'、'grouped.get'、'grouped.has'、'grouped.set' |
| 440–477 | function | `formatValidationIssues > formatGroupedIssues > grouped.forEach.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 5；循环 0；返回 0；await 0；调用 'baseMessage.includes'、'displayNames.join'、'items.map'、'items.map.filter'、'names.slice'、'pattern.replace' |
| 452–452 | function | `formatValidationIssues > formatGroupedIssues > grouped.forEach.callback#23 > items.map.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'extractName' |

## `lib/cookies.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–6 | const | `DEFAULT_MAX_AGE` | 保存 'DEFAULT_MAX_AGE' 的模块级常量、配置、路由或预计算值。 |  |
| 11–21 | function | `getCookie` | 读取或计算与 'getCookie' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 'parts.pop'、'parts.pop.split'、'parts.pop.split.shift'、'value.split' |
| 26–34 | function | `setCookie` | 设置与 'setCookie' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0 |
| 39–43 | function | `removeCookie` | 移除与 'removeCookie' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0 |

## `lib/country-flag.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 7–21 | function | `countryCodeToFlag` | 执行与 'countryCodeToFlag' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'String.fromCodePoint'、'countryCode.toUpperCase'、'countryCode.toUpperCase.split'、'countryCode.toUpperCase.split.map' |
| 18–18 | function | `countryCodeToFlag > countryCode.toUpperCase.split.map.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'char.charCodeAt' |
| 27–38 | function | `flagToCountryCode` | 执行与 'flagToCountryCode' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 4；await 0；调用 '<ArrayLiteralExpression>.map'、'codePoints.every'、'codePoints.map'、'codePoints.map.join' |
| 30–30 | function | `flagToCountryCode > <ArrayLiteralExpression>.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'char.codePointAt' |
| 34–34 | function | `flagToCountryCode > isRegionalIndicator` | 判断与 'isRegionalIndicator' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 37–37 | function | `flagToCountryCode > codePoints.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'String.fromCharCode' |
| 43–55 | function | `extractRegionFromNodeName` | 执行与 'extractRegionFromNodeName' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 4；await 0；调用 'flagToCountryCode'、'nodeName.match' |
| 60–73 | const | `REGION_GROUP_MAP` | 保存 'REGION_GROUP_MAP' 的模块级常量、配置、路由或预计算值。 |  |
| 78–91 | const | `COUNTRY_TO_GROUP_MAP` | 保存 'COUNTRY_TO_GROUP_MAP' 的模块级常量、配置、路由或预计算值。 |  |
| 96–98 | function | `findRegionGroupName` | 执行与 'findRegionGroupName' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'countryCode.toUpperCase' |
| 103–105 | function | `stripFlagEmoji` | 执行与 'stripFlagEmoji' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'name.replace' |
| 110–177 | const | `FLAG_OPTIONS` | 保存 'FLAG_OPTIONS' 的模块级常量、配置、路由或预计算值。 |  |
| 183–194 | function | `hasEmojiPrefix` | 判断是否具有与 'hasEmojiPrefix' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'emojiRegex.test' |
| 200–207 | function | `hasRegionEmoji` | 判断是否具有与 'hasRegionEmoji' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'regionEmojiRegex.test' |
| 212–221 | interface | `GeoIPInfo` | 定义 'GeoIPInfo' 的数据契约、联合类型或组件属性。 |  |
| 223–223 | const | `IPINFO_TOKEN` | 保存 'IPINFO_TOKEN' 的模块级常量、配置、路由或预计算值。 |  |
| 225–236 | function | `getGeoIPInfo` | 读取或计算与 'getGeoIPInfo' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'fetch'、'ip.replace'、'response.json' |

## `lib/handle-server-error.ts`

依赖：`axios`、`sonner`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–37 | function | `handleServerError` | 处理与 'handleServerError' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 1；返回 0；await 0；调用 'Number'、'console.log'、'toast.error'、'value.trim' |

## `lib/profile.ts`

依赖：`@/lib/api`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–10 | interface | `ProfileResponse` | 定义 'ProfileResponse' 的数据契约、联合类型或组件属性。 |  |
| 12–15 | function | `profileQueryFn` | 执行与 'profileQueryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |

## `lib/proxy-types.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–16 | interface | `ProxyNode` | 定义 'ProxyNode' 的数据契约、联合类型或组件属性。 |  |
| 19–25 | interface | `ClashProxy` | 定义 'ClashProxy' 的数据契约、联合类型或组件属性。 |  |

## `lib/sublink/clash-builder.ts`

依赖：`./types`、`./utils`、`./clash-config`、`./predefined-rules`、`./translations`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–410 | class | `ClashConfigBuilder` | 封装 'ClashConfigBuilder' 的实例状态与行为。 |  |
| 13–24 | function | `<anonymous#1>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 '<ThisKeyword>.convertLegacyCategories'、'categories.map'、'deepCopy' |
| 23–23 | function | `<anonymous#1> > categories.map.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 30–58 | function | `convertLegacyCategories` | 转换与 'convertLegacyCategories' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'RULE_CATEGORIES.map' |
| 31–57 | function | `convertLegacyCategories > RULE_CATEGORIES.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'category.ip_rules.map'、'category.site_rules.map'、'translateOutbound' |
| 39–47 | function | `convertLegacyCategories > RULE_CATEGORIES.map.callback#4 > category.site_rules.map.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 48–56 | function | `convertLegacyCategories > RULE_CATEGORIES.map.callback#4 > category.ip_rules.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 60–84 | function | `build` | 构建与 'build' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 1；返回 1；await 0；调用 '<ThisKeyword>.buildProxyGroups'、'<ThisKeyword>.buildRuleProviders'、'<ThisKeyword>.buildRules'、'<ThisKeyword>.convertProxies'、'<ThisKeyword>.toYAML'、'Object.entries' |
| 86–90 | function | `convertProxies` | 转换与 'convertProxies' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '<ThisKeyword>.proxyConfigs.map' |
| 88–88 | function | `convertProxies > <ThisKeyword>.proxyConfigs.map.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 '<ThisKeyword>.reorderProxyFields' |
| 93–112 | function | `reorderProxyFields` | 执行与 'reorderProxyFields' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 2；返回 1；await 0；调用 'Object.entries'、'priorityKeys.includes' |
| 113–155 | function | `buildRuleProviders` | 构建与 'buildRuleProviders' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 3；返回 0；await 0；调用 '<ThisKeyword>.categoryMap.get'、'console.warn' |
| 157–258 | function | `buildProxyGroups` | 构建与 'buildProxyGroups' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 2；返回 0；await 0；调用 '<ThisKeyword>.categoryMap.get'、'<ThisKeyword>.proxies.map'、'groups.push'、'translateOutbound' |
| 158–158 | function | `buildProxyGroups > <ThisKeyword>.proxies.map.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 260–336 | function | `buildRules` | 构建与 'buildRules' 对应的前端业务、状态或数据转换逻辑。 | 分支 9；循环 6；返回 0；await 0；调用 '<ThisKeyword>.categoryMap.get'、'rule.domain_keyword.split'、'rule.domain_keyword.split.forEach'、'rule.domain_suffix.split'、'rule.domain_suffix.split.forEach'、'rule.ip_cidr.split'、'rule.ip_cidr.split.forEach'、'rules.push'、'translateOutbound' |
| 270–273 | function | `buildRules > rule.domain_suffix.split.forEach.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'domain.trim'、'rules.push' |
| 277–280 | function | `buildRules > rule.domain_keyword.split.forEach.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'keyword.trim'、'rules.push' |
| 308–311 | function | `buildRules > rule.ip_cidr.split.forEach.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'cidr.trim'、'rules.push' |
| 338–387 | function | `toYAML` | 执行与 'toYAML' 对应的前端业务、状态或数据转换逻辑。 | 分支 11；循环 3；返回 1；await 0；调用 '<StringLiteral>.repeat'、'<ThisKeyword>.formatValue'、'<ThisKeyword>.toYAML'、'Array.isArray'、'Object.entries'、'Object.entries.filter'、'entries.slice' |
| 345–345 | function | `toYAML > Object.entries.filter.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 389–409 | function | `formatValue` | 格式化与 'formatValue' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 5；await 0；调用 'String'、'value.includes'、'value.startsWith' |

## `lib/sublink/clash-config.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 2–43 | const | `DEFAULT_CLASH_CONFIG` | 保存 'DEFAULT_CLASH_CONFIG' 的模块级常量、配置、路由或预计算值。 |  |
| 46–47 | const | `CLASH_SITE_RULE_SET_BASE_URL` | 保存 'CLASH_SITE_RULE_SET_BASE_URL' 的模块级常量、配置、路由或预计算值。 |  |
| 48–49 | const | `CLASH_IP_RULE_SET_BASE_URL` | 保存 'CLASH_IP_RULE_SET_BASE_URL' 的模块级常量、配置、路由或预计算值。 |  |

## `lib/sublink/predefined-rules.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–10 | interface | `RuleCategory` | 定义 'RuleCategory' 的数据契约、联合类型或组件属性。 |  |
| 12–139 | const | `RULE_CATEGORIES` | 保存 'RULE_CATEGORIES' 的模块级常量、配置、路由或预计算值。 |  |
| 145–181 | function | `buildCustomRulesFromCategories` | 构建与 'buildCustomRulesFromCategories' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 3；返回 1；await 0；调用 'RULE_CATEGORIES.find'、'ipRule.toUpperCase'、'rules.push'、'rules.some'、'selectedCategories.includes' |
| 149–149 | function | `buildCustomRulesFromCategories > RULE_CATEGORIES.find.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 175–175 | function | `buildCustomRulesFromCategories > rules.some.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'r.startsWith' |
| 187–192 | const | `PREDEFINED_RULE_SETS` | 保存 'PREDEFINED_RULE_SETS' 的模块级常量、配置、路由或预计算值。 |  |
| 191–191 | function | `RULE_CATEGORIES.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |

## `lib/sublink/proxy-groups.ts`

依赖：`@/lib/api`、`@/lib/sublink/types`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–16 | function | `fetchProxyGroupCategories` | 从后端获取与 'fetchProxyGroupCategories' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get'、'console.error' |
| 23–34 | function | `syncProxyGroupCategories` | 执行与 'syncProxyGroupCategories' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.post'、'console.error' |
| 42–50 | function | `filterCategoriesByPreset` | 筛选与 'filterCategoriesByPreset' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'categories.filter' |
| 49–49 | function | `filterCategoriesByPreset > categories.filter.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'category.presets.includes' |
| 57–61 | function | `createCategoryNameMap` | 创建与 'createCategoryNameMap' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'categories.map' |
| 60–60 | function | `createCategoryNameMap > categories.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 68–81 | function | `createPresetMap` | 创建与 'createPresetMap' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 2；返回 1；await 0；调用 'map[<key>].push' |
| 89–99 | function | `getCategoryNamesForPreset` | 读取或计算与 'getCategoryNamesForPreset' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'categories.filter'、'categories.filter.map' |
| 97–97 | function | `getCategoryNamesForPreset > categories.filter.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'category.presets.includes' |
| 98–98 | function | `getCategoryNamesForPreset > categories.filter.map.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 107–115 | function | `extractGroupLabels` | 执行与 'extractGroupLabels' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 1；返回 1；await 0；调用 'Array.from'、'labels.add' |

## `lib/sublink/translations.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–26 | const | `OUTBOUND_NAMES` | 保存 'OUTBOUND_NAMES' 的模块级常量、配置、路由或预计算值。 |  |
| 29–48 | const | `CATEGORY_TO_RULE_NAME` | 保存 'CATEGORY_TO_RULE_NAME' 的模块级常量、配置、路由或预计算值。 |  |
| 50–52 | function | `translateOutbound` | 执行与 'translateOutbound' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |

## `lib/sublink/types.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 2–6 | interface | `KanbanObject` | 定义 'KanbanObject' 的数据契约、联合类型或组件属性。 |  |
| 8–12 | interface | `KanbanFeatrure` | 定义 'KanbanFeatrure' 的数据契约、联合类型或组件属性。 |  |
| 15–34 | interface | `ProxyConfig` | 定义 'ProxyConfig' 的数据契约、联合类型或组件属性。 |  |
| 36–41 | interface | `TlsConfig` | 定义 'TlsConfig' 的数据契约、联合类型或组件属性。 |  |
| 43–49 | interface | `TransportConfig` | 定义 'TransportConfig' 的数据契约、联合类型或组件属性。 |  |
| 51–59 | interface | `CustomRule` | 定义 'CustomRule' 的数据契约、联合类型或组件属性。 |  |
| 61–78 | interface | `ClashProxy` | 定义 'ClashProxy' 的数据契约、联合类型或组件属性。 |  |
| 80–85 | interface | `ClashConfig` | 定义 'ClashConfig' 的数据契约、联合类型或组件属性。 |  |
| 87–91 | interface | `RuleSet` | 定义 'RuleSet' 的数据契约、联合类型或组件属性。 |  |
| 93–93 | type | `PredefinedRuleSetType` | 定义 'PredefinedRuleSetType' 的数据契约、联合类型或组件属性。 |  |
| 95–100 | interface | `GeneratedLinks` | 定义 'GeneratedLinks' 的数据契约、联合类型或组件属性。 |  |
| 103–111 | interface | `RuleProviderConfig` | 定义 'RuleProviderConfig' 的数据契约、联合类型或组件属性。 |  |
| 114–124 | interface | `ProxyGroupCategory` | 定义 'ProxyGroupCategory' 的数据契约、联合类型或组件属性。 |  |

## `lib/sublink/utils.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–15 | function | `decodeBase64` | 执行与 'decodeBase64' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 2；await 0；调用 'atob'、'atob.split'、'atob.split.map'、'atob.split.map.join'、'console.error'、'decodeURIComponent' |
| 8–8 | function | `decodeBase64 > atob.split.map.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 '<BinaryExpression>.slice'、'c.charCodeAt'、'c.charCodeAt.toString' |
| 17–28 | function | `encodeBase64` | 执行与 'encodeBase64' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 2；await 0；调用 'btoa'、'console.error'、'encodeURIComponent'、'encodeURIComponent.replace' |
| 20–21 | function | `encodeBase64 > encodeURIComponent.replace.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'String.fromCharCode'、'parseInt' |
| 30–37 | function | `base64ToBinary` | 执行与 'base64ToBinary' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 2；await 0；调用 'atob'、'base64.replace'、'base64.replace.replace'、'console.error' |
| 39–46 | function | `generateRandomPath` | 生成与 'generateRandomPath' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 1；返回 1；await 0；调用 'Math.floor'、'Math.random'、'chars.charAt' |
| 48–54 | function | `formatBytes` | 格式化与 'formatBytes' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'Math.floor'、'Math.log'、'Math.pow'、'Math.round' |
| 56–67 | function | `parseServerInfo` | 解析与 'parseServerInfo' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'parseInt'、'serverInfo.match'、'serverInfo.split' |
| 69–76 | function | `parseUrlParams` | 解析与 'parseUrlParams' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'urlObj.searchParams.forEach' |
| 72–74 | function | `parseUrlParams > urlObj.searchParams.forEach.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 78–88 | function | `createTlsConfig` | 创建与 'createTlsConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'params.alpn.split' |
| 90–115 | function | `createTransportConfig` | 创建与 'createTransportConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 4；await 0 |
| 117–119 | function | `deepCopy` | 执行与 'deepCopy' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'JSON.parse'、'JSON.stringify' |

## `lib/substore/producers/clash.ts`

依赖：`sonner`、`./utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–4 | type | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 6–9 | interface | `ProduceOptions` | 定义 'ProduceOptions' 的数据契约、联合类型或组件属性。 |  |
| 11–14 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 16–220 | function | `Clash_Producer` | 执行与 'Clash_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 18–218 | function | `Clash_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'list.map'、'list.map.join'、'proxies.filter'、'proxies.filter.map' |
| 20–64 | function | `Clash_Producer > produce > proxies.filter.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 4；await 0；调用 '<ArrayLiteralExpression>.includes'、'toast' |
| 65–211 | function | `Clash_Producer > produce > proxies.filter.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 26；循环 1；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'Array.isArray'、'isPresent'、'parseInt'、'reg.exec' |
| 216–216 | function | `Clash_Producer > produce > list.map.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'JSON.stringify' |

## `lib/substore/producers/clashmeta.ts`

依赖：`@/lib/substore/producers/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–3 | type | `IpVersion` | 定义 'IpVersion' 的数据契约、联合类型或组件属性。 |  |
| 4–4 | type | `ClashIpVersion` | 定义 'ClashIpVersion' 的数据契约、联合类型或组件属性。 |  |
| 6–12 | const | `ipVersions` | 保存 'ipVersions' 的模块级常量、配置、路由或预计算值。 |  |
| 14–17 | interface | `ProduceOptions` | 定义 'ProduceOptions' 的数据契约、联合类型或组件属性。 |  |
| 19–84 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 86–89 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 91–324 | function | `ClashMeta_Producer` | 执行与 'ClashMeta_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 93–322 | function | `ClashMeta_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'list.map'、'list.map.join'、'proxies.filter'、'proxies.filter.map' |
| 95–153 | function | `ClashMeta_Producer > produce > proxies.filter.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 6；循环 0；返回 7；await 0；调用 '<ArrayLiteralExpression>.includes' |
| 154–317 | function | `ClashMeta_Producer > produce > proxies.filter.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 40；循环 1；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'Array.isArray'、'isPresent'、'parseInt'、'reg.exec' |
| 321–321 | function | `ClashMeta_Producer > produce > list.map.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'JSON.stringify' |

## `lib/substore/producers/egern.ts`

依赖：`@/lib/substore/producers/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–55 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 57–60 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 62–510 | function | `Egern_Producer` | 执行与 'Egern_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 64–508 | function | `Egern_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'list.map'、'list.map.join'、'proxies.filter'、'proxies.filter.map' |
| 67–136 | function | `Egern_Producer > produce > proxies.filter.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 '<ArrayLiteralExpression>.includes' |
| 137–501 | function | `Egern_Producer > produce > proxies.filter.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 53；循环 2；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'Array.isArray'、'Object.keys'、'Object.values'、'Object.values.every'、'isPresent' |
| 476–476 | function | `Egern_Producer > produce > proxies.filter.map.callback#4 > Object.values.every.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 506–506 | function | `Egern_Producer > produce > list.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'JSON.stringify' |

## `lib/substore/producers/index.ts`

依赖：`./surge`、`./surgemac`、`./clash`、`./clashmeta`、`./stash`、`./loon`、`./uri`、`./v2ray`、`./qx`、`./shadowrocket`、`./surfboard`、`./sing-box`、`./egern`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–20 | function | `JSON_Producer` | 执行与 'JSON_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 17–18 | function | `JSON_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'JSON.stringify' |

## `lib/substore/producers/loon.ts`

依赖：`@/lib/substore/producers/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 2–2 | const | `targetPlatform` | 保存 'targetPlatform' 的模块级常量、配置、路由或预计算值。 |  |
| 5–11 | const | `ipVersions` | 保存 'ipVersions' 的模块级常量、配置、路由或预计算值。 |  |
| 13–69 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 71–73 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 75–110 | function | `Loon_Producer` | 执行与 'Loon_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 76–108 | function | `Loon_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 10；await 0；调用 'anytls'、'http'、'hysteria2'、'shadowsocks'、'shadowsocksr'、'socks5'、'trojan'、'vless'、'vmess'、'wireguard' |
| 112–229 | function | `shadowsocks` | 执行与 'shadowsocks' 对应的前端业务、状态或数据转换逻辑。 | 分支 14；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'<NonNullExpression>.startsWith'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 231–306 | function | `shadowsocksr` | 执行与 'shadowsocksr' 对应的前端业务、状态或数据转换逻辑。 | 分支 9；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 308–368 | function | `trojan` | 执行与 'trojan' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 0；返回 1；await 0；调用 'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 370–467 | function | `vmess` | 执行与 'vmess' 对应的前端业务、状态或数据转换逻辑。 | 分支 12；循环 0；返回 1；await 0；调用 'Array.isArray'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 469–571 | function | `vless` | 执行与 'vless' 对应的前端业务、状态或数据转换逻辑。 | 分支 13；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'Array.isArray'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 573–603 | function | `http` | 执行与 'http' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 'result.append'、'result.appendIfPresent'、'result.toString' |
| 604–640 | function | `socks5` | 执行与 'socks5' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 'result.append'、'result.appendIfPresent'、'result.toString' |
| 642–714 | function | `wireguard` | 执行与 'wireguard' 对应的前端业务、状态或数据转换逻辑。 | 分支 10；循环 0；返回 1；await 0；调用 'Array.isArray'、'proxy.dns.find'、'proxy.reserved.join'、'proxy[<key>].join'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 668–668 | function | `wireguard > proxy.dns.find.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'isIPv6' |
| 669–669 | function | `wireguard > proxy.dns.find.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'isIPv4' |
| 671–671 | function | `wireguard > proxy.dns.find.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'isIPv4'、'isIPv6' |
| 716–770 | function | `hysteria2` | 执行与 'hysteria2' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 '<TemplateExpression>.match'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 772–809 | function | `anytls` | 执行与 'anytls' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 1；返回 1；await 0；调用 'Number.isInteger'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |

## `lib/substore/producers/qx.ts`

依赖：`@/lib/substore/producers/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–3 | const | `targetPlatform` | 保存 'targetPlatform' 的模块级常量、配置、路由或预计算值。 |  |
| 5–42 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 44–46 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 48–71 | function | `QX_Producer` | 执行与 'QX_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 49–69 | function | `QX_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 7；await 0；调用 'http'、'shadowsocks'、'shadowsocksr'、'socks5'、'trojan'、'vless'、'vmess' |
| 73–199 | function | `shadowsocks` | 执行与 'shadowsocks' 对应的前端业务、状态或数据转换逻辑。 | 分支 12；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'append'、'appendIfPresent'、'isPresent'、'needTls'、'result.append.bind'、'result.appendIfPresent'、'result.appendIfPresent.bind'、'result.toString' |
| 201–237 | function | `shadowsocksr` | 执行与 'shadowsocksr' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'append'、'appendIfPresent'、'result.append.bind'、'result.appendIfPresent'、'result.appendIfPresent.bind'、'result.toString' |
| 239–314 | function | `trojan` | 执行与 'trojan' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 'append'、'appendIfPresent'、'isPresent'、'needTls'、'result.append.bind'、'result.appendIfPresent'、'result.appendIfPresent.bind'、'result.toString' |
| 316–417 | function | `vmess` | 执行与 'vmess' 对应的前端业务、状态或数据转换逻辑。 | 分支 11；循环 0；返回 1；await 0；调用 'Array.isArray'、'append'、'appendIfPresent'、'isPresent'、'needTls'、'result.append.bind'、'result.appendIfPresent'、'result.appendIfPresent.bind'、'result.toString' |
| 418–518 | function | `vless` | 执行与 'vless' 对应的前端业务、状态或数据转换逻辑。 | 分支 13；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'Array.isArray'、'append'、'appendIfPresent'、'isPresent'、'needTls'、'result.append.bind'、'result.appendIfPresent'、'result.appendIfPresent.bind'、'result.toString' |
| 520–579 | function | `http` | 执行与 'http' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'append'、'appendIfPresent'、'needTls'、'result.append.bind'、'result.appendIfPresent'、'result.appendIfPresent.bind'、'result.toString' |
| 581–640 | function | `socks5` | 执行与 'socks5' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'append'、'appendIfPresent'、'needTls'、'result.append.bind'、'result.appendIfPresent'、'result.appendIfPresent.bind'、'result.toString' |
| 642–644 | function | `needTls` | 执行与 'needTls' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |

## `lib/substore/producers/shadowrocket.ts`

依赖：`@/lib/substore/producers/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–3 | type | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 5–8 | interface | `ProduceOptions` | 定义 'ProduceOptions' 的数据契约、联合类型或组件属性。 |  |
| 10–13 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 15–269 | function | `Shadowrocket_Producer` | 执行与 'Shadowrocket_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 17–267 | function | `Shadowrocket_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'list.map'、'list.map.join'、'proxies.filter'、'proxies.filter.map'、'proxies.filter.map.filter' |
| 19–27 | function | `Shadowrocket_Producer > produce > proxies.filter.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 4；await 0；调用 '<ArrayLiteralExpression>.includes' |
| 28–257 | function | `Shadowrocket_Producer > produce > proxies.filter.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 45；循环 1；返回 3；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'Array.isArray'、'isPresent'、'parseInt'、'reg.exec' |
| 258–258 | function | `Shadowrocket_Producer > produce > proxies.filter.map.filter.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Boolean' |
| 263–265 | function | `Shadowrocket_Producer > produce > list.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'JSON.stringify' |

## `lib/substore/producers/sing-box.ts`

依赖：`@/lib/substore/producers/clashmeta`、`@/lib/substore/producers/utils`、`sonner`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–13 | type | `IpVersionKey` | 定义 'IpVersionKey' 的数据契约、联合类型或组件属性。 |  |
| 15–19 | type | `IpVersionValue` | 定义 'IpVersionValue' 的数据契约、联合类型或组件属性。 |  |
| 21–30 | const | `ipVersions` | 保存 'ipVersions' 的模块级常量、配置、路由或预计算值。 |  |
| 32–35 | interface | `DomainResolver` | 定义 'DomainResolver' 的数据契约、联合类型或组件属性。 |  |
| 37–49 | interface | `Multiplex` | 定义 'Multiplex' 的数据契约、联合类型或组件属性。 |  |
| 51–76 | interface | `TLS` | 定义 'TLS' 的数据契约、联合类型或组件属性。 |  |
| 78–87 | interface | `Transport` | 定义 'Transport' 的数据契约、联合类型或组件属性。 |  |
| 89–224 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 226–293 | interface | `ParsedProxy` | 定义 'ParsedProxy' 的数据契约、联合类型或组件属性。 |  |
| 295–297 | interface | `ProduceOptions` | 定义 'ProduceOptions' 的数据契约、联合类型或组件属性。 |  |
| 299–302 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 304–312 | function | `ipVersionParser` | 执行与 'ipVersionParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0 |
| 314–316 | function | `detourParser` | 执行与 'detourParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 318–320 | function | `networkParser` | 执行与 'networkParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes' |
| 322–328 | function | `tfoParser` | 执行与 'tfoParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 0；await 0 |
| 330–348 | function | `smuxParser` | 执行与 'smuxParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 8；循环 0；返回 1；await 0；调用 'parseInt' |
| 350–425 | function | `wsParser` | 执行与 'wsParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 24；循环 6；返回 0；await 0；调用 '<TemplateExpression>.split'、'Array.isArray'、'Object.keys'、'item.split'、'key.trim'、'parseInt'、'reg.exec'、'value.trim'、'value.trim.split' |
| 427–468 | function | `h1Parser` | 执行与 'h1Parser' 对应的前端业务、状态或数据转换逻辑。 | 分支 19；循环 2；返回 0；await 0；调用 '<TemplateExpression>.split'、'<TemplateExpression>.split.map'、'Array.isArray'、'Object.keys'、'key.toLowerCase' |
| 440–440 | function | `h1Parser > <TemplateExpression>.split.map.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'i.trim' |
| 444–444 | function | `h1Parser > <TemplateExpression>.split.map.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'i.trim' |
| 450–450 | function | `h1Parser > <TemplateExpression>.split.map.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'i.trim' |
| 470–491 | function | `h2Parser` | 执行与 'h2Parser' 对应的前端业务、状态或数据转换逻辑。 | 分支 11；循环 0；返回 0；await 0；调用 '<TemplateExpression>.split'、'<TemplateExpression>.split.map'、'Array.isArray' |
| 476–476 | function | `h2Parser > <TemplateExpression>.split.map.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'i.trim' |
| 482–482 | function | `h2Parser > <TemplateExpression>.split.map.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'i.trim' |
| 493–500 | function | `grpcParser` | 执行与 'grpcParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0 |
| 502–563 | function | `tlsParser` | 执行与 'tlsParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 31；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes'、'Array.isArray' |
| 565–595 | function | `sshParser` | 执行与 'sshParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 10；循环 0；返回 1；await 0；调用 'detourParser'、'ipVersionParser'、'parseInt'、'proxy[<key>].split'、'tfoParser' |
| 597–621 | function | `httpParser` | 执行与 'httpParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 1；返回 1；await 0；调用 'Object.keys'、'detourParser'、'ipVersionParser'、'parseInt'、'tfoParser'、'tlsParser' |
| 623–648 | function | `socks5Parser` | 执行与 'socks5Parser' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 0；返回 1；await 0；调用 'detourParser'、'ipVersionParser'、'networkParser'、'parseInt'、'tfoParser' |
| 650–691 | function | `shadowTLSParser` | 执行与 'shadowTLSParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 'detourParser'、'ipVersionParser'、'parseInt'、'smuxParser'、'tfoParser' |
| 693–767 | function | `ssParser` | 执行与 'ssParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 11；循环 0；返回 1；await 0；调用 'Object.keys'、'Object.keys.forEach'、'detourParser'、'ipVersionParser'、'networkParser'、'optArr.join'、'parseInt'、'smuxParser'、'tfoParser' |
| 723–735 | function | `ssParser > Object.keys.forEach.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'optArr.push' |
| 741–761 | function | `ssParser > Object.keys.forEach.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 0；await 0；调用 'JSON.stringify'、'optArr.push' |
| 770–791 | function | `ssrParser` | 执行与 'ssrParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 1；await 0；调用 'detourParser'、'ipVersionParser'、'parseInt'、'smuxParser'、'tfoParser' |
| 793–824 | function | `vmessParser` | 执行与 'vmessParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 8；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.indexOf'、'detourParser'、'grpcParser'、'h1Parser'、'h2Parser'、'ipVersionParser'、'networkParser'、'parseInt'、'smuxParser'、'tfoParser'、'tlsParser'、'wsParser' |
| 826–851 | function | `vlessParser` | 执行与 'vlessParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 8；循环 0；返回 1；await 0；调用 'detourParser'、'grpcParser'、'h1Parser'、'h2Parser'、'ipVersionParser'、'networkParser'、'parseInt'、'smuxParser'、'tfoParser'、'tlsParser'、'wsParser' |
| 853–873 | function | `trojanParser` | 执行与 'trojanParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 1；await 0；调用 'detourParser'、'grpcParser'、'ipVersionParser'、'networkParser'、'parseInt'、'smuxParser'、'tfoParser'、'tlsParser'、'wsParser' |
| 875–929 | function | `hysteriaParser` | 执行与 'hysteriaParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 17；循环 0；返回 1；await 0；调用 '<RegularExpressionLiteral>.test'、'<TemplateExpression>.endsWith'、'detourParser'、'ipVersionParser'、'networkParser'、'parseInt'、'proxy[<key>].split'、'proxy[<key>].split.map'、'reg.test'、'smuxParser'、'tfoParser'、'tlsParser' |
| 890–893 | function | `hysteriaParser > proxy[<key>].split.map.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'p.replace'、'range.includes' |
| 931–965 | function | `hysteria2Parser` | 执行与 'hysteria2Parser' 对应的前端业务、状态或数据转换逻辑。 | 分支 9；循环 0；返回 1；await 0；调用 '<RegularExpressionLiteral>.test'、'detourParser'、'ipVersionParser'、'networkParser'、'parseInt'、'proxy[<key>].split'、'proxy[<key>].split.map'、'smuxParser'、'tfoParser'、'tlsParser' |
| 947–950 | function | `hysteria2Parser > proxy[<key>].split.map.callback#30` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'p.replace'、'range.includes' |
| 967–993 | function | `tuic5Parser` | 执行与 'tuic5Parser' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 0；返回 1；await 0；调用 'detourParser'、'ipVersionParser'、'networkParser'、'parseInt'、'smuxParser'、'tfoParser'、'tlsParser' |
| 995–1015 | function | `anytlsParser` | 执行与 'anytlsParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 '<RegularExpressionLiteral>.test'、'detourParser'、'ipVersionParser'、'networkParser'、'parseInt'、'tlsParser' |
| 1017–1021 | function | `parseReserved` | 解析与 'parseReserved' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 'Array.isArray'、'reserved.map' |
| 1019–1019 | function | `parseReserved > reserved.map.callback#34` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number' |
| 1023–1100 | function | `wireguardParser` | 执行与 'wireguardParser' 对应的前端业务、状态或数据转换逻辑。 | 分支 12；循环 1；返回 1；await 0；调用 '<ArrayLiteralExpression>.map'、'<ArrayLiteralExpression>.map.filter'、'Array.isArray'、'detourParser'、'ipVersionParser'、'networkParser'、'parseInt'、'parseReserved'、'peers.push'、'smuxParser'、'tfoParser' |
| 1025–1032 | function | `wireguardParser > <ArrayLiteralExpression>.map.callback#36` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 5；await 0；调用 'isIPv4'、'isIPv6'、'val.includes' |
| 1033–1033 | function | `wireguardParser > <ArrayLiteralExpression>.map.filter.callback#37` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1102–1224 | function | `singbox_Producer` | 执行与 'singbox_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 1104–1222 | function | `singbox_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 '<AsExpression>.map'、'ClashMeta_Producer'、'ClashMeta_Producer.produce'、'JSON.stringify'、'list.reduce' |
| 1112–1205 | function | `singbox_Producer > produce > <AsExpression>.map.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 10；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes'、'anytlsParser'、'httpParser'、'hysteria2Parser'、'hysteriaParser'、'list.push'、'shadowTLSParser'、'socks5Parser'、'ssParser'、'sshParser'、'ssrParser'、'toast'、'trojanParser'、'tuic5Parser'、'vlessParser'、'vmessParser'、'wireguardParser' |
| 1210–1217 | function | `singbox_Producer > produce > list.reduce.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'result.endpoints.push'、'result.outbounds.push' |

## `lib/substore/producers/stash.ts`

依赖：`@/lib/substore/producers/utils`、`sonner`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–75 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 77–79 | interface | `ProduceOptions` | 定义 'ProduceOptions' 的数据契约、联合类型或组件属性。 |  |
| 81–84 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 86–409 | function | `Stash_Producer` | 执行与 'Stash_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 88–407 | function | `Stash_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'list.map'、'list.map.join'、'proxies.filter'、'proxies.filter.map' |
| 91–141 | function | `Stash_Producer > produce > proxies.filter.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 3；await 0；调用 '<ArrayLiteralExpression>.includes'、'toast' |
| 142–400 | function | `Stash_Producer > produce > proxies.filter.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 49；循环 1；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'<TemplateExpression>.match'、'Array.isArray'、'isPresent'、'parseInt'、'reg.exec' |
| 405–405 | function | `Stash_Producer > produce > list.map.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'JSON.stringify' |

## `lib/substore/producers/surfboard.ts`

依赖：`@/lib/substore/producers/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–3 | const | `targetPlatform` | 保存 'targetPlatform' 的模块级常量、配置、路由或预计算值。 |  |
| 5–27 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 29–31 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 33–55 | function | `Surfboard_Producer` | 执行与 'Surfboard_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 34–53 | function | `Surfboard_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 6；await 0；调用 'http'、'proxy.name.replace'、'shadowsocks'、'socks5'、'trojan'、'vmess'、'wireguard' |
| 57–110 | function | `shadowsocks` | 执行与 'shadowsocks' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 112–137 | function | `trojan` | 执行与 'trojan' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'handleTransport'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 139–168 | function | `vmess` | 执行与 'vmess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'handleTransport'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 170–188 | function | `http` | 执行与 'http' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'result.append'、'result.appendIfPresent'、'result.toString' |
| 190–208 | function | `socks5` | 执行与 'socks5' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'result.append'、'result.appendIfPresent'、'result.toString' |
| 210–221 | function | `wireguard` | 执行与 'wireguard' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'result.append'、'result.appendIfPresent'、'result.toString' |
| 223–252 | function | `handleTransport` | 处理与 'handleTransport' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 0；await 0；调用 'Object.keys'、'Object.keys.map'、'Object.keys.map.join'、'isNotBlank'、'isPresent'、'result.append'、'result.appendIfPresent' |
| 235–241 | function | `handleTransport > Object.keys.map.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes' |

## `lib/substore/producers/surge.ts`

依赖：`sonner`、`./utils`、`@/lib/substore/producers/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–5 | const | `targetPlatform` | 保存 'targetPlatform' 的模块级常量、配置、路由或预计算值。 |  |
| 7–13 | const | `ipVersions` | 保存 'ipVersions' 的模块级常量、配置、路由或预计算值。 |  |
| 15–85 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 87–90 | interface | `ProduceOptions` | 定义 'ProduceOptions' 的数据契约、联合类型或组件属性。 |  |
| 92–94 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 96–135 | function | `Surge_Producer` | 执行与 'Surge_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 97–133 | function | `Surge_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 12；await 0；调用 'String'、'direct'、'http'、'hysteria2'、'proxy.name.replace'、'shadowsocks'、'snell'、'socks5'、'ssh'、'trojan'、'tuic'、'vmess'、'wireguard'、'wireguard_surge' |
| 137–278 | function | `shadowsocks` | 执行与 'shadowsocks' 对应的前端业务、状态或数据转换逻辑。 | 分支 11；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 280–360 | function | `trojan` | 执行与 'trojan' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'handleTransport'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 362–449 | function | `vmess` | 执行与 'vmess' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'handleTransport'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 451–515 | function | `ssh` | 执行与 'ssh' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'result.append'、'result.appendIfPresent'、'result.toString' |
| 517–596 | function | `http` | 执行与 'http' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 'Object.keys'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 598–645 | function | `direct` | 执行与 'direct' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'result.append'、'result.appendIfPresent'、'result.toString' |
| 647–725 | function | `socks5` | 执行与 'socks5' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString'、'toast' |
| 727–806 | function | `snell` | 执行与 'snell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 808–907 | function | `tuic` | 执行与 'tuic' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 0；返回 1；await 0；调用 '<BinaryExpression>.replace'、'Array.isArray'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 909–1022 | function | `wireguard` | 执行与 'wireguard' 对应的前端业务、状态或数据转换逻辑。 | 分支 8；循环 0；返回 1；await 0；调用 'Array.isArray'、'Object.keys'、'Object.keys.filter'、'Object.keys.filter.map'、'Object.keys.filter.map.join'、'getIfNotBlank'、'isPresent'、'proxy.dns.join'、'proxy.reserved.join'、'proxy[<key>].join'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 1017–1017 | function | `wireguard > Object.keys.filter.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1018–1018 | function | `wireguard > Object.keys.filter.map.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1024–1083 | function | `wireguard_surge` | 执行与 'wireguard_surge' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 1085–1179 | function | `hysteria2` | 执行与 'hysteria2' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 '<BinaryExpression>.replace'、'<TemplateExpression>.match'、'isPresent'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 1181–1216 | function | `handleTransport` | 处理与 'handleTransport' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes'、'Object.keys'、'Object.keys.map'、'Object.keys.map.join'、'isNotBlank'、'isPresent'、'result.append'、'result.appendIfPresent'、'toast' |
| 1193–1199 | function | `handleTransport > Object.keys.map.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0 |

## `lib/substore/producers/surgemac.ts`

依赖：`js-base64`、`@/lib/substore/producers/surge`、`@/lib/substore/producers/clashmeta`、`@/lib/substore/producers/utils`、`sonner`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 7–7 | const | `targetPlatform` | 保存 'targetPlatform' 的模块级常量、配置、路由或预计算值。 |  |
| 9–9 | const | `surge_Producer` | 保存 'surge_Producer' 的模块级常量、配置、路由或预计算值。 |  |
| 11–39 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 41–47 | interface | `ProduceOptions` | 定义 'ProduceOptions' 的数据契约、联合类型或组件属性。 |  |
| 49–51 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 53–79 | function | `SurgeMac_Producer` | 执行与 'SurgeMac_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 54–77 | function | `SurgeMac_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 'console.log'、'external'、'mihomo'、'surge_Producer.produce' |
| 81–120 | function | `external` | 执行与 'external' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 'Array.isArray'、'isPresent'、'proxy.addresses.map'、'proxy.args.map'、'result.append'、'result.appendIfPresent'、'result.toString' |
| 91–93 | function | `external > proxy.args.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'result.append' |
| 96–98 | function | `external > proxy.addresses.map.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'result.append' |
| 122–189 | function | `mihomo` | 执行与 'mihomo' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 0；调用 '<ArrayLiteralExpression>.includes'、'<NonNullExpression>.push'、'Base64.encode'、'ClashMeta_Producer'、'ClashMeta_Producer.produce'、'JSON.stringify'、'external'、'isIP'、'toast' |
| 191–193 | function | `isIP` | 判断与 'isIP' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'isIPv4'、'isIPv6' |

## `lib/substore/producers/uri.ts`

依赖：`js-base64`、`@/lib/substore/producers/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–119 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 121–124 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 126–256 | function | `vless` | 执行与 'vless' 对应的前端业务、状态或数据转换逻辑。 | 分支 30；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'Array.isArray'、'encodeURIComponent'、'proxy.alpn.join' |
| 258–845 | function | `URI_Producer` | 执行与 'URI_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 260–843 | function | `URI_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 82；循环 1；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'Array.isArray'、'Base64.encode'、'JSON.stringify'、'Object.keys'、'Object.keys.forEach'、'anytlsParams.join'、'anytlsParams.push'、'encodeURIComponent'、'hysteria2params.join'、'hysteria2params.push'、'hysteriaParams.join'、'isIPv6'、'proxy.alpn.join'、'proxy.alpn.map'、'proxy.alpn.map.join'、'tuicParams.join… |
| 627–674 | function | `URI_Producer > produce > Object.keys.forEach.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 18；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'Array.isArray'、'encodeURIComponent'、'hysteriaParams.includes'、'hysteriaParams.push'、'key.replace' |
| 686–743 | function | `URI_Producer > produce > Object.keys.forEach.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 13；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'Array.isArray'、'encodeURIComponent'、'i.replace'、'key.replace'、'tuicParams.includes'、'tuicParams.push' |
| 802–825 | function | `URI_Producer > produce > Object.keys.forEach.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 5；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'encodeURIComponent'、'wireguardParams.push' |

## `lib/substore/producers/utils.ts`

依赖：`lodash`、`@/lib/proxy-types`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–5 | const | `IPV4_REGEX` | 保存 'IPV4_REGEX' 的模块级常量、配置、路由或预计算值。 |  |
| 8–9 | const | `IPV6_REGEX` | 保存 'IPV6_REGEX' 的模块级常量、配置、路由或预计算值。 |  |
| 11–36 | class | `Result` | 封装 'Result' 的实例状态与行为。 |  |
| 15–18 | function | `<anonymous#1>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 20–25 | function | `append` | 执行与 'append' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 '<ThisKeyword>.output.push' |
| 27–31 | function | `appendIfPresent` | 执行与 'appendIfPresent' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 '<ThisKeyword>.append'、'isPresent' |
| 33–35 | function | `toString` | 执行与 'toString' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 '<ThisKeyword>.output.join' |
| 38–46 | function | `isPresent` | 判断与 'isPresent' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 '_get' |
| 51–54 | function | `isIP` | 判断与 'isIP' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'isIPv4'、'isIPv6'、'str.replace'、'str.replace.replace' |
| 56–58 | function | `isIPv4` | 判断与 'isIPv4' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'IPV4_REGEX.test' |
| 60–62 | function | `isIPv6` | 判断与 'isIPv6' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'IPV6_REGEX.test' |
| 64–68 | function | `isValidPortNumber` | 判断与 'isValidPortNumber' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 '<RegularExpressionLiteral>.test'、'String' |
| 70–72 | function | `isNotBlank` | 判断与 'isNotBlank' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'str.trim' |
| 74–76 | function | `getIfNotBlank` | 读取或计算与 'getIfNotBlank' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'isNotBlank' |
| 78–80 | function | `getIfPresent` | 读取或计算与 'getIfPresent' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'isPresent' |
| 82–91 | function | `getPolicyDescriptor` | 读取或计算与 'getPolicyDescriptor' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 '<RegularExpressionLiteral>.test' |
| 93–97 | function | `getRandomInt` | 读取或计算与 'getRandomInt' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Math.ceil'、'Math.floor'、'Math.random' |
| 99–108 | function | `getRandomPort` | 读取或计算与 'getRandomPort' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'Math.floor'、'Math.random'、'Number'、'getRandomInt'、'portString.split'、'randomPart.includes'、'randomPart.split'、'randomPart.split.map' |
| 110–112 | function | `numberToString` | 执行与 'numberToString' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'BigInt'、'BigInt.toString'、'Number.isSafeInteger'、'String' |
| 114–119 | function | `isValidUUID` | 判断与 'isValidUUID' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 '<RegularExpressionLiteral>.test' |
| 121–140 | function | `formatDateTime` | 格式化与 'formatDateTime' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'd.getDate'、'd.getFullYear'、'd.getHours'、'd.getMinutes'、'd.getMonth'、'd.getSeconds'、'd.getTime'、'format.replace'、'isNaN'、'pad' |
| 128–128 | function | `formatDateTime > pad` | 执行与 'pad' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'String'、'String.padStart' |
| 139–139 | function | `formatDateTime > format.replace.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'String' |
| 147–154 | function | `pickFirstDefined` | 执行与 'pickFirstDefined' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 1；返回 2；await 0 |
| 160–167 | function | `shouldApplyTlsSniFallback` | 执行与 'shouldApplyTlsSniFallback' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes' |
| 172–208 | function | `getTransportHost` | 读取或计算与 'getTransportHost' 对应的前端业务、状态或数据转换逻辑。 | 分支 9；循环 1；返回 3；await 0；调用 'Array.isArray'、'optsKeys.push' |
| 220–234 | function | `applyTlsSniFallback` | 执行与 'applyTlsSniFallback' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 0；调用 'getTransportHost'、'isIP'、'shouldApplyTlsSniFallback' |

## `lib/substore/producers/v2ray.ts`

依赖：`js-base64`、`./uri`、`sonner`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–6 | const | `URI` | 保存 'URI' 的模块级常量、配置、路由或预计算值。 |  |
| 8–10 | interface | `Proxy` | 定义 'Proxy' 的数据契约、联合类型或组件属性。 |  |
| 12–15 | interface | `Producer` | 定义 'Producer' 的数据契约、联合类型或组件属性。 |  |
| 17–39 | function | `V2Ray_Producer` | 执行与 'V2Ray_Producer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 19–36 | function | `V2Ray_Producer > produce` | 执行与 'produce' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Base64.encode'、'proxies.map'、'result.join' |
| 21–33 | function | `V2Ray_Producer > produce > proxies.map.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'JSON.stringify'、'URI.produce'、'result.push'、'toast' |

## `lib/template-presets.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–7 | interface | `ACL4SSRPreset` | 定义 'ACL4SSRPreset' 的数据契约、联合类型或组件属性。 |  |
| 10–54 | const | `ACL4SSR_PRESETS` | 保存 'ACL4SSR_PRESETS' 的模块级常量、配置、路由或预计算值。 |  |
| 57–62 | const | `Aethersailor_PRESETS` | 保存 'Aethersailor_PRESETS' 的模块级常量、配置、路由或预计算值。 |  |
| 65–68 | const | `ALL_TEMPLATE_PRESETS` | 保存 'ALL_TEMPLATE_PRESETS' 的模块级常量、配置、路由或预计算值。 |  |

## `lib/template-v3-utils.ts`

依赖：`js-yaml`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–17 | const | `REGION_PROXY_GROUPS` | 保存 'REGION_PROXY_GROUPS' 的模块级常量、配置、路由或预计算值。 |  |
| 20–20 | const | `OTHER_REGIONS_EXCLUDE_FILTER` | 保存 'OTHER_REGIONS_EXCLUDE_FILTER' 的模块级常量、配置、路由或预计算值。 |  |
| 23–26 | const | `PROXY_TYPES` | 保存 'PROXY_TYPES' 的模块级常量、配置、路由或预计算值。 |  |
| 28–28 | type | `ProxyType` | 定义 'ProxyType' 的数据契约、联合类型或组件属性。 |  |
| 31–33 | const | `PROXY_GROUP_TYPES` | 保存 'PROXY_GROUP_TYPES' 的模块级常量、配置、路由或预计算值。 |  |
| 35–35 | type | `ProxyGroupType` | 定义 'ProxyGroupType' 的数据契约、联合类型或组件属性。 |  |
| 38–38 | const | `PROXY_NODES_MARKER` | 保存 'PROXY_NODES_MARKER' 的模块级常量、配置、路由或预计算值。 |  |
| 39–39 | const | `PROXY_PROVIDERS_MARKER` | 保存 'PROXY_PROVIDERS_MARKER' 的模块级常量、配置、路由或预计算值。 |  |
| 40–40 | const | `REGION_PROXY_GROUPS_MARKER` | 保存 'REGION_PROXY_GROUPS_MARKER' 的模块级常量、配置、路由或预计算值。 |  |
| 44–44 | const | `DIRECT_MARKER` | 保存 'DIRECT_MARKER' 的模块级常量、配置、路由或预计算值。 |  |
| 45–45 | const | `REJECT_MARKER` | 保存 'REJECT_MARKER' 的模块级常量、配置、路由或预计算值。 |  |
| 48–48 | type | `ProxyOrderItem` | 定义 'ProxyOrderItem' 的数据契约、联合类型或组件属性。 |  |
| 51–75 | interface | `ProxyGroupV3Config` | 定义 'ProxyGroupV3Config' 的数据契约、联合类型或组件属性。 |  |
| 78–90 | interface | `ParsedTemplate` | 定义 'ParsedTemplate' 的数据契约、联合类型或组件属性。 |  |
| 93–116 | interface | `ProxyGroupFormState` | 定义 'ProxyGroupFormState' 的数据契约、联合类型或组件属性。 |  |
| 119–126 | function | `keywordsToRegex` | 执行与 'keywordsToRegex' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'keywords.split'、'keywords.split.map'、'keywords.split.map.filter'、'keywords.split.map.filter.join'、'keywords.trim' |
| 123–123 | function | `keywordsToRegex > keywords.split.map.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'k.trim' |
| 124–124 | function | `keywordsToRegex > keywords.split.map.filter.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 129–141 | const | `STANDARD_TOP_LEVEL_KEYS` | 保存 'STANDARD_TOP_LEVEL_KEYS' 的模块级常量、配置、路由或预计算值。 |  |
| 144–158 | function | `extractTemplateVariables` | 执行与 'extractTemplateVariables' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 1；返回 3；await 0；调用 'Object.entries'、'STANDARD_TOP_LEVEL_KEYS.has'、'parseYAML' |
| 161–164 | function | `regexToKeywords` | 执行与 'regexToKeywords' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'regex.split'、'regex.split.join' |
| 167–190 | function | `createDefaultFormState` | 创建与 'createDefaultFormState' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 193–196 | function | `hasProxyNodes` | 判断是否具有与 'hasProxyNodes' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'state.filterKeywords.trim' |
| 199–201 | function | `hasProxyProviders` | 判断是否具有与 'hasProxyProviders' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 204–232 | function | `getDefaultProxyOrder` | 读取或计算与 'getDefaultProxyOrder' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 'hasProxyNodes'、'hasProxyProviders'、'order.push' |
| 235–292 | function | `formStateToConfig` | 执行与 'formStateToConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 16；循环 0；返回 1；await 0；调用 'keywordsToRegex'、'state.excludeTypes.join'、'state.includeTypes.join'、'state.proxyOrder.filter' |
| 265–271 | function | `formStateToConfig > state.proxyOrder.filter.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 5；await 0；调用 'hasProxyNodes'、'hasProxyProviders' |
| 295–373 | function | `configToFormState` | 执行与 'configToFormState' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 2；返回 1；await 0；调用 'allGroupNames.includes'、'config[<key>].split'、'config[<key>].split.filter'、'getDefaultProxyOrder'、'proxyOrder.filter'、'proxyOrder.includes'、'proxyOrder.push'、'regexToKeywords'、'state.proxyOrder.includes'、'state.proxyOrder.push'、'staticProxies.push' |
| 346–346 | function | `configToFormState > config[<key>].split.filter.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'PROXY_TYPES.includes' |
| 347–347 | function | `configToFormState > config[<key>].split.filter.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'PROXY_TYPES.includes' |
| 353–353 | function | `configToFormState > proxyOrder.filter.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 376–382 | function | `parseTemplate` | 解析与 'parseTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 2；await 0；调用 'parseYAML' |
| 385–387 | function | `serializeTemplate` | 执行与 'serializeTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'dumpYAML' |
| 390–397 | function | `extractProxyGroups` | 执行与 'extractProxyGroups' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'extractTemplateVariables'、'parseTemplate'、'template[<key>].map' |
| 395–395 | function | `extractProxyGroups > template[<key>].map.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 396–396 | function | `extractProxyGroups > template[<key>].map.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Object.keys'、'configToFormState' |
| 400–406 | function | `updateProxyGroups` | 更新与 'updateProxyGroups' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'groups.map'、'parseTemplate'、'serializeTemplate' |
| 409–409 | const | `PROXY_NODES_DISPLAY` | 保存 'PROXY_NODES_DISPLAY' 的模块级常量、配置、路由或预计算值。 |  |
| 410–410 | const | `PROXY_PROVIDERS_DISPLAY` | 保存 'PROXY_PROVIDERS_DISPLAY' 的模块级常量、配置、路由或预计算值。 |  |
| 411–411 | const | `REGION_PROXY_GROUPS_DISPLAY` | 保存 'REGION_PROXY_GROUPS_DISPLAY' 的模块级常量、配置、路由或预计算值。 |  |
| 413–413 | const | `DIRECT_DISPLAY` | 保存 'DIRECT_DISPLAY' 的模块级常量、配置、路由或预计算值。 |  |
| 414–414 | const | `REJECT_DISPLAY` | 保存 'REJECT_DISPLAY' 的模块级常量、配置、路由或预计算值。 |  |
| 417–431 | function | `generateProxyGroupsPreview` | 生成与 'generateProxyGroupsPreview' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'dumpYAML'、'groups.map'、'groups.map.map' |
| 418–429 | function | `generateProxyGroupsPreview > groups.map.map.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'config.proxies.map' |
| 421–426 | function | `generateProxyGroupsPreview > groups.map.map.callback#23 > config.proxies.map.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 4；await 0 |
| 434–457 | function | `generateRegionProxyGroups` | 生成与 'generateRegionProxyGroups' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'REGION_PROXY_GROUPS.map'、'createDefaultFormState'、'getDefaultProxyOrder'、'groups.push' |
| 435–444 | function | `generateRegionProxyGroups > REGION_PROXY_GROUPS.map.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'createDefaultFormState'、'getDefaultProxyOrder' |
| 460–462 | function | `getRegionProxyGroupNames` | 读取或计算与 'getRegionProxyGroupNames' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'REGION_PROXY_GROUPS.map' |
| 461–461 | function | `getRegionProxyGroupNames > REGION_PROXY_GROUPS.map.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 465–512 | function | `createBlankTemplate` | 创建与 'createBlankTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'serializeTemplate' |

## `lib/utils.ts`

依赖：`clsx`、`tailwind-merge`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–6 | function | `cn` | 执行与 'cn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'clsx'、'twMerge' |
| 8–10 | function | `sleep` | 执行与 'sleep' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 9–9 | function | `sleep > <anonymous#3>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTimeout' |
| 24–60 | function | `getPageNumbers` | 读取或计算与 'getPageNumbers' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 4；返回 1；await 0；调用 'rangeWithDots.push' |

