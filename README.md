# rust linear regression

Experimental package for linear regression using `nalgebra`. Also to learn a bit of rust.

## Example

```rust
let x = na::DMatrix::from_row_slice(2, 2, &[1.0, 3.0, 2.0, 13.0]);
let y = na::DMatrix::from_column_slice(2, 1, &[3.0, 5.0]);

let res = least_squares(x, y);
match res {
    None => ...,
    Some(x) => ...
}
```