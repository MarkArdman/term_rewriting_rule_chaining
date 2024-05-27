use std::collections::HashMap;

use lang_c::{ast::{BinaryOperator, BlockItem, Declaration, DeclaratorKind, Expression, FloatBase, ForInitializer, Initializer, IntegerBase, Label, LabeledStatement, Statement, UnaryOperator}, span::Node};

pub fn transpile(function: &Statement) -> String {
    let mut variable_map: HashMap<String, String> = HashMap::new();
    let transpiled = transpile_statment(&function, &mut variable_map);
    println!("{:?}", variable_map);
    transpiled
}

fn transpile_statment(node: &Statement, binding_map: &mut HashMap<String, String>) -> String {
    match &node {
        Statement::Labeled(l) => transpile_label(&l.node, binding_map),
        Statement::Compound(vec) => {
            let mut statements = Vec::new();
            if vec.is_empty() {
                return "(ignore)".to_string();
            }
            for statement in vec {
                statements.push(transpile_block_item(&statement.node, binding_map));
            }
            format!("(compound {})", statements.join(" "))
        },
        Statement::Expression(expr) => <Option<Box<Node<lang_c::ast::Expression>>> as Clone>::clone(&expr).map_or_else(|| "(ignore)".to_string(), |e| transpile_expr(&e.node, binding_map)),
        Statement::If(expr) => {
            let expr = format!("(if {} {} {})", transpile_expr(&expr.node.condition.node, binding_map), transpile_statment(&expr.node.then_statement.node, binding_map), match &expr.node.else_statement {
                Some(s) => transpile_statment(&s.node, binding_map),
                None => "(ignore)".to_string(),
            });
            expr
        },
        Statement::Switch(expr) => format!("(switch {} {})", transpile_expr(&expr.node.expression.node, binding_map), transpile_statment(&expr.node.statement.node, binding_map)),
        Statement::While(expr) => {
            let expr = format!("(while {} {})", transpile_expr(&expr.node.expression.node, binding_map), transpile_statment(&expr.node.statement.node, binding_map));
            binding_map.clear();
            expr
        },
        Statement::DoWhile(expr) => {
            let expr = format!("(do-while {} {})", transpile_expr(&expr.node.expression.node, binding_map), transpile_statment(&expr.node.statement.node, binding_map));
            binding_map.clear();
            expr
        },
        Statement::For(expr) => {
            let expr = format!("(for {} {} {} {})", transpile_for_initialiser(&expr.node.initializer.node, binding_map), &<Option<Box<Node<lang_c::ast::Expression>>> as Clone>::clone(&expr.node.condition).map_or_else(|| "(ignore)".to_string(), |e| transpile_expr(&e.node, binding_map)), &<Option<Box<Node<lang_c::ast::Expression>>> as Clone>::clone(&expr.node.step).map_or_else(|| "(ignore)".to_string(), |e| transpile_expr(&e.node, binding_map)), transpile_statment(&expr.node.statement.node, binding_map));
            binding_map.clear();
            expr
        },
        Statement::Goto(id) => {
            let expr = format!("(goto {})", &id.node.name);
            binding_map.clear();
            expr
        },
        Statement::Continue => "(continue)".to_string(), // "continue" is a reserved keyword in Rust, so we use "contiuneC" instead
        Statement::Break => "(break)".to_string(),
        Statement::Return(None) => "(return null)".to_string(),
        Statement::Return(Some(b)) => format!("(return {})", transpile_expr(&b.node, binding_map)),
        Statement::Asm(_) => "(asm)".to_string(),
    }
}

fn transpile_label(node: &LabeledStatement, binding_map: &mut HashMap<String, String>) -> String {
    match &node.label.node {
        Label::Identifier(i) => format!("(label {} {})", &i.node.name, transpile_statment(&node.statement.node, binding_map)),
        Label::Case(expr) => format!("(case {} {})", transpile_expr(&expr.node, binding_map), transpile_statment(&node.statement.node, binding_map)),
        Label::CaseRange(_) => todo!(),
        Label::Default => format!("(case default {})", transpile_statment(&node.statement.node, binding_map)),
    }
}

fn transpile_block_item(node: &BlockItem, binding_map: &mut HashMap<String, String>) -> String {
    match &node {
        BlockItem::Declaration(nodes) => transpile_declaration(&nodes.node, binding_map),
        BlockItem::Statement(s) => transpile_statment(&s.node, binding_map),
        BlockItem::StaticAssert(_) => todo!(),
    }
}

fn transpile_expr(node: &Expression, binding_map: &mut HashMap<String, String>) -> String {
    match &node {
        Expression::Identifier(i) => {
            let name = &i.node.name;
            binding_map.get(name).map_or_else(|| name.to_string(), |v| v.to_string())
        },
        Expression::Constant(c) => match &c.node {
            lang_c::ast::Constant::Integer(integer) => {
                let value = match integer.base {
                    IntegerBase::Decimal => integer.number.to_string(),
                    IntegerBase::Octal => i64::from_str_radix(&integer.number, 8).map_or_else(|_| "0".to_string(), |v| v.to_string()),
                    IntegerBase::Hexadecimal => i64::from_str_radix(&integer.number, 16).map_or_else(|_| "0".to_string(), |v| v.to_string()),
                    IntegerBase::Binary => i64::from_str_radix(&integer.number, 2).map_or_else(|_| "0".to_string(), |v| v.to_string()),
                };
                format!("{}", value)
            },
            lang_c::ast::Constant::Float(float) => {
                let value = match float.base {
                    FloatBase::Decimal => float.number.to_string(),
                    FloatBase::Hexadecimal => todo!(),  // Note: Converting hex floating point is not straightforward in Rust
                };
                format!("{}", value)
            },
            lang_c::ast::Constant::Character(character) => {
                format!("'{}'", character)
            },
        },
        Expression::StringLiteral(l) => format!("(string {})", l.node.join(" ")),
        Expression::GenericSelection(_) => todo!(),
        Expression::Member(expr) => format!("(member {} {})", transpile_expr(&expr.node.expression.node, binding_map), &expr.node.identifier.node.name),
        Expression::Call(_) => "(call)".to_string(),
        Expression::CompoundLiteral(_) => todo!(),
        Expression::SizeOfTy(_) => "(sizeoftype)".to_string(),
        Expression::SizeOfVal(_) => "(sizeofexpr)".to_string(),
        Expression::AlignOf(_) => todo!(),
        Expression::UnaryOperator(expr) => format!("({} {})", transpile_unary_op(&expr.node.operator.node), transpile_expr(&expr.node.operand.node, binding_map)),
        Expression::Cast(expr) => format!("(cast {} {})", "temp", transpile_expr(&expr.node.expression.node, binding_map)),
        Expression::BinaryOperator(expr) => {
            match &expr.node.operator.node {
                BinaryOperator::Assign => {
                    let lhs = match &expr.node.lhs.node {
                        Expression::Identifier(i) => i.node.name.to_owned(),
                        _ => todo!(),
                    };
                    let rhs = transpile_expr(&expr.node.rhs.node, binding_map);
                    binding_map.insert(lhs.clone(), rhs.clone());
                    format!("(= {} {})", lhs, rhs)
                },
                _ => format!("({} {} {})", transpile_binary_op(&expr.node.operator.node), transpile_expr(&expr.node.lhs.node, binding_map), transpile_expr(&expr.node.rhs.node, binding_map))
            }
        },
        Expression::Conditional(expr) => format!("(if {} then {} else {})", transpile_expr(&expr.node.condition.node, binding_map), transpile_expr(&expr.node.then_expression.node, binding_map), transpile_expr(&expr.node.else_expression.node, binding_map)),
        Expression::Comma(expr) => format!("(compound {})", &expr.iter().map(|e| transpile_expr(&e.node, binding_map)).collect::<Vec<String>>().join(" ")),
        Expression::OffsetOf(_) => todo!(),
        Expression::VaArg(_) => todo!(),
        Expression::Statement(expr) => transpile_statment(&expr.node, binding_map),
    }
}

fn transpile_binary_op(node: &BinaryOperator) -> String {
    match &node {
        BinaryOperator::Index => "index".to_string(),
        BinaryOperator::Multiply => "*".to_string(),
        BinaryOperator::Divide => "/".to_string(),
        BinaryOperator::Modulo => "%".to_string(),
        BinaryOperator::Plus => "+".to_string(),
        BinaryOperator::Minus => "-".to_string(),
        BinaryOperator::ShiftLeft => "<<".to_string(),
        BinaryOperator::ShiftRight => ">>".to_string(),
        BinaryOperator::Less => "<".to_string(),
        BinaryOperator::Greater => ">".to_string(),
        BinaryOperator::LessOrEqual => "<=".to_string(),
        BinaryOperator::GreaterOrEqual => ">=".to_string(),
        BinaryOperator::Equals => "==".to_string(),
        BinaryOperator::NotEquals => "!=".to_string(),
        BinaryOperator::BitwiseAnd => "&".to_string(),
        BinaryOperator::BitwiseXor => "^".to_string(),
        BinaryOperator::BitwiseOr => "|".to_string(),
        BinaryOperator::LogicalAnd => "&&".to_string(),
        BinaryOperator::LogicalOr => "||".to_string(),
        BinaryOperator::Assign => "=".to_string(),
        BinaryOperator::AssignMultiply => "*=".to_string(),
        BinaryOperator::AssignDivide => "/=".to_string(),
        BinaryOperator::AssignModulo => "%=".to_string(),
        BinaryOperator::AssignPlus => "+=".to_string(),
        BinaryOperator::AssignMinus => "-=".to_string(),
        BinaryOperator::AssignShiftLeft => "<<=".to_string(),
        BinaryOperator::AssignShiftRight => ">>=".to_string(),
        BinaryOperator::AssignBitwiseAnd => "&=".to_string(),
        BinaryOperator::AssignBitwiseXor => "^=".to_string(),
        BinaryOperator::AssignBitwiseOr => "|=".to_string(),
    }
}

fn transpile_unary_op(node: &UnaryOperator) -> String {
    match &node {
        UnaryOperator::Plus => "+".to_string(),
        UnaryOperator::Minus => "-".to_string(),
        UnaryOperator::Negate => "!".to_string(),
        UnaryOperator::Complement => "~".to_string(),
        UnaryOperator::Indirection => "*".to_string(),
        UnaryOperator::Address => "&".to_string(),
        UnaryOperator::PreIncrement => "++".to_string(),
        UnaryOperator::PreDecrement => "--".to_string(),
        UnaryOperator::PostIncrement => "++".to_string(),
        UnaryOperator::PostDecrement => "--".to_string()
    }
}

fn transpile_declaration(node: &Declaration, binding_map: &mut HashMap<String, String>) -> String {
    let mut declarations = Vec::new();
    for decl in &node.declarators {
        let name = transpile_declaration_kind(&decl.node.declarator.node.kind.node);
        let expression = <Option<Node<Initializer>> as Clone>::clone(&decl.node.initializer).map_or_else(|| "null".to_string(), |i| transpile_initialiser(&i.node, binding_map));
        binding_map.insert(name.clone(), expression.clone());
        declarations.push(format!("(declaration {} {})", name, expression));
    }
    declarations.join(" ")
}

fn transpile_declaration_kind(node: &DeclaratorKind) -> String {
    match &node {
        DeclaratorKind::Abstract => todo!(),
        DeclaratorKind::Identifier(i) => i.node.name.to_owned(),
        DeclaratorKind::Declarator(_) => todo!(),
    }
}

fn transpile_initialiser(node: &Initializer, binding_map: &mut HashMap<String, String>) -> String {
    match &node {
        Initializer::Expression(expr) => transpile_expr(&expr.node, binding_map),
        Initializer::List(expr) => {
            let mut initialisers = Vec::new();
            for i in expr {
                initialisers.push(transpile_initialiser(&i.node.initializer.node, binding_map));
            }
            format!("(list {})", initialisers.join(" "))
        }
    }
}

fn transpile_for_initialiser(node: &ForInitializer, binding_map: &mut HashMap<String, String>) -> String {
    match &node {
        ForInitializer::Empty => "(ignore)".to_string(),
        ForInitializer::Expression(expr) => transpile_expr(&expr.node, binding_map),
        ForInitializer::Declaration(expr) => transpile_declaration(&expr.node, binding_map),
        ForInitializer::StaticAssert(_) => todo!(),
    }
}