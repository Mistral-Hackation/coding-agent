# Post-Mortem: Blender Automation Script Failures (`electrical_cabinet.py`)

**Date:** January 26, 2026  
**Author:** Senior SRE / Debugging Expert  
**Status:** Resolved  
**Component:** 3D Asset Pipeline  

---

## 1. Problem Statement
The automated generation of the Electrical Enclosure Cabinet failed across multiple dimensions during execution.

### Symptoms & Error Messages:
- **API Mismatch:** `KeyError: 'bpy_prop_collection[key]: key "Transmission" not found'` during material creation.
- **Environment Conflict:** `AttributeError: 'NoneType' object has no attribute 'areas'` when running in headless/background mode.
- **Logic Defect:** malformed geometry (window cutout) due to incorrect Euler rotations.
- **Regression:** `NameError: name 'tabs' is not defined` after a refactoring step.

---

## 2. Root Cause Analysis (RCA)

### Why it happened:
1. **API Schema Drift:** The script was developed using legacy Blender API conventions (< 4.0). Blender 4.0+ refactored the "Principled BSDF" node, renaming the `Transmission` input to `Transmission Weight`.
2. **Context Assumption:** The script assumed it was running within a graphical user interface (GUI). In background mode (`--background`), `bpy.context.screen` is `None`, making viewport shading operations invalid.
3. **Geometric Redundancy:** The `create_rounded_rectangle_mesh` function already generated meshes in the target orientation. A secondary rotation was applied in the main logic, resulting in a 90-degree misalignment.
4. **Human Error:** During a code cleanup phase, the initialization of the `tabs` list was inadvertently removed while cleaning up redundant `positions` definitions.

### Discovery:
- The issues were identified through iterative execution using the Blender CLI and interpreting traceback logs provided by the Blender Python interpreter.

---

## 3. Solution

### The Fix:
- **Resilient Material Logic:** Updated `create_material` to use a `.get()` approach for shader inputs, ensuring compatibility across Blender 3.x and 4.x.
- **Headless Safety:** Wrapped viewport settings in a conditional check for `bpy.context.screen` and a `try/except` block to allow silent failure in non-interactive environments.
- **Geometric Correction:** Removed the redundant `rotation_euler` assignments for the window components.
- **State Restoration:** Restored the `tabs = []` initialization.

### Code Diff (Summary):
```python
# Material API Compatibility
transmission_input = principled.inputs.get('Transmission Weight') or principled.inputs.get('Transmission')
if transmission_input:
    transmission_input.default_value = transmission

# Headless mode safety
try:
    if bpy.context.screen:
        # Iterate areas for shading...
except AttributeError:
    pass

# Correcting window alignment
- window_cutter.rotation_euler.x = math.radians(90)
```

---

## 4. Prevention

1. **Environment Awareness:** Proactively check `bpy.app.background` before executing any UI/Viewport code.
2. **Defensive Programming:** Use `.get()` or `hasattr()` when accessing Blender node sockets, as these are prone to name changes between LTS versions.
3. **Automated Linting:** Utilize `ruff` or `pylint` with the `bpy` stubs to catch `NameError` and unused variables before the script is passed to the Blender engine.
4. **Version Manifest:** Include a `Requirements` comment or a `blender_version` check at the top of the script to warn users of potential API drift.
