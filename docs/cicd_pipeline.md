# CI/CD Pipeline

Gitea server for production.

Github actions for build and release.
Release -> Publish.

## Build step

1. Compile project
2. Run in production mode
3. wget command to download and create a static site.
  a) ensure every image is downloaded and created / we can create a cache mechanism to download images from previous build / save a ton of time
4. Backup old version
5. Publish new version


## Development

1. Build the project
2. Wget cannot be run on dev server due to tower reload

## TODO

- Some weird links (Colemak)
