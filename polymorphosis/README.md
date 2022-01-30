# polymorphosis

Meta file generator for PolyMC.

## Why is this needed? Aren't the original meta scripts working just fine?

Well yes and no. While it's true that they work, they're badly written and almost unmaintanable due to a lack of proper documentation.

I'm making the effort to fully explain what is going on in the scripts and rewrite them as a Rust application.
Rewriting the metadata generation in Rust is faster and easily deployable.

Another reason is that PolyMC might use a different meta format in the future. In which case this repository will be updated.

## How do the old meta scripts work?

1. Clone [upstream](https://github.com/PolyMC/meta-upstream) and [meta](https://github.com/PolyMC/meta-polymc) repositories, if they don't exist already.
2. Hard-reset the local upstream repo and check out the configured branch (usually either master or develop).
3. Update Mojang metadata.
    1. Try to load the local version list in the upstream repo under `mojang/version_manifest_v2.json`, if it exists.
    2. Download Mojang's current version list from `https://launchermeta.mojang.com/mc/game/version_manifest_v2.json`.
    3. Check which versions are not present locally.
    4. Check which versions are present both locally and remotely.
    5. Compare the times of the locally and remotely present versions. If the time of the remote version is newer, flag the version to be updated in the local version list.
    6. Download the version file for each version that is new or needs to be updated and save them in the upstream repo under `mojang/versions/{version_id}.json`.
    7. Download the asset meta files for each version and save them in the upstream repo under `mojang/assets/{asset_id}.json`.
    8. Save the remote version list in the upstream repo under `mojang/version_manifest_v2.json`
4. Update Forge metadata.
5. Update Fabric metadata.
6. Update Liteloader metadata.
7. If any of the steps 3-6 failed, hard-reset the upstream repo and exit with exit code 1.
8. Hard-reset the meta repository and check out the configured branch (usually either master or develop)
9. Generate PolyMC metadata from Mojang metadata.
10. Generate PolyMC metadata from Forge metadata.
11. Generate PolyMC metadata from Fabric metadata.
12. Generate PolyMC metadata from Liteloader metadata.
13. Generate a PolyMC index file.
