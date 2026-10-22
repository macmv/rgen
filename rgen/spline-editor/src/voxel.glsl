#version 330 core

// Ray marches a grid of columns of cubes. Each column is `uStep` blocks wide,
// and its height is a multiple of `uStep`, so every block is a cube.

uniform sampler2D uHeight; // R32F, the top of each column
uniform sampler2D uColor;  // RGBA8, the color of each column

uniform vec3 uEye;
uniform vec3 uForward;
uniform vec3 uRight; // scaled so that forward + right * x is a ray, for x in -1..1
uniform vec3 uUp;
uniform vec4 uViewport; // x, y, width, height in pixels
uniform float uPixel;   // the angle of one pixel, in radians

uniform int uN;
uniform float uStep;
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

void main() {
  vec2 ndc = (gl_FragCoord.xy - uViewport.xy) / uViewport.zw * 2.0 - 1.0;
  vec3 dir = normalize(uForward + ndc.x * uRight + ndc.y * uUp);
  vec3 origin = uEye;

  float extent = float(uN) * uStep;
  vec3 bounds = vec3(extent, uMaxHeight, extent);

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
    ivec2 cell = clamp(ivec2(floor(start.xz / uStep)), ivec2(0), ivec2(uN - 1));
    ivec2 stepDir = ivec2(dir.x >= 0.0 ? 1 : -1, dir.z >= 0.0 ? 1 : -1);
    vec2 d = safe.xz;
    vec2 tDelta = uStep / abs(d);
    vec2 boundary = (vec2(cell) + vec2((stepDir + 1) / 2)) * uStep;
    vec2 tMax = (boundary - origin.xz) / d;

    float tCur = tIn;
    int lastAxis = abs(dir.x) > abs(dir.z) ? 0 : 1;

    for (int i = 0; i < uN * 3; i++) {
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
        vec3 inside = (pos - normal * 0.01 * uStep) / uStep;
        vec3 block = floor(inside);
        vec3 local = inside - block;

        vec3 base = texelFetch(uColor, cell, 0).rgb;

        // How much of a block one pixel covers. Blocks smaller than a few
        // pixels can't show any detail, so their detail fades out.
        float footprint = t * uPixel / uStep;
        float detail = 1.0 - smoothstep(0.1, 0.3, footprint);

        // Each block is a flat shade, with hard edges between blocks.
        base *= 1.0 + (hash(block) - 0.5) * 0.14 * detail;

        // A thin dark line along every block edge, about one pixel wide.
        vec2 uv = normal.y > 0.5 ? local.xz : (normal.x != 0.0 ? local.zy : local.xy);
        float edge = min(min(uv.x, 1.0 - uv.x), min(uv.y, 1.0 - uv.y));
        float line = 1.0 - smoothstep(footprint * 0.5, footprint * 1.0, edge);
        base *= 1.0 - 0.3 * line * detail;

        color = base * shade;
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
      if (cell.x < 0 || cell.y < 0 || cell.x >= uN || cell.y >= uN) break;
    }
  }

  outColor = vec4(color, 1.0);
}
