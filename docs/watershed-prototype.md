# Watershed branch prototype

Enabled by `WATERSHEDS` in `rgen/rgen-biome/src/feature.rs`.

This first implementation replaces the peaks/valleys noise-band rivers with a
coarse directed drainage graph. It does not simulate water on generated chunks.

- Sites are spaced about 512 blocks apart, with deterministic position jitter.
- Each site samples the uncarved terrain splines and chooses a lower cardinal
  neighbor. A strict height/coordinate ordering prevents cycles.
- Shared sites and edges allow routes to cross planning-cell boundaries.
- Gently bent, subdivided edges connect the sites; tributaries share downstream
  endpoints. Raw endpoint water levels never rise downstream.
- Lower coastal sites terminate in the sea. Inland local minima form sink lakes.
- Local river beds and water levels shape the terrain before decoration. Density
  noise is suppressed in the channel and feathered back into ordinary banks.
- River geography uses the same route footprint. Delta selection is restricted
  to low coastal reaches rather than the old peaks/valleys river band.

## Bounded work and caching

A plan visits nine nearby sites and their four neighbors. It never recursively
walks a drainage basin or asks for Minecraft chunks. Site and plan caches are
bounded and worker-local (4096 sites and 128 plans per worker), keyed by seed and
coordinates. Eviction changes the cost of a query, not its result. Plans use the
game splines; live inspector spline edits do not rebuild the drainage topology.

The local projection query checks only edges that can reach the current planning
cell, with a 96-block halo. Broad plan costs, per-column projection costs, and
end-to-end chunk throughput still need measurement on the target machine.

## Scope and limitations

This is a first graph-hydrology implementation, not the complete hierarchical
watershed proposal. It does not yet compute explicit divides, exact upstream
area, rainfall, river order, or watershed-based river counts. Rivers currently
have a fixed width. A site can route to an inland sink instead of the ocean.
Lake outlines are simple, and coarse routing can require substantial cutting or
filling through the finer terrain. Confluence water levels are spatially blended;
monotone graph profiles do not prove every generated water block flows downhill.
Minecraft block quantization and fluid updates still need in-game inspection.

Changing this feature changes terrain generation; compare fresh test terrain.
Setting the feature false restores the original noise-band route calculation.

Regression tests cover descending graph edges, planning borders, and cache order.
The required buildRgen workflow builds and installs the mod but does not run Rust
unit tests; those checks must not be described as executed by that build.
