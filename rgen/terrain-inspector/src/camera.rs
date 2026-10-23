use eframe::egui::{Pos2, Rect};

use crate::grid::{dot, normalize};

/// An orbit camera looking at a point on the ground.
pub struct Camera {
  pub yaw:    f32,
  pub pitch:  f32,
  /// Distance from the target, as a multiple of the extent that the camera is
  /// given.
  pub zoom:   f32,
  /// The world position the camera looks at, in blocks.
  pub target: [f64; 2],
}

impl Default for Camera {
  fn default() -> Self { Camera { yaw: 0.6, pitch: 0.55, zoom: 0.9, target: [0.0, 0.0] } }
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

    self.target[0] += f64::from((-right[0] * delta[0] + forward[0] * delta[1]) * scale);
    self.target[1] += f64::from((-right[1] * delta[0] + forward[1] * delta[1]) * scale);
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

  /// The x and z of the point where the ray through a screen position crosses
  /// the horizontal plane at height `y`.
  pub fn ground_at(&self, pos: Pos2, y: f32) -> Option<[f32; 2]> {
    let (origin, dir) = self.ray(pos);
    if dir[1] >= 0.0 {
      return None;
    }
    let t = (y - origin[1]) / dir[1];
    Some([origin[0] + dir[0] * t, origin[2] + dir[2] * t])
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
