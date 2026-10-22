#!/usr/bin/env python3
"""
Generates the fabric assets that the 1.12 assets don't have: an item model for every item without
one (1.12 named item models after the variant, like `fir_log`), and the language file.

The rest of the assets are copied from `common/` by `processResources` in `mod-fabric/build.gradle`.
Run this from the root of the repo after adding blocks or items.
"""

import json
import re
from pathlib import Path

root = Path(__file__).resolve().parent.parent
common = root / "common/src/main/resources/assets/rgen"
out = root / "mod-fabric/src/main/resources/assets/rgen"
java = root / "common/src/main/java/net/macmv/rgen"

blocks_src = (java / "block/RBlocks.java").read_text()
items_src = (java / "item/RItems.java").read_text()

block_names = re.findall(r'register\("([a-z0-9_]+)"', blocks_src)
# Backported blocks are never registered.
block_names += []
item_names = re.findall(r'registerItem\("([a-z0-9_]+)"', items_src)

# The blocks that are only registered when vanilla doesn't have them are still listed here, which
# is harmless: the extra files are never loaded.

def first_model(name):
    path = common / "blockstates" / f"{name}.json"
    if not path.exists():
        return None
    state = json.loads(path.read_text())
    if "variants" in state:
        entry = next(iter(state["variants"].values()))
    elif "multipart" in state:
        entry = state["multipart"][0]["apply"]
    else:
        return None
    if isinstance(entry, list):
        entry = entry[0]
    return entry["model"]

def fix(path):
    # Blockstates are relative to `models/block` on 1.12.
    return path.replace("rgen:", "rgen:block/")

(out / "models/item").mkdir(parents=True, exist_ok=True)
(out / "lang").mkdir(parents=True, exist_ok=True)

for name in block_names:
    if (common / "models/item" / f"{name}.json").exists():
        continue
    model = first_model(name)
    if model is None:
        print(f"no model for block {name}")
        continue
    (out / "models/item" / f"{name}.json").write_text(
        json.dumps({"parent": fix(model)}, indent=2) + "\n"
    )

def title(name):
    return " ".join(word.capitalize() for word in name.split("_"))

lang = {}
for name in block_names:
    lang[f"block.rgen.{name}"] = title(name)
for name in item_names:
    if name in block_names:
        continue
    lang[f"item.rgen.{name}"] = title(name)

(out / "lang/en_us.json").write_text(json.dumps(lang, indent=2, sort_keys=True) + "\n")
