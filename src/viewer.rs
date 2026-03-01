//! Interactive 3D viewer for generated STL files.
//!
//! After the pipeline produces `.stl` output, this module generates a standalone
//! HTML file with an embedded Three.js scene and opens it in the default browser.
//! The STL binary is base64-encoded inline so no local server is needed.

use base64::Engine;
use std::path::{Path, PathBuf};

/// Errors specific to viewer generation.
#[derive(Debug)]
pub enum ViewerError {
    /// No STL file found in the output directory.
    NoStlFound,
    /// File I/O failure.
    IoError(std::io::Error),
}

impl std::fmt::Display for ViewerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoStlFound => write!(f, "No .stl file found in output directory"),
            Self::IoError(e) => write!(f, "Viewer I/O error: {}", e),
        }
    }
}

impl std::error::Error for ViewerError {}

impl From<std::io::Error> for ViewerError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

/// Scans `output_dir` for `.stl` files, generates an HTML viewer, and opens it.
///
/// Returns the path to the generated HTML file on success.
///
/// # Errors
///
/// Returns [`ViewerError::NoStlFound`] if no `.stl` file exists in the directory.
pub fn generate_and_open(output_dir: &Path) -> Result<PathBuf, ViewerError> {
    let stl_path = find_stl_file(output_dir)?;
    let stl_bytes = std::fs::read(&stl_path)?;
    let stl_base64 = base64::engine::general_purpose::STANDARD.encode(&stl_bytes);

    let stl_filename = stl_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let html = generate_viewer_html(&stl_base64, &stl_filename);

    let viewer_path = output_dir.join("viewer.html");
    std::fs::write(&viewer_path, &html)?;

    // Open in default browser (macOS)
    let _ = std::process::Command::new("open").arg(&viewer_path).spawn();

    Ok(viewer_path)
}

/// Finds the first `.stl` file in the given directory.
fn find_stl_file(dir: &Path) -> Result<PathBuf, ViewerError> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "stl") {
                return Ok(path);
            }
        }
    }
    Err(ViewerError::NoStlFound)
}

/// Generates a self-contained HTML file with embedded Three.js STL viewer.
fn generate_viewer_html(stl_base64: &str, model_name: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{model_name} — 3D Viewer</title>
<style>
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{
    background: radial-gradient(circle at 20% 20%, #202a45 0%, #101624 45%, #06080f 100%);
    overflow: hidden;
    font-family: 'Inter', system-ui, sans-serif;
  }}
  canvas {{ display: block; }}
  #info {{
    position: absolute; top: 16px; left: 16px;
    background: rgba(0,0,0,0.7); color: #e0e0e0;
    padding: 12px 16px; border-radius: 8px;
    font-size: 13px; line-height: 1.6;
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255,255,255,0.1);
  }}
  #info h2 {{ color: #64ffda; font-size: 15px; margin-bottom: 4px; }}
  #info .dim {{ color: #888; }}
  #controls {{
    position: absolute; bottom: 16px; left: 50%;
    transform: translateX(-50%);
    background: rgba(0,0,0,0.5); color: #888;
    padding: 8px 16px; border-radius: 20px;
    font-size: 12px;
    backdrop-filter: blur(10px);
  }}
</style>
</head>
<body>
<div id="info">
  <h2>📐 {model_name}</h2>
  <div id="dimensions" class="dim">Loading...</div>
</div>
<div id="controls">🖱️ Drag to rotate · Scroll to zoom · Right-click to pan</div>

<script type="importmap">
{{
  "imports": {{
    "three": "https://cdn.jsdelivr.net/npm/three@0.170.0/build/three.module.js",
    "three/addons/": "https://cdn.jsdelivr.net/npm/three@0.170.0/examples/jsm/"
  }}
}}
</script>
<script type="module">
import * as THREE from 'three';
import {{ OrbitControls }} from 'three/addons/controls/OrbitControls.js';
import {{ STLLoader }} from 'three/addons/loaders/STLLoader.js';

// --- Scene setup ---
const scene = new THREE.Scene();
scene.background = new THREE.Color(0x101624);
scene.fog = new THREE.FogExp2(0x101624, 0.0022);

const camera = new THREE.PerspectiveCamera(45, innerWidth / innerHeight, 0.1, 2000);
const renderer = new THREE.WebGLRenderer({{ antialias: true }});
renderer.setSize(innerWidth, innerHeight);
renderer.setPixelRatio(devicePixelRatio);
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.PCFSoftShadowMap;
renderer.toneMapping = THREE.ACESFilmicToneMapping;
renderer.toneMappingExposure = 1.2;
document.body.appendChild(renderer.domElement);

// --- Lights ---
const ambient = new THREE.AmbientLight(0x30425f, 1.0);
scene.add(ambient);

const hemi = new THREE.HemisphereLight(0x99ddff, 0x0a1636, 0.7);
scene.add(hemi);

const dirLight = new THREE.DirectionalLight(0xffffff, 1.35);
dirLight.position.set(80, 120, 60);
dirLight.castShadow = true;
dirLight.shadow.mapSize.set(2048, 2048);
scene.add(dirLight);

const fillLight = new THREE.DirectionalLight(0xffb36f, 0.5);
fillLight.position.set(-60, 40, -40);
scene.add(fillLight);

// --- Grid ---
const grid = new THREE.GridHelper(200, 40, 0x3b4f73, 0x27334d);
grid.position.y = -0.1;
scene.add(grid);

// --- Controls ---
const controls = new OrbitControls(camera, renderer.domElement);
controls.enableDamping = true;
controls.dampingFactor = 0.08;
controls.rotateSpeed = 0.8;

// --- Load STL from inline base64 ---
const stlBase64 = "{stl_base64}";
const stlBinary = Uint8Array.from(atob(stlBase64), c => c.charCodeAt(0));

const loader = new STLLoader();
const geometry = loader.parse(stlBinary.buffer);
geometry.computeVertexNormals();

// Center the geometry
geometry.computeBoundingBox();
const bbox = geometry.boundingBox;
const center = new THREE.Vector3();
bbox.getCenter(center);
geometry.translate(-center.x, -center.y, -center.z);

// Dimensions
const size = new THREE.Vector3();
bbox.getSize(size);
document.getElementById('dimensions').innerHTML =
  `${{size.x.toFixed(1)}} × ${{size.y.toFixed(1)}} × ${{size.z.toFixed(1)}} mm` +
  `<br><span style="color:#555">${{(geometry.attributes.position.count / 3)}} triangles</span>`;

// Material — high-contrast warm body with cool wireframe
const material = new THREE.MeshStandardMaterial({{
  color: 0xff6b3d,
  emissive: 0x330f0a,
  metalness: 0.35,
  roughness: 0.25,
  envMapIntensity: 1.1,
}});

const mesh = new THREE.Mesh(geometry, material);
mesh.castShadow = true;
mesh.receiveShadow = true;
mesh.scale.setScalar(1.0);

// Position so bottom sits on grid
const meshBbox = new THREE.Box3().setFromObject(mesh);
mesh.position.y -= meshBbox.min.y;
scene.add(mesh);

const edgeMaterial = new THREE.LineBasicMaterial({{
  color: 0x91f6ff,
  transparent: true,
  opacity: 0.55
}});
const edges = new THREE.LineSegments(
  new THREE.EdgesGeometry(geometry, 30),
  edgeMaterial
);
edges.position.copy(mesh.position);
scene.add(edges);

// --- Camera position ---
const maxDim = Math.max(size.x, size.y, size.z);
const cameraDistance = maxDim * 2.5;
camera.position.set(cameraDistance * 0.7, cameraDistance * 0.5, cameraDistance * 0.7);
controls.target.set(0, size.z * 0.4, 0);
controls.update();

// Ground plane for shadow
const groundGeo = new THREE.PlaneGeometry(400, 400);
const groundMat = new THREE.ShadowMaterial({{ opacity: 0.3 }});
const ground = new THREE.Mesh(groundGeo, groundMat);
ground.rotation.x = -Math.PI / 2;
ground.receiveShadow = true;
scene.add(ground);

// Adjust grid to bottom of part
grid.position.y = 0;

// --- Animation loop ---
function animate() {{
  requestAnimationFrame(animate);
  controls.update();
  renderer.render(scene, camera);
}}
animate();

// --- Resize ---
addEventListener('resize', () => {{
  camera.aspect = innerWidth / innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(innerWidth, innerHeight);
}});
</script>
</body>
</html>"##,
        model_name = model_name,
        stl_base64 = stl_base64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_stl_returns_error_for_empty_dir() {
        let result = find_stl_file(Path::new("/tmp/nonexistent_viewer_test_dir_99999"));
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_html_contains_three_js() {
        let html = generate_viewer_html("AQID", "test_model");
        assert!(html.contains("three"));
        assert!(html.contains("STLLoader"));
        assert!(html.contains("AQID")); // base64 data
        assert!(html.contains("test_model"));
    }
}
