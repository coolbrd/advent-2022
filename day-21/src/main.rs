use std::{fs, collections::HashMap, ops::{Mul, Div, Sub, Add, Neg}};

type MonkeyBusiness<'a> = HashMap<MonkeyName<'a>, MonkeyAction<'a>>;

type MonkeyName<'a> = &'a str;

#[derive(Debug)]
enum MonkeyAction<'a> {
    YellValue(MonkeyValue),
    DoOperation(MonkeyOperation<'a>)
}

type MonkeyValue = i128;

#[derive(Debug)]
struct MonkeyOperation<'a> {
    operator: ArithmeticOperator,
    operand_1: MonkeyName<'a>,
    operand_2: MonkeyName<'a>
}

#[derive(Debug, Clone, Copy)]
enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug)]
struct EqualityEquation {
    left_side: ArithmeticExpression,
    right_side: ArithmeticExpression
}

#[derive(Debug, Clone)]
enum ArithmeticExpression {
    Value(MonkeyValue),
    FreeVariable,
    Operation(Box<ArithmeticOperation>)
}

impl Add for ArithmeticExpression {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ArithmeticExpression::Operation(Box::new(ArithmeticOperation {
            operator: ArithmeticOperator::Add,
            operand_1: self,
            operand_2: other
        }))
    }
}

impl Sub for ArithmeticExpression {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ArithmeticExpression::Operation(Box::new(ArithmeticOperation {
            operator: ArithmeticOperator::Subtract,
            operand_1: self,
            operand_2: other
        }))
    }
}

impl Mul for ArithmeticExpression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ArithmeticExpression::Operation(Box::new(ArithmeticOperation {
            operator: ArithmeticOperator::Multiply,
            operand_1: self,
            operand_2: other
        }))
    }
}

impl Div for ArithmeticExpression {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        ArithmeticExpression::Operation(Box::new(ArithmeticOperation {
            operator: ArithmeticOperator::Divide,
            operand_1: self,
            operand_2: other
        }))
    }
}

impl Neg for ArithmeticExpression {
    type Output = Self;

    fn neg(self) -> Self {
        ArithmeticExpression::Operation(Box::new(ArithmeticOperation {
            operator: ArithmeticOperator::Multiply,
            operand_1: ArithmeticExpression::Value(-1),
            operand_2: self
        }))
    }
}

impl ArithmeticExpression {
    fn has_free_variable(&self) -> bool {
        match self {
            ArithmeticExpression::Value(_) => false,
            ArithmeticExpression::FreeVariable => true,
            ArithmeticExpression::Operation(operation) => {
                operation.operand_1.has_free_variable() || operation.operand_2.has_free_variable()
            }
        }
    }

    fn resolve_without_free_variable(&self) -> MonkeyValue {
        match self {
            ArithmeticExpression::Value(val) => *val,
            ArithmeticExpression::FreeVariable => panic!("Free variable in expression!"),
            ArithmeticExpression::Operation(operation) => {
                let operand_1 = operation.operand_1.resolve_without_free_variable();
                let operand_2 = operation.operand_2.resolve_without_free_variable();
                match operation.operator {
                    ArithmeticOperator::Add => operand_1 + operand_2,
                    ArithmeticOperator::Subtract => operand_1 - operand_2,
                    ArithmeticOperator::Multiply => operand_1 * operand_2,
                    ArithmeticOperator::Divide => operand_1 / operand_2
                }
            }
        }
    }

    fn balance_free_variable_with_other(&self, other_expression: ArithmeticExpression) -> ArithmeticExpression {
        match self {
            ArithmeticExpression::Value(_) => panic!("Value at base level of expression found when applying inverse!"),
            ArithmeticExpression::FreeVariable => other_expression,
            ArithmeticExpression::Operation(operation) => {
                let operation_clone = operation.clone();
                let (free_variable_side, constant_side, free_variable_on_left) = if operation.operand_1.has_free_variable() {
                    (operation_clone.operand_1, operation_clone.operand_2, true)
                } else {
                    (operation_clone.operand_2, operation_clone.operand_1, false)
                };
                let wrapped_other = match operation.operator {
                    ArithmeticOperator::Add => other_expression - constant_side,
                    ArithmeticOperator::Subtract => {
                        match free_variable_on_left {
                            true => other_expression + constant_side,
                            false => -(other_expression - constant_side)
                        }
                    },
                    ArithmeticOperator::Multiply => other_expression / constant_side,
                    ArithmeticOperator::Divide => {
                        match free_variable_on_left {
                            true => other_expression * constant_side,
                            false => constant_side / other_expression
                        }
                    }
                };
                free_variable_side.balance_free_variable_with_other(wrapped_other)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ArithmeticOperation {
    operator: ArithmeticOperator,
    operand_1: ArithmeticExpression,
    operand_2: ArithmeticExpression
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let monkey_business = lines.iter().map(|line| {
        let parts = line.split(":").collect::<Vec<&str>>();
        let name = parts[0].trim();
        let action = parts[1].trim();
        let action = if let Ok(val) = action.parse() {
            MonkeyAction::YellValue(val)
        } else {
            let parts = action.split(" ").collect::<Vec<&str>>();
            let operator = match parts[1] {
                "+" => ArithmeticOperator::Add,
                "-" => ArithmeticOperator::Subtract,
                "*" => ArithmeticOperator::Multiply,
                "/" => ArithmeticOperator::Divide,
                _ => panic!("Unknown operator")
            };
            let operation = MonkeyOperation {
                operator,
                operand_1: parts[0],
                operand_2: parts[2]
            };
            MonkeyAction::DoOperation(operation)
        };
        (name, action)
    }).collect::<MonkeyBusiness>();

    // Part 1
    let answer = resolve_monkey_value(&monkey_business, "root");
    println!("Answer: {}", answer);

    // Part 2
    let root_monkey = monkey_business.get("root").unwrap();
    let equality = match root_monkey {
        MonkeyAction::YellValue(_) => panic!("Root monkey is not an operation"),
        MonkeyAction::DoOperation(operation) => {
            let left_side = resolve_monkey_expression(&monkey_business, operation.operand_1);
            let right_side = resolve_monkey_expression(&monkey_business, operation.operand_2);
            EqualityEquation { left_side, right_side }
        }
    };
    let (free_variable_side, constant_side) = if equality.left_side.has_free_variable() {
        (equality.left_side, equality.right_side)
    } else {
        (equality.right_side, equality.left_side)
    };
    let answer = free_variable_side.balance_free_variable_with_other(constant_side).resolve_without_free_variable();
    println!("Answer: {:?}", answer);
}

fn resolve_monkey_value(monkey_business: &MonkeyBusiness, monkey_name: &str) -> MonkeyValue {
    match monkey_business.get(monkey_name) {
        Some(MonkeyAction::YellValue(val)) => *val,
        Some(MonkeyAction::DoOperation(operation)) => {
            let operand_1 = resolve_monkey_value(monkey_business, operation.operand_1);
            let operand_2 = resolve_monkey_value(monkey_business, operation.operand_2);
            match operation.operator {
                ArithmeticOperator::Add => operand_1 + operand_2,
                ArithmeticOperator::Subtract => operand_1 - operand_2,
                ArithmeticOperator::Multiply => operand_1 * operand_2,
                ArithmeticOperator::Divide => operand_1 / operand_2
            }
        },
        None => panic!("Unknown monkey")
    }
}

fn resolve_monkey_expression(monkey_business: &MonkeyBusiness, monkey_name: &str) -> ArithmeticExpression {
    if monkey_name == "humn" {
        return ArithmeticExpression::FreeVariable;
    }
    match monkey_business.get(monkey_name) {
        Some(MonkeyAction::YellValue(val)) => ArithmeticExpression::Value(*val),
        Some(MonkeyAction::DoOperation(operation)) => {
            let operand_1 = resolve_monkey_expression(monkey_business, operation.operand_1);
            let operand_2 = resolve_monkey_expression(monkey_business, operation.operand_2);
            let operation = ArithmeticOperation {
                operator: operation.operator,
                operand_1,
                operand_2
            };
            ArithmeticExpression::Operation(Box::new(operation))
        },
        None => panic!("Unknown monkey")
    }
}
