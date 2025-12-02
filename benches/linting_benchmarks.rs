use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use proselint_wasm::{Config, Linter};

// Sample texts of varying sizes
const SMALL_TEXT: &str = "This is very very important text.";
const MEDIUM_TEXT: &str = r#"
At the end of the day, we need to think outside the box and leverage our core competencies.
This is very important for our stakeholders. I think we should consider all options.
The team needs to be proactive, not reactive. We must avoid cliches and corporate jargon.
Free gifts are redundant, and close proximity is unnecessary. Let's be clear and concise.
"#;
const LARGE_TEXT: &str = r#"
At the end of the day, we need to think outside the box and leverage our core competencies.
This is very important for our stakeholders. I think we should consider all options.
The team needs to be proactive, not reactive. We must avoid cliches and corporate jargon.
Free gifts are redundant, and close proximity is unnecessary. Let's be clear and concise.

In my opinion, the project timeline is quite aggressive. We really need to circle back on this.
The synergy between teams is crucial for success. Let's touch base and align our strategies.
This is a game changer for the industry. We need to drill down into the details.

At this point in time, we should move the needle forward. The low-hanging fruit should be
prioritized first. We need to get all our ducks in a row before proceeding.
It goes without saying that quality is paramount. We must think outside the box.

The bottom line is that we need results. This is a win-win situation for everyone involved.
We should leverage our resources and maximize our ROI. The deliverables must meet expectations.
Let's not reinvent the wheel here. We need to be more strategic going forward.

At the end of the day, success depends on execution. We must stay focused and driven.
This initiative will take us to the next level. The team is very committed to excellence.
We really need to up our game and show thought leadership in this space.
"#;

fn bench_small_text(c: &mut Criterion) {
    let linter = Linter::new();

    c.bench_function("lint_small_text", |b| {
        b.iter(|| linter.check(black_box(SMALL_TEXT)))
    });
}

fn bench_medium_text(c: &mut Criterion) {
    let linter = Linter::new();

    c.bench_function("lint_medium_text", |b| {
        b.iter(|| linter.check(black_box(MEDIUM_TEXT)))
    });
}

fn bench_large_text(c: &mut Criterion) {
    let linter = Linter::new();

    c.bench_function("lint_large_text", |b| {
        b.iter(|| linter.check(black_box(LARGE_TEXT)))
    });
}

fn bench_warm_all(c: &mut Criterion) {
    c.bench_function("warm_all_checks", |b| {
        // Use the Proselint struct's warm_all method instead of direct engine access
        b.iter(|| {
            let linter = proselint_wasm::Proselint::new();
            linter.warm_all()
        })
    });
}

fn bench_position_tracking(c: &mut Criterion) {
    use proselint_wasm::LineTracker;

    let text = LARGE_TEXT;
    let tracker = LineTracker::new(text);

    c.bench_function("position_tracking_sequential", |b| {
        b.iter(|| {
            for offset in (0..text.len()).step_by(10) {
                black_box(tracker.offset_to_position(offset));
            }
        })
    });
}

fn bench_quote_tracking(c: &mut Criterion) {
    use proselint_wasm::QuoteTracker;

    let text = r#"He said "very good" and "really nice" but it was "not great" at all."#;
    let tracker = QuoteTracker::new(text);

    c.bench_function("quote_tracking_checks", |b| {
        b.iter(|| {
            for offset in 0..text.len() {
                black_box(tracker.is_in_quote(offset));
            }
        })
    });
}

fn bench_batch_linting(c: &mut Criterion) {
    let linter = Linter::new();
    let texts = [SMALL_TEXT, MEDIUM_TEXT, LARGE_TEXT];

    let mut group = c.benchmark_group("batch_linting");

    for size in [1, 5, 10, 20].iter() {
        let batch: Vec<&str> = texts.iter().cycle().take(*size).copied().collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &batch, |b, batch| {
            b.iter(|| {
                for text in batch {
                    black_box(linter.check(text));
                }
            })
        });
    }

    group.finish();
}

fn bench_config_lookups(c: &mut Criterion) {
    let mut config = Config::default();
    config.disable("typography");
    config.enable("typography.symbols");

    c.bench_function("config_is_check_enabled", |b| {
        b.iter(|| {
            black_box(config.is_check_enabled("typography.symbols.ellipsis"));
            black_box(config.is_check_enabled("weasel_words.very"));
            black_box(config.is_check_enabled("typography.dashes.em_dash"));
            black_box(config.is_check_enabled("redundancy.free_gift"));
        })
    });
}

fn bench_utf8_text(c: &mut Criterion) {
    let linter = Linter::new();
    let utf8_text = "Café très élégant avec des crêpes. 日本語のテキスト。Emoji: 👋🌟🎉";

    c.bench_function("lint_utf8_text", |b| {
        b.iter(|| linter.check(black_box(utf8_text)))
    });
}

fn bench_regex_compilation(c: &mut Criterion) {
    use proselint_wasm::Check;

    c.bench_function("regex_compilation_cached", |b| {
        let check = Check::new("test.check", "Test check", r"very|really|quite");

        b.iter(|| {
            black_box(check.get_regex());
        })
    });
}

criterion_group!(
    benches,
    bench_small_text,
    bench_medium_text,
    bench_large_text,
    bench_warm_all,
    bench_position_tracking,
    bench_quote_tracking,
    bench_batch_linting,
    bench_config_lookups,
    bench_utf8_text,
    bench_regex_compilation,
);

criterion_main!(benches);
