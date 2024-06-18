extern crate lang_c;

use std::{collections::HashMap, time::Duration};

use egg::{AstSize, Extractor, LpExtractor, RecExpr, Runner, TreeExplanation, TreeTerm};
use lang_c::driver::{parse, Config};
use super_optimiser::{init_rules, transpile, CCostFunction, C};

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
    
    let mut to_save = std::fs::read_to_string("rules.txt").unwrap_or_default();
    rules.iter().for_each(|x: &Vec<&Vec<std::rc::Rc<TreeTerm<C>>>>| {
        let mapped = x.iter().map(|y| format!("{}", {
            term_to_string(y)
        })).collect::<Vec<String>>();
        // Filter out strings where all the terms are the same
        if mapped.iter().all(|x| x == &mapped[0]) {
            return;
        }
        let rules = term_to_rule_chains(x.last().unwrap(), 0);
        println!("{}\n", rules);
        to_save.push_str(&rules);
        to_save.push_str("\n\n");
    });

    // Append the rules to the files content
    std::fs::write("rules.txt", to_save).expect("Unable to write file");
}

fn term_to_string(term: &TreeExplanation<C>) -> String {
    let mut expr = format!("({}", term.last().unwrap().node);
    for child in &term.last().unwrap().child_proofs {
        expr = format!("{} {}", expr, term_to_string(child));
    }
    expr = format!("{})", expr);
    expr
}

fn term_to_rule_chains(term: &TreeExplanation<C>, depth: u64) -> String {
    let mut chain = Vec::new();
    // Iterate through every TreeTerm except the first one
    for child in term.iter() {
        let mut expr = format!("({}: {:?}", depth, child.forward_rule);
        for child in &child.child_proofs {
            if child.len() > 1 {
                expr = format!("{} {}", expr, term_to_rule_chains(child, depth + 1));
            }
        }
        expr = format!("{})", expr);
        chain.push(expr);
    }

    chain.join(" -> ")
}

fn optimise_function(rules: &Vec<egg::Rewrite<C, ()>>, function: RecExpr<C>, nanos: u64, explanation: bool) -> (usize, RecExpr<C>) {
    let mut optimiser: Runner<C, ()> = Runner::default().with_iter_limit(1000)
            .with_node_limit(10000)
            .with_time_limit(Duration::from_nanos(nanos))
            .with_explanations_enabled().with_expr(&function).run(rules);
    let extractor = Extractor::new(&optimiser.egraph, AstSize);

    let best_expr = extractor.find_best(optimiser.roots[0]);

    if explanation {
        extract_rules_from_explanation(&optimiser.explain_equivalence(&function, &best_expr.1).explanation_trees);
    }
    best_expr
}

fn main() {
    // Initialise variables
    let args: Vec<String> = std::env::args().collect();
    let code_path = "codebases/".to_owned() + &args[1];
    let action = &args[2] == "rules";
    let rules: &Vec<egg::Rewrite<C, ()>> = &init_rules();
    let time_limit_independent_variable = 50000000;

    // Parse the C file to AST and transpile it to the simplified language
    let ast = parse(&Config::default(), code_path).map(|p| p.unit).map_err(|e| format!("{:?}", e)).expect("Error parsing C file");
    let functions = transpile(ast);
    let transpiled_functions = functions.iter().map(|f| f.parse().unwrap()).collect::<Vec<RecExpr<C>>>();

    // Optimise each function and store the cost and time limit
    let mut function_costs: HashMap<String, usize> = HashMap::new();
    let mut function_time: HashMap<String, u64> = HashMap::new();
    for function in transpiled_functions.iter() {
        function_time.insert(function.to_string(), time_limit_independent_variable);
    }
    
    // Parallelise the optimisation of each function
    transpiled_functions.iter().for_each(|f| {
        let (cost, _) = optimise_function(rules, f.clone(), time_limit_independent_variable, action);
        function_costs.insert(f.to_string(), cost);
    });

    if action {
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
            let (new_cost, _) = optimise_function(rules, f.clone(), time, false);
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
            let (new_cost, _) = optimise_function(rules, f.clone(), time, false);
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

