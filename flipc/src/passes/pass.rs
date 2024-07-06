pub trait Pass {
    type Input;
    type Output;

    fn run(input: Self::Input) -> Self::Output;
}
