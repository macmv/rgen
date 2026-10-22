#version 330 core

// Ray marches a grid of columns of blocks. Each column is one block wide, and
// its height is a whole number of blocks.

uniform sampler2D uHeight; // R32F, the top of each column
uniform sampler2D uColor;  // RGBA8, the color of each column

uniform vec3 uEye;
uniform vec3 uForward;
uniform vec3 uRight; // scaled so that forward + right * x is a ray, for x in -1..1
uniform vec3 uUp;
uniform vec4 uViewport; // x, y, width, height in pixels
uniform float uPixel;   // the angle of one pixel, in radians

uniform ivec2 uSize; // columns along x and z
uniform float uMaxHeight;
uniform ivec2 uHover; // the highlighted column, or (-1, -1)

out vec4 outColor;

vec3 sky(vec3 dir) {
  float t = clamp(dir.y * 1.5 + 0.2, 0.0, 1.0);
  return mix(vec3(0.74, 0.82, 0.92), vec3(0.36, 0.55, 0.85), t);
}

float hash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}

// What one spot on the ground near a hit looks like, without marching a ray of
// its own. `p` is the spot, `hitY` is the height of the hit it's near, `d` is
// the ray direction, and `r` is how wide a pixel is there, in blocks. This is
// for averaging the footprint of a pixel that covers several blocks.
//
// look does this by comparing the ray's own height over the spot against the
// column's top. That only works for rays that are close to level: a ray that
// looks steeply down is far over or under every neighbor, so the result flips
// between the top, the side and nothing at fixed angles, which shows up as
// rings on the ground below the camera. Instead this goes by how much higher
// the neighbor is than the hit, which doesn't depend on the viewing angle.
bool footprintSample(vec2 p, float hitY, vec3 d, float r, out vec3 albedo) {
  ivec2 c = ivec2(floor(p));
  if (c.x < 0 || c.y < 0 || c.x >= uSize.x || c.y >= uSize.y) return false;

  float h = texelFetch(uHeight, c, 0).r;
  if (h < 0.0) return false;

  vec3 base = texelFetch(uColor, c, 0).rgb;
  if (h - hitY > max(1.0, r)) {
    // The neighbor stands above the hit, so the pixel sees its side.
    albedo = base * (abs(d.x) > abs(d.z) ? 0.62 : 0.8);
  } else {
    albedo = base;
  }
  return true;
}

void main() {
  vec2 ndc = (gl_FragCoord.xy - uViewport.xy) / uViewport.zw * 2.0 - 1.0;
  vec3 dir = normalize(uForward + ndc.x * uRight + ndc.y * uUp);
  vec3 origin = uEye;

  vec2 extent = vec2(uSize);
  vec3 bounds = vec3(extent.x, uMaxHeight, extent.y);

  // Clip the ray to the box around the terrain.
  vec3 safe = vec3(
    abs(dir.x) < 1e-9 ? 1e-9 : dir.x,
    abs(dir.y) < 1e-9 ? 1e-9 : dir.y,
    abs(dir.z) < 1e-9 ? 1e-9 : dir.z
  );
  vec3 t0 = (vec3(0.0) - origin) / safe;
  vec3 t1 = (bounds - origin) / safe;
  vec3 tNear = min(t0, t1);
  vec3 tFar = max(t0, t1);
  float tIn = max(max(tNear.x, tNear.y), max(tNear.z, 0.0));
  float tOut = min(min(tFar.x, tFar.y), tFar.z);

  vec3 color = sky(dir);

  if (tIn < tOut) {
    vec3 start = origin + dir * (tIn + 1e-3);
    ivec2 cell = clamp(ivec2(floor(start.xz)), ivec2(0), uSize - 1);
    ivec2 stepDir = ivec2(dir.x >= 0.0 ? 1 : -1, dir.z >= 0.0 ? 1 : -1);
    vec2 d = safe.xz;
    vec2 tDelta = 1.0 / abs(d);
    vec2 boundary = vec2(cell) + vec2((stepDir + 1) / 2);
    vec2 tMax = (boundary - origin.xz) / d;

    float tCur = tIn;
    int lastAxis = abs(dir.x) > abs(dir.z) ? 0 : 1;

    for (int i = 0; i < (uSize.x + uSize.y) * 2; i++) {
      float h = texelFetch(uHeight, cell, 0).r;
      float tExit = min(tMax.x, tMax.y);

      float yIn = origin.y + dir.y * tCur;
      float yOut = origin.y + dir.y * tExit;

      // 0 is a miss, 1 is the side of a column, 2 is the top.
      int hit = 0;
      float t = 0.0;
      if (yIn < h) {
        hit = 1;
        t = tCur;
      } else if (yOut < h) {
        hit = 2;
        t = (h - origin.y) / safe.y;
      }

      if (hit != 0) {
        vec3 normal = vec3(0.0);
        float shade = 1.0;
        if (hit == 2) {
          normal.y = 1.0;
        } else if (lastAxis == 0) {
          normal.x = -float(stepDir.x);
          shade = 0.62;
        } else {
          normal.z = -float(stepDir.y);
          shade = 0.8;
        }

        // Step slightly into the block, to find which block was hit.
        vec3 pos = origin + dir * t;
        vec3 inside = pos - normal * 0.01;
        vec3 block = floor(inside);
        vec3 local = inside - block;

        vec3 base = texelFetch(uColor, cell, 0).rgb;

        // How much of a block one pixel covers. Blocks smaller than a few
        // pixels can't show any detail, so their detail fades out.
        float footprint = t * uPixel;
        float detail = 1.0 - smoothstep(0.15, 0.6, footprint);

        // Each block is a flat shade, with hard edges between blocks.
        base *= 1.0 + (hash(block) - 0.5) * 0.14 * detail;

        // A thin dark line along every block edge, about one pixel wide.
        vec2 uv = normal.y > 0.5 ? local.xz : (normal.x != 0.0 ? local.zy : local.xy);
        float edge = min(min(uv.x, 1.0 - uv.x), min(uv.y, 1.0 - uv.y));
        float line = 1.0 - smoothstep(footprint * 0.5, footprint * 1.0, edge);
        base *= 1.0 - 0.3 * line * detail;

        color = base * shade;

        // Once a pixel covers about a block, it can't pick one block's color
        // without graining, as the surface changes from block to block. So
        // average it with four more spots of its footprint on the ground: two
        // along the ray, where grazing angles stretch the footprint, and two
        // across it. The average is blended in slowly, so that blocks that are
        // still several pixels wide stay sharp.
        float filterAmount = smoothstep(0.5, 1.5, footprint);
        if (filterAmount > 0.0) {
          float lh = max(length(d), 1e-4);
          vec2 along = d / lh;
          vec2 across = vec2(-along.y, along.x);
          float alongLen = min(footprint / max(abs(dir.y), 0.05), 4.0);

          vec3 sum = color * 2.0;
          float weight = 2.0;
          for (int k = 0; k < 4; k++) {
            float sg = (k & 1) == 0 ? -0.5 : 0.5;
            vec2 offset = k < 2 ? along * (sg * alongLen) : across * (sg * footprint);
            vec3 other;
            if (footprintSample(pos.xz + offset, pos.y, dir, footprint, other)) {
              sum += other;
              weight += 1.0;
            }
          }
          color = mix(color, sum / weight, filterAmount);
        }

        if (cell == uHover) {
          color = mix(color, vec3(1.0), 0.35);
        }

        break;
      }

      if (tExit > tOut) break;

      if (tMax.x < tMax.y) {
        cell.x += stepDir.x;
        tCur = tMax.x;
        tMax.x += tDelta.x;
        lastAxis = 0;
      } else {
        cell.y += stepDir.y;
        tCur = tMax.y;
        tMax.y += tDelta.y;
        lastAxis = 1;
      }
      if (cell.x < 0 || cell.y < 0 || cell.x >= uSize.x || cell.y >= uSize.y) break;
    }
  }

  outColor = vec4(color, 1.0);
}
