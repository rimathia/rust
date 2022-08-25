//! Errors emitted by middle.
// use rustc_errors::{error_code, Applicability, DiagnosticBuilder, ErrorGuaranteed};
use rustc_macros::{SessionDiagnostic, SessionSubdiagnostic};
use rustc_span::Span;

#[derive(SessionSubdiagnostic)]
pub enum LimitMustBeNonNegativeIntegerDetail {
    #[label(middle::limit_too_large)]
    TooLarge {
        #[primary_span]
        span: Span,
    },
    #[label(middle::limit_not_valid_integer)]
    Invalid {
        #[primary_span]
        span: Span,
    },
    #[label(middle::limit_must_be_non_negative_integer)]
    Empty {
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionDiagnostic)]
#[diag(middle::limit_must_be_non_negative_integer)]
pub struct LimitMustBeNonNegativeInteger {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub detail: LimitMustBeNonNegativeIntegerDetail,
}
