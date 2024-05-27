extern crate lang_c;

use std::time::Duration;

use egg::{LpExtractor, RecExpr, Runner, TreeExplanation};
use lang_c::driver::{parse, Config};
use super_optimiser::{init_rules, transpile, CCostFunction, C};

fn find_consecutive_rule_sequences(
    explanation: &TreeExplanation<C>,
) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    find_sequences_recursive(explanation, &mut Vec::new(), &mut results);
    results
}

fn find_sequences_recursive(
    explanation: &TreeExplanation<C>,
    current_sequence: &mut Vec<String>,
    results: &mut Vec<Vec<String>>,
) {
    for term in explanation {
        if let Some(ref rule) = term.forward_rule {
            current_sequence.push(rule.clone().to_string());
            // Continue deeper with the current sequence
            for child_proof in &term.child_proofs {
                find_sequences_recursive(child_proof, current_sequence, results);
            }
            // Record the current sequence if it ends here
            if term.child_proofs.is_empty() {
                results.push(current_sequence.clone());
            }
            current_sequence.pop();
        } else {
            // Start new sequences from child proofs
            for child_proof in &term.child_proofs {
                find_sequences_recursive(child_proof, &mut Vec::new(), results);
            }
        }
    }
}

fn main() {
    // Initialise variables
    let args: Vec<String> = std::env::args().collect();
    let code_path = "codebases/".to_owned() + &args[1];
    let rules = &init_rules();

    // Parse the C file to AST and transpile it to the simplified language
    let ast = parse(&Config::default(), code_path).map(|p| p.unit).map_err(|e| format!("{:?}", e)).expect("Error parsing C file");
    let transpiled: RecExpr<C> = transpile(ast).parse().unwrap();
    println!("{}", transpiled);
    
    // Initialise the EGG optimiser
    let mut optimiser: Runner<C, ()> = Runner::default().with_time_limit(Duration::from_secs(10)).with_explanations_enabled().with_expr(&transpiled).run(rules);
    let mut extractor = LpExtractor::new(&optimiser.egraph, CCostFunction);
    
    let best_expr = extractor.solve(optimiser.roots[0]);

    println!("{}", best_expr);

    let mut _explanation = optimiser.explain_equivalence(&transpiled, &best_expr);
    
}

