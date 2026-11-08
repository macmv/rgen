#!/usr/bin/env python3
"""Split the Golden Grass 5x2 atlas into its short and tall block textures.

Requires Pillow (`python3 -m pip install Pillow`). Run from any directory:
`python3 split_textures.py`
"""

from pathlib import Path

from PIL import Image


CELL_SIZE = 16
SHEET_SIZE = (80, 32)


def save_cell(image: Image.Image, column: int, row: int, output: Path) -> None:
    left, top = column * CELL_SIZE, row * CELL_SIZE
    image.crop((left, top, left + CELL_SIZE, top + CELL_SIZE)).save(output)


def main() -> None:
    directory = Path(__file__).resolve().parent
    with Image.open(directory / "golden_grass.png") as image:
        if image.size != SHEET_SIZE:
            raise ValueError(f"golden_grass.png must be {SHEET_SIZE[0]}x{SHEET_SIZE[1]}, got {image.width}x{image.height}")

        short_dir = directory / "single"
        tall_dir = directory / "double"
        short_dir.mkdir(exist_ok=True)
        tall_dir.mkdir(exist_ok=True)

        for variant, column in enumerate(range(3), start=1):
            save_cell(image, column, 1, short_dir / f"golden_grass_{variant}.png")

        for variant, column in enumerate(range(3, 5), start=1):
            save_cell(image, column, 1, tall_dir / f"golden_grass_{variant}_bottom.png")
            save_cell(image, column, 0, tall_dir / f"golden_grass_{variant}_top.png")


if __name__ == "__main__":
    main()
