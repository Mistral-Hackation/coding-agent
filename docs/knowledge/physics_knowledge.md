# build123d CAD Code Generator: Physics & Geometry Knowledge Base

## Geometry Verification

### BREP Topology Hierarchy
```
Compound → Solid → Shell → Face → Wire → Edge → Vertex
```

Each level must be valid:
- **Solid**: Closed, watertight boundary (no gaps between faces)
- **Shell**: Connected collection of faces forming a closed surface
- **Face**: Bounded 2D surface with well-defined normal
- **Wire**: Connected sequence of edges forming a closed loop
- **Edge**: Bounded 1D curve

### Topology Validation Checks
1. **Watertight**: Solid must be fully closed (no gaps in shell)
2. **No Self-Intersection**: Faces must not penetrate each other
3. **Correct Normals**: Face normals must point outward consistently
4. **Manifold**: Each edge shared by exactly 2 faces
5. **Degenerate Geometry**: No zero-area faces or zero-length edges

---

## Engineering Standards

### Thread Specifications (ISO 261/262)
| Size | Pitch (mm) | Major Ø (mm) | Minor Ø (mm) |
|------|-----------|-------------|-------------|
| M6   | 1.0       | 6.000       | 4.917       |
| M8   | 1.25      | 8.000       | 6.647       |
| M10  | 1.5       | 10.000      | 8.376       |
| M12  | 1.75      | 12.000      | 10.106      |
| M16  | 2.0       | 16.000      | 13.835      |
| M20  | 2.5       | 20.000      | 17.294      |
| M24  | 3.0       | 24.000      | 20.752      |

### Flange Dimensions (ASME B16.5)
| NPS | Outer Ø (mm) | Bolt Circle Ø (mm) | # Bolts | Bolt Size |
|-----|-------------|-------------------|---------|-----------|
| 2"  | 152.4       | 120.7             | 4       | 5/8"      |
| 3"  | 190.5       | 152.4             | 4       | 5/8"      |
| 4"  | 228.6       | 190.5             | 8       | 5/8"      |
| 6"  | 279.4       | 241.3             | 8       | 3/4"      |
| 8"  | 342.9       | 298.5             | 8       | 3/4"      |

### Minimum Wall Thickness Guidelines
| Application | Min Thickness (mm) |
|-------------|-------------------|
| Light structural | 2.0 |
| Pressure vessel (low) | 6.0 |
| Pressure vessel (high) | 12.0 |
| Industrial piping | 3.0 |
| Casting (aluminum) | 3.0 |
| CNC machined | 1.0 |

---

## Fillet & Chamfer Constraints

### Fillet Rules
- Maximum radius ≤ min(adjacent_edge_length) / 2
- Minimum radius for CNC machining: 0.5mm (internal), 0.25mm (external)
- Standard internal radii: 1, 2, 3, 5, 8, 10, 15, 20mm

### Chamfer Rules
- Standard chamfer angles: 30°, 45°, 60°
- Chamfer length typically 1-3mm for deburring
- Thread entry chamfer: typically 45° × 1 pitch

---

## Material Properties (Common Engineering Materials)

| Material | Density (kg/m³) | Yield (MPa) | UTS (MPa) |
|----------|----------------|------------|----------|
| Steel (A36) | 7850 | 250 | 400 |
| Stainless 316L | 8000 | 170 | 485 |
| Aluminum 6061-T6 | 2700 | 276 | 310 |
| Brass (C36000) | 8500 | 138 | 338 |
| Titanium Ti-6Al-4V | 4430 | 880 | 950 |
| PEEK | 1300 | 100 | 100 |

---

## Dimensional Accuracy Checklist

1. ✅ All dimensions are in millimeters (build123d default)
2. ✅ Tolerances match manufacturing capability (±0.1mm CNC, ±0.5mm casting)
3. ✅ Thread dimensions match ISO 261/262 tables
4. ✅ Flange dimensions match ASME B16.5 for the pressure class
5. ✅ Bolt patterns are evenly distributed on the bolt circle
6. ✅ Fillet radii are machinable and don't exceed constraints
7. ✅ Wall thicknesses meet minimum requirements for the application
