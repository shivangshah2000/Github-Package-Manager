# Github Package Manager

Installs and manages packages released on github releases.

## Features

- List releases of a particular package
- Install a particular (or latest) release of a package
- Automatically check and prompt to update for outdated packages
- Allow running a per-package post-download script (to copy artifacts, for eg)

## Non Features

- Search for packages
- Ensure user safety
- Ensure package compatibilty
    - Might support semver

## Usage

```
# list installed packages
gpm list --installed
# list available versions for given repo
gpm list <user>/<repo>
# install latest version of given repo
gpm install <user>/<repo>
# install given version of repo
gpm install <user>/<repo>@x.y.z
# edit post-download script file for given package in $EDITOR
gpm post-download <user>/<repo>
# print path of post-download script file for given package
gpm post-download --path <user>/<repo>
# check for updates to installed packages
gpm outdated
# install latest versions of all outdated packages
gpm update
```
