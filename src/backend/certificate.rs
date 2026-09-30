use rcgen::{
  BasicConstraints, CertificateParams, DistinguishedName, DnType, DnValue, IsCa, Issuer, KeyPair,
  KeyUsagePurpose,
};

pub fn certificate_issuer() -> Issuer<'static, KeyPair> {
  let mut distinguished_name = DistinguishedName::new();
  distinguished_name.push(
    DnType::CommonName,
    DnValue::Utf8String("SYZYGY CA".to_string()),
  );

  let params = {
    let mut p = CertificateParams::default();
    p.distinguished_name = distinguished_name;
    // full perms, no limits. inf chain here
    p.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    p.key_usages = vec![
      // sign other certs
      KeyUsagePurpose::KeyCertSign,
      KeyUsagePurpose::CrlSign,
      KeyUsagePurpose::DigitalSignature,
    ];
    p
  };

  let kp = KeyPair::generate().unwrap();
  Issuer::new(params, kp)
}
