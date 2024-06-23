mod transpile;

use egg::{define_language, rewrite, Id, LpCostFunction, Rewrite, Symbol};
use lang_c::ast::{ExternalDeclaration, TranslationUnit};

define_language! {
    pub enum C {
        // Util
        "ignore" = Ignore,

        // Functions to denote the collection of expressions to be optimized
        "definitions" = Funcs(Box<[Id]>),
        // Sequence statement to emulate the order of statements in a multi-statement block
        "compound" = Seq(Box<[Id]>),

        // Control flow statements
        "return" = Return([Id; 1]),
        "continue" = Continue,
        "break" = Break,
        "if" = If([Id; 3]),
        "for" = For([Id; 4]),
        "while" = While([Id; 2]),
        "do-while" = DoWhile([Id; 2]),
        "switch" = Switch([Id; 2]),
        "declaration" = Declaration([Id; 2]),
        "label" = Label([Id; 2]),
        "case" = Case([Id; 2]),
        "call" = Call(Box<[Id]>),
        "sizeoftype" = SizeOfType,
        "sizeofexpr" = SizeOfExpr,
        "asm" = Asm,

        // Binary operators
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "%" = Mod([Id; 2]),
        "==" = Eq([Id; 2]),
        "!=" = Neq([Id; 2]),
        "<" = Lt([Id; 2]),
        "<=" = Lte([Id; 2]),
        ">" = Gt([Id; 2]),
        ">=" = Gte([Id; 2]),
        "&&" = And([Id; 2]),
        "||" = Or([Id; 2]),
        "index" = Index([Id; 1]),
        "&" = BitAnd([Id; 2]),
        "|" = BitOr([Id; 2]),
        "^" = BitXor([Id; 2]),
        "=" = Assign([Id; 2]),
        "<<" = Shl([Id; 2]),
        ">>" = Shr([Id; 2]),
        "*=" = MulAssign([Id; 2]),
        "/=" = DivAssign([Id; 2]),
        "%=" = ModAssign([Id; 2]),
        "+=" = AddAssign([Id; 2]),
        "-=" = SubAssign([Id; 2]),
        "<<=" = ShlAssign([Id; 2]),
        ">>=" = ShrAssign([Id; 2]),
        "&=" = BitAndAssign([Id; 2]),
        "^=" = BitXorAssign([Id; 2]),
        "|=" = BitOrAssign([Id; 2]),

        // Unary operators
        "+" = Pos([Id; 1]),
        "-" = Neg([Id; 1]),
        "!" = Not([Id; 1]),
        "~" = BitNot([Id; 1]),
        "++" = Inc([Id; 1]),
        "--" = Dec([Id; 1]),
        "*" = Deref([Id; 1]),
        "&" = AddrOf([Id; 1]),

        // Data types
        "list" = List(Box<[Id]>),
        "string" = String(Box<[Id]>),
        "true" = True,
        "false" = False,
        Num(i64),
        Symbol(Symbol),
        Other(Symbol, Vec<Id>),
    }
}

// Transpile the C AST to a simplified language that EGG can process
pub fn transpile(ast: TranslationUnit) -> Vec<String> {
    // Find all FunctionDefinition nodes
    let mut statements = Vec::new();
    for node in &ast.0 {
        // Extract the statement from the function definition
        match &node.node {
            ExternalDeclaration::FunctionDefinition(n) => statements.push(&n.node.statement.node),
            _ => (),
        }
    }
    // Transpile each statement
    let definitions = statements.iter().map(|s| transpile::transpile(s)).collect::<Vec<String>>();
    // Join the transpiled statements
    return definitions;
}

pub fn init_rules() -> Vec<Rewrite<C, ()>> {
    return vec![
        // Naive synthetic
        // rewrite!("Compound 1";"(= ?a (+ (- (+ ?a ?b) ?b) (* ?c 0)))" => "(ignore)"),
        // rewrite!("Compound 2";"(+ (- (+ ?a ?b) ?b) (* ?c 0))" => "(?a)"),

        // Common synthetic
        // rewrite!("Compound 1";"(+ (- (+ ?a ?b) ?b) (* ?c 0))"=>"?a"),

        // Naive gzip
        // rewrite!("Compound 1";"(= ?a (+ (+ ?a 1) 1))"=>"(+= ?a 2)"),
        // rewrite!("Compound 2";"(+ (+ ?a 1) 1)"=>"(+ ?a 2)"),
        // rewrite!("Compound 3";"(+ (+ (+ (+ (+ (+ (+ (+ (?a) (1)) (1)) (1)) (1)) (1)) (1)) (1)) (1)))"=>"(+ ?a (+ (+ (+ 2 2) 2) 2))"),
        // rewrite!("Compound 4";"(= ?a (+ (+ (+ (+ (+ (+ (+ (+ (+ (?a) (1)) (1)) (1)) (1)) (1)) (1)) (1)) (1)) (1)))"=>"(+= ?a (+ (+ (+ (+ 2 1) 2) 2) 2))"),
        // rewrite!("Compound 5";"(- (+ (+ (?a) (?b)) (?c)) (?b))"=>"(+ (?a) (?c))"),
        // rewrite!("Compound 6";"(+ (+ (+ (+ (+ (+ ?a (1)) (1)) (1)) (1)) (1)) (1))"=>"(+ ?a (+ (+ 2 2) 2))"),
        // rewrite!("Compound 7";"(= (?a) (+ (+ (+ (+ (?a) (1)) (1)) (1)) (1)))"=>"(+= ?a (+ 2 2))"),
        // rewrite!("Compound 8";"(= (?a) (+ (+ (+ (?a) (1)) (1)) (1)))"=>"(+= ?a (+ 2 1))"),
        // rewrite!("Compound 9";"(= (?a) (+ (+ (+ (+ (+ (?a) (1)) (1)) (1)) (1)) (1)))"=>"(+= ?a (+ (+ 2 1) 2))"),
        // rewrite!("Compound 10";"(= (?a) (+ (+ (+ (+ (+ (+ (+ ?a (1)) (1)) (1)) (1)) (1)) (1)) (1)))"=>"(+= ?a (+ (+ (+ 2 1) 2) 2))"),
        // rewrite!("Compound 11";"(&& (&& (&& (?a) (! (?b))) (! (?c))) (|| (! (?d)) (! (?e))))"=>"(&& (?a) (! (|| (?b) (|| (?c) (&& (?d) (?e))))))"),
        // rewrite!("Compound 12";"(&& (&& (! (?a)) (! (?b))) (|| (! (?c)) (! (?d))))"=>"(! (|| (?b) (|| (?a) (&& (?c) (?d)))))"),
        // rewrite!("Compound 13";"(&& (&& (?a) (! (?b))) (! (?c)))"=>"(&& (?a) (! (|| (?b) (?c))))"),
        // rewrite!("Compound 14";"(&& (&& (&& (! (?a)) (?b)) (! (?c))) (! (?d)))"=>"(&& (?b) (! (|| (?c) (|| (?d) (?a)))))"),
        // rewrite!("Compound 15";"(= ?a (+ (+ (+ (+ (+ (+ ?a (1)) (1)) (1)) (1)) (1)) (1)))"=>"(+= ?a (+ (+ 2 2) 2))"),
        // rewrite!("Compound 16";"(+ (+ (+ (+ (?a) (1)) (1)) (1)) (1))"=>"(+ ?a (+ 2 2))"),
        // rewrite!("Compound 17";"(&& (&& (&& (?a) (! (?b))) (! (?c))) (|| (! (?d)) (! (?a))))"=>"(&& (?a) (! (|| (?c) (|| (?b) (&& (?d) (?a))))))"),
        // rewrite!("Compound 18";"(&& (&& (&& (! (?a)) (?b)) (! (?c))) (! (?d)))"=>"(&& (?b) (! (|| (?d) (|| (?a) (?c)))))"),

        // Common gzip
        rewrite!("Compound 2";"(+ (+ ?a 1) 1)"=>"(+ ?a 2)"),

        rewrite!("Compound 1";"(= ?a (+ (+ ?a 1) 1))"=>"(+= ?a 2)"),

        rewrite!("Compound 16";"(+ (+ (+ (+ (?a) (1)) (1)) (1)) (1))"=>"(+ ?a (+ 2 2))"),

        rewrite!("Compound 5";"(- (+ (+ (?a) (?b)) (?c)) (?b))"=>"(+ (?a) (?c))"),

        rewrite!("Compound 6";"(+ (+ (+ (+ (+ (+ ?a (1)) (1)) (1)) (1)) (1)) (1))"=>"(+ ?a (+ (+ 2 2) 2))"),

        rewrite!("Compound 3";"(+ (+ (+ (+ (+ (+ (+ (+ (?a) (1)) (1)) (1)) (1)) (1)) (1)) (1)) (1)))"=>"(+ ?a (+ (+ (+ 2 2) 2) 2))"),
        rewrite!("Compound 8";"(= (?a) (+ (+ (+ (?a) (1)) (1)) (1)))"=>"(+= ?a (+ 2 1))"),
        rewrite!("Compound 13";"(&& (&& (?a) (! (?b))) (! (?c)))"=>"(&& (?a) (! (|| (?b) (?c))))"),

        // Naive olympiad
        // rewrite!("Compound 1";"(|| (|| (|| ?a (&& ?b ?g)) (&& (&& ?b ?c) ?d)) (&& (&& (&& ?b ?c) ?e) ?f))"=>"(|| ?a (&& ?b (|| (&& ?f (&& ?c ?e)) (|| ?g (&& ?c ?d)))))"),

        // Base ruleset

        // Arithmetic rules
        rewrite!("Commutative addition"; "(+ ?a ?b)"=>"(+ ?b ?a)"),
        rewrite!("Commutative multiplication";"(* ?a ?b)"=>"(* ?b ?a)"),
        rewrite!("Associative addition";"(+ ?a (+ ?b ?c))"=>"(+ (+ ?a ?b) ?c)"),
        rewrite!("Associative multiplication";"(* ?a (* ?b ?c))"=>"(* (* ?a ?b) ?c)"),
        rewrite!("Distributive multiplication over addition";"(* ?a (+ ?b ?c))"=>"(+ (* ?a ?b) (* ?a ?c))"),
        rewrite!("Identity element of addition";"(+ ?a 0)"=>"?a"),
        rewrite!("Identity element of multiplication";"(* ?a 1)"=>"?a"),
        rewrite!("Annihilating element of addition";"(+ ?a 0)"=>"?a"),
        rewrite!("Annihilating element of multiplication";"(* ?a 0)"=>"0"),
        rewrite!("Sum of 2 to multiplication";"(+ ?a ?a)"=>"(* ?a 2)"),
        rewrite!("Difference of 2 to 0";"(- ?a ?a)"=>"0"),
        rewrite!("Difference of 0 to self";"(- 0 ?a)"=>"(- ?a)"),
        rewrite!("Difference of self to 0";"(- ?a 0)"=>"?a"),
        rewrite!("Add and subtract the same";"(- (+ ?a ?b) ?b)"=>"?a"),
        rewrite!("Subtract and add the same";"(+ (- ?a ?b) ?b)"=>"?a"),
        rewrite!("Multiply and divide the same";"(/ (* ?a ?b) ?b)"=>"?a"),
        rewrite!("Divide and multiply the same";"(* (/ ?a ?b) ?b)"=>"?a"),

        //Assignment rules
        rewrite!("Assignment to self";"(= ?a ?a)"=>"(ignore)"),
        rewrite!("Assignment to self plus 1";"(= ?a (+ ?a 1))"=>"(++ ?a)"),
        rewrite!("Assignment to self minus 1";"(= ?a (- ?a 1))"=>"(-- ?a)"),
        rewrite!("Assignment to self addition";"(= ?a (+ ?a ?b))"=>"(+= ?a ?b)"),
        rewrite!("Assignment to self subtraction";"(= ?a (- ?a ?b))"=>"(-= ?a ?b)"),
        rewrite!("Assignment to self multiplication";"(= ?a (* ?a ?b))"=>"(*= ?a ?b)"),
        rewrite!("Assignment to self division";"(= ?a (/ ?a ?b))"=>"/= ?a ?b"),
        rewrite!("Assignment to self modulo";"(= ?a (% ?a ?b))"=>"(%= ?a ?b)"),
        rewrite!("Assignment to self bitwise and";"(= ?a (& ?a ?b))"=>"(&= ?a ?b)"),
        rewrite!("Assignment to self bitwise or";"(= ?a (| ?a ?b))"=>"(|= ?a ?b)"),
        rewrite!("Assignment to self bitwise xor";"(= ?a (^ ?a ?b))"=>"(^= ?a ?b)"),
        rewrite!("Assignment to self left shift";"(= ?a (<< ?a ?b))"=>"(<<= ?a ?b)"),
        rewrite!("Assignment to self right shift";"(= ?a (>> ?a ?b))"=>"(>>= ?a ?b)"),
        // Reverse assignment rules
        rewrite!("Reverse assignment addition";"(+= ?a ?b)"=>"(= ?a (+ ?a ?b))"),
        rewrite!("Reverse assignment subtraction";"(-= ?a ?b)"=>"(= ?a (- ?a ?b))"),
        rewrite!("Reverse assignment multiplication";"(*= ?a ?b)"=>"(= ?a (* ?a ?b))"),
        rewrite!("Reverse assignment division";"(/= ?a ?b)"=>"(= ?a (/ ?a ?b))"),
        rewrite!("Reverse assignment modulo";"(%= ?a ?b)"=>"(= ?a (% ?a ?b))"),
        rewrite!("Reverse assignment bitwise and";"(&= ?a ?b)"=>"(= ?a (& ?a ?b))"),
        rewrite!("Reverse assignment bitwise or";"(|= ?a ?b)"=>"(= ?a (| ?a ?b))"),
        rewrite!("Reverse assignment bitwise xor";"(^= ?a ?b)"=>"(= ?a (^ ?a ?b))"),
        rewrite!("Reverse assignment left shift";"(<<= ?a ?b)"=>"(= ?a (<< ?a ?b))"),
        rewrite!("Reverse assignment right shift";"(>>= ?a ?b)"=>"(= ?a (>> ?a ?b))"),

        //Boolean rules
        rewrite!("Negation of negation";"(! (! ?a))"=>"?a"),
        rewrite!("Itentity of and true";"(&& ?a true)"=>"?a"),
        rewrite!("Itentity of and false";"(&& ?a false)"=>"0"),
        rewrite!("Itentity of and zero";"(&& ?a 0)"=>"0"),
        rewrite!("Itentity of or true";"(|| ?a true)"=>"1"),
        rewrite!("Itentity of or false";"(|| ?a false)"=>"?a"),
        rewrite!("Itentity of or zero";"(|| ?a 0)"=>"?a"),
        rewrite!("Idempotent and";"(&& ?a ?a)"=>"?a"),
        rewrite!("Idempotent or";"(|| ?a ?a)"=>"?a"),
        rewrite!("Inverse and";"(&& ?a (! ?a))"=>"0"),
        rewrite!("Inverse or";"(|| ?a (! ?a))"=>"1"),
        rewrite!("Commutative and";"(&& ?a ?b)"=>"(&& ?b ?a)"),
        rewrite!("Commutative or";"(|| ?a ?b)"=>"(|| ?b ?a)"),
        rewrite!("Associative and";"(&& ?a (&& ?b ?c))"=>"(&& (&& ?a ?b) ?c)"),
        rewrite!("Associative or";"(|| ?a (|| ?b ?c))"=>"(|| (|| ?a ?b) ?c)"),
        rewrite!("Distributive or over and";"(|| ?a (&& ?b ?c))"=>"(&& (|| ?a ?b) (|| ?a ?c))"),
        rewrite!("Distributive or over and-rev";"(&& (|| ?a ?b) (|| ?a ?c))"=>"(|| ?a (&& ?b ?c))"),
        rewrite!("Distributive and over or";"(&& ?a (|| ?b ?c))"=>"(|| (&& ?a ?b) (&& ?a ?c))"),
        rewrite!("Distributive and over or-rev";"(|| (&& ?a ?b) (&& ?a ?c))"=>"(&& ?a (|| ?b ?c))"),
        rewrite!("Absorption and";"(&& ?a (|| ?a ?b))"=>"?a"),
        rewrite!("Absorption or";"(|| ?a (&& ?a ?b))"=>"?a"),
        rewrite!("De Morgan's law and";"(! (&& ?a ?b))" => "(|| (! ?a) (! ?b))"),
        rewrite!("De Morgan's law and-rev";"(|| (! ?a) (! ?b))" => "(! (&& ?a ?b))"),
        rewrite!("De Morgan's law or";"(! (|| ?a ?b))" => "(&& (! ?a) (! ?b))"),
        rewrite!("De Morgan's law or-rev";"(&& (! ?a) (! ?b))" => "(! (|| ?a ?b))"),
        rewrite!("Negation of true";"(! true)"=>"0"),
        rewrite!("Negation of false";"(! false)"=>"1"),
        rewrite!("Negation of zero";"(! 0)"=>"1"),
        rewrite!("Negation of one";"(! 1)"=>"0"),

        // Bitwise rules
        rewrite!("Commutative bitwise and";"(& ?a ?b)"=>"(& ?b ?a)"),
        rewrite!("Commutative bitwise or";"(| ?a ?b)"=>"(| ?b ?a)"),
        rewrite!("Commutative bitwise xor";"(^ ?a ?b)"=>"(^ ?b ?a)"),
        rewrite!("Associative bitwise and";"(& ?a (& ?b ?c))"=>"(& (& ?a ?b) ?c)"),
        rewrite!("Associative bitwise or";"(| ?a (| ?b ?c))"=>"(| (| ?a ?b) ?c)"),
        rewrite!("Associative bitwise xor";"(^ ?a (^ ?b ?c))"=>"(^ (^ ?a ?b) ?c)"),
        rewrite!("Distributive bitwise and over or";"(& ?a (| ?b ?c))"=>"(| (& ?a ?b) (& ?a ?c))"),
        rewrite!("Distributive bitwise and over or-rev";"(| (& ?a ?b) (& ?a ?c))"=>"(& ?a (| ?b ?c))"),
        rewrite!("Distributive bitwise or over and";"(| ?a (& ?b ?c))"=>"(& (| ?a ?b) (| ?a ?c))"),
        rewrite!("Distributive bitwise or over and-rev";"(& (| ?a ?b) (| ?a ?c))"=>"(| ?a (& ?b ?c))"),
        rewrite!("Absorption bitwise and";"(& ?a (| ?a ?b))"=>"?a"),
        rewrite!("Absorption bitwise or";"(| ?a (& ?a ?b))"=>"?a"),
        rewrite!("De Morgan's law bitwise and";"(~ (& ?a ?b))"=>"(| (~ ?a) (~ ?b))"),
        rewrite!("De Morgan's law bitwise and-rev";"(| (~ ?a) (~ ?b))"=>"(~ (& ?a ?b))"),
        rewrite!("De Morgan's law bitwise or";"(~ (| ?a ?b))"=>"(& (~ ?a) (~ ?b))"),
        rewrite!("De Morgan's law bitwise or-rev";"(& (~ ?a) (~ ?b))"=>"(~ (| ?a ?b))"),
        rewrite!("Bitwise negation of negation";"(~ (~ ?a))"=>"?a"),

        // Control flow rules
        // If rules
        rewrite!("If always true";"(if true ?a ?b)"=>"?a"),
        rewrite!("If always false";"(if false ?a ?b)"=>"?b"),
        rewrite!("If with same branches";"(if ?a ?b ?b)"=>"?b"),
        // While loop rules
        rewrite!("While true";"(while true ?a)"=>"?a"),
        rewrite!("While false";"(while false ?a)"=>"(ignore)"),
        // Do-while rules
        rewrite!("Do-while true";"(do-while true ?a)"=>"?a"),
        rewrite!("Do-while false";"(do-while false ?a)"=>"(ignore)"),
        // For loop rules
        rewrite!("For loop with no condition";"(for ?a (ignore) ?b ?c)"=>"?c"),
        rewrite!("For loop with declaration and empty body";"(for (declaration ?a ?b) ?c ?d (ignore))"=>"(ignore)"),
        // Switch rules
        rewrite!("Switch with no cases";"(switch ?a (ignore))"=>"(ignore)"),
        
        // Misc rules
        // Equality rules
        rewrite!("Equality of self";"(== ?a ?a)"=>"1"),
        rewrite!("Inequality of self";"(!= ?a ?a)"=>"0"),
        // Dereference
        rewrite!("Dereference of address";"(* (& ?a))"=>"?a"),
    ];
}

pub struct CCostFunction;
impl LpCostFunction<C, ()> for CCostFunction {
    fn node_cost(&mut self, _: &egg::EGraph<C, ()>, _: Id, enode: &C) -> f64 {
        match enode {
            C::Ignore => 0.0,
            _ => 1.0
        }
    }
}