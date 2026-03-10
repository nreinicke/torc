#!/usr/bin/env python
"""Generate summary statistics from cleaned CSV data."""

import csv
import json
import os
import sys


def main():
    input_path = sys.argv[1]
    output_path = sys.argv[2]

    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    with open(input_path, "r") as f:
        reader = csv.DictReader(f)
        rows = list(reader)
        columns = reader.fieldnames

    summary = {
        "row_count": len(rows),
        "columns": columns,
        "sample_row": rows[0] if rows else None,
    }

    with open(output_path, "w") as f:
        json.dump(summary, f, indent=2)

    print("Generated summary statistics")


if __name__ == "__main__":
    main()
