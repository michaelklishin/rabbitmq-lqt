use rlqt_ui::cli::clap_parser;

#[test]
fn test_web_serve_requires_database_path() {
    let result = clap_parser().try_get_matches_from(vec!["rlqt-ui", "web", "serve"]);
    assert!(result.is_err());
}

#[test]
fn test_web_serve_with_database_path() {
    let result = clap_parser().try_get_matches_from(vec![
        "rlqt-ui",
        "web",
        "serve",
        "--input-db-file-path",
        "/path/to/db.sqlite",
    ]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    let web_matches = matches.subcommand_matches("web").unwrap();
    let serve_matches = web_matches.subcommand_matches("serve").unwrap();
    assert_eq!(
        serve_matches
            .get_one::<String>("input_db_file_path")
            .unwrap(),
        "/path/to/db.sqlite"
    );
}

#[test]
fn test_web_serve_default_host() {
    let result = clap_parser().try_get_matches_from(vec![
        "rlqt-ui",
        "web",
        "serve",
        "--input-db-file-path",
        "/path/to/db.sqlite",
    ]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    let web_matches = matches.subcommand_matches("web").unwrap();
    let serve_matches = web_matches.subcommand_matches("serve").unwrap();
    assert_eq!(
        serve_matches.get_one::<String>("host").unwrap(),
        "127.0.0.1"
    );
}

#[test]
fn test_web_serve_default_port() {
    let result = clap_parser().try_get_matches_from(vec![
        "rlqt-ui",
        "web",
        "serve",
        "--input-db-file-path",
        "/path/to/db.sqlite",
    ]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    let web_matches = matches.subcommand_matches("web").unwrap();
    let serve_matches = web_matches.subcommand_matches("serve").unwrap();
    assert_eq!(serve_matches.get_one::<String>("port").unwrap(), "15692");
}

#[test]
fn test_web_serve_custom_host_and_port() {
    let result = clap_parser().try_get_matches_from(vec![
        "rlqt-ui",
        "web",
        "serve",
        "--input-db-file-path",
        "/path/to/db.sqlite",
        "--host",
        "0.0.0.0",
        "--port",
        "8080",
    ]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    let web_matches = matches.subcommand_matches("web").unwrap();
    let serve_matches = web_matches.subcommand_matches("serve").unwrap();
    assert_eq!(serve_matches.get_one::<String>("host").unwrap(), "0.0.0.0");
    assert_eq!(serve_matches.get_one::<String>("port").unwrap(), "8080");
}
