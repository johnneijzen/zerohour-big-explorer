use std::fs;
use std::time::Instant;

/// Performance harness for extract_file using a canonical sample.
/// This test runs only when RUN_PERF=1 environment variable is set and the sample file exists.
#[test]
fn perf_extract_sample_if_requested() {
    if std::env::var("RUN_PERF").unwrap_or_default() != "1" {
        eprintln!("skipping perf test (set RUN_PERF=1 to enable)");
        return;
    }

    let sample = std::path::Path::new("specs/002-big-archive-extraction/samples/50mb.wav");
    if !sample.exists() {
        eprintln!("perf sample missing at {} — skipping", sample.display());
        return;
    }

    // Create a small archive containing the sample and measure extract_file duration
    let tmp = tempfile::tempdir().expect("tempdir");
    let srcdir = tmp.path().join("src");
    fs::create_dir_all(&srcdir).unwrap();
    let sample_dst = srcdir.join("sample.wav");
    fs::copy(sample, &sample_dst).expect("copy sample");

    let archive = tmp.path().join("perf.big");
    big_core::pack::pack_directory(&srcdir, &archive).expect("pack");

    let (_m, _i, entries) = big_core::parser::parse_archive(&archive).expect("parse");
    let entry = entries.into_iter().find(|e| e.name.ends_with("sample.wav")).expect("entry");

    let start = Instant::now();
    let _bytes = big_core::extract::extract_file(&archive, &entry).expect("extract_file");
    let dur = start.elapsed();

    // Write a JSON artifact in temp dir for CI consumption
    let artifact = serde_json::json!({"elapsed_ms": dur.as_millis(), "entry": entry.name, "size": entry.length});
    let art_path = tmp.path().join("perf_result.json");
    fs::write(&art_path, serde_json::to_string_pretty(&artifact).unwrap()).expect("write artifact");

    eprintln!("perf result: {} ms — artifact: {}", dur.as_millis(), art_path.display());
}
