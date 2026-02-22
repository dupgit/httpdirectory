extern crate httpdirectory;
use httpdirectory::{
    httpdirectory::{HttpDirectory, get_entries_from_body},
    httpdirectoryentry::{EntryType, HttpDirectoryEntry, assert_entry},
};
use httpmock::prelude::*;

const STIL_DEMO_INPUT: &str = r##"
<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>stil</title><style>:root {
	--background-color: #eee;
	--foreground-color: #111;
	--rusty-red: rgb(228, 58, 37);
	--line-thickness: round(0.2rem, 1px);

	background-color: var(--background-color);
	color: var(--foreground-color);
	font-family: sans-serif;
}

a {
	text-decoration: unset;

	&amp;:hover {
		text-decoration: underline;
	}
	&amp;:active {
		transform: scale(0.9);
	}
}

body {
	margin: 0;
	padding: 1rem;
	box-sizing: border-box;
	display: flex;
	align-items: flex-start;
	flex-direction: column;
	min-height: 100vh;
}

main {
	display: grid;
	grid-template-columns: repeat(4, auto);
	grid-auto-rows: 2rem;
	gap: 0.5rem 2rem;
	padding: 1rem;
	flex: 1;

	.dir {
		color: var(--rusty-red);
		font-weight: bolder;
		justify-self: center;
	}
}

header {
	align-self: stretch;
	border-bottom: var(--line-thickness) var(--foreground-color) solid;
	padding: 0.5rem 1rem;
	display: flex;
	gap: 0.5rem;
	&gt; :first-child {
		font-weight: bold;
	}
}

footer {
	text-align: center;
	font-size: smaller;
}
</style><link rel="icon" type="image/svg+xml" href="data:image/svg+xml;base64,PHN2ZyBjbGFzcz0idHlwc3QtZG9jIiB2aWV3Qm94PSIwIDAgMTEgMTEiIHdpZHRoPSIxMXB0IiBoZWlnaHQ9IjExcHQiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIHhtbG5zOmg1PSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hodG1sIj4KICAgIDxnPgogICAgICAgIDxnIGNsYXNzPSJ0eXBzdC1ncm91cCI+CiAgICAgICAgICAgIDxnPgogICAgICAgICAgICAgICAgPHBhdGggY2xhc3M9InR5cHN0LXNoYXBlIiBmaWxsPSIjZTQzYTI1IiBmaWxsLXJ1bGU9Im5vbnplcm8iIGQ9Ik0gMCAwbSAwIDEuMSBjIDAgLTAuNjA3NTEzMjUgMC40OTI0ODY3NyAtMS4xIDEuMSAtMS4xIGggOC43OTk5OTkgYyAwLjYwNzUxMzQgMCAxLjEwMDAwMDQgMC40OTI0ODY3NyAxLjEwMDAwMDQgMS4xIHYgOC43OTk5OTkgYyAwIDAuNjA3NTEzNCAtMC40OTI0ODY5NSAxLjEwMDAwMDQgLTEuMTAwMDAwNCAxLjEwMDAwMDQgaCAtOC43OTk5OTkgYyAtMC42MDc1MTMyNSAwIC0xLjEgLTAuNDkyNDg2OTUgLTEuMSAtMS4xMDAwMDA0IFogIi8+CiAgICAgICAgICAgIDwvZz4KICAgICAgICA8L2c+CiAgICAgICAgPGltYWdlIHRyYW5zZm9ybT0ibWF0cml4KDEgMCAwIDEgMS4wOTk5OTk5OTk5OTk5OTk0IDEuMDk5OTk5OTk5OTk5OTk5NCkiIHhsaW5rOmhyZWY9ImRhdGE6aW1hZ2Uvc3ZnK3htbDtiYXNlNjQsUEQ5NGJXd2dkbVZ5YzJsdmJqMGlNUzR3SWlCbGJtTnZaR2x1WnowaVZWUkdMVGdpSUhOMFlXNWtZV3h2Ym1VOUltNXZJajgrRFFvOElTMHRJRlZ3Ykc5aFpHVmtJSFJ2T2lCVFZrY2dVbVZ3Ynl3Z2QzZDNMbk4yWjNKbGNHOHVZMjl0TENCSFpXNWxjbUYwYjNJNklGTldSeUJTWlhCdklFMXBlR1Z5SUZSdmIyeHpJQzB0UGcwS1BITjJaeUIzYVdSMGFEMGlPREF3Y0hnaUlHaGxhV2RvZEQwaU9EQXdjSGdpSUhacFpYZENiM2c5SWpBZ01DQXlNQ0F5TUNJZ2RtVnljMmx2YmowaU1TNHhJaUI0Yld4dWN6MGlhSFIwY0RvdkwzZDNkeTUzTXk1dmNtY3ZNakF3TUM5emRtY2lJSGh0Ykc1ek9uaHNhVzVyUFNKb2RIUndPaTh2ZDNkM0xuY3pMbTl5Wnk4eE9UazVMM2hzYVc1cklqNE5DaUFnSUNBTkNpQWdJQ0E4ZEdsMGJHVStaR2x5WldOMGIzSjVYM05sWVhKamFDQmJJekUyTWpOZFBDOTBhWFJzWlQ0TkNpQWdJQ0E4WkdWell6NURjbVZoZEdWa0lIZHBkR2dnVTJ0bGRHTm9Mand2WkdWell6NE5DaUFnSUNBOFpHVm1jejROQ2cwS1BDOWtaV1p6UGcwS0lDQWdJRHhuSUdsa1BTSlFZV2RsTFRFaUlITjBjbTlyWlQwaWJtOXVaU0lnYzNSeWIydGxMWGRwWkhSb1BTSXhJaUJtYVd4c1BTSnViMjVsSWlCbWFXeHNMWEoxYkdVOUltVjJaVzV2WkdRaVBnMEtJQ0FnSUNBZ0lDQThaeUJwWkQwaVJISnBZbUppYkdVdFRHbG5hSFF0VUhKbGRtbGxkeUlnZEhKaGJuTm1iM0p0UFNKMGNtRnVjMnhoZEdVb0xUTTRNQzR3TURBd01EQXNJQzB4TnprNUxqQXdNREF3TUNraUlHWnBiR3c5SnlNeVpUQmpNRGNuUGcwS0lDQWdJQ0FnSUNBZ0lDQWdQR2NnYVdROUltbGpiMjV6SWlCMGNtRnVjMlp2Y20wOUluUnlZVzV6YkdGMFpTZzFOaTR3TURBd01EQXNJREUyTUM0d01EQXdNREFwSWo0TkNpQWdJQ0FnSUNBZ0lDQWdJQ0FnSUNBOGNHRjBhQ0JrUFNKTk16TXhMakV4T1RVc01UWTFNQzR6TmpBNElFTXpNekV1TVRFNU5Td3hOalE1TGpJMU56Z2dNek15TGpBeE5qVXNNVFkwT0M0ek5qQTRJRE16TXk0eE1UazFMREUyTkRndU16WXdPQ0JETXpNMExqSXlNalVzTVRZME9DNHpOakE0SURNek5TNHhNVGsxTERFMk5Ea3VNalUzT0NBek16VXVNVEU1TlN3eE5qVXdMak0yTURnZ1F6TXpOUzR4TVRrMUxERTJOVEV1TkRZek9DQXpNelF1TWpJeU5Td3hOalV5TGpNMk1EZ2dNek16TGpFeE9UVXNNVFkxTWk0ek5qQTRJRU16TXpJdU1ERTJOU3d4TmpVeUxqTTJNRGdnTXpNeExqRXhPVFVzTVRZMU1TNDBOak00SURNek1TNHhNVGsxTERFMk5UQXVNell3T0NCTU16TXhMakV4T1RVc01UWTFNQzR6TmpBNElGb2dUVE16Tnk0eE1UazFMREUyTlRBdU16WXdPQ0JETXpNM0xqRXhPVFVzTVRZME9DNHhOVEU0SURNek5TNHpNamcxTERFMk5EWXVNell3T0NBek16TXVNVEU1TlN3eE5qUTJMak0yTURnZ1F6TXpNQzQ1TVRBMUxERTJORFl1TXpZd09DQXpNamt1TVRFNU5Td3hOalE0TGpFMU1UZ2dNekk1TGpFeE9UVXNNVFkxTUM0ek5qQTRJRU16TWprdU1URTVOU3d4TmpVeUxqVTJPVGdnTXpNd0xqa3hNRFVzTVRZMU5DNHpOakE0SURNek15NHhNVGsxTERFMk5UUXVNell3T0NCRE16TXpMamM0TXpVc01UWTFOQzR6TmpBNElETXpOQzR6T1RrMUxERTJOVFF1TVRnek9DQXpNelF1T1RVd05Td3hOalV6TGpnNU56Z2dURE16Tnk0ME5qWTFMREUyTlRZdU5ERXpPQ0JNTXpNNExqZzRNRFVzTVRZMU5DNDVPVGs0SUV3ek16WXVORFV4TlN3eE5qVXlMalUyT1RnZ1F6TXpOaTQ0TnpFMUxERTJOVEV1T1RNMk9DQXpNemN1TVRFNU5Td3hOalV4TGpFM056Z2dNek0zTGpFeE9UVXNNVFkxTUM0ek5qQTRJRXd6TXpjdU1URTVOU3d4TmpVd0xqTTJNRGdnV2lCTk16UXhMams1T1RVc01UWTFOaTR3TURBNElFTXpOREV1T1RrNU5Td3hOalUyTGpVMU1qZ2dNelF4TGpjMU1qVXNNVFkxTnk0d01EQTRJRE0wTVM0eU1EQTFMREUyTlRjdU1EQXdPQ0JNTXpJM0xqSXdNRFVzTVRZMU55NHdNREE0SUVNek1qWXVOalEzTlN3eE5qVTNMakF3TURnZ016STFMams1T1RVc01UWTFOaTQxTlRJNElETXlOUzQ1T1RrMUxERTJOVFl1TURBd09DQk1NekkxTGprNU9UVXNNVFkwTWk0d01EQTRJRU16TWpVdU9UazVOU3d4TmpReExqUTBOemdnTXpJMkxqWTBOelVzTVRZME1TNHdNREE0SURNeU55NHlNREExTERFMk5ERXVNREF3T0NCTU16TXhMakl3TURVc01UWTBNUzR3TURBNElFTXpNekV1TnpVeU5Td3hOalF4TGpBd01EZ2dNek14TGprNU9UVXNNVFkwTVM0ME5EYzRJRE16TVM0NU9UazFMREUyTkRJdU1EQXdPQ0JNTXpNeExqazVPVFVzTVRZME15NHdNREE0SUVNek16RXVPVGs1TlN3eE5qUTBMakV3TkRnZ016TXpMakE1TlRVc01UWTBOUzR3TURBNElETXpOQzR5TURBMUxERTJORFV1TURBd09DQk1NelF4TGpJd01EVXNNVFkwTlM0d01EQTRJRU16TkRFdU56VXlOU3d4TmpRMUxqQXdNRGdnTXpReExqazVPVFVzTVRZME5TNDBORGM0SURNME1TNDVPVGsxTERFMk5EWXVNREF3T0NCTU16UXhMams1T1RVc01UWTFOaTR3TURBNElGb2dUVE0wTWk0eU1EQTFMREUyTkRNdU1EQXdPQ0JNTXpNMUxqSXdNRFVzTVRZME15NHdNREE0SUVNek16UXVOalEzTlN3eE5qUXpMakF3TURnZ016TXpMams1T1RVc01UWTBNaTQxTlRJNElETXpNeTQ1T1RrMUxERTJOREl1TURBd09DQk1Nek16TGprNU9UVXNNVFkwTVM0d01EQTRJRU16TXpNdU9UazVOU3d4TmpNNUxqZzVOVGdnTXpNekxqTXdORFVzTVRZek9TNHdNREE0SURNek1pNHlNREExTERFMk16a3VNREF3T0NCTU16STJMakl3TURVc01UWXpPUzR3TURBNElFTXpNalV1TURrMU5Td3hOak01TGpBd01EZ2dNekl6TGprNU9UVXNNVFl6T1M0NE9UVTRJRE15TXk0NU9UazFMREUyTkRFdU1EQXdPQ0JNTXpJekxqazVPVFVzTVRZMU55NHdNREE0SUVNek1qTXVPVGs1TlN3eE5qVTRMakV3TkRnZ016STFMakE1TlRVc01UWTFPUzR3TURBNElETXlOaTR5TURBMUxERTJOVGt1TURBd09DQk1NelF5TGpJd01EVXNNVFkxT1M0d01EQTRJRU16TkRNdU16QTBOU3d4TmpVNUxqQXdNRGdnTXpRekxqazVPVFVzTVRZMU9DNHhNRFE0SURNME15NDVPVGsxTERFMk5UY3VNREF3T0NCTU16UXpMams1T1RVc01UWTBOUzR3TURBNElFTXpORE11T1RrNU5Td3hOalF6TGpnNU5UZ2dNelF6TGpNd05EVXNNVFkwTXk0d01EQTRJRE0wTWk0eU1EQTFMREUyTkRNdU1EQXdPQ0JNTXpReUxqSXdNRFVzTVRZME15NHdNREE0SUZvaUlHbGtQU0prYVhKbFkzUnZjbmxmYzJWaGNtTm9MVnNqTVRZeU0xMGlQZzBLRFFvOEwzQmhkR2crRFFvZ0lDQWdJQ0FnSUNBZ0lDQThMMmMrRFFvZ0lDQWdJQ0FnSUR3dlp6NE5DaUFnSUNBOEwyYytEUW84TDNOMlp6NE5DZz09IiB3aWR0aD0iOC44IiBoZWlnaHQ9IjguOCIgcHJlc2VydmVBc3BlY3RSYXRpbz0ibm9uZSIvPgogICAgPC9nPgo8L3N2Zz4K"></head><body><header><a href="/demo/">/</a></header><main><b>Type</b><b>Name</b><b>Last modified</b><b>Size</b><span class="file"></span><a href="/demo/CHANGELOG.md">CHANGELOG.md</a><span>2025-11-14 12:50:31</span><span>5.94 KiB</span><span class="file"></span><a href="/demo/Cargo.lock">Cargo.lock</a><span>2025-11-14 12:50:31</span><span>11.21 KiB</span><span class="file"></span><a href="/demo/Cargo.toml">Cargo.toml</a><span>2025-11-14 12:50:31</span><span>792 B</span><span class="file"></span><a href="/demo/Dockerfile">Dockerfile</a><span>2025-11-14 12:50:31</span><span>539 B</span><span class="file"></span><a href="/demo/Justfile">Justfile</a><span>2025-11-14 12:50:31</span><span>369 B</span><span class="file"></span><a href="/demo/LICENSE">LICENSE</a><span>2025-11-14 12:50:31</span><span>494 B</span><span class="file"></span><a href="/demo/README.md">README.md</a><span>2025-11-14 12:50:31</span><span>2.27 KiB</span><span class="file"></span><a href="/demo/build.rs">build.rs</a><span>2025-11-14 12:50:31</span><span>304 B</span><span class="file"></span><a href="/demo/cliff.toml">cliff.toml</a><span>2025-11-14 12:50:31</span><span>1.60 KiB</span><span class="dir">&#128448;</span><a href="/demo/media/">media</a><span>2025-11-14 12:50:31</span><span>-</span><span class="file"></span><a href="/demo/release.toml">release.toml</a><span>2025-11-14 12:50:31</span><span>83 B</span><span class="dir">&#128448;</span><a href="/demo/src/">src</a><span>2025-11-14 12:50:31</span><span>-</span></main></body></html>
"##;

fn assert_stil_demo_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 12);

    assert_entry(&entries[0], &EntryType::File, "CHANGELOG.md", 6041, "2025-11-14 12:50");
    assert_entry(&entries[1], &EntryType::File, "Cargo.lock", 11468, "2025-11-14 12:50");
    assert_entry(&entries[2], &EntryType::File, "Cargo.toml", 792, "2025-11-14 12:50");
    assert_entry(&entries[3], &EntryType::File, "Dockerfile", 539, "2025-11-14 12:50");
    assert_entry(&entries[4], &EntryType::File, "Justfile", 369, "2025-11-14 12:50");
    assert_entry(&entries[5], &EntryType::File, "LICENSE", 494, "2025-11-14 12:50");
    assert_entry(&entries[6], &EntryType::File, "README.md", 2252, "2025-11-14 12:50");
    assert_entry(&entries[7], &EntryType::File, "build.rs", 304, "2025-11-14 12:50");
    assert_entry(&entries[8], &EntryType::File, "cliff.toml", 1638, "2025-11-14 12:50");
    assert_entry(&entries[9], &EntryType::Directory, "media", 0, "2025-11-14 12:50");
    assert_entry(&entries[10], &EntryType::File, "release.toml", 83, "2025-11-14 12:50");
    assert_entry(&entries[11], &EntryType::Directory, "src", 0, "2025-11-14 12:50");
}

#[allow(dead_code)]
pub async fn mock_stil_demo() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/stil");

    let mock = server.mock(|when, then| {
        when.path("/stil");
        then.status(200).body(STIL_DEMO_INPUT);
    });

    let httpdir = match HttpDirectory::new(&url, None).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    let entries = httpdir.entries();
    assert_stil_demo_entries(entries);

    mock.assert();

    Ok(())
}

#[allow(dead_code)]
pub fn run_stil_demo() -> Result<(), Box<dyn std::error::Error>> {
    let body = STIL_DEMO_INPUT;
    let entries = get_entries_from_body(body);

    assert_stil_demo_entries(&entries);

    Ok(())
}
