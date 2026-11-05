#!/usr/bin/env bash
# Regenerate the derived app icons from assets/app-icon/aptbim-1024.png (the logo's emblem on its
# tile): aptbim-256.png and aptbim-64.png, the hicolor PNGs Linux desktops read, aptbim.ico for
# Windows and aptbim.icns for macOS. Needs Python with Pillow, on any OS. The outputs are
# committed, so builds and packaging never need these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
ID="io.github.anuj09k.aptbim"
python3 - "$DIR" "$ID" <<'PY'
import os, sys
from PIL import Image
d, app_id = sys.argv[1], sys.argv[2]
master = Image.open(f"{d}/aptbim-1024.png").convert("RGBA")
size = lambda n: master.resize((n, n), Image.LANCZOS)
size(256).save(f"{d}/aptbim-256.png", optimize=True)
size(64).save(f"{d}/aptbim-64.png", optimize=True)
for n in (16, 24, 32, 48, 64, 128, 256, 512):
    os.makedirs(f"{d}/hicolor/{n}x{n}/apps", exist_ok=True)
    size(n).save(f"{d}/hicolor/{n}x{n}/apps/{app_id}.png", optimize=True)
# macOS draws app icons on Apple's grid, an 824 px body on a 1024 px canvas, where the tile fills
# 960: shrunk to 824 and centred, it sits as large as the Dock's other icons.
body = round(1024 * 824 / 960)
mac = Image.new("RGBA", (1024, 1024), (0, 0, 0, 0))
mac.paste(size(body), ((1024 - body) // 2, (1024 - body) // 2))
mac.save(f"{d}/aptbim-macos-1024.png", optimize=True)
# Pillow writes every size an .icns holds (16 to 1024, with the @2x ones) from the 1024 px master.
mac.save(f"{d}/aptbim.icns", format="ICNS")
master.save(f"{d}/aptbim.ico", format="ICO", sizes=[(n, n) for n in (16, 24, 32, 48, 64, 128, 256)])
PY
echo "icons written to $DIR"
