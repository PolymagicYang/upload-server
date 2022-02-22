#[derive(Debug, Clone, Copy)]
pub enum FileType {
    // TODO: support more types.
    Video,
    Image,
    Music,
    Unknown,
}