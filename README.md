# GH Release Manager

`gh-release-manager` is a GitHub Action designed to automate the updating of version files in projects based on GitHub release tags. This action facilitates maintaining synchronization of project version files with release versions, enhancing version tracking and management.

## Features

- **Automatic Update**: Automatically updates the version file based on the GitHub tag.
- **Easy Integration**: Easily integrates into any CI/CD workflow using GitHub Actions.

## Prerequisites

Before using `gh-release-manager`, ensure that your project uses version tags following the semantic format `vMAJOR.MINOR.PATCH` (e.g., `v1.0.0`).

## Supported Languages

| Language | Support            |
|----------|--------------------|
| Rust     | :white_check_mark: |
| JavaScript (js) | :white_check_mark: |
| Others    | :x: (Not supported at this time) |

## Usage

To use this action, include it in your GitHub Actions workflow. Below is an example of how to set up the action in your `.github/workflows/release.yml` file:

```yaml
name: Update Project Version on Release

on:
  release:
    types: [created]

jobs:
  update-version:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: chrisllontop/gh-release-manager@v1
        with:
          lang: 'js' # Or 'rust', depending on the language of your project.
      - name: Commit changes
      - uses: stefanzweifel/git-auto-commit-action@v4
        with:
          commit_message: 'Automatically update version file'
```
