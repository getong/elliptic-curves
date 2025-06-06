//! brainpoolP384t1 elliptic curve: twisted variant

#[cfg(feature = "ecdsa")]
pub mod ecdsa;

#[cfg(feature = "arithmetic")]
mod arithmetic;

#[cfg(feature = "arithmetic")]
pub use {
    self::arithmetic::{AffinePoint, NonZeroScalar, ProjectivePoint, ScalarPrimitive},
    crate::Scalar,
};

use crate::ORDER;
use elliptic_curve::{FieldBytesEncoding, bigint::U384, consts::U48};

#[cfg(feature = "pkcs8")]
use crate::pkcs8;

/// brainpoolP384t1 elliptic curve: twisted variant
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct BrainpoolP384t1;

impl elliptic_curve::Curve for BrainpoolP384t1 {
    /// 48-byte serialized field elements.
    type FieldBytesSize = U48;

    /// 384-bit field modulus.
    type Uint = U384;

    /// Curve order.
    const ORDER: U384 = ORDER;
}

impl elliptic_curve::PrimeCurve for BrainpoolP384t1 {}

impl elliptic_curve::point::PointCompression for BrainpoolP384t1 {
    const COMPRESS_POINTS: bool = false;
}

#[cfg(feature = "pkcs8")]
impl pkcs8::AssociatedOid for BrainpoolP384t1 {
    const OID: pkcs8::ObjectIdentifier =
        pkcs8::ObjectIdentifier::new_unwrap("1.3.36.3.3.2.8.1.1.12");
}

/// brainpoolP384t1 SEC1 encoded point.
pub type EncodedPoint = elliptic_curve::sec1::EncodedPoint<BrainpoolP384t1>;

/// brainpoolP384t1 field element serialized as bytes.
///
/// Byte array containing a serialized field element value (base field or scalar).
pub type FieldBytes = elliptic_curve::FieldBytes<BrainpoolP384t1>;

impl FieldBytesEncoding<BrainpoolP384t1> for U384 {
    fn decode_field_bytes(field_bytes: &FieldBytes) -> Self {
        crate::decode_field_bytes(field_bytes)
    }

    fn encode_field_bytes(&self) -> FieldBytes {
        crate::encode_field_bytes(self)
    }
}

/// brainpoolP384t1 secret key.
pub type SecretKey = elliptic_curve::SecretKey<BrainpoolP384t1>;

#[cfg(not(feature = "arithmetic"))]
impl elliptic_curve::sec1::ValidatePublicKey for BrainpoolP384t1 {}
