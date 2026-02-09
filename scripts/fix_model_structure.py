#!/usr/bin/env python3
"""Fix ModelStructure element ordering in FMI 3.0 modelDescription.xml.

Workaround for https://github.com/jondo2010/rust-fmi/issues/XXX
fmi-export v0.1.1 interleaves ContinuousStateDerivative and InitialUnknown
elements per-variable, but the FMI 3.0 schema requires them grouped by type:
  Output*, ContinuousStateDerivative*, ClockedState*, InitialUnknown*, EventIndicator*
"""

import sys
import xml.etree.ElementTree as ET
import zipfile
import io
import shutil
import tempfile
import os

ELEMENT_ORDER = {
    "Output": 0,
    "ContinuousStateDerivative": 1,
    "ClockedState": 2,
    "InitialUnknown": 3,
    "EventIndicator": 4,
}


def fix_model_structure(xml_bytes: bytes) -> bytes:
    root = ET.fromstring(xml_bytes)
    ms = root.find("ModelStructure")
    if ms is not None:
        children = list(ms)
        sorted_children = sorted(children, key=lambda e: ELEMENT_ORDER.get(e.tag, 99))
        # Check if reordering is needed
        if [c.tag for c in children] != [c.tag for c in sorted_children]:
            ms[:] = sorted_children
    return ET.tostring(root, encoding="UTF-8", xml_declaration=True)


def fix_fmu(fmu_path: str) -> bool:
    """Fix the modelDescription.xml inside an FMU. Returns True if modified."""
    with zipfile.ZipFile(fmu_path, "r") as zin:
        xml_bytes = zin.read("modelDescription.xml")

    fixed_xml = fix_model_structure(xml_bytes)
    if fixed_xml == xml_bytes:
        return False

    # Rewrite the FMU with fixed XML
    tmp_fd, tmp_path = tempfile.mkstemp(suffix=".fmu", dir=os.path.dirname(fmu_path))
    os.close(tmp_fd)
    try:
        with zipfile.ZipFile(fmu_path, "r") as zin:
            with zipfile.ZipFile(tmp_path, "w", zipfile.ZIP_DEFLATED) as zout:
                for item in zin.infolist():
                    if item.filename == "modelDescription.xml":
                        zout.writestr(item, fixed_xml)
                    else:
                        zout.writestr(item, zin.read(item.filename))
        shutil.move(tmp_path, fmu_path)
    except Exception:
        os.unlink(tmp_path)
        raise

    return True


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <fmu-file> [<fmu-file> ...]", file=sys.stderr)
        sys.exit(1)

    for fmu_path in sys.argv[1:]:
        if fix_fmu(fmu_path):
            print(f"  Fixed ModelStructure ordering in {fmu_path}")
        else:
            print(f"  ModelStructure already valid in {fmu_path}")
