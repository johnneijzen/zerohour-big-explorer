use big_core::progress::{Progress, progress_channel};

#[test]
fn progress_channel_sends_and_receives() {
    let (tx, rx) = progress_channel();
    tx.send(Progress::Started { job: "test".to_string() }).unwrap();
    tx.send(Progress::Percent(42)).unwrap();
    tx.send(Progress::Message("halfway".to_string())).unwrap();
    tx.send(Progress::Completed).unwrap();

    let mut seen = vec![];
    for ev in rx.iter().take(4) {
        seen.push(ev);
    }

    assert_eq!(seen.len(), 4);
}
