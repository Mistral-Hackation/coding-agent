# Learned Knowledge: reviewer Agent

> Auto-generated insights from web searches and feedback.

---

## Threaded Fitting Patterns (2026-01-24 11:39)

*Source: Review Finding*

BSP 1/2" thread specifications: 14 TPI = 1.814mm pitch (25.4/14), thread depth ~0.64mm, 55° thread angle. For helical thread modeling in bmesh, use combined thread phase calculation: thread_phase = (angle / (2 * math.pi)) + (z - z_start) / pitch, then apply triangular profile with sin() smoothing for realistic appearance.
---

## Blender Context API (2026-01-24 11:39)

*Source: Review Finding*

In Blender 3.2+/4.0, use bpy.context.temp_override(**override) as a context manager for operator overrides. Example: `with bpy.context.temp_override(area=area, region=region): bpy.ops.view3d.view_selected()`. This replaces the old deprecated pattern of passing override dict directly to operators.
---

## Blender API Patterns (2026-01-26 14:39)

*Source: Review Finding*

For electrical enclosure/cabinet generation: (1) Use bmesh.ops.create_cone with radius1=radius2 for cylinders, (2) Use bmesh.ops.delete with context='FACES' to remove faces from mesh, (3) For hollow boxes, delete front face then apply Solidify modifier with offset=-1 to maintain outer dimensions, (4) Boolean operations require setting bpy.context.view_layer.objects.active before calling bpy.ops.object.modifier_apply()
---

## Industrial Equipment Modeling (2026-01-26 14:39)

*Source: Review Finding*

Electrical cabinet components hierarchy: (1) Main hull as hollow cube with front opening, (2) Rain hood with pitched roof using bmesh vertex manipulation for center peak, (3) Door with boolean-cut observation window, (4) Gland plate with grid of cable entry holes - use loop to create/apply multiple boolean cylinders, (5) Mounting tabs with bolt holes at back corners, (6) Side vents with louver slats created via bmesh faces. Material: 316 Stainless Steel with metallic=1.0, roughness=0.3
---

## Electrical Enclosure Pattern (2026-01-26 14:49)

*Source: Review Finding*

For industrial cabinet modeling: (1) Use create_rounded_rectangle_mesh() for filleted window cutouts by generating corner arcs with segments_per_corner parameter, (2) Rain hood geometry with pitched roof uses center ridge vertices raised by peak amount, (3) Gland plate hole patterns calculated with even spacing: h_spacing = plate_width / (cols + 1), (4) Louvers in vents created as angled slats with z-offset pattern. Boolean EXACT solver recommended for reliable cuts.
---

## Code Structure Pattern (2026-01-26 14:49)

*Source: Review Finding*

Industrial equipment code should include: MM_TO_M conversion constant at top, separate create_ functions for each component (hull, roof, door, gland_plate, mounting_tabs, vents), utility functions for common operations (create_cube_mesh, create_cylinder_mesh, apply_boolean_difference), and a main() function that orchestrates creation and material assignment.
---

## Blender API Patterns (2026-01-26 14:54)

*Source: Review Finding*

For creating cylinders in bmesh, use bmesh.ops.create_cone() with radius1=radius2 (same top and bottom radius). Parameters: cap_ends=True for closed ends, cap_tris=False for quad faces, segments for smoothness. This is the proper low-level alternative to bpy.ops.mesh.primitive_cylinder_add().
---

## Common Code Patterns (2026-01-26 14:54)

*Source: Review Finding*

For industrial enclosure models requiring rounded/filleted rectangle cutouts (like observation windows), create a custom mesh by generating corner arc vertices using parametric angles, then extruding to depth. Use segments_per_corner parameter for smoothness control. This avoids bevel modifier complexities.
---

## Industrial Cabinet Modeling (2026-01-26 14:56)

*Source: Review Finding*

For electrical enclosure cabinets: (1) Use Solidify modifier with offset=-1 to maintain outer dimensions when hollowing, (2) Create rounded rectangle cutouts for observation windows using calculated corner arcs, (3) Boolean EXACT solver is preferred for clean hole cutouts in gland plates, (4) Rain hoods require pitched geometry with center ridge vertices for water runoff, (5) Mounting tabs should be positioned at corners with through-holes for wall bolts.
---

## Blender Bmesh Patterns (2026-01-26 14:56)

*Source: Review Finding*

When creating rounded rectangles in bmesh: Generate corner arc vertices by iterating through angles (segments_per_corner), create front and back vertex arrays, then connect with side faces. Always call bmesh.ops.recalc_face_normals() after manual face creation to ensure consistent normals.
---

## Industrial Enclosure Patterns (2026-01-26 15:02)

*Source: Review Finding - Electrical Cabinet Code*

For electrical enclosure cabinets: (1) Use solidify modifier with offset=-1 to preserve outer dimensions when creating hollow shells; (2) Delete front face before solidify to create door opening; (3) Rain hoods should extend past hull dimensions (typically 20mm sides, 15mm front/back) for water runoff; (4) Gland plates require both grid-pattern cable entry holes AND a main cable cutout for high-amperage cables; (5) Mounting tabs are typically placed at 4 corners of back panel with standard M10/M12 mounting holes.
---

## Blender API Pattern (2026-01-26 15:02)

*Source: Review Finding - create_rounded_rectangle_mesh implementation*

For rounded rectangles in Blender: Create vertices in 2D using corner arcs (calculate with sin/cos), then extrude to create front and back faces, connecting them with side faces. Always clamp fillet radius to min(width, height)/2 to avoid invalid geometry. Pattern: for each of 4 corners, generate arc vertices from start_angle through 90 degrees using segments_per_corner resolution.
---

## Threaded Fitting Patterns (2026-01-26 16:26)

*Source: Review Finding*

For 1/2" BSP (British Standard Pipe) threads: 14 TPI (threads per inch) = 1.814mm pitch. Thread depth ~0.65mm. When creating helical threads in Blender, use bmesh with calculated vertex positions based on angle and z-offset for each turn segment. Create faces by connecting consecutive rings of vertices. Use triangular wave profile (if t < 0.5: r_offset = t * 2 * depth else r_offset = (1-t) * 2 * depth) for thread shape approximation.
---

## Dead Code Detection (2026-01-26 16:26)

*Source: Review Finding*

When reviewing complex Blender scripts, check for unused helper functions that may remain from iterative development. Common pattern: multiple approaches defined (e.g., create_threaded_cylinder, create_helical_thread_with_screw_modifier) but only one actually used in main function. These are non-blocking but indicate code cleanup opportunity. Also check unused imports (e.g., Vector imported but not used).
---

## Blender 4.0 Compatibility (2026-01-26 16:57)

*Source: Review Finding*

For Blender 4.0+ compatibility with Principled BSDF shader inputs, always use .get() method: `principled.inputs.get('Base Color')`, `principled.inputs.get('Metallic')`, `principled.inputs.get('Roughness')`. This handles renamed inputs gracefully and returns None if not found.
---

## Thread Specifications (2026-01-26 16:58)

*Source: Review Finding*

1/2 inch BSP thread specifications: Pitch = 1.814mm (14 TPI - threads per inch). Thread depth approximately 0.87mm. When modeling helical threads in Blender, create a helix of vertices at outer_radius (crest) and inner_radius (root), then connect with quad faces for realistic thread visualization.
---

## Threaded Fitting Patterns (2026-01-26 17:16)

*Source: Review Finding*

For threaded fitting adapters: (1) Use bmesh.ops.create_cone for cylindrical sections with cap_ends=True, cap_tris=False. (2) Create helix threads by iterating angle steps with z-offset based on pitch formula: z = start_z + (step/segments_per_turn) * thread_pitch. (3) Thread profile uses 4 vertices: inner-base, outer-peak at pitch/4, inner at pitch/2. (4) Join parts with bpy.ops.object.join() then apply bore boolean for hollow center. (5) Apply two materials: brass (color 0.8,0.6,0.2, metallic=1.0) for fitting, steel (0.6,0.6,0.65) for reinforcing ring.
---

## Dead Code Patterns (2026-01-26 17:16)

*Source: Review Finding*

Watch for unused helper functions in Blender scripts: functions like create_helix_thread() and create_simple_thread_with_screw_modifier() may be defined as reusable utilities but then implemented inline instead. These should either be removed or the inline code should call them. This is a common pattern when developers prototype inline first then refactor to functions but forget to update references.
---

## Threaded Fitting Patterns (2026-01-26 17:25)

*Source: Review Finding*

For BSP G 1/2" thread (EN ISO 228-1), use pitch=1.814mm (14 TPI) and thread_depth=0.960mm. Thread helix geometry can be created by iterating through segments_per_turn (48 recommended for smooth threads) and generating profile vertices at each angle. The profile should be triangular with points at inner radius (minor diameter), outer radius (major diameter at peak), then back to inner. Always create faces between consecutive profile rings using bmesh.faces.new() with try-except for potential duplicate face errors.
---

## Boolean Operations Pattern (2026-01-26 17:25)

*Source: Review Finding*

When using boolean operations for internal bores or cutouts: 1) Create the cutter slightly longer than the target (e.g., depth + 0.002 for clean cut), 2) Set the active object before applying modifier, 3) Remove the cutter object after applying with bpy.data.objects.remove(cutter_obj, do_unlink=True). For ring shapes (annulus), use two boolean operations - create outer cylinder, then difference with slightly smaller inner cylinder (radius - 0.0001 for clean boolean).
---

## Threaded Fitting Design (2026-01-26 17:30)

*Source: Review Finding*

For threaded fitting adapters with ISO metric threads: use thread_height = thread_pitch * 0.65 for the ISO metric thread height ratio. Thread depth should be approximately 0.8mm for 1.5mm pitch threads. The helical thread geometry requires segments_per_turn of 32-48 for smooth visual appearance. Standard 1/2" fitting inner diameter is approximately 12.7mm.
---

## Headless Safety Pattern (2026-01-26 17:30)

*Source: Review Finding*

When setting viewport shading in Blender scripts, use the pattern: try/except with `if bpy.context.screen:` check before iterating through areas. Example: `try: if bpy.context.screen: for area in bpy.context.screen.areas: ...` This prevents errors when running in headless mode without GUI.
---

## Threaded Fitting Patterns (2026-01-26 17:45)

*Source: Review Finding*

For threaded fitting adapters: ISO metric M20x1.5 thread uses 1.5mm pitch and ~0.8mm thread depth. Helical threads can be modeled by creating vertex pairs (outer peak, inner root) along a helix path and connecting them with quad faces. segments_per_turn=48 provides good resolution for threads. Thread profile dimensions: L1 (non-threaded body), L2 (threaded section), E (outer thread diameter), Z (total body section before threads).
---

## Blender 4.0 Compatibility (2026-01-26 17:45)

*Source: Review Finding*

For Blender 4.0+ material compatibility, always use .get() method for both nodes and inputs: nodes.get('Principled BSDF') and principled.inputs.get('Base Color'). Check for None before setting values. This pattern gracefully handles renamed or missing inputs across Blender versions.
---

## Helical Thread Modeling (2026-01-26 17:46)

*Source: Review Finding*

For realistic helical thread geometry in bmesh: (1) Calculate total_segments = num_turns * segments_per_turn, (2) For each segment: angle = (i / segments_per_turn) * 2π, z = z_offset + (i / total_segments) * thread_length, (3) Create both outer (peak) and inner (root) vertex rings at root_radius = outer_radius - thread_depth, (4) Connect with quad faces between adjacent segments. Thread pitch determines num_turns = thread_length / thread_pitch.
---

## Headless Safety Pattern (2026-01-26 17:46)

*Source: Review Finding*

For viewport/GUI operations that may fail in headless mode, use dual protection: (1) Check `if bpy.context.screen:` before iterating areas, (2) Wrap entire block in try-except AttributeError with pass. Example: `try:\n    if bpy.context.screen:\n        for area in bpy.context.screen.areas:\n            if area.type == 'VIEW_3D': ...\nexcept AttributeError:\n    pass`
---

## Threaded Fitting Patterns (2026-01-26 17:57)

*Source: Review Finding*

For threaded fitting adapters in Blender: 1) Use helical thread modeling with create_helical_thread() function taking outer_radius, thread_pitch, thread_depth, and segments_per_turn parameters. 2) Standard thread pitch for 20mm fittings is approximately 1.5mm (ISO M20 metric). 3) Create separate bmesh objects for body, threads, and reinforcing ring, then join with bpy.ops.object.join(). 4) Thread depth typically ~0.8mm for M20 threads. 5) Use separate metallic materials (brass for body, stainless steel for reinforcing ring) with roughness 0.25-0.35 for realistic appearance.
---

## Blender 4.0 Compatibility (2026-01-26 17:58)

*Source: Review Finding*

For Blender 4.0+ shader input compatibility, always use .get() method when accessing Principled BSDF inputs: principled.inputs.get('Base Color'), principled.inputs.get('Metallic'), principled.inputs.get('Roughness'). This prevents KeyError if input names change between versions. Also check if principled node exists with nodes.get('Principled BSDF') before accessing inputs.
---

## Fillet Validation Pattern (2026-02-28 14:45)

*Source: Review Finding*

When filleting all edges of a box, the fillet radius must be less than half the smallest dimension to avoid geometry errors. Best practice: validate with `max_fillet = min(length, width, height) / 2 - tolerance` and raise ValueError with clear message if exceeded. Also validate positive radius.
---

## Builder Mode Edge Selection (2026-02-28 14:45)

*Source: Review Finding*

In build123d builder mode, to fillet all edges of a part use `fillet(part.edges(), radius=value)` within the BuildPart context. The `part.edges()` selector returns all edges of the current part being built.
---

## Fillet Patterns (2026-02-28 15:06)

*Source: Review Finding*

For filleting ALL edges of a Box in build123d Builder mode, use `fillet(part.edges(), radius=value)` where `part` is the BuildPart context name. A safety margin of 0.9 * (min_dimension/2) prevents geometry failures when fillet radius approaches the theoretical limit.
---

## Edge Selection Patterns (2026-02-28 15:22)

*Source: Review Finding*

For selecting circular edges on a cylinder, use: `part.edges().filter_by(GeomType.CIRCLE).sort_by(Axis.Z)[-1]` to get the top circular edge, or `[0]` for the bottom. This is more robust than using `.group_by()` for simple cylinders with only 2 circular edges.
---

## API Pattern (2026-02-28 15:39)

*Source: ExampleSearcher - bd_warehouse/fastener.py*

For hexagonal nuts in build123d, use RegularPolygon with major_radius=False when specifying width across flats. Example: RegularPolygon(radius=width_across_flats/2, side_count=6, major_radius=False). The bd_warehouse fastener library uses this pattern consistently for hex nuts. Without major_radius=False, the radius parameter is treated as circumradius (vertex distance), not apothem (flat distance).
---

## Selector Pattern (2026-02-28 15:39)

*Source: Review Finding - comparing examples*

For selecting edges on top/bottom faces of a part, prefer: part.faces().sort_by(Axis.Z)[-1].edges() for top face edges, and part.faces().sort_by(Axis.Z)[0].edges() for bottom face edges. The filter_by(Plane.XY.offset(height)) approach is less standard and may not work as expected.
---

## API Patterns (2026-02-28 15:41)

*Source: Review Finding*

GridLocations pattern for creating corner holes: Use GridLocations(x_spacing=x_pos * 2, y_spacing=y_pos * 2, x_count=2, y_count=2) where x_pos and y_pos are the offset from center. Then inside the context, place Cylinder with mode=Mode.SUBTRACT to create through holes at all 4 corners.
---

## Best Practices (2026-02-28 15:41)

*Source: Review Finding*

Fillet radius validation: When applying fillets to all edges of a box, ensure the fillet radius is less than half the smallest dimension (typically thickness). Use pattern: actual_fillet_radius = min(fillet_radius, thickness / 2 - 0.1) to prevent geometry failure.
---

## SVG Export Pattern (2026-02-28 15:42)

*Source: ExampleSearcher - packed_boxes.py*

build123d export_svg uses ExportSVG class with project_to_viewport method. Pattern: visible, hidden = part.project_to_viewport(view_port_origin); exporter = ExportSVG(scale=...); exporter.add_layer(); exporter.add_shape(visible, layer='Visible'); exporter.write(filename). The simple export_svg(part, filename, camera_position, look_at) function also exists in build123d but requires tuple for camera position.
---

## Hex Nut Pattern (2026-02-28 15:42)

*Source: Review Finding*

For hexagonal nuts: Use RegularPolygon(radius=width_across_flats/2, side_count=6, major_radius=False) where major_radius=False means radius is apothem (inscribed circle radius = width across flats / 2). M10 standard: width_across_flats=16mm, pitch=1.5mm. Minor diameter for M10x1.5 ≈ 8.376mm. Bore diameter can be calculated as thread_diameter - 1.5 * pitch * 0.866.
---

## API Patterns (2026-02-28 15:50)

*Source: Review Finding*

For creating through-holes in build123d, both `Cylinder(radius, height, mode=Mode.SUBTRACT)` and the `Hole(radius, depth)` function are valid approaches. Cylinder+SUBTRACT is more explicit and gives full control over dimensions, while Hole is more semantic. Both are centered at the current context location by default.
---

## Boolean Operations (2026-02-28 15:55)

*Source: Review Finding*

In build123d Builder mode, Box and Cylinder are both centered at origin by default. To create a centered through-hole, use Cylinder with mode=Mode.SUBTRACT immediately after Box creation - no repositioning needed. Ensure cylinder height matches or exceeds box height for a complete through-hole.
---

## API Patterns (2026-02-28 15:59)

*Source: Review Finding*

In build123d, both Box() and Cylinder() are centered at the origin by default. When creating a centered through-hole in a box, simply use Cylinder(radius=r, height=h, mode=Mode.SUBTRACT) inside the same BuildPart context - no positioning needed if both primitives are centered.
---

## Export Patterns (2026-02-28 16:10)

*Source: ExampleSearcher - build123d examples corpus*

build123d SVG export uses ExportSVG class pattern, not a simple export_svg() function. Correct approach: (1) project_to_viewport(camera_position) to get visible/hidden lines, (2) create ExportSVG(scale=...), (3) add_layer() and add_shape(), (4) exporter.write(filename). The simple export_svg() function may exist but parameters like view_port_origin and view_port_target need verification.
---

## Geometry Patterns (2026-02-28 16:11)

*Source: Review Finding*

For creating a centered hole in a Box using build123d Builder mode: Both Box and Cylinder default to centered at origin alignment. Using Cylinder(radius, height, mode=Mode.SUBTRACT) inside the same BuildPart context as Box() creates a perfect centered through-hole. No Location adjustment needed when both primitives should be centered.
---

## API Pattern (2026-02-28 16:28)

*Source: ExampleSearcher - bd_warehouse/sprocket.py*

PolarLocations in Builder mode: Use `with PolarLocations(radius=pcd/2, count=n):` context manager for bolt hole patterns. The radius parameter is half the PCD (Pitch Circle Diameter). Inside the context, objects like Cylinder are placed at each polar location. Example from bd_warehouse sprocket: `sprocket -= PolarLocations(bolt_circle_diameter/2, num_mount_bolts) * Cylinder(mount_bolt_diameter/2, thickness)` (Algebra mode)
---

## Fillet Patterns (2026-02-28 16:28)

*Source: Review Finding - flange code analysis*

When filtering circular edges by radius for fillet operations, use list comprehension with tolerance check: `outer_fillet_edges = [e for e in edges.filter_by(GeomType.CIRCLE) if abs(e.radius - target_radius) < 0.1]`. The fillet() function in Builder mode takes an edge list and radius parameter. Safe practice: limit fillet radius to thickness/4 to avoid failures.
---

## SVG Export Patterns (2026-02-28 16:31)

*Source: ExampleSearcher - packed_boxes.py, playing_cards.py examples*

build123d provides two SVG export approaches: (1) The simple `export_svg(shape, filename, view_port_origin=...)` function for basic exports, and (2) The more powerful `ExportSVG` class approach using `project_to_viewport()` for advanced control with visible/hidden line separation. For complex isometric views, the ExportSVG class pattern is preferred: `visible, hidden = part.project_to_viewport(view_port_origin); exporter = ExportSVG(); exporter.add_shape(visible); exporter.write(filename)`
---

## Fillet Selector Pattern (2026-02-28 16:31)

*Source: Review Finding - intersecting_pipes.py example and code analysis*

When filleting edges of a specific face in build123d Builder mode, use `top_face = part.faces().sort_by(Axis.Z).last` to get the topmost face, then `top_face.edges()` to get its edges. The `fillet()` function accepts a ShapeList of edges directly. Alternative selector pattern from examples: `fillet(part.edges(Select.LAST), radius)` to fillet the most recently created edges.
---

## Table/Furniture Design Pattern (2026-02-28 16:32)

*Source: Review Finding - Coder Table implementation*

For furniture like tables in build123d: (1) Create tabletop as a Box at appropriate height using Locations, (2) Calculate leg positions based on inset from edges: `leg_offset = table_size/2 - inset - leg_width/2`, (3) Place legs at floor level using Locations with Z=leg_height/2, (4) Cable management holes use Cylinder with Mode.SUBTRACT positioned through the tabletop, (5) Fillet top edges for safety using face selector pattern.
---

## Export Functions (2026-02-28 16:46)

*Source: WebSearch and ExampleSearcher*

For 3D Parts in build123d: Use export_step() and export_stl() for file exports. The export_svg() function as a simple function call may not work correctly with 3D Parts. For SVG export of 3D objects, you need to use ExportSVG class with visible_edges or project the part to 2D first. Always include export_stl() alongside export_step() as STL is required for 3D viewers.
---

## Selector Patterns (2026-02-28 16:51)

*Source: Review Finding*

To fillet only straight edges on a face that contains both straight and circular edges (e.g., after cutting a hole), use `face.edges().filter_by(GeomType.LINE)` to select only the linear edges and exclude circular hole edges. Always add a safety check: `if len(edges) > 0 and fillet_radius > 0: fillet(edges, radius)`
---

## build123d Patterns (2026-02-28 16:52)

*Source: ExampleSearcher - trackball2.py and frame_common.py examples*

PolarLocations usage patterns: (1) In Builder mode: use as context manager `with PolarLocations(radius, count):` followed by shape operations. (2) In Algebra mode: typically use list comprehension like `[l * Cylinder(...) for l in PolarLocations(radius, count)]` then fuse or subtract. The direct `PolarLocations(...) * Cylinder(...)` may work but returns a Compound that needs careful handling with boolean ops.
---

## Export Requirements (2026-02-28 16:52)

*Source: Review - KnowledgeBase requirements*

Code must include BOTH export_step() AND export_stl() for proper file interchange. STL is needed for 3D viewers. The ExportSVG class pattern with project_to_viewport() is valid for creating 2D projections but requires proper layer setup with add_layer() and add_shape().
---

## API Pattern (2026-02-28 16:54)

*Source: Review Finding - Flange bolt hole pattern*

PolarLocations(radius, count) creates equally spaced points around a circle for bolt hole patterns. Use inside BuildPart with Cylinder(radius, height, mode=Mode.SUBTRACT) to create bolt holes. Example from real code: `with PolarLocations(pcd_radius, bolt_count): Cylinder(radius=bolt_hole_radius, height=thickness, mode=Mode.SUBTRACT)`
---

## Engineering Standards (2026-02-28 16:54)

*Source: Review Finding - Thread specifications*

M10 clearance hole sizes per ISO 273: Close fit = 10.5mm, Normal fit = 11.0mm, Loose fit = 12.0mm. The code correctly uses 11.0mm for M10 normal fit clearance holes.
---

## Thread Creation Patterns (2026-03-01 01:00)

*Source: ExampleSearcher - open_builds.py and fastener.py patterns*

For helical sweeps in build123d: Use `Helix(pitch, height, radius, center=...)` inside a `with BuildLine():` context. Then access the start plane with `helix ^ 0` for the profile sketch. Use `sweep(is_frenet=True)` for proper orientation along the helix. The Edge.make_helix() method is also valid but requires different handling. For realistic threads, bd_warehouse provides IsoThread class with proper ISO metric profiles.
---

## API Patterns (2026-03-01 01:05)

*Source: ExampleSearcher - bd_warehouse/thread.py*

For threading in build123d, the bd_warehouse library uses loft() between multiple profiles placed along a Helix path, not sweep(). The pattern is: 1) Create Helix inside BuildLine context, 2) Use helix @ u_value for position and helix % u_value for tangent, 3) Place profiles at multiple u values (e.g., 0 to 1 in steps), 4) Call loft() to create the thread solid. This produces better quality threads than simple sweep.
---

## Common Pitfalls (2026-03-01 01:05)

*Source: ExampleSearcher - fender_bender_src/frame_bottom.py*

Use filter_by_position() for filtering edges/faces by coordinate position, NOT filter_by(lambda...). Example: edges().filter_by_position(Axis.Z, minimum=0, maximum=5) to get edges with Z position between 0 and 5. The filter_by() method is for filtering by geometry type (Axis.Z for edges parallel to Z) or GeomType.
---

## Thread Creation Patterns (2026-03-01 01:05)

*Source: Review Finding - ExampleSearcher fastener.py*

For creating ISO metric threads in build123d, two approaches exist: 1) Use bd_warehouse's IsoThread class (from bd_warehouse.thread import IsoThread) which handles all thread geometry automatically with parameters: major_diameter, pitch, length, external=True/False, end_finishes, hand. 2) Manual helix sweep approach: create Helix inside BuildLine, use helix ^ 0 to get perpendicular plane for profile sketch, then sweep(is_frenet=True). The IsoThread approach is more robust and follows ISO standards properly.
---

## Sweep with Helix Path (2026-03-01 01:05)

*Source: Review Finding - ExampleSearcher open_builds.py*

When using sweep() with a helix path in BuildPart context: 1) Create Helix inside BuildLine context (adds to pending_edges), 2) Create profile sketch at helix ^ 0 (perpendicular plane at parameter 0), 3) Call sweep(is_frenet=True) to use Frenet framing for proper orientation along helical path. Example pattern from open_builds.py: with BuildLine(): helix = Helix(...); with BuildSketch(helix ^ 0): profile_shape; sweep(is_frenet=True)
---

## Thread Modeling Patterns (2026-03-01 09:56)

*Source: Review Finding*

For realistic external threads in build123d: 1) Create helix path using Edge.make_helix(pitch, height, radius, center, direction), 2) Create triangular ISO metric profile perpendicular to helix at start using Plane(origin=start_pos, z_dir=tangent), 3) Sweep profile along helix with Mode.PRIVATE to create standalone geometry, 4) Fuse with main body using + operator. Thread depth formula for ISO metric: depth = 1.227 * pitch / 2. Minor diameter = major - 2*depth.
---

## SVG Export Pattern (2026-03-01 09:56)

*Source: Review Finding*

Correct way to export SVG from 3D parts: Use project_to_viewport() method on Part/Compound to get visible and hidden edges, then use ExportSVG class with add_layer() and add_shape() methods. Example: visible, hidden = part.project_to_viewport((view_x, view_y, view_z)); exporter = ExportSVG(scale=100); exporter.add_layer("Visible"); exporter.add_shape(visible, layer="Visible"); exporter.write("output.svg"). Do NOT use export_svg() function which doesn't exist.
---

## SVG Export Pattern (2026-03-01 09:56)

*Source: Review Finding - ExampleSearcher*

project_to_viewport() returns a TUPLE (visible, hidden) - must unpack before passing to ExportSVG.add_shape(). Correct pattern: `visible, hidden = part.project_to_viewport((x, y, z))` then `exporter.add_shape(visible, layer="Visible")`
---

## Revolve Patterns (2026-03-01 10:01)

*Source: Review Finding - Threaded Fitting Adapter*

For pipe fittings and cylindrical parts with profiles, use BuildSketch on Plane.XZ with BuildLine to define the 2D profile, then revolve() around Axis.Z. The profile should be constructed with Lines connecting points in (radius, z) coordinates. This pattern creates clean rotational geometry with proper internal bores and external features.
---

## Thread Modeling (2026-03-01 10:01)

*Source: Review Finding - Threaded Fitting Adapter*

For visual thread representation without using IsoThread (which may not always be available), use Torus shapes with Mode.SUBTRACT in a loop to create thread grooves. Position each torus at z = start_z + i * pitch + pitch/2. Use major_radius equal to the thread outer diameter / 2, and minor_radius as the groove depth (typically 60-70% of thread depth). This creates a simplified but visually effective thread representation.
---

## Thread Modeling (2026-03-01 10:01)

*Source: Review Finding - ThreadedFittingAdapter*

When creating helical threads in build123d: 1) Helix can be used standalone without BuildLine context, 2) For realistic threads, bd_warehouse library has IsoThread class that handles complexities like end finishes, 3) When sweeping along a helix, use is_frenet=True for proper profile orientation, 4) Thread profile should be perpendicular to helix tangent at sweep start point using Plane(origin, x_dir, z_dir) pattern
---

## Revolve Pattern (2026-03-01 10:02)

*Source: Review Finding - ThreadedFittingAdapter*

For revolved bodies in build123d: 1) Use Plane.XZ for 2D profile with X=radius, Z=height (positive X only for revolution), 2) revolve(axis=Axis.Z) creates solid of revolution around Z axis, 3) Polyline with close=True inside BuildLine creates closed profile, 4) make_face() required after BuildLine to create fillable face, 5) Profile must not cross the revolution axis (X >= 0 for Z-axis revolution)
---

## Common Pitfalls (2026-03-01 10:24)

*Source: KnowledgeBase Review*

export_svg() does NOT exist in build123d for 3D Part objects - this will cause NameError at runtime. Only export_step() and export_stl() are valid export functions for 3D parts. Additionally, export_stl() is required for 3D viewer compatibility.
---

## API Patterns (2026-03-01 10:24)

*Source: ExampleSearcher - sweep frenet pattern*

In build123d Builder mode, sweep() with is_frenet=True works correctly for helical sweeps. When using a Helix inside BuildLine, the path can be accessed via builder.line (e.g., helix_line.line). The ^ operator on a line gives a plane perpendicular at parameter position (0=start, 1=end) for profile placement.
---

## Thread Modeling (2026-03-01 10:31)

*Source: Review Finding - Thread Pattern Analysis*

In build123d, realistic external threads can be created using: (1) BuildLine context with Helix() object, (2) BuildSketch using helix ^ 0 operator to get perpendicular plane at position 0, (3) Creating thread tooth profile with Polyline and make_face(), (4) sweep(is_frenet=True, mode=Mode.ADD) to sweep profile along helix. The Helix object can be directly used with ^ operator without accessing .line property.
---

## Thread Modeling Patterns (2026-03-01 10:43)

*Source: Review Finding - bd_warehouse/thread.py and open_builds.py examples*

build123d supports two approaches for helical threads: (1) Helical sweep with Frenet frame - use `with BuildLine(): helix = Helix(...)` then `with BuildSketch(helix ^ 0):` for profile, then `sweep(is_frenet=True)`. (2) Loft approach (more robust) - create multiple sketches along helix path using `helix @ u_value` for position and `helix % u_value` for tangent direction, then `loft()`. The bd_warehouse Thread class uses the loft approach for reliability.
---

## SVG Export Pattern (2026-03-01 10:43)

*Source: Review Finding - build123d_examples/lego.py and build123d_logo.py*

Proper SVG export for 3D parts requires: (1) Project part to viewport: `visible, hidden = part.project_to_viewport((x, y, z))`, (2) Create ExportSVG: `exporter = ExportSVG(scale=...)`, (3) Add layers with styling: `exporter.add_layer("Visible")` and `exporter.add_layer("Hidden", line_color=(99, 99, 99), line_type=LineType.ISO_DOT)`, (4) Add shapes to layers: `exporter.add_shape(visible, layer="Visible")`, (5) Write file: `exporter.write("filename.svg")`. Do NOT use non-existent `export_svg()` function.
---

## API Patterns (2026-03-01 11:07)

*Source: Review Finding - bd_warehouse sprocket.py example*

PolarLocations in build123d uses signature PolarLocations(radius, count) where radius is the distance from center (NOT diameter). For bolt holes on a PCD, use bolt_pcd/2 as the radius parameter. The usage `with PolarLocations(radius=bolt_pcd / 2, count=bolt_count):` is correct builder mode syntax for creating polar patterns.
---

## Export Functions (2026-03-01 11:07)

*Source: Review Finding - packed_boxes.py example and knowledge base*

build123d has export_step() and export_stl() as built-in functions. export_svg() is NOT a built-in - it must be implemented using the ExportSVG class with project_to_viewport(), add_layer(), add_shape(), and write() methods. Code using export_svg() as a built-in function will fail with NameError. Always require export_stl() for 3D viewer compatibility.
---

## API Patterns (2026-03-01 11:09)

*Source: Review Finding - verified against docs/general_examples.py Ex.13*

PolarLocations for bolt hole patterns: Use `with PolarLocations(radius=pcd/2, count=bolt_count):` where radius is half the Pitch Circle Diameter. Inside the context, create Cylinder with Mode.SUBTRACT for through-holes. Optional parameters include start_angle and angular_range for partial patterns.
---

## Edge Selection Patterns (2026-03-01 11:09)

*Source: Review Finding - flange fillet pattern*

To select specific circular edges for filleting by radius: Use list comprehension filter `[e for e in part.edges().filter_by(GeomType.CIRCLE) if abs(e.radius - target_radius) < tolerance]`. This is useful for flanges where you want to fillet only the outer diameter edges and not bore or bolt hole edges.
---

## Export Functions (2026-03-01 11:10)

*Source: Review Finding - build123d docs*

build123d does NOT have a direct export_svg() function for 3D parts. To export SVG from 3D parts, you must: 1) Use part.project_to_viewport(view_origin) to get visible/hidden edges, 2) Create ExportSVG exporter, 3) Add layers and shapes, 4) Call exporter.write(filename). The code pattern is: visible, hidden = part.project_to_viewport((x,y,z)); exporter = ExportSVG(scale=...); exporter.add_layer('Visible'); exporter.add_shape(visible); exporter.write('file.svg')
---

## Cylinder API (2026-03-01 11:10)

*Source: Review Finding - Example Search*

build123d Cylinder accepts 'rotation=(angle_x, angle_y, angle_z)' parameter as a tuple for rotating the cylinder orientation. This is useful for creating holes through non-Z-axis faces. Example: Cylinder(radius=3, height=10, rotation=(0, 90, 0), mode=Mode.SUBTRACT) creates a horizontal hole along X-axis.
---

## Hole Creation Patterns (2026-03-01 11:12)

*Source: Review Finding*

For creating mounting holes in build123d, there are two valid approaches: (1) Using Cylinder with mode=Mode.SUBTRACT directly within Locations context for simple through-holes perpendicular to a plane, and (2) Using BuildSketch on an offset plane with Circle + extrude(mode=Mode.SUBTRACT, both=True) for holes on non-standard planes like vertical faces. The second approach with BuildSketch allows for more control over hole placement on arbitrary planes using Plane.YZ.offset() or similar patterns.
---

## API Pattern (2026-03-01 11:25)

*Source: Review Finding*

For BuildLine sweep patterns: Use `handle_path.line` or `handle_path.wires()[0]` to get the path wire. The canonical pattern is passing the BuildLine context directly (e.g., `sweep(path=l1)`) when within the same scope, or access via `.line` attribute for the compound wire.
---

## Common Pitfalls (2026-03-01 11:25)

*Source: Review Finding*

Plane.XZ.rotated((0,0,angle)) is used to rotate the XZ plane around the Z axis for creating geometry at different angular positions around the origin - useful for radial handle placement.
---

## Sweep Pattern (2026-03-01 11:28)

*Source: Review Finding - tea_cup.py example analysis*

When sweeping a handle on a container (like a tea cup or trash can), the recommended pattern from the tea_cup.py example is: 1) Create a BuildLine context to define the path (e.g., Spline or ThreePointArc), 2) Get path endpoints with path @ 0, path @ 1 and tangent with path % 0, 3) Create BuildSketch on Plane(origin=path @ 0, z_dir=path % 0) for cross-section alignment, 4) Use sweep() inside BuildPart context. The code should use the sweep pattern: sweep() with implicit pending sketch, or sweep(sections=sketch.sketch, path=wire) for explicit control.
---

## Export Patterns (2026-03-01 11:48)

*Source: Review Finding + ExampleSearcher*

For SVG export of 3D Parts in build123d, you CANNOT use export_svg() directly. You must use ExportSVG class with project_to_viewport(). Correct pattern: visible, hidden = part.project_to_viewport((x, y, z)); exporter = ExportSVG(scale=N); exporter.add_layer("Visible"); exporter.add_shape(visible, layer="Visible"); exporter.write("filename.svg")
---

## SVG Export Pattern (2026-03-01 11:53)

*Source: Review Finding - trash can model*

For 3D parts, use `part.project_to_viewport(view_origin)` to get visible/hidden edges, then use `ExportSVG` class with `add_layer()` and `add_shape()` methods. Example: `visible, hidden = part.project_to_viewport((200, -200, 200)); exporter = ExportSVG(scale=100/max_dim); exporter.add_layer("Visible"); exporter.add_shape(visible, layer="Visible"); exporter.write("file.svg")`. Note: There is no direct `export_svg(part, filename)` function for 3D Parts - must use projection approach.
---

## API Patterns (2026-03-01 12:19)

*Source: Review Finding + ExampleSearcher*

To create a hollow shell with an opening (like a cup or trash can) in build123d, use `offset(amount=-wall_thickness, openings=[face])` where the face is selected using topology selectors like `part.faces().sort_by(Axis.Z).last` for the top face. This creates a shell with uniform wall thickness and leaves specified faces open. Example from trackball2.py: `top = offset(top.solids()[0], amount=-wall, openings=base_plate)`
---

## SVG Export (2026-03-01 12:19)

*Source: Review Finding + ExampleSearcher*

To export 3D parts to SVG in build123d, use `project_to_viewport()` method on the part to get visible and hidden edges, then create an `ExportSVG` object with layers. Example: `visible, hidden = part.project_to_viewport((x, y, z))` followed by `exporter = ExportSVG(scale=...)`, then `exporter.add_layer()`, `exporter.add_shape(visible, layer=...)`, and `exporter.write(filename)`. The LineType.ISO_DOT can be used for hidden lines.
---

## API Patterns (2026-03-01 12:58)

*Source: ExampleSearcher - fender_bender_src/frame_top.py*

build123d Cylinder supports arc_size parameter for creating partial cylinders (e.g., arc_size=180 for half cylinder). Combined with rotation=(90, 0, 0) and align parameters, this can create curved hook sections. Example: Cylinder(radius=r, height=h, arc_size=180, rotation=(90, 0, 0), align=(Align.CENTER, Align.CENTER, Align.CENTER))
---

## Design Patterns (2026-03-01 12:58)

*Source: Review Finding - comparing pegboard_j_hook.py vs load_lifter approach*

For J-shaped hooks, there are two approaches: 1) Using sweep() with JernArc path (cleaner for wire-frame hooks, see pegboard_j_hook.py), 2) Using half-cylinders with Cylinder(arc_size=180) plus rectangular extrusions (better for thick solid hooks). The choice depends on the desired hook geometry - sweep for constant cross-section, cylinder subtraction for chunky industrial hooks.
---

## API Patterns (2026-03-01 13:23)

*Source: Review Finding - Knowledge Base Warning*

export_svg() is NOT a built-in build123d function. To export SVG from 3D parts, you must use the ExportSVG class with project_to_viewport() method. Example pattern: visible, hidden = part.project_to_viewport(view_port_origin); exporter = ExportSVG(scale=100/max_dim); exporter.add_shape(visible); exporter.write(filename)
---

## Edge Selector Patterns (2026-03-01 13:26)

*Source: Review Finding + ExampleSearcher (bd_warehouse/flange.py)*

To select edges by radius in build123d, use `.sort_by(SortBy.RADIUS)` instead of trying to access `e.radius` attribute directly. Example: `edges().filter_by(GeomType.CIRCLE).sort_by(SortBy.RADIUS)[-1]` selects the largest circular edge. The `hasattr(e, 'radius')` check used in some code is unreliable - use SortBy.RADIUS for proper edge filtering by size.
