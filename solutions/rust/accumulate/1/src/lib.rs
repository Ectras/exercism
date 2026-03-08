/// What should the type of _function be?
pub fn map<I, O, F>(input: Vec<I>, mut function: F) -> Vec<O>
where
    F: FnMut(I) -> O,
{
    let mut out = Vec::with_capacity(input.len());
    for val in input {
        out.push(function(val));
    }
    out
}
