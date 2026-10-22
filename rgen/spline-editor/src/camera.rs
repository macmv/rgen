use eframe::egui::{Pos2, Rect};

use crate::grid::{dot, normalize};

/// An orbit camera looking at a point above the middle of the grid.
pub struct Camera {
  pub yaw:   f32,
  pub pitch: f32,
  /// Distance from the target, as a multiple of the grid's width.
  pub zoom:  f32,
}

impl Default for Camera {
  fn default() -> Self { Camera { yaw: 0.6, pitch: 0.55, zoom: 0.9 } }
}

/// A camera for one frame, with its basis computed.
pub struct View {
  eye:     [f32; 3],
  forward: [f32; 3],
  right:   [f32; 3],
  up:      [f32; 3],
  center:  Pos2,
  focal:   f32,
}

impl Camera {
  pub fn view(&self, rect: Rect, extent: f32, target: [f32; 3]) -> View {
    let dist = self.zoom * extent;
    let (sy, cy) = self.yaw.sin_cos();
    let (sp, cp) = self.pitch.sin_cos();

    let eye = [target[0] + dist * cp * sy, target[1] + dist * sp, target[2] + dist * cp * cy];
    let forward = normalize([target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]]);
    let right = normalize([-forward[2], 0.0, forward[0]]);
    let up = [
      right[1] * forward[2] - right[2] * forward[1],
      right[2] * forward[0] - right[0] * forward[2],
      right[0] * forward[1] - right[1] * forward[0],
    ];

    // 45 degree vertical field of view.
    let focal = rect.height() * 0.5 / (22.5f32.to_radians()).tan();

    View { eye, forward, right, up, center: rect.center(), focal }
  }
}

impl View {
  pub fn eye(&self) -> [f32; 3] { self.eye }

  /// Projects a point to the screen. Returns the position and the depth, or
  /// `None` if the point is behind the camera.
  pub fn project(&self, p: [f32; 3]) -> Option<(Pos2, f32)> {
    let d = [p[0] - self.eye[0], p[1] - self.eye[1], p[2] - self.eye[2]];
    let z = dot(d, self.forward);
    if z < 1.0 {
      return None;
    }

    Some((
      Pos2::new(
        self.center.x + dot(d, self.right) / z * self.focal,
        self.center.y - dot(d, self.up) / z * self.focal,
      ),
      z,
    ))
  }
}
