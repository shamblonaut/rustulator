use std::io::Write;
use std::str::FromStr;

// 2 + 3 * -2 - 8 ^ 3 / (-4 - 6 * 4 / 2)

#[derive(Debug)]
struct Expression {
    value: Vec<String>,
    
    negative: bool,
    inverse: bool,
}

impl Expression {
    fn parse(&self) -> f64 {
        self.parse_as_layer(&self.value)
    }

    fn parse_as_layer(&self, value: &Vec<String>) -> f64 {
        let mut sign_active: bool = true; let mut external_sign_active: bool = true; // TODO: Check if this breaks with false

        let mut bracket_active: bool = false;
        let mut brackets: i32 = 0;

        let mut child_value_negative: bool = false;
        
        let mut tokens: Vec<String> = Vec::new();
        let mut children: Vec<Expression> = Vec::new();

        for (i, token) in value.iter().enumerate() {
            if token == "(" {
                bracket_active = true;
                tokens.push(token.to_string());
                brackets += 1;
                continue;
            }
            else if token == ")" {
                brackets -= 1;
                if brackets == 0 {
                    bracket_active = false;
                }
            }

            if bracket_active {
                tokens.push(token.to_string());
                continue;
            }

            if token == "*" || token == "/" || token == "^" {
                external_sign_active = true;
                tokens.push(token.to_string());
                continue;
            }

            if token == "+" {
                if sign_active || external_sign_active {
                    continue;
                }

                let child = Expression {
                    value: tokens.clone(),
                    negative: child_value_negative,
                    inverse: false,
                };
                children.push(child);
                
                sign_active = true;
                child_value_negative = false;

                tokens = Vec::new();
            }
            else if token == "-" {
                if external_sign_active {
                    tokens.push(token.to_string());
                    continue;
                }
                if sign_active {
                    child_value_negative = !child_value_negative;
                    continue;
                }

                let child = Expression {
                    value: tokens.clone(),
                    negative: child_value_negative,
                    inverse: false,
                };
                children.push(child);

                sign_active = true;
                child_value_negative = true;

                tokens = Vec::new();
            }
            else if i == value.len() - 1 {
                tokens.push(token.to_string());

                let child = Expression {
                    value: tokens.clone(),
                    negative: child_value_negative,
                    inverse: false,
                };
                children.push(child);
            }
            else {
                sign_active = false;
                external_sign_active = false;
                tokens.push(token.to_string());
            }
        }
        
        let mut sum: f64 = 0.0;
        for child in children {
            let mut result: f64;
            
            if child.value.len() == 1 {
                result = f64::from_str(&child.value[0]).expect("Could not parse value");
            }
            else {
                result = self.parse_dm_layer(child.value);
            }

            if child.negative {
                result = -result;
            }

            sum += result;
        }

        sum
    }

    fn parse_dm_layer(&self, value: Vec<String>) -> f64 {
        let mut external_sign_active: bool = false;

        let mut bracket_active: bool = false;
        let mut brackets: i32 = 0;

        let mut child_value_negative: bool = false;
        let mut child_value_inverse: bool = false;

        let mut tokens: Vec<String> = Vec::new();
        let mut children: Vec<Expression> = Vec::new();

        for (i, token) in value.iter().enumerate() {
            if token == "(" {
                bracket_active = true;
                tokens.push(token.to_string());
                brackets += 1;
                continue;
            }
            else if token == ")" {
                brackets -= 1;
                if brackets == 0 {
                    bracket_active = false;
                }
            }

            if bracket_active {
                tokens.push(token.to_string());
                continue;
            }

            if token == "^" {
                external_sign_active = true;
                tokens.push(token.to_string());
                continue;
            }

            if token == "-" {
                if external_sign_active {
                    tokens.push(token.to_string());
                }
                else {
                    child_value_negative = !child_value_negative;
                }
                continue;
            }

            if token == "*" || token == "/" {
                let child = Expression {
                    value: tokens.clone(),
                    negative: child_value_negative,
                    inverse: child_value_inverse,
                };
                children.push(child);

                child_value_negative = false;
                child_value_inverse = token == "/";
                external_sign_active = false;

                tokens = Vec::new();
            }
            else if i == value.len() - 1 {
                tokens.push(token.to_string());

                let child = Expression {
                    value: tokens.clone(),
                    negative: child_value_negative,
                    inverse: child_value_inverse,
                };
                children.push(child);
            }
            else {
                tokens.push(token.to_string());

                external_sign_active = false;
            }
        }

        let mut product: f64 = 1.0;
        for child in children {
            let mut result: f64;

            if child.value.len() == 1 {
                result = f64::from_str(&child.value[0]).expect("Could not parse value");
            }
            else {
                result = self.parse_exponent_layer(child.value);
            }

            if child.negative {
                result = -result;
            }
            if child.inverse {
                result = 1.0 / result;
            }

            product *= result;
        }

        product
    }

    fn parse_exponent_layer(&self, value: Vec<String>) -> f64 {
        let mut bracket_active: bool = false;
        let mut brackets: i32 = 0;

        let mut child_value_negative: bool = false;

        let mut tokens: Vec<String> = Vec::new();
        let mut children: Vec<Expression> = Vec::new();

        for (i, token) in value.iter().enumerate() {
            if token == "(" {
                bracket_active = true;
                tokens.push(token.to_string());
                brackets += 1;
                continue;
            }
            else if token == ")" {
                brackets -= 1;
                if brackets == 0 {
                    bracket_active = false;
                }
            }

            if bracket_active {
                tokens.push(token.to_string());
                continue;
            }

            if token == "-" {
                child_value_negative = !child_value_negative;
                continue;
            }

            if token == "^" {
                let child = Expression {
                    value: tokens.clone(),
                    negative: child_value_negative,
                    inverse: false,
                };
                children.push(child);

                child_value_negative = false;

                tokens = Vec::new();
            }
            else if i == value.len() - 1 {
                tokens.push(token.to_string());

                let child = Expression {
                    value: tokens.clone(),
                    negative: child_value_negative,
                    inverse: false,
                };
                children.push(child);
            }
            else {
                tokens.push(token.to_string());
            }
        }

        let mut product: f64 = 1.0;
        for (i, child) in children.iter().enumerate() {
            let mut result: f64;

            if child.value.len() == 1 {
                result = f64::from_str(&child.value[0]).expect("Could not parse value");
            }
            else {
                result = self.parse_bracket_layer(child.value.clone());
            }

            if child.negative {
                result = -result;
            }

            if i == 0 {
                product = result;
            }
            else {
                product = product.powf(result);
            }
        }

        product
    }

    fn parse_bracket_layer(&self, value: Vec<String>) -> f64 {
        let mut expression = value.clone();
        expression.remove(0);
        expression.remove(expression.len() - 1);

        self.parse_as_layer(&expression)
    }
}

fn main() {
    loop {
        let e = accept_expression();
        
        if e == "q" || e == "quit" {
            break;
        }

        let tokens = tokenize_expression(&e);
       
        if !tokens.is_empty() {
            let expression = Expression {
                value: tokens,
                negative: false,
                inverse: false,
            };
            println!("{}", expression.parse());
        }
    }
}

fn accept_expression() -> String {
    print!(">> ");
    std::io::stdout().flush().expect("Unable to flush stdout");

    let mut expression = String::new();
    std::io::stdin()
        .read_line(&mut expression)
        .expect("Could not read line");

    expression.trim().to_string()
}

fn tokenize_expression(expression: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    let mut cursor: usize = 0;
    let mut sign_active: bool = false;
    for (i, character) in expression.chars().enumerate() {
        if  character == '+' || 
            character == '-' ||
            character == '*' ||
            character == '/' ||
            character == '^' ||
            character == '(' ||
            character == ')'
        {
            if !sign_active {
                let expression: String = expression[cursor..i].trim().to_string();
                if !expression.is_empty() {
                    tokens.push(expression);
                }
            }
            tokens.push(character.to_string());

            cursor = i + 1;
            sign_active = true;
        }
        else if !character.is_whitespace() {
            sign_active = false;
        }

        if i == expression.len() - 1 {
            let expression: String = expression[cursor..expression.len()].trim().to_string();
            if !expression.is_empty() {
                tokens.push(expression);
            }
        }
    }

    // println!("{:?}", tokens);

    let mut syntax_check_pass: bool = true;
    for token in &tokens {
        if  token == "+" || 
            token == "-" ||
            token == "*" ||
            token == "/" ||
            token == "^" ||
            token == "(" ||
            token == ")"
        {
            continue;
        }

        match token.parse::<f64>() {
            Ok(_) => (),
            Err(_) => {
                syntax_check_pass = false;
                eprintln!("Invalid expression");
                break;
            },
        }
    }
    if !syntax_check_pass {
        tokens = Vec::new()
    }

    tokens
}
