extern crate lang_c;

use std::{collections::HashMap, time::Duration};

use egg::{AstSize, Extractor, RecExpr, Runner, TreeExplanation, TreeTerm};
use lang_c::driver::{parse, Config};
use super_optimiser::{init_rules, transpile, C};

/*
 * Represents a rule chain
 */
#[derive(Debug, Clone)]
struct RuleChain {
    chain: Vec<String>,
    child_chains: Vec<Option<RuleChain>>
}

/*
 * Extracts the rule chains from the explanation tree
 */
fn extract_rules_from_explanation(root: &TreeExplanation<C>) {
    let mut rules: Vec<Vec<&TreeExplanation<C>>> = Vec::new();

    if root.len() < 2 {
        return;
    }

    for term in root.iter() {
        if rules.is_empty() {
            rules = term.child_proofs.iter().map(|x| vec![x]).collect();
        } else {
            let mut temp = Vec::new();
            rules.iter().zip(term.child_proofs.iter()).for_each(|(x, y)| {
                // Add the current rule to the end of the current rule chain
                let mut new_chain = x.clone();
                new_chain.push(y);
                temp.push(new_chain);
            });
            rules = temp.clone();
        }
    }

    // rules.iter().for_each(|x: &Vec<&Vec<std::rc::Rc<TreeTerm<C>>>>| {
    //     let mapped = x.iter().map(|y| format!("{}", {
    //         term_to_string(y)
    //     })).collect::<Vec<String>>();
    //     // Filter out strings where all the terms are the same
    //     if mapped.iter().all(|x| x == &mapped[0]) {
    //         return;
    //     }
    //     // Remove the duplicates from the list
    //     let mut seen = std::collections::HashSet::new();
    //     let deduplicated = mapped.into_iter().filter(|y| seen.insert(y.clone())).collect::<Vec<String>>();

    //     println!("{}", deduplicated.join(" -> "));
    // });

    rules.iter().for_each(|x: &Vec<&Vec<std::rc::Rc<TreeTerm<C>>>>| {
        let mapped = x.iter().map(|y| format!("{}", {
            term_to_string(y)
        })).collect::<Vec<String>>();
        // Filter out strings where all the terms are the same
        if mapped.iter().all(|x| x == &mapped[0]) {
            return;
        }
        let rules = term_to_rule_chains(x.last().unwrap(), 0);
        if rules.1 > 1 {
            println!("{:?}\n", rules.0.unwrap());
        }
    });
}

/*
 * Converts a TreeExplanation to the initial expression string
 */
fn term_to_initial_string(term: &TreeExplanation<C>) -> String {
    let mut expr = format!("({}", term.first().unwrap().node);
    for child in &term.first().unwrap().child_proofs {
        expr = format!("{} {}", expr, term_to_string(child));
    }
    format!("{})", expr)
}

/*
 * Converts a TreeExplanation to the final expression string
 */
fn term_to_string(term: &TreeExplanation<C>) -> String {
    let mut expr = format!("({}", term.last().unwrap().node);
    for child in &term.last().unwrap().child_proofs {
        expr = format!("{} {}", expr, term_to_string(child));
    }
    format!("{})", expr)
}

/**
 * Converts a TreeExplanation to a RuleChain
 */
fn term_to_rule_chains(term: &TreeExplanation<C>, depth: u64) -> (Option<RuleChain>, u64) {
    let mut chain = RuleChain {
        chain: Vec::new(),
        child_chains: Vec::new()
    };

    let mut has_rule = 0;
    let mut child_counters = Vec::new();

    // Iterate through every TreeTerm except the first one
    for child in term.iter() {
        if child.forward_rule.is_some() {
            chain.chain.push(format!("{}", child.forward_rule.unwrap().as_str()));
            has_rule = 1;
        }

        for child in &child.child_proofs {
            if child.len() > 1 {
                let (child_expr, chain_indicator) = term_to_rule_chains(child, depth + 1);
                child_counters.push(chain_indicator);
                chain.child_chains.push(child_expr);
            }
        }
    }

    let mut chain_indicator = child_counters.iter().max().or_else(|| Some(&0)).unwrap().clone();
    if has_rule == 0 {
        if chain_indicator > 1 {
            println!("{:?}\n", chain);
            println!("{} -> {}\n", term_to_initial_string(term), term_to_string(term));
        }
        return (None, 0);
    } else {
        chain_indicator += has_rule;
        return (Some(chain), chain_indicator);
    }
}

/**
 * Collects statistics on the rules used in the explanation tree
 */
fn count_rules(term: &TreeExplanation<C>) -> HashMap<String, u64> {
    let mut counts = HashMap::new();
    for child in term.iter() {
        if child.forward_rule.is_some() {
            let count = counts.entry(child.forward_rule.unwrap().to_string()).or_insert(0);
            *count += 1;
        }
        for child in &child.child_proofs {
            let child_count = count_rules(child);
            child_count.iter().for_each(|(k, v)| {
                let count = counts.entry(k.to_string()).or_insert(0);
                *count += v;
            });
        }
    }
    counts
}

fn optimise_function(rules: &Vec<egg::Rewrite<C, ()>>, function: RecExpr<C>, nanos: u64, explanation: bool) -> (usize, RecExpr<C>, HashMap<String, u64>) {
    let mut optimiser: Runner<C, ()> = Runner::default().with_iter_limit(10000000)
            .with_node_limit(1000000)
            .with_time_limit(Duration::from_nanos(nanos))
            .with_explanations_enabled().with_expr(&function).run(rules);
    let extractor = Extractor::new(&optimiser.egraph, AstSize);

    let best_expr = extractor.find_best(optimiser.roots[0]);
    let mut counts = HashMap::new();

    if explanation {
        let (init_cost, _, _) = optimise_function(rules, function.clone(), 0, false);
        let explanation = &optimiser.explain_equivalence(&function, &best_expr.1).explanation_trees;

        if best_expr.0 < init_cost {
            counts = count_rules(explanation).to_owned();
            extract_rules_from_explanation(explanation);
        }
    }

    (best_expr.0, best_expr.1, counts)
}

fn main() {
    // Initialise variables
    let args: Vec<String> = std::env::args().collect();
    let code_path = "codebases/".to_owned() + &args[1];
    let action = &args[2] == "rules";
    let rules: &Vec<egg::Rewrite<C, ()>> = &init_rules();
    let time_limit_independent_variable = if action { 1000000000 } else { 50000000 };

    // Parse the C file to AST and transpile it to the simplified language
    let mut transpiled_functions = Vec::new();
    let paths = std::fs::read_dir(code_path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".c") {
            let ast = parse(&Config::default(), &path).map(|p| p.unit).map_err(|e| format!("{:?}", e)).expect("Error parsing C file");
            let functions = transpile(ast);
            transpiled_functions.extend(functions.iter().map(|x| x.parse().unwrap()).collect::<Vec<RecExpr<C>>>());
        }
    }
    

    // Optimise each function and store the cost and time limit
    let mut function_costs: HashMap<String, usize> = HashMap::new();
    let mut function_time: HashMap<String, u64> = HashMap::new();
    let mut rule_counts: HashMap<String, u64> = HashMap::new();
    for function in transpiled_functions.iter() {
        function_time.insert(function.to_string(), time_limit_independent_variable);
    }
    
    // Parallelise the optimisation of each function
    transpiled_functions.iter().for_each(|f| {
        let (cost, _, count) = optimise_function(rules, f.clone(), time_limit_independent_variable, action);
        function_costs.insert(f.to_string(), cost);
        count.iter().for_each(|(k, v)| {
            let count = rule_counts.entry(k.to_string()).or_insert(0);
            *count += v;
        });
    });

    if action {
        rule_counts.iter().for_each(|(k, v)| {
            println!("{}: {}", k, v);
        });
        return;
    }

    // Gradually shrink the time limit for each function until the cost drops below the one in function_costs
    transpiled_functions.iter().for_each(|f| {
        let mut time = *function_time.get(&f.to_string()).unwrap();
        let cost = *function_costs.get(&f.to_string()).unwrap();
        let mut updated_cost = cost;
        println!("Function: {}", f);
        println!("Initial Time: {:?}, Initial Cost: {:?}", time, updated_cost);
        while updated_cost <= cost {
            let (new_cost, _, _) = optimise_function(rules, f.clone(), time, false);
            function_time.insert(f.to_string(), time);
            updated_cost = new_cost;
            time = ((time as f64) / 1.1) as u64;
            if time == 0 {
                break;
            }
        }
        time = ((time as f64) * 1.2) as u64;
        updated_cost = cost;
        while updated_cost <= cost {
            let (new_cost, _, _) = optimise_function(rules, f.clone(), time, false);
            function_time.insert(f.to_string(), time);
            updated_cost = new_cost;
            time = ((time as f64) / 1.001) as u64;
            if time == 0 {
                break;
            }
        }
        println!("Time: {:?}, Cost: {:?}", function_time.get(&f.to_string()).unwrap(), *function_costs.get(&f.to_string()).unwrap())
    });

    // Sum the lowest time limits for each function
    let total_time: u64 = function_time.iter().map(|(_, v)| v).sum();
    println!("Total time: {:?}", total_time);
}

