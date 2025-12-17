#!/usr/bin/env python3
"""
Fetch all Zed DeepWiki pages and save as markdown files.
"""

import urllib.request
import os
import html2text
import time
import re

# All pages to fetch
PAGES = [
    # 1. Overview
    "1-overview",

    # 2. Core Architecture
    "2-core-architecture",
    "2.1-application-initialization-and-lifecycle",
    "2.2-gpui-framework",
    "2.3-window-and-platform-abstraction",
    "2.4-event-flow-and-input-handling",
    "2.5-keybinding-and-action-system",
    "2.6-focus-management-and-hit-testing",

    # 3. Editor Architecture
    "3-editor-architecture",
    "3.1-editor-component-and-ui",
    "3.2-buffer-system-and-text-storage",
    "3.3-display-pipeline-and-rendering",
    "3.4-selections-and-editing-operations",
    "3.5-code-intelligence-integration",
    "3.6-diff-integration",

    # 4. Workspace and Panel System
    "4-workspace-and-panel-system",
    "4.1-workspace-organization",
    "4.2-item-system-and-lifecycle",
    "4.3-pane-management",
    "4.4-search-system",

    # 5. Project Management
    "5-project-management",
    "5.1-project-orchestration",
    "5.2-worktree-and-file-system",
    "5.3-buffer-store",

    # 6. Language Intelligence
    "6-language-intelligence",
    "6.1-lsp-store-architecture",
    "6.2-language-server-lifecycle",
    "6.3-completions-and-diagnostics",
    "6.4-multi-language-server-coordination",

    # 7. Settings and Configuration
    "7-settings-and-configuration",
    "7.1-settings-store-and-layering",
    "7.2-settings-ui",
    "7.3-settings-migration",
    "7.4-keymap-system",

    # 8. Git Integration
    "8-git-integration",
    "8.1-git-panel-and-ui",
    "8.2-git-store-and-state-management",
    "8.3-repository-operations",
    "8.4-diff-system",

    # 9. Terminal and Task Execution
    "9-terminal-and-task-execution",
    "9.1-terminal-core",
    "9.2-terminal-view-and-rendering",
    "9.3-task-system",

    # 10. Vim Mode
    "10-vim-mode",
    "10.1-mode-state-machine",
    "10.2-operators-motions-and-objects",
    "10.3-visual-mode",
    "10.4-helix-mode-integration",

    # 11. AI Agent System
    "11-ai-agent-system",
    "11.1-agent-communication-protocol-(acp)",
    "11.2-agent-ui-and-thread-management",
    "11.3-agent-connection-and-implementations",
    "11.4-tool-system",
    "11.5-mention-system-and-context",
    "11.6-legacy-agent-thread-system",

    # 12. Remote Development and Collaboration
    "12-remote-development-and-collaboration",
    "12.1-local-vs-remote-architecture",
    "12.2-remote-project-architecture",
    "12.3-collaboration-features",
    "12.4-crdt-and-synchronization",
]

BASE_URL = "https://deepwiki.com/zed-industries/zed"
OUTPUT_DIR = "/Users/manavpanchal/Desktop/Projects/moodboard/zed-deepwiki-docs"


def setup_html2text():
    """Configure html2text for optimal markdown output."""
    h = html2text.HTML2Text()
    h.ignore_links = False
    h.ignore_images = False
    h.ignore_emphasis = False
    h.body_width = 0  # Don't wrap lines
    h.unicode_snob = True
    h.skip_internal_links = False
    h.inline_links = True
    h.protect_links = True
    h.ignore_tables = False
    h.single_line_break = False
    return h


def clean_markdown(markdown_text, page_name):
    """Clean up the markdown output."""
    # Remove excessive blank lines
    markdown_text = re.sub(r'\n{4,}', '\n\n\n', markdown_text)

    # Add title if not present
    if not markdown_text.strip().startswith('#'):
        title = page_name.replace('-', ' ').title()
        markdown_text = f"# {title}\n\n{markdown_text}"

    # Add source URL at the top
    source_url = f"{BASE_URL}/{page_name}"
    header = f"<!-- Source: {source_url} -->\n\n"

    return header + markdown_text.strip() + "\n"


def fetch_page(page_name, converter):
    """Fetch a single page and convert to markdown."""
    url = f"{BASE_URL}/{page_name}"

    try:
        req = urllib.request.Request(
            url,
            headers={
                'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'
            }
        )

        with urllib.request.urlopen(req, timeout=30) as response:
            html = response.read().decode('utf-8')

        # Convert to markdown
        markdown = converter.handle(html)

        # Clean up
        markdown = clean_markdown(markdown, page_name)

        return markdown

    except Exception as e:
        print(f"  ERROR: {e}")
        return None


def main():
    """Main function to fetch all pages."""
    # Create output directory
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    # Setup converter
    converter = setup_html2text()

    print(f"Fetching {len(PAGES)} DeepWiki pages...")
    print(f"Output directory: {OUTPUT_DIR}")
    print("-" * 60)

    success_count = 0
    failed_pages = []

    for i, page in enumerate(PAGES, 1):
        # Create safe filename (handle parentheses in URLs)
        filename = page.replace('(', '').replace(')', '') + ".md"
        output_path = os.path.join(OUTPUT_DIR, filename)

        print(f"[{i}/{len(PAGES)}] Fetching {page}...", end=" ", flush=True)

        markdown = fetch_page(page, converter)

        if markdown:
            with open(output_path, 'w', encoding='utf-8') as f:
                f.write(markdown)
            print(f"OK ({len(markdown)} chars)")
            success_count += 1
        else:
            print("FAILED")
            failed_pages.append(page)

        # Small delay to be nice to the server
        time.sleep(0.5)

    print("-" * 60)
    print(f"Completed: {success_count}/{len(PAGES)} pages")

    if failed_pages:
        print(f"\nFailed pages ({len(failed_pages)}):")
        for page in failed_pages:
            print(f"  - {page}")

    print(f"\nFiles saved to: {OUTPUT_DIR}")


if __name__ == "__main__":
    main()
