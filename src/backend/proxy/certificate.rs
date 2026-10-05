use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, DnValue, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use std::fs;
use std::path::PathBuf;

// get the storage path for the CA certificate. if not there, create it
pub fn storage_path() -> Result<PathBuf> {
    let dir = ProjectDirs::from("com", "syzygy", "SyZyGy");
    dir.map_or_else(
        || Err(anyhow!("Could not determine system project directories")),
        |proj_dirs| {
            let config_dir = proj_dirs.config_dir().to_path_buf();
            if !config_dir.exists() {
                fs::create_dir_all(&config_dir)?;
            }
            Ok(config_dir)
        },
    )
}

// save the certificate on first launch
fn ca_paths() -> Result<(PathBuf, PathBuf)> {
    let keys_dir = storage_path()?.join("keys");
    fs::create_dir_all(&keys_dir)?;

    Ok((
        keys_dir.join("syzygy_ca.crt"),
        keys_dir.join("syzygy_ca.key"),
    ))
}

fn load_ca(cp: &PathBuf, kp: &PathBuf) -> Result<Issuer<'static, KeyPair>> {
    let cp = fs::read_to_string(cp)?;
    let kp = fs::read_to_string(kp)?;
    let sign_key = KeyPair::from_pem(&kp)?;
    let issuer = Issuer::from_ca_cert_pem(&cp, sign_key)?;
    Ok(issuer)
}

pub fn cert_gen(cert_path: &PathBuf, key_path: &PathBuf) -> Result<Issuer<'static, KeyPair>> {
    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(
        DnType::CommonName,
        DnValue::Utf8String("SYZYGY CA".to_string()),
    );

    let params = {
        let mut p = CertificateParams::default();
        p.distinguished_name = distinguished_name;
        p.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        p.key_usages = vec![
            KeyUsagePurpose::KeyCertSign,
            KeyUsagePurpose::CrlSign,
            KeyUsagePurpose::DigitalSignature,
        ];
        p
    };

    let kp = KeyPair::generate()?;
    let cert = params.self_signed(&kp)?;

    let c_pem = cert.pem();
    let key_pem = kp.serialize_pem();

    fs::write(cert_path, c_pem)?;
    fs::write(key_path, key_pem)?;

    let issuer = Issuer::new(params, kp);
    Ok(issuer)
}

pub fn certificate_issuer() -> Result<Issuer<'static, KeyPair>> {
    let (cert_path, key_path) = ca_paths()?;
    if cert_path.exists() && key_path.exists() {
        load_ca(&cert_path, &key_path)
    } else {
        cert_gen(&cert_path, &key_path)
    }
}
