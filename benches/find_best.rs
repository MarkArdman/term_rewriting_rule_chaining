use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use egg::{AstSize, Extractor, RecExpr, Runner};
use lang_c::driver::{parse, Config};
use super_optimiser::{init_rules, transpile, C};

fn bench_find_best(c: &mut Criterion) {
    let code_path = "codebases/dmc.c";
    let rules = &init_rules();

    // Parse the C file to AST and transpile it to the simplified language
    let ast = parse(&Config::default(), code_path).map(|p| p.unit).map_err(|e| format!("{:?}", e)).expect("Error parsing C file");
    let transpiled: RecExpr<C> = transpile(ast).parse().unwrap();

    // Initialise the EGG optimiser
    let optimiser: Runner<C, ()> = Runner::default().with_time_limit(Duration::from_secs(10)).with_explanations_enabled().with_expr(&transpiled).run(rules);
    let extractor = Extractor::new(&optimiser.egraph, AstSize);
    
    c.bench_function("Find optimal expression bench", |b| {
        b.iter(|| {
            extractor.find_best(optimiser.roots[0]);
        });
    });
}

criterion_group!(benches, bench_find_best);

criterion_main!(benches);