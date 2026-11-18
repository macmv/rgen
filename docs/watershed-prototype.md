# Coast-rooted watershed prototype

Enabled by `WATERSHEDS` in `rgen/rgen-biome/src/feature.rs`; the current climate override is Mediterranean.

The previous four-neighbor lattice routing and elevated sink lakes have been
replaced. The starting point is continentalness/coastline, then a broad highland
field, then a drainage graph, then detailed terrain/carving. This draws on the
upstream-growth approach in [Génevaux et al., 2013](https://perso.liris.cnrs.fr/egalin/Articles/2013-river-networks.pdf),
but is a bounded procedural adaptation, not that paper's full algorithm.

## Water and valleys

**All generated water stops at Y=63.** The block filler has a fixed sea level;
virtual drainage elevation cannot raise it. Upstream graph nodes increase in
virtual elevation, used only for planning. Lowland channels cut toward Y=59 at
the center and Y=64 at the edge. Banks rise outward from Y=64, so the
112-block valley blend does not extend submerged terrain outside the channel. Offshore carving cannot raise the seabed.

Continentalness is carried upstream as a running maximum. Between 0.56 and 0.70,
river carving transitions smoothly into dry-valley carving. Fully dry valleys
cut at most 18 blocks and target a floor no lower than Y=66; carving never raises
existing terrain. River geography is limited to the lowland wet footprint.
Existing oceans remain oceans. Sparse inline lowland lake widenings share the
river's sea level; local minima do not automatically become lakes.

## Planning

- Mouths use the uncarved terrain waterline (including submerged beaches),
  not just the Ocean biome threshold. Straight and smoothed routes are checked
  at intervals no longer than one block so they cannot skip broad sea inlets.
- Coastline samples locate candidate mouths within 768-block search cells.
  Rivers do not follow the cell grid.
- Each mouth owns a nearest-mouth catchment capped at 1536 blocks. Growth starts
  at the mouth and proceeds upstream, selecting low virtual-elevation frontiers.
- Broad continentalness/highlands guide direction; arbitrary-angle candidate
  branches are rejected for sea crossings, intersecting branches, inadequate
  spacing, leaving their catchment, or excessive virtual downhill movement.
- A maximum of 128 nodes and 16 generations bounds each catchment's work.
- Tributaries share downstream nodes. Subtree node counts estimate relative
  contribution for widths; they are **not measured drainage area or runoff**.
- Short Bezier pieces soften bends. Lowland lake widenings occur on connected
  branches, not as independent elevated ponds.
- The legacy negative peaks/valleys trough is disabled in watershed mode.
  Inland terrain stays dry before explicit river/lake/delta carving; legacy
  noise rivers retain their original trough when watersheds are disabled.
- Broad highlands contribute 80% of peaks/valleys noise and finer relief 20%.
  Terrain uses that field and the planned network; local valley carving and
  density suppression make room for the network before surface decoration.

## Bounded cost and caches

No Minecraft chunks are generated for planning. Shared immutable basin plans use
an LRU cache of 256 basins; worker-local caches hold 2048 mouth results and 512
128-block spatial tiles. A tile indexes only nearby canonical segments plus the
full 112-block carving halo. Query order/cache eviction cannot change topology.
In-progress basin builds are shared through a per-basin `OnceLock`: workers
requesting the same basin reuse one build, while unrelated basins can build in
parallel. The global cache mutex is released before generation. A bounded
4096-entry cache per worker reuses exact noise samples, keyed by seed and
coordinate bits; inspector mode bypasses this sample cache.

The spatial grid is only a cache/index. It does not define the visible channels.
Growth is bounded, but cold planning and heavily branched tile queries are more
expensive than the old noise river lookup. See the measured optimization results
below; map preview timing is not a chunk-speed benchmark.

## Preview and checks

The normal `buildRgen` workflow also builds this diagnostic executable:

```
rgen/target/release/watershed-preview /tmp/rgen-watershed.svg 1
```

It writes an 8192-block-wide plan centered at the origin: blue oceans/rivers,
brown dry-valley routes, and a broad highland backdrop. It uses the actual cached
planner, not a reimplementation. It also checks 578 tile-border samples and
physical sea levels, plus dry-bank regression checks through the terrain
height calculation. Planner construction asserts increasing virtual heights and
nondecreasing upstream inlandness. This is a planning preview, not a rendering of
final terrain, caves, biome coloring or block-quantized river water.

## Remaining limitations

These are bounded coastal catchments, not a complete hierarchical basin system.
Interior regions beyond their reach can remain undrained. Nearest-mouth domains
are an approximation to divides; explicit watershed ridge construction and
terrain-patch synthesis from the paper are not implemented. Subtree width
estimates ignore rainfall and true contributing area. Pools are river widenings,
not a lake spillway simulation. Curvature is local, not a meander/erosion model.
In-game terrain appearance and throughput still need evaluation. Setting
`WATERSHEDS` false restores the original noise rivers and fine peaks field.

## River-shape and outlet update

Mouths now have a separate, submerged continuation: up to 384 blocks offshore,
following decreasing uncarved depth/continentalness until reaching a seabed of
Y=52 or lower. They cannot return to land. Beyond the first 128 blocks inland,
branches use a 64-block coastal buffer, sampled around the route, to reject
long shoreline-parallel channels. No water heights change.

Local bends in the emitted vector geometry, world-coordinate width noise,
variable channel depth/profile, irregular bank slopes and roughness, and
patchy off-center bars replace the single uniform trough. Broad style noise
uses a 256-block scale, width/bank spread 64, bed detail 16, and bank roughness
32. Styles blend continuously; this is not an implementation of every named
river morphology in the reference diagram. An open channel center is retained.
Banks still cannot turn previously dry ground into water outside the channel.
Delta shaping cannot refill an offshore channel. All fields are seed-based and
cached planning remains bounded; full chunk throughput has not been measured.

### Shallower channels and inland escarpments

River depth is reduced by five blocks, with a two-block minimum nominal depth
for reaches that were already shallow. Bed noise cannot dry out the center.
Emphasis is now on irregular/scalloped widths and winding single channels:
stronger geometric bends and multiscale edge variation, with far fewer/lower
braid bars. Lowland bank slopes retain their existing base gradient, with more
patchy shelves and cuts. From inlandness 0.50 to 0.70, bank transition width
smoothly falls to 25% of its lowland run (minimum eight blocks), creating much
steeper inland river/valley sides without forcing highland valleys to sea level.

### Mouth, coast, canyon and delta corrections

- Removed the seven-block mouth depth boost. Offshore carving still connects
  to the naturally deeper seabed without raising it, but river depth no longer
  jumps at the beach.
- River geography includes a noisy 5–9-block dry bank, excluding offshore edges.
- Beach geography stops at a natural floor of Y=62; deeper coastal water is Ocean.
- Canyon geography follows carved inland valleys with at least eight blocks of
  relief and a drop/run ratio of at least 0.6. At ratios from 1.0 to 1.8, walls
  increasingly compress to a three-block horizontal transition (sheer cliffs).
- Delta strength is tied to the mouth factor and shallow natural shelf. Rarity
  remains seeded; full delta terrain ranges from submerged Y=60.5–63 channels
  to Y=64–64.6 islands. Original upland height no longer lifts the delta platform.
  The existing offshore channel remains open through delta shaping.

### Excavation must not form dikes

The terrain height is an upper envelope for a 3D density field, not the actual
surface. River/canyon carving now preserves the original density threshold and
only lowers the maximum permitted height. It no longer fills density gaps into
solid walls, so lower surrounding ground is left lower instead of enclosed by
artificial ridges. Delta deposition remains a separate, intentional fill.
Dry valley floors combine existing terrain, bank noise, fine noise, and a small
cross-channel rise; their lower bound varies rather than flattening to Y=66.
The preview audit now compares carved solid blocks against the original density
field and rejects any added solid outside delta deposition.

## Computation reduction (2026-11-17)

This pass preserves the current terrain and network, rather than reducing their
resolution. It avoids full height/noise evaluations for coast checks that are
guaranteed inland (`continentalness >= 0.37` under the watershed relief spline),
checks expensive routes only for candidates that can beat the current choice,
shares in-progress basin construction, and reuses exact column samples.
One-block shoreline validation remains in place.

The added `compare_fixed_workloads` test uses 32 distant chunks and 128 neighboring
chunks at fixed valid coordinates. It times base generation, then hashes every
block outside the timer. Run from `rgen/`:

```sh
cargo +nightly test --release -p rgen-jni-impl --bench chunk compare_fixed_workloads -- --nocapture
```

`RGEN_BENCH_SEED` selects a seed (default 1). These use the synthetic test registry
and the Mediterranean override, not live Minecraft end-to-end timings.
The original adaptive spread benchmark samples extreme coordinates that can
overflow block-coordinate conversion; retain it for historical comparison, but
prefer the fixed valid-coordinate workload for assessing distant generation.

Validation: seed 1 fixed-workload block checksums match before/after
(`2a1c8f5dd7109775` distant; `cb76b4bc85e58785` neighboring). The full preview SVGs
for seeds 1, 7 and 11 are byte-identical. Each seed passes coastline, tile-border,
sea-level, dry-bank, inland-height, delta and original-density excavation checks.

Measured before/after on this machine (milliseconds per chunk/iteration):

| Workload | Before | After | Result |
| --- | ---: | ---: | --- |
| Fixed distant, 32 chunks | 276.6893 | 41.4909 | 6.67× faster |
| Fixed neighboring, 128 chunks | 4.1667 | 3.7156 | 1.12× faster |
| Original base benchmark | 4.4149 | 3.7830 | 1.17× faster |
| Original spread benchmark | 280.3661 | 41.9645 | 6.68× faster; extreme-coordinate caveat above |
| Original threaded sequential | 1.9230 | 2.1746 | 13% slower in this run |

These are individual runs, not confidence intervals. The adaptive benchmarks
had large reported variation; an earlier optimized run measured 8.3061 ms base
and 9.7029 ms sequential. Fixed optimized distant runs measured 47.4104 and
41.4909 ms, with identical checksums. Thus the large reduction in cold planning
is clear, but a 2× general or threaded-generation speedup is not demonstrated.

Removing the column cache experimentally measured 98.1932 ms distant and
12.0002 ms neighboring; it was restored. Final `buildRgen` succeeded and
installed the local test mod; its output is in `/tmp/log.rgen`.
