use std::collections::HashSet;
use std::fs;
use wcloud::{Tokenizer, WordCloud, WordCloudSize, DEFAULT_EXCLUDE_WORDS_TEXT};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn wcloud(c: &mut Criterion) {
    let mut group = c.benchmark_group("create star wars");
    group.sample_size(10);

    let script_path = "examples/custom_colors/a_new_hope.txt";
    let script_text = fs::read_to_string(script_path)
        .expect("Unable to find a_new_hope.txt")
        .replace("HAN", "Han")
        .replace("LUKE'S", "Luke");

    group.bench_function("generate word cloud", |b| {
        b.iter(|| {
            let mut filter = DEFAULT_EXCLUDE_WORDS_TEXT.lines().collect::<HashSet<_>>();

            filter.insert("int");
            filter.insert("ext");

            let tokenizer = Tokenizer::default()
                .with_max_words(1000)
                .with_filter(filter);

            let cloud = WordCloud::default()
                .with_tokenizer(tokenizer)
                .with_word_margin(10)
                .with_rng_seed(1);

            let mask_path = "examples/custom_colors/stormtrooper_mask.png";
            let mask_image = image::open(mask_path).unwrap().to_luma8();
            let mask = WordCloudSize::FromMask(mask_image);

            cloud.generate_from_text(&script_text, mask, black_box(1.0))
        })
    });

    group.finish();
}

criterion_group!(benches, wcloud);
criterion_main!(benches);
