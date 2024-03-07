

## v0.6.0 (2024-03-02)

### New Features

 - <csr-id-c1d1b51ee26cc9946af59ac02c040e0a33d15fde/> unify send/demux/tagged APIs
   feat(hydroflow_plus): unify send/demux/tagged APIs
 - <csr-id-eb34ccd13f56e1d07cbae35ead79daeb3b9bad20/> use an IR before lowering to Hydroflow
   Makes it possible to write custom optimization passes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 2 calendar days.
 - 28 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#1070](https://github.com/hydro-project/hydroflow/issues/1070), [#1080](https://github.com/hydro-project/hydroflow/issues/1080)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1070](https://github.com/hydro-project/hydroflow/issues/1070)**
    - Use an IR before lowering to Hydroflow ([`eb34ccd`](https://github.com/hydro-project/hydroflow/commit/eb34ccd13f56e1d07cbae35ead79daeb3b9bad20))
 * **[#1080](https://github.com/hydro-project/hydroflow/issues/1080)**
    - Unify send/demux/tagged APIs ([`c1d1b51`](https://github.com/hydro-project/hydroflow/commit/c1d1b51ee26cc9946af59ac02c040e0a33d15fde))
</details>

## v0.5.1 (2024-02-03)

<csr-id-e9c7ced8760f88e3215a4b1b4e23f8b9db159a84/>
<csr-id-1b555e57c8c812bed4d6495d2960cbf77fb0b3ef/>
<csr-id-03ce73d2eb1377179f5ebd6f80e082c2889588a7/>

### Chore

 - <csr-id-e9c7ced8760f88e3215a4b1b4e23f8b9db159a84/> prep for initial release
   > it contains the logic linking hydroflow+ to deploy, it should be published that’s a bug
 - <csr-id-1b555e57c8c812bed4d6495d2960cbf77fb0b3ef/> manually set lockstep-versioned crates (and `lattices`) to version `0.5.1`
   Setting manually since
   https://github.com/frewsxcv/rust-crates-index/issues/159 is messing with
   smart-release

### Chore

 - <csr-id-03ce73d2eb1377179f5ebd6f80e082c2889588a7/> fix/add releasing for `hydroflow_plus_cli_integration`, `stageleft[_macro/_tool]`

### New Features

 - <csr-id-7d930a2ccf656d3d6bc5db3e22eb63c5fd6d37d1/> add APIs for declaring external ports on clusters
 - <csr-id-6eeb9be9bc4136041a2855f650ae640c478b7fc9/> improve API naming and polish docs
 - <csr-id-46d87fa364d3fe01422cf3c404fbc8a1d5e9fb88/> pass subgraph ID through deploy metadata
 - <csr-id-b7aafd3c97897db4bff62c4ab0b7480ef9a799e0/> improve API naming and eliminate wire API for builders
 - <csr-id-53d7aee8dcc574d47864ec89bfea30a82eab0ee7/> improve Rust API for defining services
 - <csr-id-c50ca121b6d5e30dc07843f82caa135b68626301/> split Rust core from Python bindings

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 43 calendar days.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#1013](https://github.com/hydro-project/hydroflow/issues/1013), [#1056](https://github.com/hydro-project/hydroflow/issues/1056), [#986](https://github.com/hydro-project/hydroflow/issues/986), [#987](https://github.com/hydro-project/hydroflow/issues/987), [#995](https://github.com/hydro-project/hydroflow/issues/995), [#996](https://github.com/hydro-project/hydroflow/issues/996)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1013](https://github.com/hydro-project/hydroflow/issues/1013)**
    - Improve API naming and polish docs ([`6eeb9be`](https://github.com/hydro-project/hydroflow/commit/6eeb9be9bc4136041a2855f650ae640c478b7fc9))
 * **[#1056](https://github.com/hydro-project/hydroflow/issues/1056)**
    - Prep for initial release ([`e9c7ced`](https://github.com/hydro-project/hydroflow/commit/e9c7ced8760f88e3215a4b1b4e23f8b9db159a84))
 * **[#986](https://github.com/hydro-project/hydroflow/issues/986)**
    - Split Rust core from Python bindings ([`c50ca12`](https://github.com/hydro-project/hydroflow/commit/c50ca121b6d5e30dc07843f82caa135b68626301))
 * **[#987](https://github.com/hydro-project/hydroflow/issues/987)**
    - Improve Rust API for defining services ([`53d7aee`](https://github.com/hydro-project/hydroflow/commit/53d7aee8dcc574d47864ec89bfea30a82eab0ee7))
 * **[#995](https://github.com/hydro-project/hydroflow/issues/995)**
    - Improve API naming and eliminate wire API for builders ([`b7aafd3`](https://github.com/hydro-project/hydroflow/commit/b7aafd3c97897db4bff62c4ab0b7480ef9a799e0))
 * **[#996](https://github.com/hydro-project/hydroflow/issues/996)**
    - Pass subgraph ID through deploy metadata ([`46d87fa`](https://github.com/hydro-project/hydroflow/commit/46d87fa364d3fe01422cf3c404fbc8a1d5e9fb88))
 * **Uncategorized**
    - Release hydroflow_plus_cli_integration v0.5.1 ([`8202feb`](https://github.com/hydro-project/hydroflow/commit/8202febcd8ed160c23d08c37d7c13773831c97da))
    - Fix/add releasing for `hydroflow_plus_cli_integration`, `stageleft[_macro/_tool]` ([`03ce73d`](https://github.com/hydro-project/hydroflow/commit/03ce73d2eb1377179f5ebd6f80e082c2889588a7))
    - Release hydroflow_lang v0.5.2, hydroflow_datalog_core v0.5.2, hydroflow_macro v0.5.2, lattices v0.5.2, hydroflow v0.5.2, hydro_cli v0.5.1, hydroflow_plus_cli_integration v0.5.1 ([`6ac8720`](https://github.com/hydro-project/hydroflow/commit/6ac872081753548ebb8ec95549b4d820dc050d3e))
    - Manually set lockstep-versioned crates (and `lattices`) to version `0.5.1` ([`1b555e5`](https://github.com/hydro-project/hydroflow/commit/1b555e57c8c812bed4d6495d2960cbf77fb0b3ef))
    - Add APIs for declaring external ports on clusters ([`7d930a2`](https://github.com/hydro-project/hydroflow/commit/7d930a2ccf656d3d6bc5db3e22eb63c5fd6d37d1))
</details>

