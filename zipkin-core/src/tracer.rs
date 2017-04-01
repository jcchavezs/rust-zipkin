use sampler::Sampler;
use span::Span;

#[derive(Clone, Debug, Default)]
pub struct Tracer<S> {
    pub sampler: Option<S>,
}

impl<'a, S> Tracer<S> {
    pub fn new() -> Self {
        Tracer { sampler: None }
    }
}

impl<'a, S> Tracer<S>
    where S: Sampler<Item = Span<'a>>
{
    pub fn with_sampler(sampler: S) -> Self {
        Tracer { sampler: Some(sampler) }
    }

    pub fn span(&mut self, name: &'a str) -> Span<'a> {
        let span = Span::new(name);
        let sampled = self.sampler.as_mut().map(|ref mut sampler| sampler.sample(&span));

        Span { sampled: sampled, ..span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::sampler::*;

    #[test]
    fn sampling() {
        let mut tracer = Tracer::with_sampler(FixedRate::new(2));

        assert_eq!(tracer.span("test1").sampled, Some(true));
        assert_eq!(tracer.span("test2").sampled, Some(false));
        assert_eq!(tracer.span("test3").sampled, Some(true));

        tracer = Tracer::new();

        assert_eq!(tracer.span("test1").sampled, None);
    }
}