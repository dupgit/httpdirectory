extern crate httpdirectory;
use httpdirectory::httpdirectory::HttpDirectory;

#[test]
fn test_me() {
    if let Err(myerr) = HttpDirectory::new("https://cloud.centos.org/centos/10-stream/x86_64/images/") {
        assert_eq!(
            myerr.to_string(),
            r#"HttpError(Io(Custom { kind: Uncategorized, error: "failed to lookup address information: Temporary failure in name resolution" }))"#
        );
    }
}
