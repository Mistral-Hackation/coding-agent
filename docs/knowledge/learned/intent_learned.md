# Learned Knowledge: intent_reviewer Agent

> Auto-generated insights from web searches and feedback.

---

## Threaded Fittings (2026-01-26 17:19)

*Source: Review Finding*

For 20x1/2" pipe fitting adapters: User specified dimensions L1=16mm (non-threaded), L2=15mm (threaded), E=30mm (threaded diameter), Z=26mm (total non-threaded section). A 2.5mm thread pitch is appropriate for M30 metric threads. The 1/2" internal bore equals 12.7mm diameter. Realistic thread modeling should use true 3D helix geometry via bmesh, not displacement or bump mapping.
---

## Threaded Fitting Patterns (2026-01-26 17:32)

*Source: Review Finding*

For threaded fitting adapters in Blender: (1) Use helical geometry with bmesh for realistic threads rather than textures, (2) ISO metric thread pitch ~1.5mm for 1/2" fittings is acceptable for visual accuracy, (3) Thread depth of ~0.8mm works well, (4) segments_per_turn=48 provides good visual quality, (5) Edge split modifier at 30° helps define sharp thread edges while keeping smooth body surfaces, (6) Separate materials for reinforcing rings (steel vs brass) adds realism
---

## Thread Fitting Standards (2026-01-26 17:34)

*Source: Review Finding*

For 20x1/2" threaded fittings: 1/2" BSP standard uses 14 TPI (~1.814mm pitch). Common approximation in Blender is 1.5mm pitch for visual purposes. Inner diameter for 1/2" fittings is approximately 12.7mm. Thread depth ratio for ISO metric is typically 0.65 of pitch. When reviewing threaded components, verify: (1) thread pitch is appropriate for size, (2) helical geometry vs texture, (3) inner bore diameter matches fitting standard.
---

## Threaded Fitting Standards (2026-01-26 17:49)

*Source: Review Finding*

For 20x1/2" threaded fitting adapters: M20 metric thread pitch is 1.5mm with approximately 0.8mm thread depth. Inner diameter for 1/2" fitting is approximately 12.7mm. Realistic thread modeling requires helical geometry created with bmesh, not just texture/normal maps. Key dimensions pattern: L1 (non-threaded body), L2 (threaded section), E (threaded outer diameter), Z (total non-threaded section including reinforcing ring). Reinforcing rings should have distinct material (stainless steel) from body (brass) for visual differentiation.
---

## Thread Specifications (2026-02-28 15:46)

*Source: WebSearch - ISO 4032 Standard Reference*

ISO 4032 M10 Hex Nut Standard Dimensions: Thread Pitch = 1.5mm, Width Across Flats (F) = 16.00mm max / 15.73mm min, Width Across Corners (E) = 17.77mm min, Height (T) = 8.40mm max / 8.04mm min. The code's 16mm width across flats and 8mm height are within standard tolerance (user specified 8mm is slightly under standard 8.04-8.40mm but acceptable as explicitly requested).
---

## Fastener Standards (2026-02-28 16:33)

*Source: WebSearch*

ISO 273 M10 clearance hole specifications: Fine series = 10.5mm, Medium series = 11mm, Coarse series = 12mm. The code correctly uses 11.0mm for M10 medium fit clearance holes. Tolerance fields: Fine=H12, Medium=H13, Coarse=H14.
---

## Flange Design Standards (2026-02-28 16:35)

*Source: Review Finding*

Flanges should include fillets or chamfers on BOTH outer diameter edges AND inner bore edges. Bore edge treatment is critical for: (1) deburring/safe handling, (2) easier pipe/shaft insertion, (3) stress concentration reduction. M10 clearance hole per ISO 273 medium fit = 11.0mm diameter.
---

## Desk/Table Specifications (2026-02-28 16:35)

*Source: WebSearch*

Ergonomic desk dimensions for coders/developers:
- Standard desk width: 120-150cm (48-60 inches), 140cm is ideal for dual monitors
- Large desk for developers: 160-180cm (63-72 inches) for dual/ultrawide monitors
- Desk depth: Critical for eye/neck strain prevention, typically 60-80cm
- Standard desk height: 720-750mm (ergonomic range), 730mm is standard
- Cable management holes: 60mm standard grommet size
- Tabletop thickness: 25mm is common for sturdy desks
---

## Bolt Hole Standards (2026-02-28 16:36)

*Source: Review Finding - Flange Design*

M10 clearance hole diameters per ISO 273: Fine fit = 10.5mm, Medium fit = 11.0mm, Coarse fit = 12.0mm. For general flange applications, medium fit (11.0mm) is standard practice as it allows for manufacturing tolerances while ensuring proper bolt alignment.
---

## Flange Design Standards (2026-02-28 16:39)

*Source: WebSearch + Engineering best practices*

Flange bore edges should have chamfers (typically 1-2mm x 45°) for: 1) Ease of pipe/shaft insertion during assembly, 2) Deburring sharp machined edges, 3) Safety during handling. Bolt holes may also benefit from small chamfers or countersinks on the mating face side. Outer edge fillets are good for stress distribution and aesthetics.
---

## Thread Specifications (2026-03-01 10:38)

*Source: WebSearch*

1/2" BSP (British Standard Pipe) thread specifications: 14 TPI (threads per inch), pitch = 1.814mm, major diameter (external) = 20.955mm, minor diameter (internal) = 18.631mm. Thread angle is 55 degrees (27.5° half angle). Height of sharp V = 1.743mm. This is different from metric threads which use 60° angle.
---

## Thread Specifications (2026-03-01 10:49)

*Source: WebSearch - BSP Thread Standards*

1/2" BSP Thread Specifications: TPI=14, Pitch=1.814mm, Major (External) Diameter=20.955mm, Minor (Internal) Diameter=18.631mm, Thread V half angle=27.5° (55° included), Height of sharp V=1.743mm. Thread depth for external threads is approximately (20.955-18.631)/2 = 1.162mm. The code's 1.2mm thread depth is accurate.
---

## Flange Design Patterns (2026-03-01 11:12)

*Source: Review Finding*

For parametric flanges with bolt holes: (1) Use PolarLocations(radius=PCD/2, count=N) to place bolt holes on pitch circle diameter, (2) M10 clearance holes should be 11mm diameter per ISO 273 medium fit, (3) Always validate that bolt_pcd is between bore and outer diameter, (4) Apply fillet to outer edges for manufacturing friendliness, (5) Consider chamfers on bolt holes and bore edges for industrial applications.
---

## Flange Design Patterns (2026-03-01 11:13)

*Source: Review Finding*

For M10 bolt clearance holes, 11mm diameter is correct per ISO 273 (medium fit). Flanges should have: (1) Parameter validation to ensure bolt_pcd is between bore and OD, (2) Bolt hole clearance validation, (3) Fillets on outer edges for manufacturing. Bore edge fillets are optional but recommended for stress relief in high-load applications. PolarLocations(radius=bolt_pcd/2, count=bolt_count) is the correct build123d pattern for bolt hole placement.
---

## Mechanical Parts (2026-03-01 11:14)

*Source: Review Finding*

Angle bracket design pattern: (1) L-shaped body using two perpendicular Box primitives or extruded L-profile, (2) Mounting holes on each arm - typically 2+ per arm for anti-rotation, (3) Consider inner corner fillet for stress relief at the intersection, (4) Outer edge fillets for handling safety and aesthetics, (5) Parameters should include arm_length, arm_width, arm_thickness, hole_diameter, hole_inset, hole_spacing
---

## Angle Bracket Patterns (2026-03-01 11:15)

*Source: Review Finding*

For angle brackets: (1) Create L-shape using two perpendicular Box operations at proper offsets, (2) Use Locations context for positioning holes on horizontal arm (Cylinder with Mode.SUBTRACT), (3) Use BuildSketch on Plane.YZ.offset() for vertical arm holes, (4) Fillet edges parallel to Y-axis (filter_by(Axis.Y)) for professional appearance. Parametric values should include: arm_length, arm_width, arm_thickness, hole_diameter, hole_inset, hole_spacing, fillet_radius.
---

## Angle Bracket Design (2026-03-01 11:17)

*Source: Review Finding*

For angle brackets: (1) Inner corner fillet is critical for stress relief - this is the highest stress concentration area, (2) Two holes per arm is standard for anti-rotation, (3) Parameters should include: arm_length, arm_width, thickness, hole_diameter, hole_inset, hole_spacing, fillet_radius, (4) Verify hole placement doesn't overlap with corner region or exceed arm length
---

## Container Design Patterns (2026-03-01 11:32)

*Source: Review Finding*

Trash can design best practices: (1) Use loft() for tapered cylindrical bodies - wider at top for easy insertion/removal, (2) Handle placement at ~75% height for optimal carrying balance, (3) Add rolled lip at top for strength and smooth edges using extrude + fillet, (4) Loop handles created with ThreePointArc path + sweep with circular profile, (5) For opposing handles use for loop with angles [0, 180], (6) Compute radius at handle height via linear interpolation for proper alignment with tapered surface.
---

## Container Design Patterns (2026-03-01 11:33)

*Source: Review Finding*

Trash can design best practices: 1) Use tapered body (wider at top) for easy bag removal and nesting, 2) Position handles at ~75% of can height for balanced lifting, 3) Add rolled lip at top for structural strength and smooth edge, 4) Handle bar diameter 10-15mm for comfortable grip, 5) Use loft() for tapered cylinders, loft(mode=Mode.SUBTRACT) for hollow interior, 6) ThreePointArc creates smooth U-shaped handle paths for sweep operations.
---

## Container Design Patterns (2026-03-01 11:57)

*Source: Review Finding*

Trash can / container design pattern: (1) Use loft() between two circles at different planes for tapered body - wider at top for easy access; (2) Create hollow interior with second loft in Mode.SUBTRACT starting above bottom_thickness; (3) Calculate interpolated radius at any height using: radius = bottom_radius + (z/height) * (top_radius - bottom_radius); (4) Handles can be recessed cutouts (SlotOverall shape extruded as SUBTRACT) positioned on opposite sides using angle loop [0, 180]; (5) Add reinforced lip at top for strength and bag holding; (6) Include fillets on bottom edge for safety.
---

## Design Patterns (2026-03-01 11:59)

*Source: Review Finding*

Trash can / container design pattern: Use loft() between two circles of different radii to create a tapered body. Hollow the interior with a second loft() using Mode.SUBTRACT, starting from an offset plane (bottom thickness). For handles, use SlotOverall shapes extruded with Mode.SUBTRACT to create recessed grip pockets. Position handles near top for ergonomic lifting. Key parameters: body radii, height, wall thickness, handle dimensions, lip height.
---

## Container Design Patterns (2026-03-01 12:00)

*Source: Review Finding*

Trash can design pattern: Use loft() between two Circle sketches at different heights to create tapered body. Hollow interior created with second loft(mode=Mode.SUBTRACT). Handles are recessed slot cutouts using SlotOverall shape extruded through wall. Key parameters: bottom_radius, top_radius (larger for taper), wall_thickness, handle_width/height/depth. Lip/rim reinforcement adds durability and bag holding capability.
---

## Trash Can Design Patterns (2026-03-01 12:02)

*Source: Review Finding*

Trash can with handles design checklist: 1) Tapered body (wider at top) for bag insertion, 2) Hollow interior with wall thickness, 3) Handles on opposite sides (180° apart), 4) Reinforced lip/rim at top for bag holding, 5) Recessed handle cutouts using slot shapes for ergonomics, 6) Fillets on edges for safety. Handle positioning should use linear interpolation for radius at handle height on tapered bodies.
---

## Design Patterns (2026-03-01 12:04)

*Source: Review Finding*

Trash can design pattern: Use loft() between two circles at different heights to create tapered cylindrical body. Inner cavity created with second loft(mode=Mode.SUBTRACT) starting above bottom plate. Handles created as SlotOverall shapes extruded through wall using custom Plane orientations with z_dir pointing toward center for proper subtraction direction.
---

## Container Design Patterns (2026-03-01 12:22)

*Source: Review Finding*

For hollow container designs (trash cans, buckets, cups): Use Cone for tapered bodies, then offset() with openings=[top_face] to hollow out while preserving solid bottom. For handles on tapered walls, interpolate the radius at handle height: radius_at_z = bottom_radius + (z/height) * (top_radius - bottom_radius). Create handle planes with radial normal vectors for proper attachment.
---

## Container Design Patterns (2026-03-01 12:23)

*Source: Review Finding*

Trash can / container design pattern: Use Cone for tapered cylindrical body (wider at top), offset() with openings parameter to create hollow shell with solid bottom, handles created using RectangleRounded on custom Plane positioned tangent to the tapered wall at the correct height. Key formula for radius at height h on tapered wall: radius_at_h = bottom_radius + (h/total_height) * (top_radius - bottom_radius)
---

## Hollow Container Patterns (2026-03-01 12:26)

*Source: Review Finding*

For hollow containers with handles (trash cans, buckets, etc.): Use Cone for tapered body with offset(amount=-wall_thickness, openings=[top_face]) to create shell with solid bottom. Position handles by interpolating radius at handle height: radius_at_handle = bottom_radius + (handle_z / height) * (top_radius - bottom_radius). Use BuildSketch on a Plane with radially outward normal for handle construction.
---

## Container Design Patterns (2026-03-01 12:28)

*Source: Review Finding*

Trash can with handles design pattern: 1) Use Cone for tapered body (wider at top for bag insertion), 2) Use offset() with openings=[top_face] to create hollow shell with solid bottom, 3) Create handles by positioning planes radially on the tapered wall surface, then extrude + subtract for grip opening, 4) For tapered bodies, interpolate radius at handle height: radius_at_z = bottom_radius + (z/height) * (top_radius - bottom_radius)
---

## Edge Selection Patterns (2026-03-01 13:28)

*Source: Review Finding*

When selecting outer edges of a cylinder for filleting in build123d, using hasattr(e, 'radius') may not be reliable. Edge objects in build123d are OCCT wrappers and may not expose radius directly. A more robust approach is to use positional filtering like edges().filter_by(Axis.Z).sort_by(SortBy.RADIUS) or similar topology selectors. The code pattern using hasattr with a fallback is safe but may silently skip operations.
---

## Flange Design Patterns (2026-03-01 13:30)

*Source: Review Finding*

Parametric flange checklist: (1) Outer diameter, bore diameter, thickness as core params; (2) Bolt pattern needs: count, PCD, hole diameter (use ISO 273 clearance - M10=11mm); (3) Validate: bore < outer, bolt PCD between bore and outer; (4) Optional enhancements: outer edge fillets (aesthetic/handling), bore chamfers (assembly aid); (5) PolarLocations(radius=PCD/2, count=N) for bolt patterns; (6) Always export STEP for CAD interchange
---

## Flange Design Patterns (2026-03-01 13:31)

*Source: Review Finding*

Parametric flange design checklist: (1) Outer diameter as parameter, (2) Bore/inner diameter with OD > bore validation, (3) Thickness, (4) PolarLocations for bolt pattern with count and radius=PCD/2, (5) Bolt clearance holes per ISO 273 (M10 = 11mm), (6) Validate PCD is between bore and OD, (7) Optional fillets on outer edges for manufacturing, (8) Export STEP + STL + SVG for complete deliverable
