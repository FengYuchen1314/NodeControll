#!/usr/bin/env python3
"""Capture and normalize the public 妙妙屋 X documentation.

The script deliberately uses only Python's standard library so that the
evidence capture is reproducible without installing a scraping stack.
"""

from __future__ import annotations

import argparse
import concurrent.futures
import gzip
import hashlib
import html
import json
import re
import time
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import urlparse


USER_AGENT = "NodeControll research crawler/1.0 (+source analysis; public docs only)"
SITEMAP_INDEX = "https://miaomiaowux.com/docs/sitemap-index.xml"
SKIP_TAGS = {"script", "style", "svg", "template", "noscript"}
BLOCK_TAGS = {
    "address", "article", "aside", "blockquote", "div", "dl", "fieldset",
    "figcaption", "figure", "footer", "form", "header", "hr", "main", "nav",
    "ol", "p", "pre", "section", "table", "tbody", "tfoot", "thead", "tr", "ul",
}


def fetch(url: str, attempts: int = 3) -> bytes:
    last_error: Exception | None = None
    for attempt in range(1, attempts + 1):
        try:
            request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
            with urllib.request.urlopen(request, timeout=45) as response:
                return response.read()
        except (OSError, urllib.error.URLError, urllib.error.HTTPError) as error:
            last_error = error
            if attempt < attempts:
                time.sleep(attempt)
    raise RuntimeError(f"failed to fetch {url}: {last_error}")


class MainContentParser(HTMLParser):
    """Extract readable Markdown-ish text and metadata from Starlight HTML."""

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.in_main = False
        self.main_depth = 0
        self.skip_depth = 0
        self.title_depth = 0
        self.heading_level = 0
        self.heading_parts: list[str] = []
        self.headings: list[dict[str, object]] = []
        self.parts: list[str] = []
        self.title = ""
        self.description = ""
        self.canonical = ""
        self.in_pre = False
        self.in_code = False

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attrs_dict = dict(attrs)
        if tag == "title":
            self.title_depth += 1
        if tag == "meta" and attrs_dict.get("name") == "description":
            self.description = attrs_dict.get("content") or ""
        if tag == "link" and attrs_dict.get("rel") == "canonical":
            self.canonical = attrs_dict.get("href") or ""

        if tag == "main":
            self.in_main = True
            self.main_depth = 1
            self._newline(2)
            return
        if not self.in_main:
            return
        self.main_depth += 1
        if self.skip_depth:
            self.skip_depth += 1
            return
        if tag in SKIP_TAGS:
            self.skip_depth = 1
            return
        if tag in {"h1", "h2", "h3", "h4", "h5", "h6"}:
            self.heading_level = int(tag[1])
            self.heading_parts = []
            self._newline(2)
            self.parts.append("#" * self.heading_level + " ")
        elif tag == "li":
            self._newline(1)
            self.parts.append("- ")
        elif tag == "br":
            self._newline(1)
        elif tag == "pre":
            self._newline(2)
            self.parts.append("```\n")
            self.in_pre = True
        elif tag == "code" and not self.in_pre:
            self.parts.append("`")
            self.in_code = True
        elif tag in BLOCK_TAGS:
            self._newline(1)
        elif tag in {"th", "td"}:
            self.parts.append(" | ")

    def handle_endtag(self, tag: str) -> None:
        if tag == "title" and self.title_depth:
            self.title_depth -= 1
        if not self.in_main:
            return
        if self.skip_depth:
            self.skip_depth -= 1
        elif tag in {"h1", "h2", "h3", "h4", "h5", "h6"}:
            heading = re.sub(r"\s+", " ", "".join(self.heading_parts)).strip()
            if heading:
                self.headings.append({"level": self.heading_level, "text": heading})
            self.heading_level = 0
            self.heading_parts = []
            self._newline(2)
        elif tag == "pre":
            self.parts.append("\n```\n")
            self.in_pre = False
        elif tag == "code" and self.in_code:
            self.parts.append("`")
            self.in_code = False
        elif tag in BLOCK_TAGS or tag in {"li", "tr"}:
            self._newline(1)

        self.main_depth -= 1
        if tag == "main" or self.main_depth <= 0:
            self.in_main = False
            self.main_depth = 0

    def handle_data(self, data: str) -> None:
        if self.title_depth:
            self.title += data
        if not self.in_main or self.skip_depth:
            return
        normalized = data if self.in_pre else re.sub(r"\s+", " ", data)
        if not normalized.strip():
            return
        if self.heading_level:
            self.heading_parts.append(normalized)
        if self.parts and not self.parts[-1].endswith((" ", "\n", "`")) and not normalized.startswith(" "):
            self.parts.append(" ")
        self.parts.append(normalized)

    def markdown(self) -> str:
        text = html.unescape("".join(self.parts))
        text = re.sub(r"[ \t]+\n", "\n", text)
        text = re.sub(r"\n[ \t]+", "\n", text)
        text = re.sub(r"\n{3,}", "\n\n", text)
        return text.strip() + "\n"

    def _newline(self, count: int) -> None:
        if not self.parts:
            return
        trailing = len(self.parts[-1]) - len(self.parts[-1].rstrip("\n"))
        if trailing < count:
            self.parts.append("\n" * (count - trailing))


@dataclass
class PageRecord:
    slug: str
    url: str
    canonical: str
    title: str
    description: str
    fetched_at: str
    html_bytes: int
    sha256_html: str
    pro_mentions: int
    headings: list[dict[str, object]]
    extracted_path: str
    raw_path: str


def page_slug(url: str) -> str:
    path = urlparse(url).path.rstrip("/")
    suffix = path.removeprefix("/docs").strip("/")
    return suffix or "index"


def capture_page(url: str, out_dir: Path, fetched_at: str) -> PageRecord:
    body = fetch(url)
    parser = MainContentParser()
    parser.feed(body.decode("utf-8", errors="replace"))
    slug = page_slug(url)
    raw_path = out_dir / "raw" / f"{slug}.html.gz"
    extracted_path = out_dir / "extracted" / f"{slug}.md"
    raw_path.parent.mkdir(parents=True, exist_ok=True)
    extracted_path.parent.mkdir(parents=True, exist_ok=True)
    with gzip.open(raw_path, "wb", compresslevel=9) as archive:
        archive.write(body)
    markdown = parser.markdown()
    front_matter = (
        f"# {parser.title.strip() or slug}\n\n"
        f"- 来源：{url}\n"
        f"- 抓取时间：{fetched_at}\n"
        f"- 原始 HTML SHA-256：`{hashlib.sha256(body).hexdigest()}`\n\n"
    )
    extracted_path.write_text(front_matter + markdown, encoding="utf-8", newline="\n")
    return PageRecord(
        slug=slug,
        url=url,
        canonical=parser.canonical,
        title=parser.title.strip(),
        description=parser.description,
        fetched_at=fetched_at,
        html_bytes=len(body),
        sha256_html=hashlib.sha256(body).hexdigest(),
        pro_mentions=len(re.findall(r"\bPRO\b", markdown, flags=re.IGNORECASE)),
        headings=parser.headings,
        extracted_path=extracted_path.relative_to(out_dir).as_posix(),
        raw_path=raw_path.relative_to(out_dir).as_posix(),
    )


def sitemap_urls() -> list[str]:
    index_root = ET.fromstring(fetch(SITEMAP_INDEX))
    namespace = {"sm": "http://www.sitemaps.org/schemas/sitemap/0.9"}
    sitemap_locations = [node.text for node in index_root.findall("sm:sitemap/sm:loc", namespace) if node.text]
    urls: set[str] = set()
    for sitemap in sitemap_locations:
        root = ET.fromstring(fetch(sitemap))
        for node in root.findall("sm:url/sm:loc", namespace):
            if not node.text:
                continue
            parsed = urlparse(node.text)
            if parsed.path == "/docs/en" or parsed.path.startswith("/docs/en/"):
                continue
            urls.add(node.text.rstrip("/"))
    return sorted(urls)


def write_public_index(records: list[PageRecord], out_dir: Path, fetched_at: str) -> None:
    total_bytes = sum(record.html_bytes for record in records)
    pro_pages = sum(1 for record in records if record.pro_mentions)
    lines = [
        "# 妙妙屋 X 文档证据索引",
        "",
        f"- 抓取时间：{fetched_at}",
        f"- Sitemap：{SITEMAP_INDEX}",
        f"- 中文页面：{len(records)}",
        f"- 原始 HTML 总字节：{total_bytes}",
        f"- 含 PRO 明文页面：{pro_pages}",
        "- 发布说明：本目录只保留来源、短元数据与完整哈希；页面链接直接指向官方站点，不包含 HTML 或正文镜像。",
        "",
        "| 页面 | 标题 | PRO 次数 | HTML 字节 | SHA-256 |",
        "|---|---|---:|---:|---|",
    ]
    for record in records:
        lines.append(
            f"| [`{record.slug}`]({record.url}) | {record.title.replace('|', '\\|')} | "
            f"{record.pro_mentions} | {record.html_bytes} | `{record.sha256_html[:12]}…` |"
        )
    (out_dir / "PAGE_INDEX.md").write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
    public_records = []
    for record in records:
        item = asdict(record)
        item.pop("raw_path")
        item.pop("extracted_path")
        public_records.append(item)
    (out_dir / "manifest.json").write_text(
        json.dumps(public_records, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--archive-out",
        type=Path,
        default=Path("upstream/mmwx-doc-evidence"),
        help="ignored/private destination for raw HTML and extracted page bodies",
    )
    parser.add_argument(
        "--metadata-out",
        type=Path,
        default=Path("docs/03-mmwx-gap/evidence"),
        help="public destination for URL/title/description/hash/heading metadata only",
    )
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    archive_out = args.archive_out.resolve()
    metadata_out = args.metadata_out.resolve()
    workspace_root = Path(__file__).resolve().parent.parent
    private_root = workspace_root / "upstream"
    if archive_out == workspace_root or (
        archive_out.is_relative_to(workspace_root)
        and archive_out != private_root
        and not archive_out.is_relative_to(private_root)
    ):
        parser.error("inside this workspace, --archive-out must be under the ignored upstream/ directory")
    if archive_out == metadata_out or archive_out.is_relative_to(metadata_out):
        parser.error("raw capture and public metadata destinations must be separate")
    fetched_at = datetime.now(timezone.utc).astimezone().isoformat(timespec="seconds")
    urls = sitemap_urls()
    archive_out.mkdir(parents=True, exist_ok=True)
    metadata_out.mkdir(parents=True, exist_ok=True)
    records: list[PageRecord] = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=max(1, args.workers)) as pool:
        futures = {pool.submit(capture_page, url, archive_out, fetched_at): url for url in urls}
        for future in concurrent.futures.as_completed(futures):
            url = futures[future]
            record = future.result()
            records.append(record)
            print(f"captured {record.slug}: {record.html_bytes} bytes ({url})", flush=True)
    records.sort(key=lambda item: item.slug)
    (archive_out / "manifest.json").write_text(
        json.dumps([asdict(record) for record in records], ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
        newline="\n",
    )
    write_public_index(records, metadata_out, fetched_at)
    print(f"captured {len(records)} Chinese pages", flush=True)


if __name__ == "__main__":
    main()
