#!/usr/bin/env python
"""Generate an HTML report from summary statistics."""

import json
import os
import sys


def main():
    summary_path = sys.argv[1]
    output_path = sys.argv[2]

    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    with open(summary_path, "r") as f:
        summary = json.load(f)

    html = f"""<!DOCTYPE html>
<html>
<head><title>Data Report</title></head>
<body>
  <h1>Data Processing Report</h1>
  <p>Rows processed: {summary["row_count"]}</p>
  <p>Columns: {", ".join(summary["columns"])}</p>
</body>
</html>"""

    with open(output_path, "w") as f:
        f.write(html)

    print("Generated HTML report")


if __name__ == "__main__":
    main()
