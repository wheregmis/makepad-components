#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shutil
import tempfile
import urllib.error
import urllib.request
import zipfile
from pathlib import Path

ARCHIVE_URL = "https://github.com/lucide-icons/lucide/archive/refs/heads/main.zip"


def download_icons(output_dir: Path, clean: bool) -> int:
    if clean and output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix="lucide-icons-") as temp_dir:
        archive_path = Path(temp_dir) / "lucide-main.zip"
        try:
            urllib.request.urlretrieve(ARCHIVE_URL, archive_path)
        except urllib.error.HTTPError as error:
            raise RuntimeError(
                f"Failed to download Lucide icons from {ARCHIVE_URL}: "
                f"HTTP {error.code} {error.reason}"
            ) from error
        except urllib.error.URLError as error:
            reason = getattr(error, "reason", error)
            raise RuntimeError(
                f"Failed to download Lucide icons from {ARCHIVE_URL}: Network error: {reason}"
            ) from error

        count = 0
        with zipfile.ZipFile(archive_path) as archive:
            icon_members = sorted(
                member
                for member in archive.namelist()
                if member.startswith("lucide-main/icons/") and member.endswith(".svg")
            )
            if not icon_members:
                raise RuntimeError(
                    "No Lucide icons found in downloaded archive. "
                    "Expected files matching pattern 'lucide-main/icons/*.svg'."
                )

            for index, member in enumerate(icon_members, start=1):
                destination = output_dir / Path(member).name
                with archive.open(member) as source, destination.open("wb") as target:
                    shutil.copyfileobj(source, target)
                count += 1
                if index % 100 == 0:
                    print(f"Downloaded {index}/{len(icon_members)} icons...")

    print(f"Downloaded {count} icons into {output_dir}")
    return count


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Download all Lucide SVG icons into makepad-icon resources."
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path(__file__).resolve().parents[1] / "resources" / "icons",
        help="Destination directory for SVG files.",
    )
    parser.add_argument(
        "--clean",
        action="store_true",
        help="Delete destination directory before downloading icons.",
    )
    args = parser.parse_args()

    download_icons(args.output_dir, clean=args.clean)


if __name__ == "__main__":
    main()
