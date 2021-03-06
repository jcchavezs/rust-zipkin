#[macro_use]
extern crate log;
extern crate hexplay;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate time;
extern crate rand;
extern crate xoroshiro128;
extern crate bytes;
extern crate tokio_io;
extern crate mime;

pub mod constants;
pub mod errors;
mod span;
mod sampler;
mod tracer;
mod collector;

pub use bytes::{BufMut, BytesMut};
pub use tokio_io::codec::Encoder;

pub use constants::*;
pub use span::{TraceId, SpanId, Timestamp, timestamp, now, ToMicrosecond, Duration, Endpoint,
               Annotation, Value, BinaryAnnotation, BinaryAnnotationValue, Annotatable, Span};
pub use sampler::{Sampler, FixedRate, RateLimit};
pub use tracer::Tracer;
pub use collector::{MimeType, Codec, Transport, Collector, BaseCollector};