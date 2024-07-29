use std::collections::HashMap;
use std::collections::HashSet;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

/* 
pub fn gen_name(already_seen: Vec<String>) -> String {
    for i in 0..already_seen.len() {
        let old_name = already_seen[i].clone();
        let name = format!("{}{}", old_name, i);
        if !already_seen.contains(&name) {
            return name;
        }
    }
    unreachable!()
}
*/

pub fn gen_name(vec: Vec<String>) -> String {
    // Convert the Vec to a HashSet for O(1) lookup times
    let existing_strings: HashSet<_> = vec.into_iter().collect();

    // Generate single character strings first (shortest possible strings)
    for c in 'a'..='z' {
        let s = c.to_string();
        if !existing_strings.contains(&s) {
            return s;
        }
    }
    
    // If all single character strings are taken, generate longer strings
    for len in 2.. {
        for s in generate_strings_of_length(len) {
            if !existing_strings.contains(&s) {
                return s;
            }
        }
    }

    // Fallback, though realistically we'll always find a new string
    panic!("Could not generate a new string");
}

// Helper function to generate all strings of a given length
fn generate_strings_of_length(len: usize) -> Vec<String> {
    let chars: Vec<char> = ('a'..='z').collect();
    let mut results = vec![];
    generate_strings_recursive(&chars, &mut String::new(), len, &mut results);
    results
}

// Recursive helper function to generate strings
fn generate_strings_recursive(chars: &[char], current: &mut String, len: usize, results: &mut Vec<String>) {
    if current.len() == len {
        results.push(current.clone());
        return;
    }
    for &c in chars {
        current.push(c);
        generate_strings_recursive(chars, current, len, results);
        current.pop();
    }
}