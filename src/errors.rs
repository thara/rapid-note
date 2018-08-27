error_chain! {

    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
    }

    errors {
        NoMatchNotes {
            description("no match notes")
            display("no match notes")
        }
    }
}
