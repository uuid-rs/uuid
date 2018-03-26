use slog;
use Uuid;

impl slog::Value for Uuid {
    fn serialize(
        &self,
        _: &slog::Record,
        key: slog::Key,
        serializer: &mut slog::Serializer,
    ) -> Result<(), slog::Error> {
        serializer.emit_arguments(key, &format_args!("{}", self))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_slog_kv() {
        use slog;
        use test_util;
        use slog::Drain;

        let root = slog::Logger::root(slog::Discard.fuse(), o!());
        let u1 = test_util::new();
        crit!(root, "test"; "u1" => u1);
    }
}