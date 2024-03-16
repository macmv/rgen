#![feature(test)]

extern crate test;

use rgen_placer::noise::{NoiseGenerator, OctavedNoise, OpenSimplexNoise, PerlinNoise};
use test::Bencher;

#[bench]
fn bench_perlin(b: &mut Bencher) {
  let noise: OctavedNoise<PerlinNoise> =
    OctavedNoise { octaves: 16, freq: 1.0 / 4096.0, ..Default::default() };

  b.iter(|| {
    test::black_box(noise.generate(0.0, 0.0, 0));
  });
}

#[bench]
fn bench_open_simplex(b: &mut Bencher) {
  let noise: OctavedNoise<OpenSimplexNoise> =
    OctavedNoise { octaves: 16, freq: 1.0 / 4096.0, ..Default::default() };

  b.iter(|| {
    test::black_box(noise.generate(0.0, 0.0, 0));
  });
}
