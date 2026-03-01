# build123d CAD Code Generator: Reviewer Knowledge Base

## Code Quality Standards

### Python Best Practices
- Use type hints for function parameters and return values
- Docstrings for all public functions (Google style or NumPy style)
- Consistent naming: `snake_case` for functions/variables, `PascalCase` for classes
- No magic numbers — use named constants or parameters
- Proper error handling for file I/O operations

### build123d-Specific Quality Checks

#### Import Pattern
✅ Good: `from build123d import *`
❌ Bad: Cherry-picking individual imports (clutters the code)

#### Builder Context Usage
✅ Good: `with BuildPart() as part:`
❌ Bad: Using global namespace for part creation

#### Selector Pattern
✅ Good: `part.edges().filter_by(Axis.Z).sort_by(Axis.Z)[-1]`
❌ Bad: Hardcoding edge indices (fragile, breaks on topology changes)

---

## Review Checklist

### Stage 1: Structural Review
- [ ] All imports present (`from build123d import *`, `math` if needed)
- [ ] Uses Builder mode or Algebra mode consistently (don't mix)
- [ ] Proper `with` context managers for all builders
- [ ] Functions are parameterized (no hardcoded dimensions)
- [ ] Export call at the end (`export_step` or `export_stl`)

### Stage 2: Topology Review
- [ ] `make_face()` called after BuildLine inside BuildSketch
- [ ] Selectors reference correct topology (edges, faces, vertices)
- [ ] Fillet/chamfer radii are reasonable (not exceeding edge length / 2)
- [ ] Boolean operations have target geometry to subtract from
- [ ] Sketch planes are correct (XY for horizontal, XZ or YZ for vertical)

### Stage 3: Engineering Review
- [ ] Dimensions are in mm (build123d default)
- [ ] Wall thicknesses are realistic for the application
- [ ] Thread specifications match standards (if applicable)
- [ ] Tolerances are appropriate for manufacturing
- [ ] Assembly relationships are correct (if multi-part)

---

## Common Code Smells

1. **Unnamed Part**: `with BuildPart():` — always name it: `with BuildPart() as part:`
2. **Missing Export**: No `export_step()` or `export_stl()` at the end
3. **Hardcoded Geometry**: Dimensions as literals instead of parameters
4. **No Docstring**: Missing description of what the script creates
5. **Overly Complex Single Part**: Parts with >50 operations should be split
6. **Unused Constructs**: Sketches or lines created but never extruded or swept
7. **⚠️ Using `export_svg()`**: This function does NOT exist for 3D Parts. It will cause `NameError: name 'export_svg' is not defined`. **REJECT any code that calls `export_svg()` on Part objects.** Only `export_step()` and `export_stl()` are valid.
8. **Missing STL Export**: Always require `export_stl()` alongside `export_step()` — STL is needed for the 3D viewer.

---

## Approval Criteria

For `APPROVED:` status, ALL of the following must be met:
1. Code is syntactically valid Python
2. All imports are present
3. Uses build123d API correctly
4. All features from the objective are implemented
5. Code is parameterized with good defaults
6. Export includes BOTH `export_step()` AND `export_stl()`
7. **No `export_svg()` calls on Part objects** (this function does not exist)
8. Inline comments explain non-obvious logic
