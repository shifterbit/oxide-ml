use codespan_reporting::diagnostic::Label;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::{diagnostic::Diagnostic, files::SimpleFile};

use crate::source_location::SourceLocation;

pub trait CompilerError {
    fn new(
        message: &str,
        location: SourceLocation,
        additional_messages: Option<Vec<(String, SourceLocation)>>,
    ) -> Self;
    fn location(&self) -> SourceLocation;
    fn message(&self) -> &str;
    fn error_type(&self) -> &str;
    fn additional_messages(&self) -> Option<Vec<(String, SourceLocation)>>;
}

pub fn errors_from_file(filename: &str, text: &str, errors: Vec<impl CompilerError>) {
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = codespan_reporting::term::Config::default();

    let file = SimpleFile::new(filename, text);
    for error in errors {
        let start = error.location().start;
        let end = error.location().end;
        let mut labels = vec![Label::primary((), start..end).with_message(error.message())];
        if let Some(messages) = error.additional_messages() {
            for (message, location) in messages {
                let msg_start = location.start;
                let msg_end = location.end;
                let label = Label::secondary((), msg_start..msg_end).with_message(message);
                labels.push(label);
            }
        }
        let diagnostic: Diagnostic<()> = Diagnostic::error()
            .with_message(error.message())
            .with_code(error.error_type())
            .with_labels(labels);

        let _emit = term::emit(&mut writer.lock(), &config, &file, &diagnostic);
    }
}
