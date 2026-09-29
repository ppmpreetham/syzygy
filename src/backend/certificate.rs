use rcgen::{
  BasicConstraints, CertificateParams, DistinguishedName, DnType, DnValue, IsCa, Issuer, KeyPair,
  KeyUsagePurpose,
};

pub fn certificate_issuer() -> Issuer<'static, KeyPair> {
  let mut distinguished_name = DistinguishedName::new();
  distinguished_name.push(
    DnType::CommonName,
    DnValue::Utf8String("Proxy CA".to_string()),
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

  let key_pair = KeyPair::generate().unwrap();
  Issuer::new(params, key_pair)
}
