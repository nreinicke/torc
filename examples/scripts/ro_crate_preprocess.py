#!/usr/bin/env python
"""Clean and validate raw CSV data by removing rows with empty values and deduplicating."""

import csv
import sys


def main():
    input_path = sys.argv[1]
    output_path = sys.argv[2]

    with open(input_path, "r") as infile:
        reader = csv.DictReader(infile)
        rows = list(reader)
        fieldnames = reader.fieldnames

    # Remove rows with empty values and deduplicate
    seen = set()
    clean_rows = []
    for row in rows:
        if all(row.values()):  # Skip rows with empty values
            key = tuple(row.values())
            if key not in seen:
                seen.add(key)
                clean_rows.append(row)

    with open(output_path, "w", newline="") as outfile:
        writer = csv.DictWriter(outfile, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(clean_rows)

    print(f"Cleaned {len(clean_rows)} rows")


if __name__ == "__main__":
    main()
