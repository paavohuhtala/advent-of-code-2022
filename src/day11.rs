use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("./day11.input");

#[derive(Debug)]
enum OperationValue {
    Old,
    Int(i64),
}

#[derive(Debug)]
enum Operator {
    Mul,
    Add,
}

type Expression = (OperationValue, Operator, OperationValue);

fn eval_expression(expr: &Expression, old: i64) -> i64 {
    let (l, op, r) = expr;
    let l = match l {
        OperationValue::Old => old,
        OperationValue::Int(i) => *i,
    };
    let r = match r {
        OperationValue::Old => old,
        OperationValue::Int(i) => *i,
    };
    match op {
        Operator::Mul => l * r,
        Operator::Add => l + r,
    }
}

fn parse_op_value(op: &str) -> OperationValue {
    if op == "old" {
        OperationValue::Old
    } else {
        OperationValue::Int(op.parse().unwrap())
    }
}

fn parse_operation(expr: &str) -> Expression {
    let (_, expr) = expr.split_once(" = ").unwrap();
    if expr.contains("+") {
        let (l, r) = expr.split(" + ").collect_tuple().unwrap();
        (parse_op_value(l), Operator::Add, parse_op_value(r))
    } else {
        // Assume *
        let (l, r) = expr.split(" * ").collect_tuple().unwrap();
        (parse_op_value(l), Operator::Mul, parse_op_value(r))
    }
}

fn parse_starting_items(line: &str) -> Vec<i64> {
    let (_, items) = line.split_once(": ").unwrap();
    items
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse_divisible(line: &str) -> i64 {
    let (_, divisible) = line.split_once("by ").unwrap();
    divisible.parse().unwrap()
}

fn parse_divisible_result(line: &str) -> usize {
    let (_, divisible) = line.split_once("monkey ").unwrap();
    divisible.parse().unwrap()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Expression,
    divisible: i64,
    divisible_if_true: usize,
    divisible_if_false: usize,
}

fn parse_input() -> Vec<Monkey> {
    INPUT
        .split("\n\n")
        .map(|monkey_lines| {
            let mut monkey_lines = monkey_lines.lines();

            // skip monkey ID
            monkey_lines.next().unwrap();
            let starting_items = parse_starting_items(monkey_lines.next().unwrap());
            let operation = parse_operation(monkey_lines.next().unwrap());
            let divisible = parse_divisible(monkey_lines.next().unwrap());
            let divisible_if_true = parse_divisible_result(monkey_lines.next().unwrap());
            let divisible_if_false = parse_divisible_result(monkey_lines.next().unwrap());

            Monkey {
                items: starting_items,
                operation,
                divisible,
                divisible_if_true,
                divisible_if_false,
            }
        })
        .collect()
}

impl Monkey {
    fn process_turn(
        &mut self,
        outbound_items: &mut HashMap<usize, Vec<i64>>,
        divide: bool,
        mod_by: i64,
    ) -> usize {
        if self.items.is_empty() {
            return 0;
        }

        let inspected_items = self.items.len();

        for item in self.items.iter().copied() {
            let mut item = eval_expression(&self.operation, item);

            if divide {
                item /= 3;
            }

            item %= mod_by;

            if item % self.divisible == 0 {
                outbound_items
                    .entry(self.divisible_if_true)
                    .or_default()
                    .push(item);
            } else {
                outbound_items
                    .entry(self.divisible_if_false)
                    .or_default()
                    .push(item);
            }
        }

        self.items.clear();

        inspected_items
    }
}

fn process_round(
    monkeys: &mut [Monkey],
    inspections: &mut HashMap<usize, usize>,
    divide: bool,
    mod_by: i64,
) -> HashMap<usize, Vec<i64>> {
    let mut outbound_items = HashMap::new();

    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let inspected_items = monkey.process_turn(&mut outbound_items, divide, mod_by);

        *inspections.entry(i).or_default() += inspected_items;

        for (monkey, items) in outbound_items.drain() {
            monkeys[monkey].items.extend(items);
        }
    }

    outbound_items
}

fn process_rounds(mut monkeys: Vec<Monkey>, rounds: usize, divide: bool) -> usize {
    let mod_by = monkeys
        .iter()
        .map(|monkey| monkey.divisible)
        .product::<i64>();

    let mut inspections = HashMap::new();

    for _ in 0..rounds {
        process_round(&mut monkeys, &mut inspections, divide, mod_by);
    }

    inspections
        .values()
        .copied()
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

pub fn day11a() {
    let monkeys = parse_input();
    let monkey_business = process_rounds(monkeys, 20, true);
    println!("Day 11a: {}", monkey_business);
}

pub fn day11b() {
    let monkeys = parse_input();
    let monkey_business = process_rounds(monkeys, 10_000, false);
    println!("Day 11b: {}", monkey_business);
}
