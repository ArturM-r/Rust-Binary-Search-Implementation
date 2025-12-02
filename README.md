# Rust-Binary-Search-Implementation
This repository contains a safe and efficient implementation of binary search in Rust. The function bin_search searches for a target element in a sorted array of integers and returns both the value and its index if found. It is designed with safety in mind, handling edge cases like empty arrays and preventing integer underflow.


mplementation Details

The function first checks if the array is empty and immediately returns None if it is.

Uses min and max indices to keep track of the current search range.

Calculates the middle index using (min + max) / 2 and compares the middle value with the target using mid_value.cmp(&target).

Adjusts the search range based on the comparison:

Ordering::Equal → element found, return Some(value, index)

Ordering::Greater → search left, safely subtract 1 from mid using checked_sub to prevent underflow

Ordering::Less → search right, increment min

Returns None if the element is not found.
