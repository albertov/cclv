#!/usr/bin/env python3
"""
Extract representative JSONL fixtures from large Claude Code session logs.

This script extracts ~100 representative entries from a large JSONL session log file,
ensuring coverage of different entry types (system, user, assistant, result) and
content block types (text, tool_use, tool_result, thinking).

Usage:
    python scripts/extract-fixtures.py INPUT [--output OUTPUT] [--limit LIMIT]

Examples:
    # Extract to stdout
    python scripts/extract-fixtures.py tests/fixtures/cc-session-log.jsonl

    # Extract to file
    python scripts/extract-fixtures.py tests/fixtures/cc-session-log.jsonl \\
        --output tests/fixtures/cc-session-sample.jsonl --limit 100

Arguments:
    INPUT                Input JSONL file path
    --output OUTPUT      Output file path (default: stdout)
    --limit LIMIT        Maximum number of lines to extract (default: 100)
"""

import argparse
import json
import random
import sys
from collections import defaultdict
from typing import Dict, List, TextIO


def classify_entry(line: str) -> tuple[str, dict]:
    """
    Classify a JSONL entry by type and content characteristics.

    Returns:
        (classification, parsed_json) where classification is one of:
        - "system:init" - Session initialization
        - "system:hook" - Hook response
        - "user:tool_result" - User message with tool results
        - "user:text" - User message with text
        - "assistant:tool_use" - Assistant with tool_use blocks
        - "assistant:thinking" - Assistant with thinking blocks
        - "assistant:text" - Assistant with text
        - "result" - Result entry type
        - "unknown" - Unrecognized format
    """
    try:
        entry = json.loads(line)
    except json.JSONDecodeError:
        return "invalid", {}

    entry_type = entry.get("type", "unknown")

    # System entries
    if entry_type == "system":
        subtype = entry.get("subtype", "")
        if subtype == "init":
            return "system:init", entry
        elif subtype == "hook_response":
            return "system:hook", entry
        else:
            return "system:other", entry

    # Result entries
    if entry_type == "result":
        return "result", entry

    # User and assistant messages - check content blocks
    message = entry.get("message", {})
    content = message.get("content", [])

    if not isinstance(content, list):
        # Text content (string)
        if entry_type == "user":
            return "user:text", entry
        elif entry_type == "assistant":
            return "assistant:text", entry
        else:
            return "unknown", entry

    # Analyze content blocks
    has_tool_use = any(
        block.get("type") == "tool_use" for block in content if isinstance(block, dict)
    )
    has_thinking = any(
        block.get("type") == "thinking"
        for block in content
        if isinstance(block, dict)
    )
    has_tool_result = any(
        block.get("type") == "tool_result"
        for block in content
        if isinstance(block, dict)
    )

    if entry_type == "user":
        if has_tool_result:
            return "user:tool_result", entry
        else:
            return "user:text", entry
    elif entry_type == "assistant":
        if has_thinking:
            return "assistant:thinking", entry
        elif has_tool_use:
            return "assistant:tool_use", entry
        else:
            return "assistant:text", entry

    return "unknown", entry


def extract_fixtures(
    input_file: TextIO, output_file: TextIO, limit: int = 100
) -> None:
    """
    Extract representative fixtures from input JSONL file.

    Strategy:
    1. Always include all system:init entries (typically 1-2)
    2. Sample from each category to get representative coverage
    3. Preserve original line order in output
    """
    # First pass: classify all entries and collect indices
    entries_by_category: Dict[str, List[tuple[int, str]]] = defaultdict(list)

    print(f"Reading and classifying entries from {input_file.name}...", file=sys.stderr)

    line_num = 0
    for line in input_file:
        line = line.strip()
        if not line:
            continue

        category, parsed = classify_entry(line)
        entries_by_category[category].append((line_num, line))
        line_num += 1

    print(f"Processed {line_num} total entries", file=sys.stderr)
    print("\nEntry distribution:", file=sys.stderr)
    for category in sorted(entries_by_category.keys()):
        count = len(entries_by_category[category])
        print(f"  {category}: {count}", file=sys.stderr)

    # Selection strategy
    selected_indices = set()

    # 1. Sample from system:init entries (include at least 2, up to 5 max)
    system_init = entries_by_category.get("system:init", [])
    init_count = min(5, max(2, len(system_init)))
    if len(system_init) > init_count:
        sampled_init = random.sample(system_init, init_count)
        for idx, line in sampled_init:
            selected_indices.add(idx)
    else:
        for idx, line in system_init:
            selected_indices.add(idx)

    remaining = limit - len(selected_indices)
    print(f"\nIncluded {len(selected_indices)} system:init entries", file=sys.stderr)
    print(f"Selecting {remaining} more entries...", file=sys.stderr)

    # 2. Define target distribution for remaining entries
    # Aim for realistic distribution similar to actual sessions
    target_distribution = {
        "user:tool_result": 0.15,  # ~15 entries
        "user:text": 0.15,  # ~15 entries
        "assistant:tool_use": 0.25,  # ~25 entries
        "assistant:thinking": 0.10,  # ~10 entries
        "assistant:text": 0.20,  # ~20 entries
        "result": 0.05,  # ~5 entries
        "system:hook": 0.05,  # ~5 entries
        "system:other": 0.05,  # ~5 entries
    }

    # Calculate target counts for each category
    for category, proportion in target_distribution.items():
        available = entries_by_category.get(category, [])
        if not available:
            continue

        target_count = max(1, int(remaining * proportion))
        # Don't sample more than available
        sample_count = min(target_count, len(available))

        # Random sample from this category
        sampled = random.sample(available, sample_count)
        for idx, line in sampled:
            selected_indices.add(idx)

    print(f"Selected {len(selected_indices)} total entries", file=sys.stderr)

    # Second pass: write selected entries in original order
    input_file.seek(0)
    line_num = 0
    written = 0

    for line in input_file:
        line = line.strip()
        if not line:
            continue

        if line_num in selected_indices:
            output_file.write(line + "\n")
            written += 1

        line_num += 1

    print(f"\nWrote {written} entries to {output_file.name}", file=sys.stderr)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Extract representative JSONL fixtures from Claude Code session logs",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__.split("Usage:")[1],
    )

    parser.add_argument("input", help="Input JSONL file path")
    parser.add_argument(
        "--output",
        "-o",
        help="Output file path (default: stdout)",
        default=None,
    )
    parser.add_argument(
        "--limit",
        "-l",
        type=int,
        default=100,
        help="Maximum number of lines to extract (default: 100)",
    )
    parser.add_argument(
        "--seed",
        "-s",
        type=int,
        default=None,
        help="Random seed for reproducible sampling (optional)",
    )

    args = parser.parse_args()

    # Set random seed if provided
    if args.seed is not None:
        random.seed(args.seed)

    # Open input file
    try:
        with open(args.input, "r", encoding="utf-8") as input_file:
            # Open output file or use stdout
            if args.output:
                with open(args.output, "w", encoding="utf-8") as output_file:
                    extract_fixtures(input_file, output_file, args.limit)
            else:
                extract_fixtures(input_file, sys.stdout, args.limit)
    except FileNotFoundError:
        print(f"Error: Input file not found: {args.input}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
