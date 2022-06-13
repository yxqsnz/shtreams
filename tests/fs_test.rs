#[test]
fn write_test() {
    use shtreams::WriteStream;
    let mut buf = Vec::with_capacity(256);
    let mut s = WriteStream::new(&mut buf)
        << "Hello, world\n!"
        << "This is a test.\n"
        << "but, it's not a real test.\n"
        << "its a integration test!\n";

    s.finish().unwrap();
    assert_eq!(
        String::from_utf8(buf).unwrap(),
        "Hello, world\n!This is a test.\nbut, it's not a real test.\nits a integration test!\n"
    );
}
#[test]
fn read_test() {
    use shtreams::ReadStream;
    use std::fs::File;

    let mut f = File::open("tests/fs_test.rs").unwrap();
    let s = ReadStream::new(&mut f);
    let mut buf = Vec::new();
    (s >> &mut buf).unwrap();
    assert_eq!(buf, include_bytes!("fs_test.rs"));
}
