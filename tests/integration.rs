extern crate httpdirectory;
use httpdirectory::httpdirectory::HttpDirectory;
use httpmock::prelude::*;
use unwrap_unreachable::UnwrapUnreachable;
mod common;

#[tokio::test]
async fn test_empty_200_status() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/hello");

    let mock = server.mock(|when, then| {
        when.path("/hello");
        then.status(200);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert!(httpdir.is_empty());
    mock.assert();
}

#[tokio::test]
async fn test_empty_404_status() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/hello");

    let mock = server.mock(|when, then| {
        when.path("/hello");
        then.status(404);
    });

    match HttpDirectory::new(&url).await {
        Ok(httpdir) => panic!("This test should return an Error. We got {httpdir:?}"),
        Err(e) => assert_eq!(e.to_string(), format!("Error: Error while retrieving url {url} content: 404 Not Found")),
    };

    mock.assert();
}

/// Tests <table> tag
#[tokio::test]
pub async fn test_debian_example() {
    common::table::run_debian_example().await.unreachable();
}

/// Tests <table> tag with another format
#[tokio::test]
async fn test_old_bsd_example() {
    common::table::run_old_bsd_example().await.unreachable();
}

/// Tests <pre> tag with other formatted columns
#[tokio::test]
async fn test_bsd_example() {
    common::pre::run_bsd_example().await.unreachable();
}

#[tokio::test]
async fn test_pre_img_example() {
    common::pre::run_pre_img_example().await.unreachable();
}

#[tokio::test]
async fn test_debian_archive_trafficmanager_net() {
    common::traffic_manager::run_debian_archive_trafficmanager_net().await.unreachable();
}

#[tokio::test]
async fn test_debian_h5ai() {
    common::h5ai::run_debian_h5ai().await.unreachable();
}

#[tokio::test]
async fn test_debian_ul() {
    common::ul::run_debian_ul().await.unreachable();
}

#[tokio::test]
async fn test_debian_snt() {
    common::snt::run_debian_snt().await.unreachable();
}

#[tokio::test]
async fn test_self_miniserve() {
    common::miniserve::run_self_miniserve().await.unreachable();
}
