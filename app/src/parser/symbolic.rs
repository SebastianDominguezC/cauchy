use num_complex::Complex;
use std::collections::HashMap;
use std::f32::consts::E;
use std::f32::consts::PI;
enum Precedence {
    Higher,
    Lower,
    Equal,
}

#[derive(Debug)]
struct OperationStack {
    operations: Vec<String>,
}

impl OperationStack {
    fn new() -> OperationStack {
        OperationStack {
            operations: Vec::new(),
        }
    }
    fn add(&mut self, c: String) {
        self.operations.insert(0, c);
    }
    fn pop(&mut self) -> String {
        self.operations.remove(0)
    }
    fn top(&self) -> Option<&String> {
        self.operations.get(0)
    }
    fn is_empty(&self) -> bool {
        self.operations.len() == 0
    }
}

fn precedence(operator: &String) -> u8 {
    if operator == "^" {
        return 3;
    } else if operator == "*" || operator == "/" {
        return 2;
    } else if operator == "+" || operator == "-" {
        return 1;
    } else if is_function(operator) {
        return 3;
    }
    0
}

fn compare_precedence(current: &String, top: &String) -> Precedence {
    let current = precedence(current);
    let top = precedence(top);
    if current == top {
        return Precedence::Equal;
    } else if current < top {
        return Precedence::Lower;
    } else if current > top {
        return Precedence::Higher;
    } else {
        return Precedence::Equal;
    }
}

pub fn shunting_yard(operation: String) -> Vec<String> {
    let mut stack = OperationStack::new();
    let mut print = vec![];

    let operation = operation.replace(" ", "");
    if !operation.contains("+")
        && !operation.contains("-")
        && !operation.contains("*")
        && !operation.contains("/")
        && !operation.contains("^")
        && !operation.contains("sin")
        && !operation.contains("cos")
        && !operation.contains("tan")
        && !operation.contains("csc")
        && !operation.contains("sec")
        && !operation.contains("cot")
        && !operation.contains("asin")
        && !operation.contains("acos")
        && !operation.contains("atan")
        && !operation.contains("sinh")
        && !operation.contains("cosh")
        && !operation.contains("tanh")
        && !operation.contains("asinh")
        && !operation.contains("acosh")
        && !operation.contains("atanh")
        && !operation.contains("inv")
        && !operation.contains("conj")
        && !operation.contains("exp")
        && !operation.contains("ln")
        && !operation.contains("sqrt")
        && !operation.contains("cbrt")
    {
        return Vec::new();
    }

    // Split by variables with n length
    let mut op_split = vec![];
    let mut temporary_index = 0;
    let mut last = String::new();
    for i in 0..operation.chars().count() {
        let current = &operation[i..i + 1];
        if is_operation(&current.to_string())
            || current == "("
            || current == ")"
        {
            if temporary_index != i {
                op_split.push(String::from(&operation[temporary_index..i]));
            }
            op_split.push(String::from(current));
            temporary_index = i + 1;
            last = String::from("");
        } else {
            last = String::from(&operation[temporary_index..]);
        }
    }
    if last != "" {
        op_split.push(last);
    }

    for c in op_split.iter() {
        // Left parenthesis -> push stack
        if c == "(" {
            stack.add(c.clone());
            continue;
        }
        // Right parenthesis -> discard, print stack until left parenthesis
        if c == ")" {
            let mut op = stack.pop();
            while op != "(" {
                print.push(op);
                op = stack.pop();
            }
            continue;
        }
        // Operator
        if is_operation(c) {
            // empy stack or on top is  ( -> push on stack
            if stack.is_empty() {
                stack.add(c.clone());
                continue;
            }

            // Top MUST exist at this point - unwrap
            let top = stack.top().unwrap();
            if top == &"(" {
                stack.add(c.clone());
                continue;
            }

            let compare = compare_precedence(c, top);
            match compare {
                // higher precedence than top -> push
                Precedence::Higher => {
                    stack.add(c.clone());
                    continue;
                }
                // lower precedence than top -> pop stack and print until it is not true, then push operator
                // or same precedence with left association
                _ => {
                    print.push(stack.pop());
                    while let Some(top) = stack.top() {
                        if let Precedence::Lower = compare_precedence(c, top) {
                            print.push(stack.pop());
                        } else if let Precedence::Lower =
                            compare_precedence(c, top)
                        {
                            print.push(stack.pop());
                        } else {
                            break;
                        }
                    }
                    stack.add(c.clone());
                    continue;
                }
            }
        }
        // any other case, just print the char
        print.push(c.clone());
    }
    for op in stack.operations {
        print.push(op);
    }
    print
}

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz ";

pub fn num_to_letter_vec(n: usize) -> Vec<String> {
    let letters = 25;
    if n > letters {
        let mut n = n;
        let mut count: i32 = -1;
        while n > letters {
            n -= letters;
            count += 1;
        }
        let mut last = num_to_letter_vec(n);
        let mut rest = num_to_letter_vec(count as usize);
        rest.append(&mut last);
        return rest;
    } else {
        return vec![String::from(&LETTERS[n..n + 1])];
    }
}

pub fn parse_complex_calculation(
    mut operations: Vec<String>,
) -> Option<Complex<f32>> {
    let mut cache_counter = 0;
    let mut cache_store: HashMap<String, Complex<f32>> = HashMap::new();
    let mut answer = Complex::new(0.0, 0.0);

    while operations.len() > 1 {
        if operations.len() == 2 {
            let first = &operations[0];
            let second = &operations[1];
            if !is_function(second) {
                break;
            }
            let first = convert_to_complex(first, &cache_store);
            let z1;
            match first {
                Some(v) => z1 = v,
                None => {
                    z1 = Complex::new(0.0, 0.0);
                }
            }
            let res = evaulate_function(second, z1);
            answer = res;
            break;
        }
        for i in 0..operations.len() {
            let first = &operations[i];
            let second = &operations[i + 1];

            // If second is function, run function
            if is_function(second) {
                let first = convert_to_complex(first, &cache_store);
                let z1;
                match first {
                    Some(v) => z1 = v,
                    None => {
                        z1 = Complex::new(0.0, 0.0);
                    }
                }
                let res = evaulate_function(second, z1);
                let cache = format!("cache{}", cache_counter);
                cache_counter += 1;

                cache_store.insert(cache.clone(), res);
                operations.insert(i + 2, cache);
                operations = operations
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| {
                        // filter first 2 indices
                        j != &(i) && j != &(i + 1)
                    })
                    .map(|(_, v)| v.clone())
                    .collect();
                break;
            }

            let third = &operations[i + 2];
            if !is_operation(second) && is_function(third) {
                let second = convert_to_complex(second, &cache_store);
                let z1;
                match second {
                    Some(v) => z1 = v,
                    None => {
                        z1 = Complex::new(0.0, 0.0);
                    }
                }
                let res = evaulate_function(third, z1);
                let cache = format!("cache{}", cache_counter);
                cache_counter += 1;

                cache_store.insert(cache.clone(), res);
                operations.insert(i + 1, cache);
                operations = operations
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| {
                        // filter index 2 and 3
                        j != &(i + 2) && j != &(i + 3)
                    })
                    .map(|(_, v)| v.clone())
                    .collect();
                break;
            }
            if !is_operation(second) && !is_operation(third) {
                continue;
            }
            let first = convert_to_complex(first, &cache_store);
            let second = convert_to_complex(second, &cache_store);
            let z1;
            let z2;
            match first {
                Some(v) => z1 = v,
                None => {
                    z1 = Complex::new(0.0, 0.0);
                }
            }
            match second {
                Some(v) => z2 = v,
                None => {
                    z2 = Complex::new(0.0, 0.0);
                }
            }
            let res = evaluate_operation(third, z1, z2);
            let cache = format!("cache{}", cache_counter);
            cache_counter += 1;

            cache_store.insert(cache.clone(), res);
            operations.insert(i + 3, cache);
            operations = operations
                .iter()
                .enumerate()
                .filter(|(j, _)| {
                    // filter current 3 indices
                    j != &i && j != &(i + 1) && j != &(i + 2)
                })
                .map(|(_, v)| v.clone())
                .collect();
            answer = res;

            break;
        }
    }
    if operations.len() != 0 {
        return Some(answer);
    }
    None
}

fn evaluate_operation(
    op: &String,
    z1: Complex<f32>,
    z2: Complex<f32>,
) -> Complex<f32> {
    let res: Complex<f32>;
    if op == "+" {
        res = z1 + z2;
    } else if op == "-" {
        res = z1 - z2;
    } else if op == "*" {
        res = z1 * z2;
    } else if op == "/" {
        res = z1 / z2;
    } else if op == "^" {
        res = z1.powc(z2);
    } else {
        res = Complex::new(0.0, 0.0);
    }
    res
}

fn evaulate_function(fx: &String, z: Complex<f32>) -> Complex<f32> {
    let res: Complex<f32>;
    if fx == "cos" {
        res = z.cos();
    } else if fx == "sin" {
        res = z.sin();
    } else if fx == "tan" {
        res = z.tan();
    } else if fx == "csc" {
        res = 1.0 / z.sin();
    } else if fx == "sec" {
        res = 1.0 / z.cos();
    } else if fx == "cot" {
        res = 1.0 / z.tan();
    } else if fx == "asin" {
        res = z.asin();
    } else if fx == "acos" {
        res = z.acos();
    } else if fx == "atan" {
        res = z.atan();
    } else if fx == "sinh" {
        res = z.sinh();
    } else if fx == "cosh" {
        res = z.cosh();
    } else if fx == "tanh" {
        res = z.tanh();
    } else if fx == "asinh" {
        res = z.asinh();
    } else if fx == "acosh" {
        res = z.acosh();
    } else if fx == "atanh" {
        res = z.atanh();
    } else if fx == "inv" {
        res = z.inv();
    } else if fx == "conj" {
        res = z.conj();
    } else if fx == "exp" {
        res = z.exp();
    } else if fx == "sqrt" {
        res = z.sqrt();
    } else if fx == "cbrt" {
        res = z.cbrt();
    } else if fx == "ln" {
        res = z.ln();
    } else {
        res = Complex::new(0.0, 0.0);
    }
    res
}

fn convert_to_complex(
    s: &String,
    cache: &HashMap<String, Complex<f32>>,
) -> Option<Complex<f32>> {
    if s.starts_with("cache") {
        if let Some(z) = cache.get(s) {
            return Some(*z);
        } else {
            return None;
        }
    }

    let s = insert_constants(s);
    if s.contains("i") {
        let mut s = s.replace("i", "");
        if s == "" {
            s = "1".to_string();
        }
        if let Ok(n) = s.parse::<f32>() {
            let z = Complex::new(0.0, n);
            Some(z)
        } else {
            None
        }
    } else {
        if let Ok(n) = s.parse::<f32>() {
            let z = Complex::new(n, 0.0);
            Some(z)
        } else {
            None
        }
    }
}

fn insert_constants(s: &str) -> String {
    if s.contains("PI") {
        let s = s.replace("PI", &PI.to_string());
        return s;
    } else if s.contains("E") {
        let s = s.replace("E", &E.to_string());
        return s;
    }
    s.to_string()
}

fn is_operation(s: &String) -> bool {
    s == "+" || s == "-" || s == "*" || s == "/" || s == "^" || is_function(s)
}
fn is_function(s: &String) -> bool {
    s == "sin"
        || s == "cos"
        || s == "tan"
        || s == "csc"
        || s == "sec"
        || s == "cot"
        || s == "asin"
        || s == "acos"
        || s == "atan"
        || s == "sinh"
        || s == "cosh"
        || s == "tanh"
        || s == "asinh"
        || s == "acosh"
        || s == "atanh"
        || s == "inv"
        || s == "conj"
        || s == "exp"
        || s == "ln"
        || s == "sqrt"
        || s == "cbrt"
}

pub fn test_ops() {
    let operation = String::from("AA");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("AB * BC");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("AA * BB + CC");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("AA + BB * CC");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("AA * (BB+CCC)");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("A * BB^ C + DD");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("AA * (B + C * DDDD) + EB");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("12 * 32 * (14 / 2 + i20) + i");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("tan(z)");
    let res = shunting_yard(operation);
    println!("{:?}", res);
    let operation = String::from("2 * sin(x + 1)");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("2 * sin((z^2 + 2i)/2z)");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    let operation = String::from("tan(sin(cos(1 / i + 2 + i)))");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    println!("{:?}", parse_complex_calculation(res));

    let operation = String::from("cos(sin(i^2))");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    println!("{:?}", parse_complex_calculation(res));

    let operation = String::from("12 * 32 * (14 / 2 + 20i) + i");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    println!("{:?}", parse_complex_calculation(res));

    let operation = String::from("(2i+1)^2");
    let res = shunting_yard(operation);
    println!("{:?}", res);

    println!("{:?}", parse_complex_calculation(res));
}
