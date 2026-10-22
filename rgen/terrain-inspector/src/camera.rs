use eframe::egui::{Pos2, Rect};

use crate::grid::{dot, normalize};

/// An orbit camera looking at a point above the middle of the grid.
pub struct Camera {
  pub yaw:   f32,
  pub pitch: f32,
  /// Distance from the target, as a multiple of the grid's width.
  pub zoom:  f32,
  /// How far the target is from the middle of the grid, in blocks.
  pub pan:   [f32; 2],
}

impl Default for Camera {
  fn default() -> Self { Camera { yaw: 0.6, pitch: 0.55, zoom: 0.9, pan: [0.0, 0.0] } }
}

/// A camera for one frame, with its basis computed.
pub struct View {
  eye:     [f32; 3],
  forward: [f32; 3],
  right:   [f32; 3],
  up:      [f32; 3],
  center:  Pos2,
  focal:   f32,
  height:  f32,
}

impl Camera {
  /// Moves the target along the ground, as if dragging the terrain by
  /// `delta` pixels.
  pub fn drag_pan(&mut self, delta: [f32; 2], extent: f32, rect: Rect) {
    let (sy, cy) = self.yaw.sin_cos();
    let scale = self.zoom * extent / rect.height();
    // The ground-plane right and forward vectors of the camera.
    let right = [cy, -sy];
    let forward = [-sy, -cy];

    self.pan[0] += (-right[0] * delta[0] + forward[0] * delta[1]) * scale;
    self.pan[1] += (-right[1] * delta[0] + forward[1] * delta[1]) * scale;

    let limit = extent / 2.0;
    self.pan = self.pan.map(|p| p.clamp(-limit, limit));
  }

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

    View { eye, forward, right, up, center: rect.center(), focal, height: rect.height() }
  }
}

impl View {
  pub fn eye(&self) -> [f32; 3] { self.eye }
  pub fn forward(&self) -> [f32; 3] { self.forward }

  /// The right vector, scaled so that `forward + right * x` is the direction
  /// at the screen's horizontal position `x` in `-1..=1`.
  pub fn right_scaled(&self, aspect: f32) -> [f32; 3] {
    self.right.map(|v| v * aspect * self.height * 0.5 / self.focal)
  }

  /// Like `right_scaled`, for the vertical position.
  pub fn up_scaled(&self) -> [f32; 3] { self.up.map(|v| v * self.height * 0.5 / self.focal) }

  /// The ray through a screen position.
  pub fn ray(&self, pos: Pos2) -> ([f32; 3], [f32; 3]) {
    let x = pos.x - self.center.x;
    let y = self.center.y - pos.y;
    let dir = normalize([
      self.forward[0] * self.focal + self.right[0] * x + self.up[0] * y,
      self.forward[1] * self.focal + self.right[1] * x + self.up[1] * y,
      self.forward[2] * self.focal + self.right[2] * x + self.up[2] * y,
    ]);
    (self.eye, dir)
  }

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
