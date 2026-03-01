# Learned Knowledge: physics_reviewer Agent

> Auto-generated insights from web searches and feedback.

---

## Thread Specifications (2026-01-26 17:18)

*Source: Review Finding*

For 20x1/2" pipe fittings: The "20" typically refers to the nominal pipe size (DN20 = 20mm nominal bore), while 1/2" refers to the thread size. Standard BSP/NPT 1/2" threads use 14 TPI (1.814mm pitch). The code used 2.5mm pitch which may be for metric compatibility or custom industrial applications. Thread depth formula for ISO metric: depth = 0.6134 × pitch.
---

## Thread Specifications (2026-01-26 17:26)

*Source: Review Finding - Threaded Fitting Adapter Code*

BSP G 1/2" thread per EN ISO 228-1: Pitch = 1.814mm (14 TPI), Major OD = 20.955mm, Practical thread depth = 0.960mm. The theoretical depth formula (0.6403 × pitch = 1.162mm) gives full depth, but standards specify 0.960mm accounting for flat crests/roots.
---

## Thread Specifications (2026-01-26 17:48)

*Source: Review Finding*

For 20x1/2" threaded fittings: The designation combines metric sizing (20mm nominal) with imperial thread (1/2"). Thread depth for ISO metric should be 0.6134 × pitch. For 1.5mm pitch, ideal depth is ~0.92mm. Using 0.8mm is slightly conservative but acceptable for 3D visualization. Inner diameter for 1/2" fitting is approximately 12.7mm.
---

## Chamfer Validation (2026-02-28 15:24)

*Source: Review Finding*

For chamfers on cylinder top/bottom circular edges: the chamfer size must be less than both the cylinder radius (to not eliminate the top face) and a reasonable fraction of the cylinder height (to not create excessive taper). A 5mm chamfer on a 15mm radius, 40mm height cylinder is valid because 5mm < 15mm and 5mm << 40mm. The top circular edge can be selected using: part.edges().filter_by(GeomType.CIRCLE).sort_by(Axis.Z)[-1] for the top edge.
---

## Fastener Standards (2026-02-28 15:44)

*Source: WebSearch - wermac.org ISO 4032 chart*

ISO 4032 Hex Nut Dimensions for M10: Thread pitch=1.5mm, Width across flats (F)=16.00mm (max), Width across corners (E)=17.77mm (min), Height (T)=8.40mm (max) / 8.04mm (min). The formula for width across corners from flats is approximately: E = F / cos(30°) ≈ F × 1.1547
---

## Fastener Standards (2026-02-28 16:31)

*Source: WebSearch*

ISO 273 M10 Clearance Holes: Close Fit = 10.5mm, Normal Fit = 11mm, Loose Fit = 12mm. The code correctly uses 11mm for M10 clearance holes (normal/medium fit).
---

## Furniture Design Standards (2026-02-28 16:34)

*Source: WebSearch*

Standard ergonomic desk heights: Sitting desk 540-590mm for average person, Standing desk 900-970mm. Standard desk height of 720-750mm is adequate for taller people (around 6ft). Cable grommet holes: 60mm and 80mm are standard sizes. Desk width for dual monitors: 1200-1600mm recommended. Desk depth for keyboard+monitor: 600-800mm.
---

## Clearance Hole Standards (2026-02-28 16:55)

*Source: WebSearch*

ISO 273 Metric Bolt Clearance Hole Diameters (mm):
- M6: Close=6.4, Normal=6.5, Loose=7.0
- M8: Close=8.4, Normal=9.0, Loose=10.0
- M10: Close=10.5, Normal=11.0, Loose=12.0
- M12: Close=13.0, Normal=14.0, Loose=15.0
- M16: Close=17.0, Normal=18.0, Loose=19.0
- M20: Close=21.0, Normal=22.0, Loose=24.0
- M24: Close=25.0, Normal=26.0, Loose=28.0

Use: Close fit for tighter tolerances, Normal fit for general assembly, Loose fit for simplified assembly.
---

## Thread Specifications (2026-03-01 09:59)

*Source: WebSearch - machiningdoctor.com*

M30x2 ISO Metric Thread specifications:
- Major Diameter: 30mm
- Pitch: 2mm
- Pitch Diameter: 28.701mm
- Minor Diameter: 27.835mm
- Thread Height/Depth: 1.083mm
- Addendum: 0.65mm
- Crest (External) / Root (Internal): 0.25mm
- Tap Drill Size (75% depth): 28.05mm
- Lead Angle (Single Start): 1.27°

Thread depth formula verification: The actual thread depth is 1.083mm for 2mm pitch. The formula H = 0.5413 × P gives H = 1.0826mm which matches. The code uses 1.227 × P / 2 = 1.227mm which is slightly different (represents full theoretical height H = P × sqrt(3)/2 = 0.866P, then depth = 5/8 × H).
---

## Thread Specifications (2026-03-01 10:36)

*Source: WebSearch*

1/2" BSP Thread Specifications: Pitch = 1.814mm (14 TPI), Thread depth = ~1.74mm, Major diameter = 20.955mm, Minor diameter = 18.632mm. When reviewing threaded fittings, verify thread pitch and depth match standard specifications (BSP, NPT, or ISO metric).
---

## Thread Specifications (2026-03-01 10:46)

*Source: WebSearch*

BSP (British Standard Pipe) Thread Specifications:
- 1/2" BSP: TPI=14, Pitch=1.814mm, Major Ø=20.955mm, Minor Ø=18.631mm
- Thread angle: 55° (Whitworth form), half-angle = 27.5°
- Height of sharp V (H) = 0.960491 × pitch
- Thread depth for practical threads ≈ 0.640327 × pitch
- For 1/2" BSP: thread depth ≈ 1.16mm
- Common BSP sizes: 1/8" (28 TPI), 1/4" (19 TPI), 3/8" (19 TPI), 1/2" (14 TPI), 3/4" (14 TPI), 1" (11 TPI)
---

## Flange Design Validation (2026-03-01 11:11)

*Source: Review Finding*

For parametric flanges with bolt holes on PCD: 1) Validate outer wall = (OD - PCD)/2 - hole_radius > min_wall (typically 3mm+); 2) Inner wall = (PCD - bore_dia)/2 - hole_radius > min_wall; 3) Material between bolts = (π × PCD × spacing_angle/360) - hole_dia > min_ligament; 4) M10 clearance holes per ISO 273 medium fit = 11mm diameter; 5) PolarLocations takes RADIUS not diameter for bolt circle positioning.
---

## Container Design Patterns (2026-03-01 11:29)

*Source: Review Finding*

For cylindrical containers with handles: 1) Handle center height at ~75% of body height is ergonomically appropriate for lifting. 2) Linear interpolation for radius at handle height: radius_at_handle = bottom_radius + (handle_height/total_height) * (top_radius - bottom_radius). 3) ThreePointArc in rotated XZ plane creates U-shaped handles efficiently. 4) Lip design: extend outer diameter by lip_height/2 for rolled edge effect. 5) Fillet radius should be lip_height/2 - small_offset to maintain edge geometry.
---

## Tapered Container Design (2026-03-01 11:55)

*Source: Review Finding*

For trash cans and tapered cylindrical containers: 1) Use loft() between two Circle sketches at different Z heights to create the tapered body. 2) For hollowing, calculate inner radius at each height using linear interpolation: inner_radius = (outer_radius_at_height) - wall_thickness. 3) Wall thickness of 4mm is adequate for plastic trash cans. 4) Handle positioning should use linear interpolation: radius_at_height = bottom_radius + (z_height/total_height) * (top_radius - bottom_radius). 5) Handle cutouts can be created using SlotOverall or SlotCenterPoint for rounded ends.
---

## Lifting Hook Design Patterns (2026-03-01 13:00)

*Source: Review Finding*

Industrial lifting hook design key considerations:
1. Hook throat opening (inner radius) typically 30-50mm for light-medium duty
2. Hook material thickness should be substantial (15-25mm) for load bearing
3. Safety tip angle typically 15-30 degrees to prevent accidental release
4. Gusset reinforcements at plate-to-arm transition distribute stress
5. Mounting plate should be thick enough (10-15mm) for bolt pre-load
6. 4-bolt pattern with holes at 35% offset from center provides good load distribution
7. Rounded edges (fillets) reduce stress concentrations at high-load points
---

## Flange Design Standards (2026-03-01 13:27)

*Source: Review Finding*

M10 clearance hole diameter per ISO 273: 11mm (medium fit). For flanges, bolt PCD must be between bore diameter and outer diameter. Wall thickness between bolt holes and edges should be checked: minimum ligament = (outer_diameter - bolt_pcd)/2 - bolt_hole_diameter/2 must be positive for structural integrity. For 120mm PCD with 150mm OD and 11mm holes: (150-120)/2 - 11/2 = 15 - 5.5 = 9.5mm wall (adequate).
