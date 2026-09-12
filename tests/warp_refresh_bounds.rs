use recur::warp_refresh::Reads;
use std::fs;
use tempfile::tempdir;
#[test]
fn publication_revalidation_and_history_inventory_boundaries() {
    let d = tempdir().unwrap();
    let r = d.path();
    fs::write(r.join("José.txt"), "before").unwrap();
    let mut reads = Reads::default();
    reads.read(r, "José.txt", false).unwrap();
    reads.verify(r).unwrap();
    fs::write(r.join("José.txt"), "after").unwrap();
    assert!(reads.verify(r).is_err());
    fs::create_dir(r.join("history")).unwrap();
    for i in 0..64 {
        fs::write(r.join(format!("history/{i}.json")), "{}").unwrap();
    }
    assert_eq!(
        recur::warp_refresh::inventory(r, "history").unwrap().len(),
        64
    );
    fs::write(r.join("history/extra.json"), "{}").unwrap();
    assert!(recur::warp_refresh::inventory(r, "history").is_err());
}
#[test]
fn exact_byte_and_aggregate_limits() {
    let d = tempdir().unwrap();
    let r = d.path();
    for (name, n, json) in [("j", 2 * 1024 * 1024, true), ("b", 32 * 1024 * 1024, false)] {
        let f = fs::File::create(r.join(name)).unwrap();
        f.set_len(n).unwrap();
        assert_eq!(
            Reads::default().read(r, name, json).unwrap().len() as u64,
            n
        );
        f.set_len(n + 1).unwrap();
        assert!(Reads::default().read(r, name, json).is_err());
    }
    for p in ["a", "b"] {
        fs::File::create(r.join(p))
            .unwrap()
            .set_len(32 * 1024 * 1024)
            .unwrap();
    }
    fs::write(r.join("extra"), "x").unwrap();
    let mut reads = Reads::default();
    reads.read(r, "a", false).unwrap();
    reads.read(r, "b", false).unwrap();
    assert!(reads.read(r, "a", false).is_ok());
    assert!(reads.read(r, "extra", false).is_err());
}
#[test]
fn file_count_and_path_limits() {
    let d = tempdir().unwrap();
    let r = d.path();
    let mut reads = Reads::default();
    for i in 0..128 {
        let p = format!("{i}.txt");
        fs::write(r.join(&p), "x").unwrap();
        assert!(reads.read(r, &p, false).is_ok());
    }
    fs::write(r.join("extra"), "x").unwrap();
    assert!(reads.read(r, "extra", false).is_err());
    for p in [
        "./0.txt",
        "../0.txt",
        "C:/0.txt",
        "//server/share",
        "0.txt/../1.txt",
        "0.txt\\x",
        "",
    ] {
        assert!(Reads::default().read(r, p, false).is_err(), "{p}");
    }
}
