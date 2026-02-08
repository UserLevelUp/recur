# Improvement 11: Installers and Packaging

Status: `planned`

## Overview

Add professional installer and packaging options for distributing recur and recur-git binaries to end users.

## Goals

1. **Windows MSI Installer** - Professional Windows installation experience
2. **Linux packages** - .deb and .rpm packages for major distros
3. **Homebrew formula** - macOS and Linux package manager support
4. **Archive releases** - ZIP/tar.gz for direct download
5. **Automated CI releases** - Build installers on every release

## Current State

**What we have:**
- CI builds both binaries (`recur` and `recur-git`)
- CI uploads raw binaries as artifacts
- No installer packages yet

**What we need:**
- Package both binaries together in installers
- Add installation scripts
- Include README and license
- Set up PATH automatically

## Implementation Options

### 1. Windows MSI Installer

**Tool:** cargo-wix (Rust-native WiX wrapper)

**Setup:**

```toml
# Add to Cargo.toml
[package.metadata.wix]
upgrade-guid = "GENERATE-UUID-HERE"
path-guid = "GENERATE-UUID-HERE"
license = false
eula = false
```

**CI Integration:**

```yaml
- name: Install cargo-wix
  run: cargo install cargo-wix

- name: Build MSI installer
  run: cargo wix --nocapture

- name: Upload MSI
  uses: actions/upload-artifact@v4
  with:
    name: recur-windows-installer
    path: target/wix/*.msi
```

**Challenge:** cargo-wix defaults to single binary. Need custom WiX template to package both `recur.exe` and `recur-git.exe`.

**Custom WiX template location:** `wix/main.wxs`

### 2. ZIP/TAR Archives (Simpler alternative)

**Windows ZIP:**

```powershell
mkdir recur-windows
cp target/release-safe/recur.exe recur-windows/
cp target/release-safe/recur-git.exe recur-windows/
cp README.md LICENSE recur-windows/
Compress-Archive -Path recur-windows -DestinationPath recur-windows.zip
```

**Linux TAR.GZ:**

```bash
mkdir recur-linux
cp target/release-safe/recur recur-linux/
cp target/release-safe/recur-git recur-linux/
cp README.md LICENSE recur-linux/
tar -czf recur-linux.tar.gz recur-linux/
```

**Benefits:**
- Simple, no dependencies
- Works immediately
- Good for advanced users
- Easy CI integration

### 3. Debian Package (.deb)

**Tool:** cargo-deb

```bash
cargo install cargo-deb
cargo deb
```

**Configuration in Cargo.toml:**

```toml
[package.metadata.deb]
maintainer = "User Level Up Contributors"
depends = "$auto"
section = "utility"
priority = "optional"
assets = [
    ["target/release/recur", "usr/bin/", "755"],
    ["target/release/recur-git", "usr/bin/", "755"],
    ["README.md", "usr/share/doc/recur/", "644"],
]
```

**Install command:**

```bash
sudo dpkg -i recur_*.deb
```

### 4. RPM Package

**Tool:** cargo-generate-rpm

```bash
cargo install cargo-generate-rpm
cargo build --release
cargo generate-rpm
```

### 5. Homebrew Formula

**Create formula:** `homebrew-recur/Formula/recur.rb`

```ruby
class Recur < Formula
  desc "Hierarchical search tool - both recur and recur-git"
  homepage "https://github.com/userlevelup/recur"
  url "https://github.com/userlevelup/recur/archive/v0.1.11.tar.gz"
  sha256 "CALCULATE-AFTER-RELEASE"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
    bin.install "target/release/recur"
    bin.install "target/release/recur-git"
  end

  test do
    system "#{bin}/recur", "--version"
    system "#{bin}/recur-git", "--help"
  end
end
```

**Usage:**

```bash
brew tap userlevelup/recur
brew install recur
```

### 6. GitHub Releases Integration

**Tool:** softprops/action-gh-release

```yaml
- name: Create Release
  uses: softprops/action-gh-release@v1
  if: startsWith(github.ref, 'refs/tags/')
  with:
    files: |
      target/wix/*.msi
      recur-windows.zip
      recur-linux.tar.gz
      target/debian/*.deb
      target/generate-rpm/*.rpm
```

## Recommended Phases

### Phase 1: Simple Archives (Quick win)
- Add ZIP/TAR.GZ creation to CI
- Upload as artifacts
- Good enough for GitHub releases

### Phase 2: Native Packages
- Windows: MSI via cargo-wix
- Linux: .deb via cargo-deb
- More professional installation

### Phase 3: Package Managers
- Homebrew formula
- RPM packages
- Consider cargo-binstall metadata

### Phase 4: Automated Releases
- Tag-triggered releases
- Automatic installer builds
- GitHub releases with all formats

## Design Decisions

### Should both binaries be in one package?

**Yes** - recur and recur-git are a toolset:
- Users want the full capability
- recur-git depends on recur library
- Simpler than separate packages

### Package naming?

**"recur"** - The main package includes both binaries:
- Install: `recur` (gets both `recur` and `recur-git` commands)
- No separate `recur-git` package needed

### Version synchronization?

**Same version** - Both binaries share Cargo.toml version:
- Single source of truth
- No version mismatch issues
- Simpler release process

## CI Workflow Example

```yaml
build-installers:
  runs-on: windows-latest
  steps:
    - uses: actions/checkout@v4

    - name: Build binaries
      run: cargo build --release --locked

    # Option 1: Simple ZIP
    - name: Create Windows ZIP
      run: |
        mkdir recur-windows
        cp target/release/recur.exe recur-windows/
        cp target/release/recur-git.exe recur-windows/
        cp README.md LICENSE recur-windows/
        Compress-Archive -Path recur-windows -DestinationPath recur-windows.zip

    # Option 2: MSI (requires cargo-wix setup)
    - name: Install cargo-wix
      run: cargo install cargo-wix

    - name: Build MSI
      run: cargo wix --nocapture

    - name: Upload artifacts
      uses: actions/upload-artifact@v4
      with:
        name: recur-installers
        path: |
          recur-windows.zip
          target/wix/*.msi
```

## References

**Tools:**
- cargo-wix: https://github.com/volks73/cargo-wix
- cargo-deb: https://github.com/kornelski/cargo-deb
- cargo-generate-rpm: https://github.com/cat-in-136/cargo-generate-rpm
- softprops/action-gh-release: https://github.com/softprops/action-gh-release

**Documentation:**
- WiX Toolset: https://wixtoolset.org/
- Debian packaging: https://www.debian.org/doc/manuals/maint-guide/
- Homebrew formula cookbook: https://docs.brew.sh/Formula-Cookbook

## Success Criteria

- [x] Document installer options (this file!)
- [ ] Phase 1: ZIP/TAR.GZ archives in CI
- [ ] Phase 2: MSI installer for Windows
- [ ] Phase 3: .deb package for Debian/Ubuntu
- [ ] Phase 4: Homebrew formula
- [ ] GitHub releases with all formats

## Related

- CI configuration: `.github/workflows/ci.yml`
- Binary definitions: `Cargo.toml` (lines 20-26)
- recur-git artifact: `docs/main.recur-git.artifact.md`
