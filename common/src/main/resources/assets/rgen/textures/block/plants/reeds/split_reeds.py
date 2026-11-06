from pathlib import Path
from PIL import Image


HERE = Path(__file__).parent
SOURCE = HERE / "reeds.png"
OUTPUT = HERE / "cells"

# Source atlas is 4 columns by 3 rows, with row 0 at the top.
CELLS = {
    "single_0": (0, 1),
    "single_1": (1, 1),
    "bottom_2": (2, 1),
    "bottom_3": (3, 1),
    "top_2": (2, 0),
    "top_3": (3, 0),
    "roots_0": (0, 2),
    "roots_3": (3, 2),
}


def main():
    atlas = Image.open(SOURCE).convert("RGBA")
    if atlas.size != (64, 48):
        raise ValueError(f"Expected a 64x48 atlas, got {atlas.size}")

    OUTPUT.mkdir(exist_ok=True)
    for name, (column, row) in CELLS.items():
        box = (column * 16, row * 16, (column + 1) * 16, (row + 1) * 16)
        atlas.crop(box).save(OUTPUT / f"{name}.png")


if __name__ == "__main__":
    main()
