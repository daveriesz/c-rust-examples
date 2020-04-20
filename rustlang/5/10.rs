use std::env;

const MAXOP : usize = 100;    /// Max size of operand or operator
const EOF : char = '#';       /// Simulate C EOF
const NUMBER : char = '0';    /// Signal that a number was found
const LAST : char = 'L';      /// signal that last value command was found
const SWAP : char = '~';      /// signal that swap command was found
const DUPLICATE : char = 'D'; /// signal that duplicate command was found
const CLEAR : char = 'C';     /// signal that clear was found

const FAKE_DOUBLE_MIN : f32 = -999999.0; /// set a minimum value for stack operands

const MAXVAL : usize = 100;             /// Maximum depth of operand stack
static mut VAL : Vec<f32> = Vec::new(); /// Our operand stack

/// reverse Polish calculator
fn main()
{
    let mut _type : char;
    let mut op2 : f32;
    let mut aux : f32;
    let mut _lp : f32 = 0.0;
    let mut _last_read_variable : usize = 0;

    let argv: Vec<String> = env::args().collect();

    let mut idx = 1;
    let argc = argv.len();
    if argc < 2 {
        panic!("Usage: {} <EXPRESSION>", &argv[0]);
    }

    let mut s = String::new();
    s.reserve(MAXOP);

    let mut variables : Vec<f32> = Vec::new(); // keeps track all variable values

    // init variables value
    for _i in 0..26 {
        variables.push(FAKE_DOUBLE_MIN);
    }

    // We need a unsafe scope here, because we create global (static) variables
    // and manipulate them here. Rust makes a big effort to avoid concurrency
    // bugs hard to run into, that's the reason for code that doesn't guarantee
    // data race protection being labeled unsafe.

    // SOURCES:
    // https://doc.rust-lang.org/reference/items/static-items.html
    unsafe {
        while idx < argc {
            _type = getop(&mut s, &argv[idx]);
            idx += 1;

            if _type == EOF {
                break;
            }

            match _type {
                NUMBER => { push(s.parse().unwrap_or(-0.0)); },
                '+' => { push(pop() + pop()); },
                '*' => { push(pop() * pop()); },
                '-' => {
                    op2 = pop();
                    push(pop() - op2);
                },
                '/' => {
                    op2 = pop();
                    if op2 != 0.0 {
                        push(pop() / op2);
                    } else {
                        print!("error: zero divisor\n");
                    }
                },
                '%' => {
                    op2 = pop();
                    if op2 != 0.0 {
                        push(pop() as f32 % op2 as f32);
                    } else {
                        print!("error: zero divisor\n");
                    }
                },
                'S' => { push( pop().sin() ); },
                'E' => { push( pop().exp() ); },
                '^' => {
                    op2 = pop();
                    aux = pop();
                    if aux == 0.0 && op2 <= 0.0 {
                        print!("error: pow({}, {}) is not a valid operation.\n", aux, op2);
                    } else {
                        push( aux.powf(op2) );
                    }
                },
                LAST => { _lp = last(); print!("top value: {}\n", _lp); },
                DUPLICATE => { duplicate(); },
                SWAP => { swap(); },
                CLEAR => { clear(); },
                '=' => { variables[_last_read_variable] = pop(); },
                '_' => { push(_lp); },
                '?' => {
                    for _i in 0..26 {
                        if variables[_i] != FAKE_DOUBLE_MIN {
                            print!("'{}' == {}\n", std::char::from_u32('a' as u32 + (_i as u32)).unwrap_or('#'), variables[_i]);
                        }
                    }
                },
                _ => {
                    if _type.is_ascii_lowercase() {
                        _last_read_variable = (_type as u32 - 'a' as u32) as usize;
                        //if variable had a value assigned before, push it to stack
                        if variables[_last_read_variable] != FAKE_DOUBLE_MIN {
                            push(variables[_last_read_variable]);
                        } else {
                            // if variable wasn't used before, initialize it to 0
                            variables[_last_read_variable] = 0.0;
                        }
                    } else {
                        print!("error: unknown command {}\n", s);
                    }
                }
            }
        }

       _lp = pop();
       print!("\t{}\n", _lp);

    }
}

/// push:  push f onto value stack
/// It is unsafe because we manipulate a static mut variable inside it.
unsafe fn push(f: f32)
{
    if VAL.len() < MAXVAL {
        VAL.push(f);
    } else {
        print!("error: stack full, can't push {}\n", f);
    }
}

/// pop:  pop and return top value from stack
/// It is unsafe because we manipulate a static mut variable inside it.
unsafe fn pop() -> f32
{
    if VAL.len() > 0 {
        return VAL.pop().unwrap_or(0.0);
    } else {
        print!("error: stack empty\n");
        return 0.0;
    }
}

/* return top value from stack without popping */
unsafe fn last() -> f32 {
    if VAL.len() > 0 {
        return *VAL.last().unwrap_or(&0.0);
    } else {
        print!("error: stack empty\n");
        return 0.0;
    }
}

/* duplicate top value from stack */
unsafe fn duplicate() {
    if VAL.len() > 0 {
        push(last());
    } else {
        print!("error: stack empty\n");
    }
}

/* swap top two values from stack */
unsafe fn swap() {
    let aux1 : f32;
    let aux2 : f32;

    if VAL.len() >= 2 {
        aux1 = pop();
        aux2 = pop();
        push(aux1);
        push(aux2);
    } else {
        print!("error: can't swap with {} elements\n", VAL.len());
    }
}

/* clear all stack */
unsafe fn clear() {
    print!("stack cleared\n");
    VAL.clear();
}

/// getop:  get next character or numeric operand
/// It is unsafe because we manipulate a static mut variable inside it.
unsafe fn getop(s: &mut String, next: &String) -> char
{
    let mut c: char;
    let mut getchar = next.chars();

    s.clear();
    loop {
        c = getchar.next().unwrap_or(EOF);

        if c != ' ' && c != '\t' {
            break;
        }
    }

    s.push(c);

    if !c.is_digit(10) && c != '.' {
        return c; // not a number
    }

    if c.is_digit(10) { // collect integer part
        loop {
            c = getchar.next().unwrap_or(EOF);

            if c.is_digit(10) {
                s.push(c);
            } else {
                break;
            }
        }
    }

    if c == '.' { // collect fraction part
        loop {
            c = getchar.next().unwrap_or(EOF);

            if c.is_digit(10) {
                s.push(c);
            } else {
                break;
            }
        }
    }

    return NUMBER;
}
