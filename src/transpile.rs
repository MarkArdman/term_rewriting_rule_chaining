use std::collections::HashMap;

use lang_c::{ast::{BinaryOperator, BlockItem, Declaration, DeclaratorKind, Expression, FloatBase, ForInitializer, Initializer, IntegerBase, Label, LabeledStatement, Statement, UnaryOperator}, span::Node};

pub fn transpile(function: &Statement) -> String {
    let mut variable_map: HashMap<String, String> = HashMap::new();
    let transpiled = transpile_statment(&function, &mut variable_map);
    transpiled.0
}

fn transpile_statment(node: &Statement, binding_map: &mut HashMap<String, String>) -> (String, Vec<String>) {
    match &node {
        Statement::Labeled(l) => transpile_label(&l.node, binding_map),
        Statement::Compound(vec) => {
            let mut statements = Vec::new();
            let mut mutated_vars = Vec::new();
            if vec.is_empty() {
                return ("(ignore)".to_string(), Vec::new());
            }
            for statement in vec {
                let expr = transpile_block_item(&statement.node, binding_map);
                statements.push(expr.0);
                mutated_vars.extend(expr.1);
            }
            (format!("(compound {})", statements.join(" ")), mutated_vars)
        },
        Statement::Expression(expr) => <Option<Box<Node<lang_c::ast::Expression>>> as Clone>::clone(&expr).map_or_else(|| ("(ignore)".to_string(), Vec::new()), |e| transpile_expr(&e.node, binding_map)),
        Statement::If(expr) => {
            let cond = transpile_expr(&expr.node.condition.node, binding_map);
            let if_clause = transpile_statment(&expr.node.then_statement.node, binding_map);
            let else_clause = match &expr.node.else_statement {
                Some(s) => transpile_statment(&s.node, binding_map),
                None => ("(ignore)".to_string(), Vec::new()),
            };
            let expr = format!("(if {} {} {})", cond.0, if_clause.0, else_clause.0);

            let mut mutated_vars = cond.1;
            mutated_vars.extend(if_clause.1);
            mutated_vars.extend(else_clause.1);

            // Remove the variables that are mutated in the if and else clauses
            for var in mutated_vars.iter() {
                binding_map.remove(var);
            }

            (expr, mutated_vars)
        },
        Statement::Switch(expr) => {
            let cond = transpile_expr(&expr.node.expression.node, binding_map);
            let statement = transpile_statment(&expr.node.statement.node, binding_map);
            let expr = format!("(switch {} {})", cond.0, statement.0);

            let mut mutated_vars = cond.1;
            mutated_vars.extend(statement.1);

            // Remove the variables that are mutated in the switch body
            for var in mutated_vars.iter() {
                binding_map.remove(var);
            }

            (expr, mutated_vars)
        },
        Statement::While(expr) => {
            let cond = transpile_expr(&expr.node.expression.node, binding_map);
            let statement = transpile_statment(&expr.node.statement.node, binding_map);
            let expr = format!("(while {} {})", cond.0, statement.0);
            
            let mut mutated_vars = cond.1;
            mutated_vars.extend(statement.1);

            // Remove the variables that are mutated in the while body
            for var in mutated_vars.iter() {
                binding_map.remove(var);
            }

            (expr, mutated_vars)
        },
        Statement::DoWhile(expr) => {
            let cond = transpile_expr(&expr.node.expression.node, binding_map);
            let statement = transpile_statment(&expr.node.statement.node, binding_map);
            let expr = format!("(do-while {} {})", cond.0, statement.0);
            
            let mut mutated_vars = cond.1;
            mutated_vars.extend(statement.1);

            // Remove the variables that are mutated in the do-while body
            for var in mutated_vars.iter() {
                binding_map.remove(var);
            }

            (expr, mutated_vars)
        },
        Statement::For(expr) => {
            let initialiser = transpile_for_initialiser(&expr.node.initializer.node, binding_map);
            let cond = <Option<Box<Node<lang_c::ast::Expression>>> as Clone>::clone(&expr.node.condition).map_or_else(|| ("(ignore)".to_string(), Vec::new()), |e| transpile_expr(&e.node, binding_map));
            let step = <Option<Box<Node<lang_c::ast::Expression>>> as Clone>::clone(&expr.node.step).map_or_else(|| ("(ignore)".to_string(), Vec::new()), |e| transpile_expr(&e.node, binding_map));

            // Remove the variables that are mutated in the for loop step
            for var in step.1.iter() {
                binding_map.remove(var);
            }

            let statement = transpile_statment(&expr.node.statement.node, binding_map);
            let expr = format!("(for {} {} {} {})", initialiser.0, cond.0, step.0, statement.0);
            
            let mut mutated_vars = initialiser.1;
            mutated_vars.extend(cond.1);
            mutated_vars.extend(statement.1);
            mutated_vars.extend(step.1);

            // Remove the variables that are mutated in the for loop body
            for var in mutated_vars.iter() {
                binding_map.remove(var);
            }

            (expr, mutated_vars)
        },
        Statement::Goto(id) => {
            let expr = format!("(goto {})", &id.node.name);
            binding_map.clear();
            (expr, Vec::new())
        },
        Statement::Continue => ("(continue)".to_string(), Vec::new()), // "continue" is a reserved keyword in Rust, so we use "contiuneC" instead
        Statement::Break => ("(break)".to_string(), Vec::new()),
        Statement::Return(None) => ("(return null)".to_string(), Vec::new()),
        Statement::Return(Some(b)) => {
            let statement = transpile_expr(&b.node, binding_map);
            let expr = format!("(return {})", statement.0);

            (expr, statement.1)
        },
        Statement::Asm(_) => ("(asm)".to_string(), Vec::new())
    }
}

fn transpile_label(node: &LabeledStatement, binding_map: &mut HashMap<String, String>) -> (String, Vec<String>) {
    match &node.label.node {
        Label::Identifier(i) => {
            let expr = transpile_statment(&node.statement.node, binding_map);
            (format!("(label {} {})", &i.node.name, expr.0), expr.1)
        },
        Label::Case(expr) => {
            let statement = transpile_statment(&node.statement.node, binding_map);
            (format!("(case {} {})", transpile_expr(&expr.node, binding_map).0, statement.0), statement.1)
        },
        Label::CaseRange(_) => todo!(),
        Label::Default => {
            let statement = transpile_statment(&node.statement.node, binding_map);
            (format!("(case default {})", statement.0), statement.1)
        },
    }
}

fn transpile_block_item(node: &BlockItem, binding_map: &mut HashMap<String, String>) -> (String, Vec<String>) {
    match &node {
        BlockItem::Declaration(nodes) => transpile_declaration(&nodes.node, binding_map),
        BlockItem::Statement(s) => transpile_statment(&s.node, binding_map),
        BlockItem::StaticAssert(_) => todo!(),
    }
}

fn transpile_expr(node: &Expression, binding_map: &mut HashMap<String, String>) -> (String, Vec<String>) {
    match &node {
        Expression::Identifier(i) => {
            let name = &i.node.name;
            (binding_map.get(name).map_or_else(|| name.to_string(), |v| v.to_string()), Vec::new())
        },
        Expression::Constant(c) => match &c.node {
            lang_c::ast::Constant::Integer(integer) => {
                let value = match integer.base {
                    IntegerBase::Decimal => integer.number.to_string(),
                    IntegerBase::Octal => i64::from_str_radix(&integer.number, 8).map_or_else(|_| "0".to_string(), |v| v.to_string()),
                    IntegerBase::Hexadecimal => i64::from_str_radix(&integer.number, 16).map_or_else(|_| "0".to_string(), |v| v.to_string()),
                    IntegerBase::Binary => i64::from_str_radix(&integer.number, 2).map_or_else(|_| "0".to_string(), |v| v.to_string()),
                };
                (format!("{}", value), Vec::new())
            },
            lang_c::ast::Constant::Float(float) => {
                let value = match float.base {
                    FloatBase::Decimal => float.number.to_string(),
                    FloatBase::Hexadecimal => todo!(),  // Note: Converting hex floating point is not straightforward in Rust
                };
                (format!("{}", value), Vec::new())
            },
            lang_c::ast::Constant::Character(character) => {
                (format!("'{}'", character), Vec::new())
            },
        },
        Expression::StringLiteral(l) => (format!("(string {})", l.node.join(" ")), Vec::new()),
        Expression::GenericSelection(_) => todo!(),
        Expression::Member(expr) => {
            let struct_name = transpile_expr(&expr.node.expression.node, binding_map);
            let member_name = expr.node.identifier.node.name.to_owned();
            let expr = format!("(member {} {})", struct_name.0, member_name);

            let mutated_vars = struct_name.1;
            
            (expr, mutated_vars)
        },
        Expression::Call(_) => ("(call)".to_string(), Vec::new()),
        Expression::CompoundLiteral(_) => todo!(),
        Expression::SizeOfTy(_) => ("(sizeoftype)".to_string(), Vec::new()),
        Expression::SizeOfVal(_) => ("(sizeofexpr)".to_string(), Vec::new()),
        Expression::AlignOf(_) => todo!(),
        Expression::UnaryOperator(expr) => {
            let statement = transpile_expr(&expr.node.operand.node, binding_map);
            let var = match &expr.node.operand.node {
                Expression::Identifier(i) => i.node.name.to_owned(),
                _ => statement.0.clone(),
            };
            match &expr.node.operator.node {
                UnaryOperator::Plus => (format!("(+ {})", statement.0), statement.1),
                UnaryOperator::Minus => (format!("(- {})", statement.0), statement.1),
                UnaryOperator::Negate => (format!("(! {})", statement.0), statement.1),
                UnaryOperator::Complement => (format!("(~ {})", statement.0), statement.1),
                UnaryOperator::Indirection => (format!("(* {})", statement.0), statement.1),
                UnaryOperator::Address => (format!("(& {})", statement.0), statement.1),
                UnaryOperator::PreIncrement => {
                    let expr = format!("(+ {} 1)", statement.0);
                    binding_map.insert(var.clone(), expr.clone());
                    let mut mutated_vars = statement.1;
                    mutated_vars.push(var.clone());
                    (format!("(= {} {})", var, expr), mutated_vars)
                }, 
                UnaryOperator::PreDecrement => {
                    let expr = format!("(- {} 1)", statement.0);
                    binding_map.insert(var.clone(), expr.clone());
                    let mut mutated_vars = statement.1;
                    mutated_vars.push(var.clone());
                    (format!("(= {} {})", var, expr), mutated_vars)
                },
                UnaryOperator::PostIncrement => {
                    let expr = format!("(+ {} 1)", statement.0);
                    binding_map.insert(var.clone(), expr.clone());
                    let mut mutated_vars = statement.1;
                    mutated_vars.push(var.clone());
                    (format!("(= {} {})", var, expr), mutated_vars)
                },
                UnaryOperator::PostDecrement => {
                    let expr = format!("(- {} 1)", statement.0);
                    binding_map.insert(var.clone(), expr.clone());
                    let mut mutated_vars = statement.1;
                    mutated_vars.push(var.clone());
                    (format!("(= {} {})", var, expr), mutated_vars)
                },
            }
        },
        Expression::Cast(expr) => {
            let statement = transpile_expr(&expr.node.expression.node, binding_map);
            
            (format!("(cast temp {})", statement.0), statement.1)
        },
        Expression::BinaryOperator(expr) => {
            let lhs = transpile_expr(&expr.node.lhs.node, binding_map);
            let rhs = transpile_expr(&expr.node.rhs.node, binding_map);
            let var_l = match &expr.node.lhs.node {
                Expression::Identifier(i) => i.node.name.to_owned(),
                _ => lhs.0.clone(),
            };

            let mut mutated_vars = lhs.1;
            mutated_vars.extend(rhs.1);

            match &expr.node.operator.node {
                BinaryOperator::Index => (format!("(index {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::Multiply => (format!("(* {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::Divide => (format!("(/ {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::Modulo => (format!("(% {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::Plus => (format!("(+ {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::Minus => (format!("(- {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::ShiftLeft => (format!("(<< {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::ShiftRight => (format!("(>> {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::Less => (format!("(< {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::Greater => (format!("(> {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::LessOrEqual => {
                    let expr = format!("(< {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::GreaterOrEqual => {
                    let expr = format!("(> {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::Equals => (format!("(== {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::NotEquals => (format!("(!= {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::BitwiseAnd => (format!("(& {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::BitwiseXor => (format!("(^ {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::BitwiseOr => (format!("(| {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::LogicalAnd => (format!("(&& {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::LogicalOr => (format!("(|| {} {})", lhs.0, rhs.0), mutated_vars),
                BinaryOperator::Assign => {
                    let expr = format!("{}", rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignMultiply => {
                    let expr = format!("(* {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignDivide => {
                    let expr = format!("(/ {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignModulo => {
                    let expr = format!("(% {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignPlus => {
                    let expr = format!("(+ {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignMinus => {
                    let expr = format!("(- {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignShiftLeft => {
                    let expr = format!("(<< {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignShiftRight => {
                    let expr = format!("(>> {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignBitwiseAnd => {
                    let expr = format!("(& {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignBitwiseXor => {
                    let expr = format!("(^ {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
                BinaryOperator::AssignBitwiseOr => {
                    let expr = format!("(| {} {})", lhs.0, rhs.0);
                    binding_map.insert(var_l.clone(), expr.clone());
                    mutated_vars.push(var_l.clone());
                    (format!("(= {} {})", var_l, expr), mutated_vars)
                },
            }
        },
        Expression::Conditional(expr) => {
            let cond = transpile_expr(&expr.node.condition.node, binding_map);
            let if_clause = transpile_expr(&expr.node.then_expression.node, binding_map);
            let else_clause = transpile_expr(&expr.node.else_expression.node, binding_map);
            let expr = format!("(if {} {} {})", cond.0, if_clause.0, else_clause.0);

            let mut mutated_vars = cond.1;
            mutated_vars.extend(if_clause.1);
            mutated_vars.extend(else_clause.1);

            // Remove the variables that are mutated in the if and else clauses
            for var in mutated_vars.iter() {
                binding_map.remove(var);
            }

            (expr, mutated_vars)
        },
        Expression::Comma(expr) => {
            let mut statements = Vec::new();
            let mut mutated_vars = Vec::new();
            if expr.is_empty() {
                return ("(ignore)".to_string(), Vec::new());
            }
            for statement in expr.iter(){
                let expr = transpile_expr(&statement.node, binding_map);
                statements.push(expr.0);
                mutated_vars.extend(expr.1);
            }
            (format!("(compound {})", statements.join(" ")), mutated_vars)
        },
        Expression::OffsetOf(_) => todo!(),
        Expression::VaArg(_) => todo!(),
        Expression::Statement(expr) => transpile_statment(&expr.node, binding_map),
    }
}

fn transpile_declaration(node: &Declaration, binding_map: &mut HashMap<String, String>) -> (String, Vec<String>) {
    let mut declarations = Vec::new();
    for decl in &node.declarators {
        let name = transpile_declaration_kind(&decl.node.declarator.node.kind.node);
        let expression = <Option<Node<Initializer>> as Clone>::clone(&decl.node.initializer).map_or_else(|| ("null".to_string(), Vec::new()), |i| transpile_initialiser(&i.node, binding_map));
        binding_map.insert(name.clone(), expression.0.clone());
        declarations.push(format!("(declaration {} {})", name, expression.0));
    }
    (declarations.join(" "), Vec::new())
}

fn transpile_declaration_kind(node: &DeclaratorKind) -> String {
    match &node {
        DeclaratorKind::Abstract => todo!(),
        DeclaratorKind::Identifier(i) => i.node.name.to_owned(),
        DeclaratorKind::Declarator(_) => todo!(),
    }
}

fn transpile_initialiser(node: &Initializer, binding_map: &mut HashMap<String, String>) -> (String, Vec<String>) {
    match &node {
        Initializer::Expression(expr) => transpile_expr(&expr.node, binding_map),
        Initializer::List(expr) => {
            let mut initialisers = Vec::new();
            let mut mutated_vars = Vec::new();
            for i in expr {
                let init = transpile_initialiser(&i.node.initializer.node, binding_map);
                initialisers.push(init.0);
                mutated_vars.extend(init.1);
            }
            (format!("(list {})", initialisers.join(" ")) , mutated_vars)
        }
    }
}

fn transpile_for_initialiser(node: &ForInitializer, binding_map: &mut HashMap<String, String>) -> (String, Vec<String>) {
    match &node {
        ForInitializer::Empty => ("(ignore)".to_string(), Vec::new()),
        ForInitializer::Expression(expr) => transpile_expr(&expr.node, binding_map),
        ForInitializer::Declaration(expr) => transpile_declaration(&expr.node, binding_map),
        ForInitializer::StaticAssert(_) => todo!(),
    }
}