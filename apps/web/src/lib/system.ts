export function formatStartedAt(value: string, locale = 'zh-CN'): string {
  const instant = new Date(value)
  if (Number.isNaN(instant.valueOf())) return '—'
  return new Intl.DateTimeFormat(locale, {
    dateStyle: 'medium',
    timeStyle: 'medium',
  }).format(instant)
}

