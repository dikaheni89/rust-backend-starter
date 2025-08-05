//! Stream Processing (dummy)

pub fn process_stream<I, F>(input: I, mut f: F)
where
    I: Iterator,
    F: FnMut(I::Item),
{
    for item in input {
        f(item);
    }
}
