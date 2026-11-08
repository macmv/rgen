#!/usr/bin/env python3
"""Split the dried grass sprite sheets into single and double plant textures.

Requires Pillow (`python3 -m pip install Pillow`). Run from any directory:
`python3 split_textures.py`
"""

from pathlib import Path

from PIL import Image


CELL_SIZE = 16
SHEET_COLUMNS = 5
SHEET_ROWS = 2
COLORS = ("brown", "green", "yellow")


def save_cell(sheet: Image.Image, column: int, row: int, output: Path) -> None:
    left = column * CELL_SIZE
    top = row * CELL_SIZE
    sheet.crop((left, top, left + CELL_SIZE, top + CELL_SIZE)).save(output)


def split_color(color: str, directory: Path) -> None:
    source = directory / f"{color}.png"
    with Image.open(source) as image:
        if image.size != (SHEET_COLUMNS * CELL_SIZE, SHEET_ROWS * CELL_SIZE):
            expected = f"{SHEET_COLUMNS * CELL_SIZE}x{SHEET_ROWS * CELL_SIZE}"
            raise ValueError(f"{source} must be {expected}, got {image.width}x{image.height}")

        single_dir = directory / "single"
        double_dir = directory / "double"
        single_dir.mkdir(exist_ok=True)
        double_dir.mkdir(exist_ok=True)

        # Bottom row, columns 0-2: the three single-height variants.
        for variant, column in enumerate(range(3), start=1):
            save_cell(image, column, 1, single_dir / f"{color}_{variant}.png")

        # Top row has double-height tops; bottom row has their matching bottoms.
        for variant, column in enumerate(range(3, 5), start=1):
            save_cell(image, column, 0, double_dir / f"{color}_{variant}_top.png")
            save_cell(image, column, 1, double_dir / f"{color}_{variant}_bottom.png")


def main() -> None:
    directory = Path(__file__).resolve().parent
    for color in COLORS:
        split_color(color, directory)


if __name__ == "__main__":
    main()
