use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[test]
#[ignore]
fn wav_preview_performance() {
    // T035: Performance test for WAV preview. This test is ignored by default.
    // Enable by running with `RUN_PERF=1 cargo test -- --ignored` in CI or locally.
    if std::env::var("RUN_PERF").unwrap_or_default() != "1" {
        eprintln!("Skipping perf test; set RUN_PERF=1 to enable");
        return;
    }

    let sample_path = std::path::Path::new("specs/002-big-archive-extraction/samples/50mb.wav");
    if !sample_path.exists() {
        panic!("Sample WAV not found at {} — place a 50MB WAV at this path to run the perf test", sample_path.display());
    }

    // Create a small archive around the sample and measure extract_file latency
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src");
    let _ = std::fs::create_dir_all(&src);

    let sample_dest = src.join("sample.wav");
    std::fs::copy(sample_path, &sample_dest).expect("copy sample");

    let archive = tmp.path().join("perf.big");
    big_core::pack::pack_directory(&src, &archive).expect("pack directory");

    let (_meta, _index, entries) = big_core::parser::parse_archive(&archive).expect("parse");
    let entry = entries.into_iter().find(|e| e.name.ends_with("sample.wav")).unwrap();

    let start = Instant::now();
    let _ = big_core::extract::extract_file(&archive, &entry).expect("extract");
    let elapsed = start.elapsed();

    println!("Extract elapsed: {:?}", elapsed);
}
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
