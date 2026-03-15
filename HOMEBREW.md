# Homebrew

```bash
brew tap YOUR_USERNAME/sshtui
brew install sshtui
```

---

## Release Steps

1. **Create a release**
   Push a tag (e.g. `v1.0.0`) so the Release workflow runs and uploads binaries to GitHub Releases.

2. **Formula SHAs**
   The Release workflow has an **Update Formula SHAs** step that commits updated SHA256 values to the default branch after attaching assets. So you normally do not need to fill them in by hand.
   If you need to do it manually, after the release is published run (with your tag and repo):

   ```bash
   curl -sL "https://github.com/YOUR_USER/sshtui/releases/download/v1.0.0/sshtui-x86_64-unknown-linux-gnu.tar.gz" | shasum -a 256
   curl -sL "https://github.com/YOUR_USER/sshtui/releases/download/v1.0.0/sshtui-aarch64-unknown-linux-gnu.tar.gz" | shasum -a 256
   curl -sL "https://github.com/YOUR_USER/sshtui/releases/download/v1.0.0/sshtui-aarch64-apple-darwin.zip" | shasum -a 256
   ```

   Put each output (first column) into the matching `sha256 "..."` line in `Formula/sshtui.rb`. macOS is a notarized zip; Linux assets are tar.gz.

3. **Commit and push** the formula. After that, `brew install sshtui` (with the tap) will work.

4. **For new versions**
   Bump `version "X.Y.Z"` and the URLs in the formula, recompute the SHAs for the new release assets, update the formula, and push. Users get updates with `brew upgrade sshtui`.

---

macOS binaries are **signed and notarized** in CI. To set that up, see [docs/MACOS_SIGNING.md](docs/MACOS_SIGNING.md).
