# Intent Reviewer Knowledge Base

> Internal reference for the IntentReviewer agent when verifying user intent is satisfied.

## 🎯 Role Summary

The IntentReviewer ensures that the generated build123d CAD script fully implements the user's original objective — every requested feature, dimension, and output format.

---

## 📚 Intent Verification Framework

### Core Questions

1. **What geometry did the user ask for?**
   - Extract key nouns (box, flange, fitting, housing, bracket)
   - Identify shape operations (extrude, revolve, fillet, chamfer, hole)
   - Note quantity patterns (bolt pattern, array, mirror)

2. **What constraints were specified?**
   - Dimensions (length=100mm, diameter=30mm, wall_thickness=5mm)
   - Features (threaded, reinforced, hollow, through-hole)
   - Relationships (concentric, coplanar, perpendicular)

3. **What is the output context?**
   - CAD interchange → needs STEP export
   - 3D printing → needs STL export
   - 2D drawing → needs SVG export
   - Parametric reuse → needs function parameters

---

## 🔧 Specification Matching

### Dimension Verification

User says: `"100mm x 60mm x 30mm box with 5mm fillets"`

**Checklist:**
- [ ] Box dimensions match: length=100, width=60, height=30
- [ ] Fillet radius = 5mm applied to all edges
- [ ] Fillet doesn't exceed min(edge_length)/2 constraint
- [ ] Units are in mm (build123d default)

### Feature Verification

User says: `"parametric flange with 8 bolt holes on a 120mm bolt circle"`

**Checklist:**
- [ ] Flange body present (Cylinder or extruded Circle)
- [ ] 8 holes created with PolarLocations(radius=60, count=8)
- [ ] Holes use Cylinder with mode=Mode.SUBTRACT
- [ ] Bolt circle diameter = 120mm (radius=60mm)

### Topology Verification

User says: `"export to STEP for CAD analysis"`

**Checklist:**
- [ ] `export_step(part.part, "filename.step")` present
- [ ] Solid is watertight (no open shells)
- [ ] No degenerate geometry (zero-area faces)
- [ ] Part reference is correct (part.part, not part)

---

## 📐 Common User Intent Patterns

### Industrial Equipment
**Keywords:** fitting, valve, pump, pipe, flange, vessel, bearing, coupling
**Expectations:**
- Accurate dimensions to engineering standards
- Functional geometry (holes align, threads match)
- Parametric design with named dimensions
- STEP export for downstream CAD use

### Mechanical Parts
**Keywords:** bracket, housing, mount, spacer, bushing, gear
**Expectations:**
- Precise hole patterns and mounting features
- Proper fillet/chamfer for stress relief
- Manufacturable geometry (draft angles, tool access)
- Clean topology for FEA analysis

### Quick Prototypes
**Keywords:** simple, basic, prototype, concept, sketch
**Expectations:**
- Functional geometry, aesthetic details optional
- Reasonable defaults for unspecified dimensions
- STL export suitable for 3D printing
- Comments explaining design choices

---

## 🔍 Completeness Checklist

### Geometry Requirements
- [ ] All requested shapes created
- [ ] Specified dimensions applied correctly
- [ ] Boolean operations produce expected result
- [ ] Fillets/chamfers on specified edges

### Code Requirements
- [ ] `from build123d import *` import present
- [ ] Builder mode used correctly (BuildPart/BuildSketch/BuildLine)
- [ ] Dimensions are parameters, not hardcoded
- [ ] Export call present with correct format
- [ ] Descriptive docstring explaining what the script creates

### Output Requirements
- [ ] Script produces a valid solid (no open geometry)
- [ ] Exported file format matches user request
- [ ] Print statement confirms success
- [ ] Code is self-documenting with comments

---

## ⚠️ Intent Mismatch Red Flags

1. **Missing Components**: User asked for X+Y, only X present
2. **Wrong Scale**: Dimensions off by factor of 10 or 1000
3. **Incomplete Features**: "bolt holes" mentioned but flat surface created
4. **Wrong Export**: User wants STEP, script exports STL (or vice versa)
5. **Not Parametric**: Hardcoded values when user said "parametric"
6. **Over-Engineering**: Simple ask, complex unnecessary solution
7. **Missing Fillet/Chamfer**: User specified edge treatment, not applied

---

## 📋 Approval Criteria

**APPROVE** if:
- All specified geometry is present and correct
- Dimensions match specifications (within tolerance)
- Export format matches user request
- Code is parametric with sensible defaults
- Script runs without errors

**REVISE** if:
- Missing requested features or geometry
- Dimension errors > specified tolerance
- Wrong export format
- Hardcoded values that should be parameters
- Script produces invalid geometry
