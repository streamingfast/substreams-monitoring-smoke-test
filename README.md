## Substreams Monitoring Smoke Test

A Substreams that emits few metrics out of a Substreams for each network to serve as a long running up check for Substreams deployments.

### Release

## Release

1. Define the version information that we are about to release:

    ```bash
    version=1.0.0 # Use correct version, latest released is given by 'git describe --tags --abbrev=0'
    ```

    > **Note** Those instructions uses [sd](https://github.com/chmln/sd#installation), `brew install sd` (or see [sd](https://github.com/chmln/sd#installation))

1. Prepare the release by updating the [CHANGELOG.md](./CHANGELOG.md) file, updating `## Unreleased` title:

    ```bash
    sd "## Unreleased" "## [$version](https://github.com/streamingfast/substreams-monitoring-smoke-test/releases/tag/v$version)" CHANGELOG.md
    ```

1. Update [substreams.yaml](./substreams.yaml) `version: vX.Y.Z` to `version: v1.0.0`:

    ```bash
    sd "version: v.*" "version: v$version" substreams.yaml
    ```

1. Commit to prepare release:

    ```bash
    git add CHANGELOG.md substreams.yaml
    git commit -m "Preparing for release v$version"
    ```

1. Run the [./bin/release.sh](./bin/release.sh) Bash script to perform a new release. It will ask you questions as well as driving all the required commands, performing the necessary operation automatically. The Bash script publishes a GitHub release by default, so you can check first that everything is all right.

    ```bash
    ./bin/release.sh v$version
    ```

#### One-Liner

```
version=1.0.0 # Use correct version, latest released is given by 'git describe --tags --abbrev=0'

sd "## Unreleased" "## [$version](https://github.com/streamingfast/substreams-monitoring-smoke-test/releases/tag/v$version)" CHANGELOG.md &&\
sd "version: v.*" "version: v$version" substreams.yaml &&\
git add CHANGELOG.md substreams.yaml &&\
git commit -m "Preparing for release v$version" &&\
./bin/release.sh v$version
```
