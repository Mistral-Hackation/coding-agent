# build123d CAD Code Generator: Coder Knowledge Base

## build123d Framework Overview

build123d is a Python-based, parametric (BREP) CAD framework built on Open Cascade.
It provides a Pythonic interface for creating precise engineering models.

### Standard Import
```python
from build123d import *
```

### Key Constants
- All dimensions are in **millimeters** (mm) by default
- Coordinate system: X=right, Y=forward, Z=up

---

## Core API Patterns

### Builder Mode (Context Managers)

```python
# 3D Part (BuildPart)
with BuildPart() as part:
    Box(length=100, width=50, height=20)
    Cylinder(radius=10, height=30)
    fillet(part.edges(), radius=2)

# 2D Sketch (BuildSketch)
with BuildSketch() as sketch:
    Circle(radius=25)
    Rectangle(width=40, height=20, mode=Mode.SUBTRACT)

# 1D Line (BuildLine)
with BuildLine() as line:
    Line((0, 0), (10, 0))
    Line((10, 0), (10, 10))
```

### Algebra Mode (Direct Operations)

```python
# Boolean operations using operators
result = Box(10, 20, 5) - Cylinder(radius=3, height=10)
result = Box(10, 10, 10) + Cylinder(radius=5, height=15)
result = Box(10, 10, 10) & Cylinder(radius=8, height=10)  # intersection
```

---

## 3D Primitives (BuildPart Context)

| Primitive | Example |
|-----------|---------|
| Box | `Box(length=100, width=50, height=20)` |
| Cylinder | `Cylinder(radius=25, height=50)` |
| Sphere | `Sphere(radius=10)` |
| Cone | `Cone(bottom_radius=20, top_radius=5, height=30)` |
| Torus | `Torus(major_radius=50, minor_radius=10)` |
| Wedge | `Wedge(dx=10, dy=20, dz=5, xmin=2, xmax=8)` |

---

## 2D Primitives (BuildSketch Context)

| Primitive | Example |
|-----------|---------|
| Circle | `Circle(radius=25)` |
| Ellipse | `Ellipse(x_radius=30, y_radius=15)` |
| Rectangle | `Rectangle(width=40, height=20)` |
| RegularPolygon | `RegularPolygon(radius=20, side_count=6)` |
| SlotOverall | `SlotOverall(height=30, width=10)` |
| Text | `Text("Hello", font_size=10)` |

---

## Operations

### Extrude
```python
with BuildPart() as part:
    with BuildSketch() as sketch:
        Circle(radius=25)
    extrude(amount=50)                    # Simple extrude
    extrude(amount=-10, mode=Mode.SUBTRACT)  # Cut from bottom
```

### Revolve
```python
with BuildPart() as part:
    with BuildSketch(Plane.XZ) as sketch:
        with BuildLine() as line:
            Line((10, 0), (30, 0))
            Line((30, 0), (30, 20))
            Line((30, 20), (10, 20))
            Line((10, 20), (10, 0))
        make_face()
    revolve(axis=Axis.Z)
```

### Sweep
```python
with BuildPart() as part:
    with BuildLine() as path:
        Spline((0, 0, 0), (10, 0, 5), (20, 0, 0))
    with BuildSketch(Plane(origin=path @ 0, z_dir=path % 0)) as sweep_profile:
        Circle(radius=2)
    sweep()
```

### Loft
```python
with BuildPart() as part:
    with BuildSketch() as bottom:
        Rectangle(width=40, height=40)
    with BuildSketch(Plane.XY.offset(50)) as top:
        Circle(radius=15)
    loft()
```

### Fillet & Chamfer
```python
with BuildPart() as part:
    Box(100, 50, 20)
    fillet(part.edges().filter_by(Axis.Z), radius=5)
    chamfer(part.edges().sort_by(Axis.Z)[-1], length=2)
```

---

## Selectors (Topology Access)

### Accessing Topology
```python
part.faces()      # All faces
part.edges()      # All edges
part.vertices()   # All vertices
part.wires()      # All wires
```

### Filtering
```python
edges().filter_by(Axis.Z)           # Edges parallel to Z
faces().filter_by(Plane.XY)         # Faces on XY plane
edges().filter_by(GeomType.CIRCLE)  # Circular edges
```

### Sorting & Grouping
```python
edges().sort_by(Axis.Z)     # Sort by Z position
edges().sort_by(Axis.Z)[-1] # Highest Z edge
faces().group_by(Axis.Z)    # Group faces by Z height
faces().group_by(Axis.Z)[0] # Bottom group of faces
```

---

## Location Patterns

### Linear
```python
with Locations((10, 0, 0), (20, 0, 0), (30, 0, 0)):
    Cylinder(radius=3, height=10, mode=Mode.SUBTRACT)
```

### Polar (Bolt Pattern)
```python
with PolarLocations(bolt_circle_radius=40, count=8):
    Cylinder(radius=5, height=20, mode=Mode.SUBTRACT)
```

### Grid
```python
with GridLocations(x_spacing=20, y_spacing=20, x_count=3, y_count=3):
    Cylinder(radius=2, height=10, mode=Mode.SUBTRACT)
```

---

## Export Patterns

```python
# STEP (for CAD interchange) — ALWAYS include this
export_step(part.part, "output.step")

# STL (for 3D printing / 3D viewer) — ALWAYS include this
export_stl(part.part, "output.stl")
```

> **⚠️ CRITICAL: `export_svg` does NOT exist for 3D Part objects.**
> There is NO `export_svg()` function in build123d for 3D parts.
> Do NOT attempt to call `export_svg()` on a Part — it will raise `NameError`.
> Only `export_step()` and `export_stl()` are valid for 3D geometry export.

### Required Export Template (copy this exactly)
```python
if __name__ == "__main__":
    result = create_my_part()  # Your function
    export_step(result, "output.step")
    export_stl(result, "output.stl")
```

---

## Common Pitfalls

1. **Missing `make_face()`**: After BuildLine inside BuildSketch, you must call `make_face()` to close the profile.
2. **Wrong Plane**: Default sketch plane is `Plane.XY`. Use `Plane.XZ` or `Plane.YZ` for vertical sketches.
3. **Fillet too large**: Fillet radius cannot exceed half the edge length. Use a guard: `min(radius, edge_length / 2)`.
4. **Empty extrude**: If sketch area is zero, extrude will fail silently. Always verify sketch geometry.
5. **Mode.SUBTRACT before geometry**: The subtract only works on an existing body in the same BuildPart context.
6. **Locations context**: `Locations`, `PolarLocations`, `GridLocations` must be used inside BuildPart, not standalone.
7. **Using `export_svg()`**: This function does NOT exist for 3D Parts. Only use `export_step()` and `export_stl()`.
8. **Missing STL export**: Always include `export_stl()` alongside `export_step()` — STL is needed for the 3D viewer.

