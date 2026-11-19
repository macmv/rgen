use rgen_biome::{DrainageVector, TerrainNoise};
use sdl2::{pixels::Color, rect::Point};
use crate::render::Render;

pub struct VectorOverlay {
  terrain: TerrainNoise,
  bounds: Option<(f64, f64, f64, f64)>,
  segments: Vec<DrainageVector>,
}
impl VectorOverlay {
  pub fn new(seed:u64) -> Self {
    Self { terrain:TerrainNoise::new(seed),bounds:None,segments:Vec::new() }
  }

  pub fn draw(&mut self, render:&mut Render, origin:(f64,f64), zoom:f64,
    width:u32, height:u32) -> Result<(),String> {
    let right = origin.0 + width as f64/zoom;
    let bottom = origin.1 + height as f64/zoom;
    // Cache a padded viewport so toggling or small pans do not rebuild plans.
    // Replace the cache when zooming far in too; don't retain a world-sized list.
    let refresh = self.bounds.is_none_or(|(x,z,r,b)| origin.0 < x || origin.1 < z
      || right > r || bottom > b
      || (r-x)*(b-z) > (right-origin.0+512.0)*(bottom-origin.1+512.0)*4.0);
    if refresh {
      let x = origin.0-256.0;
      let z = origin.1-256.0;
      self.segments = self.terrain.drainage_vectors(x,z,right-x+256.0,bottom-z+256.0);
      self.bounds = Some((x,z,right+256.0,bottom+256.0));
    }
    let screen = |p:(f64,f64)| Point::new(((p.0-origin.0)*zoom).round() as i32,
      ((p.1-origin.1)*zoom).round() as i32);
    for (index,s) in self.segments.iter().enumerate() {
      if s.from.0.min(s.to.0) > right || s.from.0.max(s.to.0) < origin.0
        || s.from.1.min(s.to.1) > bottom || s.from.1.max(s.to.1) < origin.1 { continue; }
      let a = screen(s.from);
      let b = screen(s.to);
      // A dark one-pixel shadow keeps vectors readable on every map mode.
      render.canvas.set_draw_color(Color::RGB(20,25,30));
      render.canvas.draw_line(Point::new(a.x+1,a.y+1),Point::new(b.x+1,b.y+1))?;
      render.canvas.set_draw_color(if s.valley { Color::RGB(255,160,65) }
        else { Color::RGB(40,235,255) });
      render.canvas.draw_line(a,b)?;
      // Canonical pieces point upstream; arrows must point the opposite way.
      let dx = (a.x-b.x) as f64;
      let dz = (a.y-b.y) as f64;
      let length = dx.hypot(dz);
      if index % 8 == 0 && length >= 10.0 {
        let ux = dx/length;
        let uz = dz/length;
        let tip = Point::new((a.x+b.x)/2,(a.y+b.y)/2);
        for side in [-1.0,1.0] {
          let tail = Point::new((tip.x as f64-ux*6.0-uz*side*3.0) as i32,
            (tip.y as f64-uz*6.0+ux*side*3.0) as i32);
          render.canvas.draw_line(tip,tail)?;
        }
      }
    }
    Ok(())
  }
}
