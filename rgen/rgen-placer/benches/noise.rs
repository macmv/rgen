#![feature(test)]

extern crate test;

use rgen_placer::noise::{NoiseGenerator, OctavedNoise, OpenSimplexNoise, PerlinNoise};
use test::Bencher;

#[bench]
fn bench_perlin(b: &mut Bencher) {
  let noise: OctavedNoise<PerlinNoise, 16> = OctavedNoise::new(1.0 / 4096.0, PerlinNoise::new);

  b.iter(|| {
    test::black_box(noise.generate(0.0, 0.0));
  });
}

#[bench]
fn bench_open_simplex(b: &mut Bencher) {
  let noise: OctavedNoise<OpenSimplexNoise, 16> =
    OctavedNoise::new(1.0 / 4096.0, OpenSimplexNoise::new);

  b.iter(|| {
    test::black_box(noise.generate(0.0, 0.0));
  });
}
