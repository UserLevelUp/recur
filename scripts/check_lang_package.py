"""Package all declared binaries and smoke-test the extracted native archive.

Uses Python 3.11+ standard library only. Never installs or publishes a package.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tomllib
import zipfile


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin-dir", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--platform", choices=["windows", "linux"], required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    cargo = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    version = cargo["package"]["version"]
    assert (root / "VERSION").read_text().strip() == f"a.{version}"
    if os.environ.get("GITHUB_REF_TYPE") == "tag":
        assert os.environ["GITHUB_REF_NAME"] == f"v{version}", "Tag/Cargo version mismatch"
    windows = args.platform == "windows"
    assert windows == (os.name == "nt"), "Smoke tests must run natively"
    names = sorted(item["name"] + (".exe" if windows else "") for item in cargo["bin"])
    assert len(names) == 6
    output = args.output_dir.resolve()
    output.mkdir(parents=True, exist_ok=True)
    stage = output / f"{args.platform}-staging"
    extracted = output / f"{args.platform}-extracted"
    fixture = output / f"{args.platform}-fixture"
    for directory in [stage, extracted, fixture]:
        directory.mkdir(exist_ok=True)
    for name in names:
        shutil.copy2(args.bin_dir / name, stage / name)
    shutil.copy2(root / "README.md", stage / "README.md")
    expected = names + ["README.md"]
    assert sorted(p.name for p in stage.iterdir()) == sorted(expected), "Unexpected staging contents"
    platform = "x86_64-pc-windows-msvc" if windows else "x86_64-unknown-linux-gnu"
    archive = output / f"recur-v{version}-{platform}.{'zip' if windows else 'tar.gz'}"
    if windows:
        with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as package:
            for name in sorted(expected):
                package.write(stage / name, name)
        with zipfile.ZipFile(archive) as package:
            assert sorted(package.namelist()) == sorted(expected)
            package.extractall(extracted)
    else:
        with tarfile.open(archive, "w:gz") as package:
            for name in sorted(expected):
                package.add(stage / name, arcname=name)
        with tarfile.open(archive) as package:
            assert sorted(package.getnames()) == sorted(expected)
            package.extractall(extracted, filter="data")
    for name in expected:
        assert digest(stage / name) == digest(extracted / name)
    for filename in ["main.lang.algorithm-lab.recur", "main.lang.skippy-watch-coordination.recur"]:
        shutil.copy2(root / "demos/main.lang" / filename, fixture / filename)
    before = {p.name: digest(p) for p in fixture.iterdir()}
    smoke = []

    def run(binary, *arguments, structured=False):
        cmd = [str(extracted / (binary + (".exe" if windows else ""))), *arguments]
        result = subprocess.run(cmd, cwd=fixture, check=True, capture_output=True, text=True, encoding="utf-8")
        value = json.loads(result.stdout) if structured else result.stdout.strip()
        smoke.append({"binary": binary, "arguments": arguments, "exit_code": result.returncode, "result": value})
        return value

    for name in names:
        binary = name.removesuffix(".exe")
        assert version in run(binary, "--version")
        assert "Usage:" in run(binary, "--help")
    run("recur", "lang", "--help")
    result = run("recur", "lang", "show", "main.lang.algorithm-lab.recur", "--scope", "merge.f", "--json", structured=True)
    assert result["header"][0]["input"]["canonical_identity"] == "bubble.o(b)"
    assert result["footer"]["execution"] == "not-run"
    result = run("recur", "lang", "check", "main.lang.skippy-watch-coordination.recur", "--json", structured=True)
    assert result["footer"]["graph"]["orchestration_sound"]
    result = run("recur-lang", "warp", "main.lang.algorithm-lab.recur", "gcd", "--json", structured=True)
    assert result["dry_run"] and result["confirmation_required"]
    assert before == {p.name: digest(p) for p in fixture.iterdir()}, "Smoke test mutated its source root"
    report = {"schema": "recur-lang-package-smoke-v1", "version": version, "platform": platform,
              "archive": archive.name, "sha256": digest(archive),
              "files": {name: digest(extracted / name) for name in sorted(expected)},
              "source_inputs": before, "read_only_inventory": "unchanged", "smoke": smoke}
    receipt = output / f"{args.platform}-package-smoke.json"
    receipt.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"PASS {archive.name}: six binaries; extracted help/version, pure queries, companion dry run")
    print(f"SHA256 {report['sha256']}")
    print(f"Receipt: {receipt}")


if __name__ == "__main__":
    main()
