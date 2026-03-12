# Homebrew

## Install (users)

After the first release is published and the formula has real SHA256 values:

```bash
brew tap yourusername/sshtui
brew install sshtui
```

Replace `yourusername` with the GitHub username or org that owns the repo.

## Maintainers: updating the formula

1. In `Formula/sshtui.rb`, replace every `yourusername` with your GitHub username or org.

2. After creating a new release (e.g. v0.2.0), get the SHA256 for each asset:

   ```bash
   curl -sL "https://github.com/YOUR_USER/sshtui/releases/download/v0.2.0/sshtui-x86_64-apple-darwin.tar.gz" | shasum -a 256
   curl -sL "https://github.com/YOUR_USER/sshtui/releases/download/v0.2.0/sshtui-aarch64-apple-darwin.tar.gz" | shasum -a 256
   curl -sL "https://github.com/YOUR_USER/sshtui/releases/download/v0.2.0/sshtui-x86_64-unknown-linux-gnu.tar.gz" | shasum -a 256
   curl -sL "https://github.com/YOUR_USER/sshtui/releases/download/v0.2.0/sshtui-aarch64-unknown-linux-gnu.tar.gz" | shasum -a 256
   ```

3. Update `version "X.Y.Z"` and all four `url` and `sha256` lines in the formula (or use a script to substitute).

4. Commit and push the formula changes. Users with the tap will get the update on `brew upgrade sshtui`.

## First-time setup (before first release)

The formula ships with placeholder SHA256 values (`REPLACE_WITH_..._SHA`). Until you:

1. Create a release (e.g. tag `v0.1.0` and push so the Release workflow builds assets),
2. Compute the four SHAs as above,
3. Replace the placeholders in `Formula/sshtui.rb`,

`brew install sshtui` will fail. Do the steps in "Maintainers: updating the formula" once the first release is available.
