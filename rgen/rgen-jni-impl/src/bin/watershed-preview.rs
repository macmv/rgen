//! Run the binary built by buildRgen; no Minecraft chunks or registry required.
use std::{fmt::Write, path::PathBuf};
use rgen_biome::TerrainNoise;
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<_> = std::env::args().collect();
  let output = PathBuf::from(args.get(1).map(String::as_str).unwrap_or("/tmp/rgen-watershed.svg"));
  let seed: u64 = args.get(2).map(String::as_str).unwrap_or("1").parse()?;
  let terrain = TerrainNoise::new(seed);
  let start = std::time::Instant::now();
  let mut svg = String::from("<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1024 1064'><rect width='1024' height='1064' fill='#faf7ed'/>");
  // 8192 blocks across, centered on origin. Only broad map samples, no chunks.
  for z in 0..512 {
    let mut run_start = 0;
    let mut previous = "";
    for x in 0..=512 {
      let color = if x == 512 { "" } else {
        let wx = x as f64 * 16.0 - 4096.0;
        let wz = z as f64 * 16.0 - 4096.0;
        let c = terrain.sample_continentalness(wx,wz);
        let p = terrain.sample_peaks_valleys(wx,wz);
        if c < 0.32 { "#619fb5" } else if c < 0.37 { "#dfd39d" }
          else if c > 0.65 || p > 0.66 { "#a4a18a" } else { "#c4d3a0" }
      };
      if color != previous {
        if x > 0 {
          write!(svg,"<path d='M{} {}h{}v2H{}z' fill='{previous}'/>",
            run_start*2,z*2,(x-run_start)*2,run_start*2)?;
        }
        run_start = x;
        previous = color;
      }
    }
  }
  let (edges, basins) = terrain.watershed_preview_paths(-4096.0,-4096.0,8192.0,0.125);
  svg.push_str(&edges);
  write!(svg,"<text x='12' y='1050' font-family='sans-serif' font-size='15'>Seed {seed} · 8192 × 8192 blocks · {basins} coastal catchments · blue: wet route · brown: valley</text></svg>")?;
  std::fs::write(&output,svg)?;
  let report = terrain.audit_watershed_preview();
  let context = rgen_world::Context::new_test(seed);
  let generator = rgen_biome::WorldBiomes::new(&context.blocks,seed);
  let excavation_checks = generator.audit_river_excavation();
  println!("{excavation_checks} original-density excavation checks passed");
  println!("{}; {report}; elapsed {:.2}s",output.display(),start.elapsed().as_secs_f64());
  Ok(())
}
