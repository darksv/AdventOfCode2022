use std::cell::RefCell;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
enum Value {
    Const(u64),
    Old,
}

#[derive(Debug, Clone)]
struct Expr {
    lhs: Value,
    op: Operator,
    rhs: Value,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: RefCell<Vec<u64>>,
    expr: Expr,
    div_test: u64,
    next_if_true: usize,
    next_if_false: usize,
}

fn parse_value(v: &str) -> Value {
    match v {
        "old" => Value::Old,
        other => Value::Const(other.parse().unwrap()),
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for chunk in input.split("\r\n\r\n") {
        let mut items = None;
        let mut expr = None;
        let mut div_test = None;
        let mut if_true = None;
        let mut if_false = None;

        for line in chunk.lines() {
            if let Some(val) = line.strip_prefix("  Starting items: ") {
                items = Some(RefCell::new(val.split(", ").map(|it| it.parse().unwrap()).collect()));
            } else if let Some(op) = line.strip_prefix("  Operation: new = ") {
                let [lhs, op, rhs] = op.split(" ").array_chunks().next().unwrap();
                let lhs = parse_value(lhs);
                let rhs = parse_value(rhs);
                let op = match op {
                    "+" => Operator::Add,
                    "*" => Operator::Mul,
                    _ => unimplemented!(),
                };

                expr = Some(Expr { lhs, op, rhs });
            } else if let Some(val) = line.strip_prefix("  Test: divisible by ") {
                div_test = val.parse().ok();
            } else if let Some(val) = line.strip_prefix("    If true: throw to monkey ") {
                if_true = val.parse().ok();
            } else if let Some(val) = line.strip_prefix("    If false: throw to monkey ") {
                if_false = val.parse().ok();
            } else if let Some(_) = line.strip_prefix("Monkey ") {
                //
            } else {
                unimplemented!("{}", line);
            }
        }

        monkeys.push(Monkey {
            items: items.unwrap(),
            expr: expr.unwrap(),
            div_test: div_test.unwrap(),
            next_if_true: if_true.unwrap(),
            next_if_false: if_false.unwrap(),
        })
    }

    monkeys
}

fn eval(e: &Expr, old_val: u64) -> u64 {
    let lhs = match e.lhs {
        Value::Const(c) => c,
        Value::Old => old_val,
    };
    let rhs = match e.rhs {
        Value::Const(c) => c,
        Value::Old => old_val,
    };
    match e.op {
        Operator::Add => lhs + rhs,
        Operator::Mul => lhs * rhs,
    }
}

fn simulate(monkeys: Vec<Monkey>, div_by: u64, n: usize) -> usize {
    let mut inspection = vec![0; monkeys.len()];

    let product: u64 = monkeys.iter().map(|it| it.div_test).product();

    for _ in 0..n {
        for (idx, monkey) in monkeys.iter().enumerate() {
            let mut items = monkey.items.borrow_mut();
            for item in items.drain(..) {
                inspection[idx] += 1;
                let new_worry = eval(&monkey.expr, item);
                let val = (new_worry / div_by) % product;
                let next = if val % monkey.div_test == 0 {
                    monkey.next_if_true
                } else {
                    monkey.next_if_false
                };
                monkeys[next].items.borrow_mut().push(val);
            }
        }
    }

    inspection.sort();
    inspection.iter().rev().take(2).product()
}

pub(crate) fn day_11(input: &str) -> (usize, usize) {
    let monkeys = parse_input(input);
    (
        simulate(monkeys.clone(), 3, 20),
        simulate(monkeys.clone(), 1, 10_000),
    )
}