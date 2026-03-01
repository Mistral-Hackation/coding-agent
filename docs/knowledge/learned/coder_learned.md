# Learned Knowledge: coder Agent

> Auto-generated insights from web searches and feedback.

---

## Thread Specifications (2026-01-24 11:24)

*Source: Domain Knowledge - Plumbing Standards*

BSP G 1/2" (20mm nominal) thread specifications: 14 TPI (threads per inch), pitch = 25.4/14 ≈ 1.814mm, major diameter = 20.955mm, thread depth = 0.64 × pitch ≈ 1.16mm. For 20x1/2" fitting adapter: this typically refers to a 20mm pipe with 1/2" BSP thread connection.
---

## Thread Specifications (2026-01-24 11:36)

*Source: Review Finding*

BSP (British Standard Pipe) Thread Specifications for 1/2": 
- Thread pitch: 14 TPI (threads per inch) = 25.4/14 ≈ 1.814mm
- Standard thread depth: 0.64 × pitch
- Thread angle: 55° for BSP (vs 60° for NPT/metric)
- For modeling threads in Blender, use sinusoidal displacement along helix pattern:
  thread_phase = (z / pitch) * 2π + angle
  displacement = (sin(thread_phase) + 1) / 2 * thread_depth
---

## Threaded Fitting Geometry (2026-01-24 11:36)

*Source: Review Finding*

Pattern for creating hollow threaded cylinders with bmesh:
1. Create vertex rings at multiple Z heights (4+ segments per thread pitch for smooth threads)
2. For each ring, calculate thread displacement using sinusoidal helix formula
3. Create both outer (threaded) and inner (hollow) vertex rings
4. Connect rings with quad faces for smooth shading
5. Create end caps as ring faces connecting outer to inner surfaces
6. Always call bmesh.ops.recalc_face_normals() after creating geometry
---

## Geometry Patterns (2026-01-26 14:42)

*Source: Review Finding*

To create a rounded rectangle (filleted corners) in bmesh: 1) Calculate corner arc points using trigonometry (sin/cos), 2) Create vertices for each corner arc segment, 3) Connect all vertices to form the rounded rectangle face, 4) Extrude for 3D volume. Formula for corner arc: for each corner, iterate through angles (0 to 90 degrees in segments) and calculate x,y offsets from corner center using radius * cos(angle) and radius * sin(angle).
---

## Thread Modeling (2026-01-26 16:24)

*Source: Review Finding*

For 1/2" BSP (British Standard Pipe) threads: 14 TPI (threads per inch), pitch = 1.814mm. Thread depth approximately 0.65mm. For realistic helical threads in Blender bmesh: create vertices along helix path using parametric equations x = r*cos(angle), y = r*sin(angle), z = t*length. Use triangular wave modulation for thread profile (crest to root). Verts_per_turn should match cylinder segments for smooth results. Connect faces between adjacent helix loops to form thread surface.
---

## Thread Modeling (2026-01-26 16:56)

*Source: Review Finding*

Helical thread creation in Blender using bmesh:
1. Calculate total segments based on thread pitch and length: num_threads = length / pitch
2. Create helix by iterating through segments, calculating angle = 2 * pi * t and z = t * pitch
3. Create outer verts at major diameter and inner verts at minor diameter (major - thread_depth)
4. Connect consecutive vertex pairs with quad faces
5. Standard 1/2" BSP thread: pitch ~1.814mm (14 TPI), thread depth ~0.87mm
6. Use turns_per_segment (32-48) for smooth helix appearance
7. Always recalculate normals after creating thread geometry
---

## Thread Geometry Patterns (2026-01-26 17:15)

*Source: Review Finding*

For creating realistic helix threads in Blender: 1) Use parametric helix with segments_per_turn (48+ for smooth threads), 2) Create triangular thread profile with inner_radius (valley) and outer_radius (peak), 3) Thread pitch determines spacing between turns, 4) Generate vertices in a spiral pattern: angle = (seg/segments_per_turn) * 2π, z = start_z + (seg/segments_per_turn) * pitch, 5) Create quad faces between successive profile rings, 6) Use bmesh.ops.recalc_face_normals() after geometry creation, 7) Typical metric thread depth is 0.6-0.65 * pitch.
---

## Thread Specifications (2026-01-26 17:22)

*Source: ComplianceReviewer Feedback*

BSP G 1/2" Thread (EN ISO 228-1): Pitch = 1.814mm (14 TPI), Thread depth = 0.960mm. For EU compliance, always reference EN ISO 228-1 standard. Brass fittings should specify material grade EN 12165 CW617N for PED documentation requirements.
---

## Thread Modeling Patterns (2026-01-26 17:28)

*Source: Review Finding*

For realistic helical thread modeling in Blender: 1) Use parametric helix generation with segments_per_turn for resolution control. 2) Create thread profile with inner (root) and outer (peak) vertex rings. 3) ISO metric threads use 60° angle and ~0.65 pitch-to-height ratio. 4) Common thread pitches: 1.5mm for metric, variable for imperial. 5) Thread depth typically 50-60% of pitch. 6) Use high segment count (48+) for smooth threads. 7) Apply EdgeSplit modifier at 30° for sharp thread edges while maintaining smooth body.
---

## Fitting Adapter Dimensions (2026-01-26 17:28)

*Source: User Specification*

Standard threaded fitting adapter (20x1/2"): L1=16mm (non-threaded), L2=15mm (threaded), E=30mm (thread OD), Z=26mm (body section), Total=41mm. Inner diameter for 1/2" fitting is ~12.7mm. Reinforcing ring typically adds 3mm radial thickness. Body diameter slightly smaller than thread OD (~26mm vs 30mm).
---

## Thread Specifications (2026-01-26 17:43)

*Source: Review Finding - OilGasReviewer approved threaded fitting adapter*

Threaded Fitting Adapter Modeling Pattern:
- For 20x1/2" fitting adapters, use ISO metric M20x1.5 thread approximation (1.5mm pitch)
- Thread depth typically 0.8mm for visualization quality
- 60° thread angle for ISO metric threads
- Use 48 segments_per_turn for high-resolution helical threads
- Thread height ratio: pitch * 0.65 for ISO metric
- Wall thickness calculation: (outer_diameter - inner_diameter)/2 should be adequate for pressure ratings
- 1/2" inner diameter ≈ 12.7mm
- Reinforcing ring typically adds 3mm thickness with 6mm width
- Brass material: (0.78, 0.57, 0.11) with metallic=1.0, roughness=0.35
- Stainless steel: (0.8, 0.8, 0.82) with metallic=1.0, roughness=0.25
---

## Thread Specifications (2026-01-26 17:56)

*Source: Review Finding*

For threaded fitting adapters with hybrid designations (e.g., 20x1/2"): M20x1.5 metric thread uses 1.5mm pitch with ~0.8mm thread depth. ISO metric threads use 60° thread angle. For 1/2" internal passages, use 12.7mm bore diameter. Thread height ratio for ISO metric is approximately 0.65 × pitch. Helical threads should use 48 segments per turn for high-resolution visualization.
---

## Fastener Dimensions (2026-02-28 15:36)

*Source: WebSearch - wermac.org ISO 4032 tables*

ISO 4032 Metric Hex Nut Dimensions:
- M10: pitch=1.5mm, width_across_flats=16mm, width_across_corners=17.77mm, height=8.04-8.40mm
- M8: pitch=1.25mm, width_across_flats=13mm, height=6.44-6.80mm
- M12: pitch=1.75mm, width_across_flats=18mm, height=10.37-10.80mm
- Hexagon circumradius = width_across_flats / (2 * cos(30°))
- Minor diameter (approx) = nominal_diameter - 1.5 * pitch * 0.866
---

## RegularPolygon Patterns (2026-02-28 15:40)

*Source: ExampleSearcher - bd_warehouse fastener.py*

For hexagonal nuts, use RegularPolygon with major_radius=False so the radius parameter refers to the apothem (inscribed circle radius = width across flats / 2), not the circumradius (vertex distance). Example: RegularPolygon(radius=width_across_flats / 2, side_count=6, major_radius=False)
---

## Edge Selection Patterns (2026-02-28 15:40)

*Source: ExampleSearcher - filament_bracket.py and Reviewer feedback*

To select only straight edges on a face (excluding circular bore edges), use filter_by(GeomType.LINE). To select only circular edges, use filter_by(GeomType.CIRCLE). This is cleaner than manual distance calculations. Example: face.edges().filter_by(GeomType.LINE) returns only straight line edges.
---

## Bolt Hole Patterns (2026-02-28 16:27)

*Source: Review Finding*

For M10 bolt clearance holes, use 11.0mm diameter (medium fit per ISO 273). PolarLocations usage for bolt patterns: `with PolarLocations(radius=bolt_pcd / 2, count=bolt_count):` - note that PolarLocations takes the radius (half of PCD), not the full diameter. Always validate that bolt_pcd is between bore diameter and outer diameter.
---

## Thread Specifications (2026-02-28 16:45)

*Source: WebSearch*

M10 bolt clearance hole diameters (ISO): Close Fit: 10.5mm, Normal Fit: 11mm, Loose Fit: 12mm. For general flange applications, use Normal Fit (11mm).
---

## API Patterns (2026-02-28 16:45)

*Source: ExampleSearcher*

PolarLocations in Algebra Mode: Use `part -= PolarLocations(radius, count) * Cylinder(radius, height)` to create equally spaced bolt holes on a PCD. Example from bd_warehouse sprocket: `sprocket -= PolarLocations(self.bolt_circle_diameter / 2, self.num_mount_bolts) * Cylinder(self.mount_bolt_diameter / 2, self.thickness)`
---

## API Patterns (2026-02-28 16:47)

*Source: ExampleSearcher*

SVG Export in build123d: Use ExportSVG class with project_to_viewport. Pattern:
```python
visible, hidden = part.project_to_viewport(viewport_origin=(100, -110, 100), look_at=(0, 0, z))
max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
exporter = ExportSVG(scale=100 / max_dimension)
exporter.add_layer("Visible", line_weight=0.5)
exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
exporter.add_shape(visible, layer="Visible")
exporter.add_shape(hidden, layer="Hidden")
exporter.write("output.svg")
```
---

## build123d Locations Patterns (2026-02-28 16:53)

*Source: KnowledgeBase + ExampleSearcher verification*

PolarLocations MUST be used inside a BuildPart context manager, not in algebra mode. The correct pattern is:
```python
with BuildPart() as part:
    Cylinder(radius=outer_radius, height=thickness)
    with PolarLocations(radius, count):
        Cylinder(radius=hole_radius, height=thickness, mode=Mode.SUBTRACT)
```
The algebra mode pattern `PolarLocations(r, n) * Cylinder(...)` does NOT work. Always use the context manager approach for PolarLocations, GridLocations, and similar location helpers.
---

## Thread Modeling Patterns (2026-03-01 00:26)

*Source: ExampleSearcher - bd_warehouse/open_builds.py helix sweep pattern*

For external threads in build123d:
1. If bd_warehouse is available, use IsoThread(major_diameter, pitch, length, external=True, end_finishes=("chamfer", "fade"), hand="right")
2. For manual thread creation, use Helix with sweep:
   - Create Helix(pitch, height, radius) for thread path
   - Use `helix ^ 0` to get perpendicular plane at helix start
   - Create triangular profile (60° ISO) pointing inward for external thread cut
   - Sweep with is_frenet=True for proper orientation along helix
   - Subtract from main body to create thread grooves
3. ISO metric thread depth formula: thread_depth = 0.6134 * pitch
---

## Threading Patterns (2026-03-01 00:57)

*Source: ExampleSearcher - bd_warehouse/fastener.py*

For creating external ISO metric threads in build123d:
1. Use Edge.make_helix(pitch, height, radius, center, direction) to create the helical path
2. Create a triangular profile (60-degree for ISO metric) on a plane perpendicular to the helix start
3. Use sweep(path=helix) within BuildPart context after creating the profile sketch
4. Thread depth for ISO metric: depth = 1.227 * pitch / 2
5. Thread profile positioned at (major_radius + minor_radius) / 2 for balanced sweep
6. Parameters: helix @ 0 gives start position, helix % 0 gives tangent direction
---

## SVG Export (2026-03-01 00:57)

*Source: ExampleSearcher - packed_boxes.py*

For exporting SVG in build123d:
1. Use project_to_viewport(view_port_origin) to get visible and hidden edges
2. Create ExportSVG(scale=...) instance
3. Add layers with add_layer("LayerName", line_color=..., line_type=LineType.ISO_DOT)
4. Add shapes with add_shape(edges, layer="LayerName")
5. Write with exporter.write("filename.svg")
Note: The simple export_svg() function may not exist in all versions - use ExportSVG class for compatibility.
---

## Thread Modeling Patterns (2026-03-01 01:01)

*Source: Review Finding*

External thread creation in build123d using Helix + sweep:
1. Create helix path at thread pitch radius: Helix(pitch=PITCH, height=LENGTH, radius=RADIUS)
2. Create triangular profile perpendicular to helix start:
   - Get position: helix @ 0
   - Get tangent: helix % 0
   - Create Plane with z_dir=tangent, x_dir=(0,0,1)
3. Draw V-shaped profile (60° for ISO/BSP threads)
4. sweep(sections=profile.sketch, path=helix, is_frenet=True)
5. Combine with cylinder body using + operator

BSP thread specs (G1/2): Major=20.955mm, Minor=18.631mm, Pitch=1.814mm (14 TPI)
---

## Thread Creation Patterns (2026-03-01 01:02)

*Source: ExampleSearcher - bd_warehouse/open_builds.py*

For creating external threads in build123d, use the Helix class inside BuildLine() context, then create a triangular profile sketch using `helix ^ 0` to get the perpendicular plane at the start of the helix. Sweep with `is_frenet=True` for proper orientation. Pattern:
```python
with BuildPart() as threads:
    with BuildLine() as helix_path:
        helix = Helix(pitch=PITCH, height=HEIGHT, radius=RADIUS, center=(0, 0, Z_START))
    with BuildSketch(helix ^ 0) as thread_profile:
        with BuildLine():
            Polyline(...) # triangular profile
        make_face()
    sweep(is_frenet=True)
```
ISO metric thread depth formula: depth = 1.227 * pitch / 2
---

## Export Patterns (2026-03-01 09:58)

*Source: Review Finding*

When using ExportSVG with 3D parts, project_to_viewport() returns a tuple (visible, hidden) that MUST be unpacked before passing to add_shape(). Correct pattern:
```python
visible, hidden = part.project_to_viewport((70, -70, 50))
svg_exporter = ExportSVG(scale=3.0)
svg_exporter.add_layer("visible")
svg_exporter.add_layer("hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
svg_exporter.add_shape(visible, layer="visible")
svg_exporter.add_shape(hidden, layer="hidden")
svg_exporter.write("output.svg")
```
---

## Thread Creation (2026-03-01 09:59)

*Source: ExampleSearcher - bd_warehouse/fastener.py*

For creating ISO metric external threads in build123d:
1. Use Edge.make_helix(pitch, height, radius, lefthand=False) to create helical path
2. Create triangular thread profile (60° for ISO metric) on a plane perpendicular to helix
3. Profile plane: origin at helix start, x_dir = radial direction, z_dir = helix tangent
4. Thread depth for ISO metric: ~0.6495 * pitch
5. Sweep the profile along helix: sweep(profile_face, path=helix_edge)
6. Add chamfers at thread entry/exit using Cone with Mode.SUBTRACT
7. Position helix with .moved(Location((0, 0, z_offset)))
---

## Export Patterns (2026-03-01 09:59)

*Source: KnowledgeBase*

For 3D Part objects in build123d, use export_step() and export_stl() for export. The export_svg() function does NOT work directly on 3D Parts - it's only for 2D projections. To create an SVG from a 3D part, you need to use project_to_viewport() first, then ExportSVG class to manually build the SVG.
---

## Thread Modeling Patterns (2026-03-01 10:20)

*Source: ExampleSearcher - bd_warehouse/open_builds.py helix sweep pattern*

For creating external threads in build123d:
1. Use Helix(pitch, height, radius, center) to create helical path
2. Get perpendicular plane at helix start with `helix_line ^ 0`
3. Create thread profile (triangular for metric) on that plane
4. Sweep with `sweep(path=helix_line, is_frenet=True)` for proper orientation
5. Thread depth typically ~0.866 * pitch for 60° metric threads
6. Leave lead-in/lead-out space (half pitch) at thread ends
7. Small crest flat (0.1mm) improves manufacturability
---

## Thread Modeling Patterns (2026-03-01 10:28)

*Source: ExampleSearcher - bd_warehouse/open_builds.py FlexibleCoupler pattern*

For creating realistic threads in build123d using helical sweep:
1. Use `with BuildLine():` containing `Helix(pitch, height, radius, center)` to create the path
2. Get perpendicular plane at helix start using `helix_path ^ 0` operator
3. Create thread profile in that plane using `with BuildSketch(helix_path ^ 0):`
4. Use trapezoidal/triangular profile with Polyline for ISO-style threads
5. Call `sweep(is_frenet=True, mode=Mode.ADD)` to follow the helix with Frenet frame
6. The `is_frenet=True` parameter ensures correct orientation along curved path
---

## Export Functions (2026-03-01 10:28)

*Source: Build123dDocs - import_export examples*

For build123d exports:
- `export_step(part, "filename.step")` - Standard CAD interchange format
- `export_stl(part, "filename.stl")` - For 3D printing and mesh viewers
- For SVG export, use the ExportSVG class with project_to_viewport(), NOT a simple function call
- Example SVG export:
```python
visible, hidden = part.project_to_viewport((100, -110, 100))
exporter = ExportSVG(scale=100 / max_dimension)
exporter.add_layer("Visible")
exporter.add_shape(visible, layer="Visible")
exporter.write("output.svg")
```
---

## Thread Specifications (2026-03-01 10:39)

*Source: IntentReviewer Feedback*

1/2" BSP (British Standard Pipe) Thread Specifications:
- Thread pitch: 1.814mm (14 threads per inch / TPI)
- Thread angle: 55° (Whitworth form) - NOT 60° like metric threads
- Half-angle: 27.5° for profile calculations
- Thread depth: approximately 1.1-1.3mm
- BSP uses trapezoidal/Whitworth profile, not ISO metric 60° triangle
---

## Export Patterns (2026-03-01 10:39)

*Source: Build123dDocs Review*

Proper SVG export in build123d uses ExportSVG class with project_to_viewport:
```python
visible, hidden = part.project_to_viewport((-100, -100, 70))
max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
exporter = ExportSVG(scale=100 / max_dimension)
exporter.add_layer("Visible")
exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
exporter.add_shape(visible, layer="Visible")
exporter.add_shape(hidden, layer="Hidden")
exporter.write("filename.svg")
```
Note: There is NO direct export_svg() function for 3D Part objects.
---

## Flange Patterns (2026-03-01 11:05)

*Source: Review Finding*

Parametric flange with bolt holes pattern:
1. Use Cylinder for main body and bore (Mode.SUBTRACT)
2. Use PolarLocations(radius=pcd/2, count=n) for bolt hole placement
3. M10 clearance hole diameter: 11mm (medium fit per ISO 273)
4. Validate: bore < pcd < outer_diameter
5. Validate bolt holes don't overlap bore or outer edge
6. Filter outer edges for fillet: flange.edges().filter_by(GeomType.CIRCLE).filter_by(lambda e: abs(e.radius - outer_diameter/2) < 0.01)
---

## Export Patterns (2026-03-01 11:08)

*Source: Build123dDocs*

SVG export in build123d requires using the ExportSVG class, not a simple export_svg function. The proper pattern is:
```python
def export_svg_file(part: Part, filename: str, view_origin: tuple = (-100, -100, 70)):
    visible, hidden = part.project_to_viewport(view_origin)
    max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
    exporter = ExportSVG(scale=100 / max_dimension)
    exporter.add_layer("Visible")
    exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
    exporter.add_shape(visible, layer="Visible")
    exporter.add_shape(hidden, layer="Hidden")
    exporter.write(filename)
```
This projects the 3D part to a 2D viewport, creates visible/hidden layers with appropriate styling, and writes the SVG file.
---

## Export Patterns (2026-03-01 11:11)

*Source: Review Finding*

SVG Export in build123d uses the ExportSVG class, NOT export_svg() function. Proper pattern:
```python
visible, hidden = part.project_to_viewport((100, -110, 100))
max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
exporter = ExportSVG(scale=100 / max_dimension)
exporter.add_layer("Visible", line_weight=0.5)
exporter.add_layer("Hidden", line_color=(0.6, 0.6, 0.6), line_type=LineType.ISO_DOT)
exporter.add_shape(visible, layer="Visible")
exporter.add_shape(hidden, layer="Hidden")
exporter.write("output.svg")
```
Always include STL export for 3D viewer compatibility: export_stl(part, "output.stl")
---

## Container Design Patterns (2026-03-01 11:23)

*Source: Review Finding*

To create a hollow container (trash can, cup, bucket) in build123d:
1. Use loft() between two Circle sketches at different Z heights for tapered walls
2. Create inner hollow using a second loft() with Mode.SUBTRACT
3. Offset the inner bottom sketch by bottom_thickness to leave a solid base
4. Use offset(amount=-wall_thickness, openings=top_face) as alternative hollowing method
5. For lips/rims: Add annular sketch at top and extrude, then fillet edges
---

## Handle Creation Patterns (2026-03-01 11:23)

*Source: Review Finding*

To create loop handles on a part:
1. Use BuildLine to create the handle path (ThreePointArc for U-shape)
2. Work in XZ plane and rotate around Z axis for multiple handles
3. Get path wire: path_wire = handle_path.wires()[0]
4. Create profile at start: Plane(origin=path_wire @ 0, z_dir=path_wire % 0)
5. Sweep: sweep(sections=profile.sketch, path=path_wire)
6. For opposing handles, use for angle in [0, 180] and Plane.XZ.rotated((0, 0, angle))
---

## Container Patterns (2026-03-01 11:47)

*Source: Review Finding*

Trash can / bucket with handles pattern:
1. Create tapered body using loft() between two Circle sketches at different heights
2. Hollow out interior with another loft(mode=Mode.SUBTRACT) starting above bottom_thickness
3. Calculate wall radius at any height using linear interpolation: r = r_bottom + (z/height) * (r_top - r_bottom)
4. Create handle cutouts using SlotOverall shape on a plane positioned tangent to the can surface
5. Extrude handles with Mode.SUBTRACT to cut through the wall
6. Add reinforcement lip at top using a ring extrusion
7. Apply fillets to bottom edges for aesthetics
---

## Export Patterns (2026-03-01 11:49)

*Source: Review Finding*

ExportSVG class pattern for generating SVG visual previews of 3D parts:
```python
visible, hidden = part.project_to_viewport((200, -200, 200))
max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
exporter = ExportSVG(scale=100 / max_dimension)
exporter.add_layer("Visible")
exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
exporter.add_shape(visible, layer="Visible")
exporter.add_shape(hidden, layer="Hidden")
exporter.write("output.svg")
```
Note: Do NOT use `export_svg()` function directly - it doesn't exist or has different signature.
---

## Export Patterns (2026-03-01 11:51)

*Source: Review Finding*

For SVG export of 3D parts in build123d, create a helper function called `export_svg` that uses the ExportSVG class internally:

```python
def export_svg(part, filename: str, view_port_origin=(200, -200, 200)):
    """Export a 3D part to SVG format using 2D projection."""
    visible, hidden = part.project_to_viewport(view_port_origin)
    max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
    exporter = ExportSVG(scale=100 / max_dimension)
    exporter.add_layer("Visible")
    exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
    exporter.add_shape(visible, layer="Visible")
    exporter.add_shape(hidden, layer="Hidden")
    exporter.write(filename)
```

This function should be called as `export_svg(part, "filename.svg")` to match the system's expected export pattern validation.
---

## Container Patterns (2026-03-01 12:14)

*Source: Review Finding*

Trash can / bucket pattern: Use loft() between two circles at different Z heights to create tapered cylindrical body. Hollow interior by subtracting another loft with smaller radii, starting at wall_thickness height for solid bottom. For handles on curved surface, calculate radius at handle height by interpolating: radius_at_handle = bottom_radius + (handle_z/height) * (top_radius - bottom_radius). Create handles using RectangleRounded profiles on Planes with outward-facing normals.
---

## Export Patterns (2026-03-01 12:18)

*Source: Build123dDocs and code review*

For 3D Part SVG export, build123d doesn't have a built-in export_svg() function. Instead, define a custom export_svg function using project_to_viewport() and ExportSVG class:

```python
def export_svg(part, filename, view_origin=None, look_at=None):
    if view_origin is None:
        view_origin = (400, -400, 300)
    visible, hidden = part.project_to_viewport(view_origin)
    max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
    exporter = ExportSVG(scale=100 / max_dimension)
    exporter.add_layer("Visible", line_weight=0.5)
    exporter.add_layer("Hidden", line_color=(150, 150, 150), line_type=LineType.ISO_DOT)
    exporter.add_shape(visible, layer="Visible")
    exporter.add_shape(hidden, layer="Hidden")
    exporter.write(filename)
```

This pattern is derived from the build123d readthedocs documentation for 3D to 2D projection.
---

## Export Patterns (2026-03-01 12:55)

*Source: ExampleSearcher - packed_boxes.py*

For SVG export in build123d, use the ExportSVG class with project_to_viewport:

```python
visible, hidden = part.project_to_viewport(
    look_at=(0, 0, height/2),
    camera_position=(200, -250, 150),
)
max_dim = max(*Compound(children=visible + hidden).bounding_box().size)
exporter = ExportSVG(scale=100 / max_dim)
exporter.add_layer("Visible")
exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
exporter.add_shape(visible, layer="Visible")
exporter.add_shape(hidden, layer="Hidden")
exporter.write("output.svg")
```

The project_to_viewport method takes camera_position and look_at parameters to define the view angle.
---

## Flange Patterns (2026-03-01 13:22)

*Source: Review Finding*

Parametric flange with bolt hole pattern: Use PolarLocations(radius=pcd/2, count=bolt_count) inside BuildPart context to create evenly spaced bolt holes. M10 clearance hole is 11mm (ISO 273 standard). Typical flange creation: 1) Cylinder for body, 2) Cylinder with Mode.SUBTRACT for bore, 3) PolarLocations + Cylinder with Mode.SUBTRACT for bolt holes, 4) Optional fillet on outer edges.
---

## Export Patterns (2026-03-01 13:24)

*Source: Build123dDocs + ExampleSearcher*

SVG Export in build123d requires the ExportSVG class, NOT a direct export_svg() function. Correct pattern:

```python
view_port_origin = (100, -110, 100)
visible, hidden = part.project_to_viewport(view_port_origin)
max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
exporter = ExportSVG(scale=100 / max_dimension)
exporter.add_layer("Visible")
exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
exporter.add_shape(visible, layer="Visible")
exporter.add_shape(hidden, layer="Hidden")
exporter.write("output.svg")
```

Key points:
1. Use project_to_viewport() to get visible and hidden edges
2. Calculate scale from bounding box for proper sizing
3. ExportSVG class with add_layer() and add_shape() methods
4. LineType.ISO_DOT for hidden lines
---

## Export Patterns (2026-03-01 13:25)

*Source: Build123dDocs and Examples*

build123d does NOT have a built-in export_svg() function. To export SVG, use the ExportSVG class pattern:

```python
def export_svg(part, filename, view_origin=(100, -110, 100)):
    visible, hidden = part.project_to_viewport(view_origin)
    max_dimension = max(*Compound(children=visible + hidden).bounding_box().size)
    exporter = ExportSVG(scale=100 / max_dimension)
    exporter.add_layer("Visible")
    exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)
    exporter.add_shape(visible, layer="Visible")
    exporter.add_shape(hidden, layer="Hidden")
    exporter.write(filename)
```

This creates a helper function named export_svg() that can be called like: `export_svg(part, "output.svg")`

## 2026-03-01 — Web search learning and governance alignment
- The project now distinguishes discovery sources: web search findings are advisory until validated and persisted through `LearnKnowledge` into internal markdown knowledge.
- Internal policy updates should prioritize implementation safety: prefer internal standards when available, and only use web-derived behavior after validation.
- OpenTelemetry data should be used for operational policy suggestions (e.g., repeated tool warnings), not for automatic hard policy enforcement without review.
