#version 330 core

// Ray marches a world of blocks, which is split into chunks. A ray steps
// through the chunks first. A chunk is empty, all solid, or has blocks of its
// own, which are stored in a slot of the brick atlas. Those the ray steps
// through block by block.

uniform sampler2D uColor;   // RGBA8, the color of each column
uniform usampler3D uTable;  // R32UI, one per chunk: 0 empty, 1 solid, else slot + 2
uniform usampler3D uBricks; // R8UI, the atlas. Texels are laid out as (y, x, z).

uniform vec3 uEye;
uniform vec3 uForward;
uniform vec3 uRight; // scaled so that forward + right * x is a ray, for x in -1..1
uniform vec3 uUp;
uniform vec4 uViewport; // x, y, width, height in pixels
uniform float uPixel;   // the angle of one pixel, in radians

uniform ivec3 uChunks; // chunks along x, y and z
uniform int uAtlasSide; // slots along each side of the atlas
uniform ivec2 uShift; // how far the grid is rotated to get the window, in blocks
uniform ivec2 uHover; // the highlighted column, or (-1, -1)

out vec4 outColor;

const int CHUNK = 32;

vec3 sky(vec3 dir) {
  float t = clamp(dir.y * 1.5 + 0.2, 0.0, 1.0);
  return mix(vec3(0.74, 0.82, 0.92), vec3(0.36, 0.55, 0.85), t);
}

float hash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}

ivec3 atlasOrigin(int slot) {
  return ivec3(
    slot % uAtlasSide,
    (slot / uAtlasSide) % uAtlasSide,
    slot / (uAtlasSide * uAtlasSide)
  ) * CHUNK;
}

// The table cell of a chunk of the window. The grid wraps around, and the
// window starts uShift blocks into it.
ivec3 wrapCell(ivec3 c) {
  c.xz = (c.xz + uShift / CHUNK) % uChunks.xz;
  return c;
}

// The texel of the color texture for a column of the window.
ivec2 wrapColumn(ivec2 v) {
  return (v + uShift) % (uChunks.xz * CHUNK);
}

bool solidAt(ivec3 v) {
  if (any(lessThan(v, ivec3(0))) || any(greaterThanEqual(v, uChunks * CHUNK))) return false;

  ivec3 c = v / CHUNK;
  uint state = texelFetch(uTable, wrapCell(c), 0).r;
  if (state < 2u) return state == 1u;

  ivec3 l = v - c * CHUNK;
  return texelFetch(uBricks, atlasOrigin(int(state) - 2) + ivec3(l.y, l.x, l.z), 0).r != 0u;
}

// How bright a face is, by the axis it points along.
float faceShade(int axis, bool up) {
  return axis == 1 ? (up ? 1.0 : 0.5) : (axis == 0 ? 0.62 : 0.8);
}

// What one spot on the ground near a hit looks like, without marching a ray of
// its own. `spot` is the spot on the ground, `hitY` is the height of the hit
// it's near, `d` is the ray direction, and `r` is how wide a pixel is there, in
// blocks. This is for averaging the footprint of a pixel that covers several
// blocks.
//
// look does this by comparing the ray's own height over the spot against the
// column's top. That only works for rays that are close to level: a ray that
// looks steeply down is far over or under every neighbor, so the result flips
// between the top, the side and nothing at fixed angles, which shows up as
// rings on the ground below the camera. Instead this goes by how much higher
// the neighbor is than the hit, which doesn't depend on the viewing angle.
//
// The spots are always on the ground, around the hit, even if the hit was on
// the side of something. A pixel that hits a side still covers some of the top
// of what's next to it, and averaging only the sides would leave it dark.
//
// The neighbor's top is found by walking up or down the blocks at the spot,
// starting from the height of the hit. That finds the surface near the hit, so
// it's the right one even if there are overhangs above or below it.
bool footprintSample(vec2 spot, float hitY, vec3 d, float r, out vec3 albedo) {
  // Walking this far is enough to tell if the neighbor is more than `r` away.
  int steps = clamp(int(r) + 2, 3, 16);

  ivec3 s = ivec3(int(floor(spot.x)), int(floor(hitY - 0.01)), int(floor(spot.y)));
  float top;
  if (solidAt(s)) {
    for (int k = 0; k < steps && solidAt(s + ivec3(0, 1, 0)); k++) s.y++;
  } else {
    bool found = false;
    for (int k = 0; k < steps; k++) {
      s.y--;
      if (solidAt(s)) {
        found = true;
        break;
      }
    }
    if (!found) return false;
  }
  top = float(s.y + 1);

  vec3 base = texelFetch(uColor, wrapColumn(s.xz), 0).rgb;
  if (top - hitY > max(1.0, r)) {
    // The neighbor stands above the hit, so the pixel sees its side.
    albedo = base * (abs(d.x) > abs(d.z) ? 0.62 : 0.8);
  } else {
    albedo = base;
  }
  return true;
}

// Keeps a ray direction away from zero, so that it can be divided by.
float nonzero(float v) { return abs(v) < 1e-9 ? (v < 0.0 ? -1e-9 : 1e-9) : v; }

int argmin(vec3 v) {
  if (v.x < v.y) return v.x < v.z ? 0 : 2;
  return v.y < v.z ? 1 : 2;
}

int argmax3(vec3 v) {
  if (v.x > v.y) return v.x > v.z ? 0 : 2;
  return v.y > v.z ? 1 : 2;
}

void main() {
  vec2 ndc = (gl_FragCoord.xy - uViewport.xy) / uViewport.zw * 2.0 - 1.0;
  vec3 dir = normalize(uForward + ndc.x * uRight + ndc.y * uUp);
  vec3 origin = uEye;

  vec3 bounds = vec3(uChunks * CHUNK);

  // Clip the ray to the box around the terrain.
  vec3 safe = vec3(nonzero(dir.x), nonzero(dir.y), nonzero(dir.z));
  vec3 t0 = (vec3(0.0) - origin) / safe;
  vec3 t1 = (bounds - origin) / safe;
  vec3 tNear = min(t0, t1);
  vec3 tFar = max(t0, t1);
  float tIn = max(max(tNear.x, tNear.y), max(tNear.z, 0.0));
  float tOut = min(min(tFar.x, tFar.y), tFar.z);

  vec3 color = sky(dir);

  if (tIn < tOut) {
    ivec3 stepDir = ivec3(safe.x > 0.0 ? 1 : -1, safe.y > 0.0 ? 1 : -1, safe.z > 0.0 ? 1 : -1);
    vec3 tDelta = float(CHUNK) / abs(safe);

    vec3 start = origin + safe * (tIn + 1e-3);
    ivec3 cell = clamp(ivec3(floor(start / float(CHUNK))), ivec3(0), uChunks - 1);
    vec3 tMax = (vec3((cell + (stepDir + 1) / 2) * CHUNK) - origin) / safe;

    float tCur = tIn;
    int lastAxis = argmax3(tNear);

    bool found = false;
    float tHit = 0.0;
    int hitAxis = 0;
    ivec3 voxel = ivec3(0);

    for (int i = 0; i < uChunks.x + uChunks.y + uChunks.z + 4; i++) {
      uint state = texelFetch(uTable, wrapCell(cell), 0).r;

      if (state != 0u) {
        // The block the ray enters the chunk through.
        ivec3 lo = cell * CHUNK;
        vec3 p = origin + safe * (tCur + 1e-3);
        ivec3 v = clamp(ivec3(floor(p)), lo, lo + (CHUNK - 1));

        if (state == 1u) {
          // The whole chunk is solid, so the ray hits where it enters.
          found = true;
          tHit = tCur;
          hitAxis = lastAxis;
          voxel = v;
          break;
        }

        ivec3 atlas = atlasOrigin(int(state) - 2);

        vec3 fMax = (vec3(v + (stepDir + 1) / 2) - origin) / safe;
        vec3 fDelta = 1.0 / abs(safe);
        float tF = tCur;
        int fAxis = lastAxis;

        for (int k = 0; k < 3 * CHUNK + 2; k++) {
          ivec3 l = v - lo;
          if (any(lessThan(l, ivec3(0))) || any(greaterThanEqual(l, ivec3(CHUNK)))) break;

          if (texelFetch(uBricks, atlas + ivec3(l.y, l.x, l.z), 0).r != 0u) {
            found = true;
            tHit = tF;
            hitAxis = fAxis;
            voxel = v;
            break;
          }

          int a = argmin(fMax);
          tF = fMax[a];
          fMax[a] += fDelta[a];
          v[a] += stepDir[a];
          fAxis = a;
        }
        if (found) break;
      }

      int a = argmin(tMax);
      if (tMax[a] > tOut) break;
      tCur = tMax[a];
      tMax[a] += tDelta[a];
      cell[a] += stepDir[a];
      lastAxis = a;
      if (cell[a] < 0 || cell[a] >= uChunks[a]) break;
    }

    if (found) {
      vec3 normal = vec3(0.0);
      normal[hitAxis] = -float(stepDir[hitAxis]);
      float shade = faceShade(hitAxis, normal[hitAxis] > 0.0);

      vec3 pos = origin + dir * tHit;
      vec3 block = vec3(voxel);
      vec3 local = clamp(pos - block, 0.0, 1.0);

      vec3 base = texelFetch(uColor, wrapColumn(voxel.xz), 0).rgb;

      // How much of a block one pixel covers. Blocks smaller than a few
      // pixels can't show any detail, so their detail fades out.
      float footprint = tHit * uPixel;
      float detail = 1.0 - smoothstep(0.15, 0.6, footprint);

      // Each block is a flat shade, with hard edges between blocks.
      base *= 1.0 + (hash(block) - 0.5) * 0.14 * detail;

      // A thin dark line along every block edge, about one pixel wide.
      vec2 uv = hitAxis == 1 ? local.xz : (hitAxis == 0 ? local.zy : local.xy);
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
        vec2 d = safe.xz;
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

      if (voxel.xz == uHover) {
        color = mix(color, vec3(1.0), 0.35);
      }
    }
  }

  outColor = vec4(color, 1.0);
}
