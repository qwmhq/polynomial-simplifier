use std::{cmp::Ordering, collections::HashMap};

pub fn split_monomial(monomial: &str) -> (i32, String) {
    let mut buffer = String::new();

    let mut monomial_iter = monomial.chars().enumerate();
    let coeff_is_negative = &monomial[0..1] == "-";
    let mut first_variable_char: Option<char> = None;

    // push the coefficient to the buffer
    while let Some((idx, c)) = monomial_iter.next() {
        if idx == 0 && (c == '+' || c == '-') {
            continue;
        }
        if c.is_numeric() {
            buffer.push(c);
        } else {
            first_variable_char = Some(c);
            break;
        }
    }

    // parse the coefficient into an integer
    let coeff: i32 = buffer.parse().unwrap_or(1);
    let coeff = match coeff_is_negative {
        true => coeff * -1,
        false => coeff,
    };

    buffer.clear();

    // push the variables to the buffer
    if first_variable_char.is_some() {
        buffer.push(first_variable_char.unwrap());
    }
    for (_idx, c) in monomial_iter {
        buffer.push(c);
    }

    //sort the variables in alphabetical order
    let buffer_slice = &buffer[..];
    let mut buffer_chars: Vec<char> = buffer_slice.chars().collect();
    buffer_chars.sort_by(|a, b| a.cmp(b));

    (coeff, String::from_iter(buffer_chars))
}

pub fn simplify(polynomial: &str) -> String {
    let mut a = 0;
    let mut monomial_map: HashMap<String, i32> = HashMap::new();

    for (i, c) in polynomial.chars().enumerate() {
        let monomial_slice: &str;
        if i != 0 && (c == '-' || c == '+') {
            monomial_slice = &polynomial[a..i];
            a = i;
        } else if i == polynomial.len() - 1 {
            monomial_slice = &polynomial[a..];
        } else {
            continue;
        }
        let (coeff, variables) = split_monomial(monomial_slice);
        let value = monomial_map.entry(variables).or_insert(0);
        *value += coeff;
    }

    let mut result = String::new();

    // sort variables
    let mut variables = monomial_map.keys().collect::<Vec<&String>>();
    variables.sort_by(|a, b| {
        if a.len() != b.len() {
            if &a[..] == "" {
                return Ordering::Greater;
            } else if &b[..] == "" {
                return Ordering::Less;
            } else {
                return a.len().cmp(&b.len());
            }
        }
        return a.cmp(&b);
    });

    // combine the coefficient and variables into monomials and append to the result string
    let mut middle = false;
    for var in variables.into_iter() {
        let coeff = monomial_map.get(var).unwrap();
        match *coeff {
            0 => continue,
            -1 => {
                result.push('-');
                if var.is_empty() {
                    result.push('1');
                }
            }
            1 => {
                if middle {
                    result.push('+');
                }
                if var.is_empty() {
                    result.push('1');
                }
            }
            x => {
                if middle && x.is_positive() {
                    result.push('+');
                }
                result.push_str(coeff.to_string().as_str());
            }
        }
        result.push_str(var);
        middle = true;
    }
    if result.is_empty() {
        result.push('0');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_is_zero() {
        assert_eq!(simplify("x-x"), String::from("0"));
        assert_eq!(simplify("y-x-y+x"), String::from("0"));
    }

    #[test]
    fn result_has_term_with_no_variables() {
        assert_eq!(simplify("2xy-xy+2"), String::from("xy+2"));
    }

    #[test]
    fn result_has_coefficient_1() {
        assert_eq!(simplify("2xy-xy"), String::from("xy"));
        assert_eq!(simplify("3z-2z+y"), String::from("y+z"));
        assert_eq!(simplify("3z-2z+y+2-1"), String::from("y+z+1"));
        assert_eq!(simplify("4k-4+2j+3k+3"), String::from("2j+7k-1"));
    }

    #[test]
    fn result_is_ordered_correctly() {
        assert_eq!(simplify("dcba+bc"), String::from("bc+abcd"));
        assert_eq!(simplify("adc+dcba"), String::from("acd+abcd"));
        assert_eq!(simplify("a+ca-ab"), String::from("a-ab+ac"));
        assert_eq!(simplify("xyz-xz"), String::from("-xz+xyz"));
    }

    #[test]
    fn correct_result_for_long_polynomials() {
        assert_eq!(
            simplify("+n-5hn+7tjhn-4nh-3n-6hnjt+2jhn+9hn"),
            String::from("-2n+2hjn+hjnt")
        );
        assert_eq!(
            simplify("-8fk+5kv-4yk+7kf-qk+yqv-3vqy+4ky+4kf+yvqkf"),
            String::from("3fk-kq+5kv-2qvy+fkqvy")
        );
        assert_eq!(
            simplify("-26y+29cda-26d+4yba+32zc-47zbc-89ydb+54cdx+69cay+0a-27yzx-59d-37d-37y-23ac+48b+76cdy"), 
            String::from("48b-122d-63y-23ac+32cz+4aby+29acd+69acy-47bcz-89bdy+54cdx+76cdy-27xyz")
        );
    }

    #[test]
    fn polynomial_contains_coefficient_0() {
        assert_eq!(
            simplify("-0axz-0xz+0axz+0x+4xaz+14x+14zax"),
            String::from("14x+18axz")
        );
    }

    #[test]
    fn test_split_monomial() {
        assert_eq!(split_monomial("xy"), (1, String::from("xy")));
        assert_eq!(split_monomial("-xy"), (-1, String::from("xy")));
        assert_eq!(split_monomial("32ab"), (32, String::from("ab")));
        assert_eq!(split_monomial("-32ab"), (-32, String::from("ab")));
        assert_eq!(split_monomial("+32ab"), (32, String::from("ab")));
        assert_eq!(split_monomial("32bac"), (32, String::from("abc")));
        assert_eq!(split_monomial("-7"), (-7, String::from("")));
    }
}
