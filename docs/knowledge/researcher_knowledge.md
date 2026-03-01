# Researcher Agent Knowledge Base

> Internal reference for the Researcher agent when gathering information via WebSearcher.

## 🎯 Role Summary

The Researcher uses the WebSearcher tool to find relevant information, tutorials, specifications, and best practices for **build123d** CAD modeling before the Coder generates scripts.

---

## 📚 Search Strategies

### Effective Queries

**For build123d API Patterns:**
```
"build123d Python BuildPart [operation] example"
"build123d selector filter_by sort_by tutorial"
"build123d extrude revolve sweep loft example"
"build123d fillet chamfer edges selector"
```

**For build123d Documentation:**
```
"build123d.readthedocs.io [topic]"
"build123d open cascade [operation]"
"build123d export STEP STL [format]"
```

**For Technical Specifications:**
```
"[component] dimensions specifications standards mm"
"API ASME [equipment] design requirements dimensions"
"ISO [topic] standard specifications tolerances"
```

**For Industry Standards:**
```
"[industry] equipment design standards regulations"
"CE ATEX PED requirements [equipment type]"
"BSP NPT thread specifications dimensions"
"ASME B16.5 flange dimensions bolt pattern"
```

### Search Refinement

1. **Too Many Results**: Add "build123d" or "cadquery" (similar API) for filtering
2. **Too Few Results**: Search for "cadquery" equivalents (build123d evolved from cadquery)
3. **Wrong Results**: Add "BREP" or "Open Cascade" to narrow to solid modeling

---

## 🔧 Information Extraction

### Key Data Points to Capture

**For CAD Modeling Tasks:**
- [ ] Exact dimensions (always in mm — build123d default)
- [ ] Material specifications (for documentation, not rendering)
- [ ] Thread/fitting standards (ISO 261 thread data)
- [ ] Assembly relationships (mates, clearances)
- [ ] Required tolerances

**For Industry Equipment:**
- [ ] Applicable standards (API, ASME, ISO)
- [ ] Pressure/temperature ratings
- [ ] Material requirements and grades
- [ ] Certification requirements
- [ ] Standard component dimensions (flanges, bolts, pipes)

**For build123d Techniques:**
- [ ] Which builder mode to use (BuildPart, BuildSketch, BuildLine)
- [ ] Correct operation sequence (sketch → extrude → fillet)
- [ ] Selector patterns for topology access
- [ ] Export format requirements (STEP, STL, SVG)
- [ ] Common pitfalls (make_face(), wrong Plane, fillet constraints)

---

## 📐 Source Evaluation

### Reliable Sources

| Source Type | Reliability | Best For |
|-------------|-------------|----------|
| build123d ReadTheDocs | ⭐⭐⭐⭐⭐ | API reference, tutorials |
| build123d GitHub Repo | ⭐⭐⭐⭐⭐ | Examples, source code |
| CadQuery Docs (related) | ⭐⭐⭐⭐ | Similar patterns, concepts |
| Open Cascade Docs | ⭐⭐⭐⭐ | Low-level BREP operations |
| CAD Forum Posts | ⭐⭐⭐ | Problem solving |
| Engineering Toolbox | ⭐⭐⭐⭐ | Dimensions, specifications |

### Industry Standards Sources

| Source | Reliability | Coverage |
|--------|-------------|----------|
| API Publications | ⭐⭐⭐⭐⭐ | Oil & Gas |
| ASME Digital Collection | ⭐⭐⭐⭐⭐ | Pressure Equipment |
| ISO | ⭐⭐⭐⭐⭐ | International |
| Engineering Toolbox | ⭐⭐⭐⭐ | Quick reference |
| Wikipedia | ⭐⭐⭐ | Overview only |

---

## ⚠️ Research Pitfalls

1. **CadQuery vs build123d**: APIs are similar but NOT identical — always verify
2. **Version Mismatch**: build123d API evolves — check for current syntax
3. **Incomplete Data**: Verify all dimensions with engineering tables, don't assume
4. **Conflicting Sources**: Cross-reference critical dimensions from multiple standards
5. **Over-Research**: Know when you have enough to proceed with modeling
6. **Under-Research**: Don't hand off with missing critical dimensions or standards

---

## 📋 Handoff Checklist

Before delegating to Coder, ensure:

- [ ] All requested dimensions found (in mm)
- [ ] Material specifications identified (for documentation)
- [ ] Relevant standards documented (API, ASME, ISO numbers)
- [ ] build123d approach determined (BuildPart, extrude/revolve/sweep/loft)
- [ ] Sketch planes identified (XY, XZ, YZ for each feature)
- [ ] Export format confirmed (STEP for CAD interchange, STL for 3D printing)
- [ ] Potential challenges noted (complex topology, multi-body)
- [ ] Sources documented for reference
