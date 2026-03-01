# Learned Knowledge: researcher Agent

> Auto-generated insights from web searches and feedback.

---

## Thread Specifications (2026-02-28 15:33)

*Source: WebSearch - wermac.org ISO 4032 chart*

M10 Hex Nut ISO 4032 Dimensions: Thread pitch=1.5mm, Width across flats=16mm, Width across corners=17.77mm, Standard height=8.4mm (max)/8.04mm (min). Hole diameter for M10 = 10mm nominal.
---

## Common Patterns (2026-02-28 16:01)

*Source: Review Finding*

Box with centered hole pattern: Use BuildPart context, create Box() for base shape, then Cylinder() with Mode.SUBTRACT for through-hole. Both primitives center at origin by default, so no positioning needed for centered holes. Example: Box(50,50,30) + Cylinder(radius=5, height=30, mode=Mode.SUBTRACT) creates a box with 10mm diameter centered hole.
---

## Furniture Design - Desk Dimensions (2026-02-28 16:29)

*Source: WebSearch - ergonomic desk standards*

Ergonomic desk dimensions for programmers/coders:
- Width: Standard 120-150cm (48-60in), Large/Developer 160-180cm (63-72in) for dual monitors
- Depth: 60-80cm (24-32in) recommended for proper monitor distance
- Height: Standard 72-76cm (28-30in), adjustable standing desks 60-125cm
- Tabletop thickness: typically 25-30mm for solid construction
- Leg dimensions: typically 50-80mm square or round for stability
A 140cm x 70cm desk is ideal sweet spot for most programmer setups with dual monitors.
---

## Furniture Dimensions (2026-02-28 16:43)

*Source: WebSearch - ergonomic desk standards*

Standard ergonomic desk/table dimensions for coding workspace:
- Width: 1200-1820 mm (typical single-person: 1200-1600 mm)
- Depth: 600-800 mm (ideal: 700 mm to reduce eye strain)
- Height: 650-750 mm (typical: 720-750 mm)
- Minimum legroom width: 550 mm
These dimensions ensure proper ergonomics for computer work: relaxed shoulders, natural arm positioning, and reduced eye strain.
---

## Thread Specifications (2026-03-01 00:19)

*Source: WebSearch - machiningdoctor.com*

NPT 1/2" Thread Specs: Pitch=1.814mm (14 TPI), Pipe OD=21.336mm, Pitch Diameter E0=19.264mm, Minor Diameter K0=17.813mm, Thread Height h=1.451mm, Thread V half angle=30°, Taper angle=1.7893°, Hand-tight engagement L1=8.128mm, Useful thread length L2=13.556mm. For BSP parallel threads (G1/2), similar dimensions but no taper.
---

## Thread Modeling Patterns (2026-03-01 00:23)

*Source: Domain Knowledge - Thread Standards*

For BSP (British Standard Pipe) threads in build123d: BSP G1/2 has pitch of 1.814mm (14 TPI), major diameter ~20.955mm, 55° thread angle. To model threads realistically use: 1) Helix() to create helical path with pitch parameter, 2) Create triangular thread profile sketch perpendicular to helix start, 3) sweep() the profile along the helix path. For visual threads without exact specs, can also use simplified grooves with revolve of zigzag profile. Metal reinforcing rings are typically created as separate cylinders with Boolean union.
---

## Thread Specifications (2026-03-01 00:43)

*Source: WebSearch*

BSP G 1/2" Thread Specifications (ISO 7-1, ISO 7-2):
- Pitch (TPI): 14
- Pitch (Distance): 1.814mm
- Thread V half Angle: 27.5°
- Height of sharp V (H): 1.743mm
- Thread height (h): 1.162mm
- Root/Crest Radius (r): 0.249mm
- Major Diameter (D): 20.955mm
- Pitch Diameter (D2): 19.793mm
- Minor Diameter (D1): 18.632mm
- Tap Drill Size: 18.5mm
This is ideal for plumbing fittings and adapters with 1/2" thread designation.
---

## Thread Modeling Techniques (2026-03-01 00:43)

*Source: WebSearch*

build123d Thread Creation Pattern (from 123DScrew project):
1. Create thread profile using BuildSketch with trapezoidal/triangular shape
2. Use helical sweep to create the thread: sweep profile along helix path
3. Key functions/approach:
   - trapezoidal_profile(major_d, pitch, length) -> creates 2D thread cross-section
   - threaded_segment(major_d, pitch, length) -> helical sweep of profile around core
   - Use Helix() for the sweep path with parameters: pitch, height, radius
   - Trim ends flush after sweep
4. For BSP threads: use 55° included angle (27.5° half angle) instead of ACME's 29°
5. Core cylinder + helical thread sweep = complete threaded part
6. Export to STL for 3D printing or STEP for CAD interchange
---

## Thread Specifications (2026-03-01 00:52)

*Source: WebSearch*

BSP 1/2" Thread Specifications (G1/2): Major Diameter: 20.955mm, Pitch: 1.814mm (14 TPI), Thread Height: 1.162mm, Thread Angle: 55° total (27.5° half angle), Pitch Diameter: 19.793mm, Minor Diameter: 18.632mm, Root/Crest Radius: 0.249mm. Standards: ISO 7-1, ISO 7-2 for BSPP (G) threads.
---

## Thread Modeling Patterns (2026-03-01 00:55)

*Source: Domain Knowledge - Thread Standards*

For realistic thread modeling in build123d: 1) Use Helix path with appropriate pitch and height matching thread length; 2) Create triangular/trapezoidal thread profile sketch; 3) Sweep the profile along the helix; 4) Boolean add to the base cylinder. For 1/2" BSP/G1/2 threads: pitch ≈ 1.814mm (14 TPI), major diameter ≈ 20.955mm. Alternative: Use IsoThread from build123d extensions if available. Thread depth typically 0.64 × pitch for metric ISO threads.
---

## Thread Specifications (2026-03-01 00:58)

*Source: WebSearch - machiningdoctor.com*

BSP 1/2" Thread Specifications (G1/2 / BSPP): Major Diameter (D) = 20.955mm, Pitch Diameter (D2) = 19.793mm, Minor Diameter (D1) = 18.632mm, Pitch = 1.814mm (14 TPI), Thread Height (h) = 1.162mm, Thread V half Angle = 27.5°, Root/Crest Radius = 0.249mm. Standards: ISO 7-1, ISO 7-2. For external threads, use these nominal values.
---

## Thread Specifications (2026-03-01 01:06)

*Source: WebSearch - machiningdoctor.com BSP thread specifications*

1/2" BSP Thread (G1/2) specifications: Major Diameter = 20.955mm, Pitch = 1.814mm (14 TPI), Thread Height = 1.162mm, Minor Diameter = 18.632mm, Pitch Diameter = 19.793mm, Thread V half angle = 27.5°, Root/Crest Radius = 0.249mm. Standards: ISO 7-1, ISO 7-2. The "20x" notation often refers to ~20mm nominal which corresponds to 1/2" BSP.
---

## Thread Specifications (2026-03-01 09:54)

*Source: WebSearch - machiningdoctor.com*

NPT 1/2" thread specs: Pipe OD=21.336mm, Pitch=1.814mm (14 TPI), Thread height=1.451mm, Pitch diameter at end=19.264mm, Minor diameter at end=17.813mm, Thread V half angle=30°, Taper angle=1.7893°, Hand-tight engagement length=8.128mm, Useful thread length=13.556mm, Overall thread length=19.85mm
---

## Thread Modeling Techniques (2026-03-01 09:56)

*Source: Domain Knowledge - Thread Standards*

For realistic thread modeling in build123d: 1) Use Helix(pitch, height, radius) to create spiral path, 2) Create triangular thread profile sketch on plane perpendicular to helix start, 3) Use sweep() operation along the helix path, 4) For BSP 1/2" threads: major diameter ~20.955mm, pitch 1.814mm (14 TPI), thread angle 55°. For metric threads use 60° angle. Thread depth typically = 0.6495 × pitch. Common approach: create base cylinder first, then add/subtract thread geometry.
---

## Thread Specifications (2026-03-01 10:17)

*Source: WebSearch - machiningdoctor.com*

NPT 1/2" thread specs: Pipe OD=21.336mm, Pitch=1.814mm (14 TPI), Thread height=1.451mm, Pitch diameter E0=19.264mm, Minor diameter K0=17.813mm, Thread V half angle=30°, Taper angle=1.7893°. Hand-tight engagement L1=8.128mm, Overall thread length L4=19.85mm.
---

## Bolt Hole Patterns (2026-03-01 11:04)

*Source: ExampleSearcher - bd_warehouse/sprocket.py*

For creating circular bolt hole patterns in build123d, use PolarLocations:
1. PolarLocations(radius, count) - creates evenly spaced locations at given radius
2. To cut holes: `part -= PolarLocations(pcd/2, num_holes) * Cylinder(hole_radius, thickness)`
3. For central bore: `part -= Cylinder(bore_radius, thickness)`
4. Works with algebraic mode using subtraction operator (-=)
5. PCD (Pitch Circle Diameter) should be divided by 2 to get radius for PolarLocations
---

## Flange Patterns (2026-03-01 13:20)

*Source: ExampleSearcher - bd_warehouse/sprocket.py*

PolarLocations bolt hole pattern for flanges:
- Use `PolarLocations(radius, count)` where radius is PCD/2 (pitch circle diameter / 2)
- Multiply by Cylinder to create hole shapes: `PolarLocations(pcd/2, num_bolts) * Cylinder(hole_radius, thickness)`
- Subtract from main body: `flange -= PolarLocations(...) * Cylinder(...)`
- For central bore: `flange -= Cylinder(bore_diameter/2, thickness)`
- M10 clearance hole is approximately 10.5mm diameter (ISO standard clearance)
- Example from bd_warehouse sprocket.py shows this pattern clearly

## 2026-03-01 — OpenTelemetry + learned policy updates
- Added a bounded web-learning workflow for the agent: discover -> validate -> summarize -> persist via `LearnKnowledge`.
- Internal knowledge now captures source URLs, confidence, and policy impact instead of raw prompt fragments.
- Sensitive fields from telemetry should be redacted before storage/export; raw telemetry should keep only aggregate operational metadata by default.
- Policy refresh from telemetry is now defined as: aggregate run spans, detect repeated warnings/errors, generate low-risk suggestion entries, and route to manual review before becoming enforcement policy.
