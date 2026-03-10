#!/usr/bin/env python
"""Transform cleaned CSV data to JSON format."""

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

    with open(output_path, "w") as f:
        json.dump(rows, f, indent=2)

    print(f"Converted {len(rows)} rows to JSON")


if __name__ == "__main__":
    main()
