use std::fs;
use std::io::{BufReader, Read};
use std::sync::Arc;

use rustls::server::{NoClientAuth};
use crate::toml_schema::CertConfig;

pub fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}

pub fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::ECKey(key)) => return rustls::PrivateKey(key),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}

pub fn load_ocsp(filename: &String) -> Vec<u8> {
    let mut ret = Vec::new();
    if !filename.is_empty() {
        fs::File::open(filename)
            .expect("cannot open ocsp file")
            .read_to_end(&mut ret)
            .unwrap();
    }
    ret
}

pub fn make_config(cfg: &CertConfig) -> Arc<rustls::ServerConfig> {
    let client_auth = NoClientAuth::new();
    let suites = rustls::ALL_CIPHER_SUITES.to_vec();
    let versions = rustls::ALL_VERSIONS.to_vec();

    let certs = load_certs(
        cfg.cert.as_str(),
    );
    let privkey = load_private_key(
        cfg.key.as_str(),
    );
    let ocsp = load_ocsp(&cfg.ocsp);
    let mut config = rustls::ServerConfig::builder()
        .with_cipher_suites(&suites)
        .with_safe_default_kx_groups()
        .with_protocol_versions(&versions)
        .expect("inconsistent cipher-suites/versions specified")
        .with_client_cert_verifier(client_auth)
        .with_single_cert_with_ocsp_and_sct(certs, privkey, ocsp, vec![])
        .expect("bad certificates/private key");
    config.key_log = Arc::new(rustls::KeyLogFile::new());

    Arc::new(config)
}