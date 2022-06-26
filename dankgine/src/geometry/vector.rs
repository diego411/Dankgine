#[cfg(not(features = "simd"))]
mod vector {
    use serde_derive::Serialize;
    use serde_derive::Deserialize;
    use std::ops::{Add, Mul, Div, Sub};

    #[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }
    impl Vec2 {
        pub fn new(x: f32, y: f32) -> Vec2 {
            Vec2 { x, y }
        }

        pub fn length(self) -> f32 {
            (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
        }
    }
    impl Add for Vec2 {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Sub for Vec2 {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Mul<f32> for Vec2 {
        type Output = Self;

        fn mul(self, factor: f32) -> Self::Output {
            Self {
                x: self.x * factor,
                y: self.y * factor,
            }
        }
    }

    impl Div<f32> for Vec2 {
        type Output = Self;

        fn div(self, demoniator: f32) -> Self::Output {
            Self {
                x: self.x / demoniator,
                y: self.y / demoniator,
            }
        }
    }
}

#[cfg(features = "simd")]
mod vector_simd {

    use std::ops::{Add, Div, Mul, Sub};
    use core::simd::f32x2;

    use serde::ser::SerializeSeq;

    #[derive(Clone, Debug, Copy, PartialEq)]
    pub struct Vec2 {
        pub inner: f32x2,
    }
    
    impl Vec2 {
        pub fn new(x: f32, y: f32) -> Vec2 {
            Vec2 { 
                inner: f32x2::from_slice(&[x, y])
            }
        }

        pub fn length(self) -> f32 {
            (self.inner * self.inner).reduce_sum().sqrt()
        }
    }

    impl Add for Vec2 {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Self {
                inner: self.inner + other.inner,
            }
        }
    }

    impl Sub for Vec2 {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self {
                inner: self.inner - other.inner,
            }
        }
    }

    impl Mul<f32> for Vec2 {
        type Output = Self;

        fn mul(self, factor: f32) -> Self::Output {
            Self {
                inner: self.inner * f32x2::from_slice(&[factor, factor])
            }           
        }
    }

    impl Div<f32> for Vec2 {
        type Output = Self;

        fn div(self, demoniator: f32) -> Self::Output {
            Self {
                inner: self.inner / f32x2::from_slice(&[demoniator, demoniator])
            }
        }
    }

    impl serde::Serialize for Vec2 {
        fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
        where
            S: serde::Serializer {
            
            let mut seq = serializer.serialize_seq(Some(2)).unwrap();
            
            for lane in self.inner.as_array() {
                seq.serialize_element(lane)?;
            }
            seq.end()
        }
    }

    impl<'a> serde::de::Deserialize<'a> for Vec2 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'a>,
        {
            struct VectorVisitor;

            impl<'a> serde::de::Visitor<'a> for VectorVisitor 
            {
                type Value = Vec2;

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    formatter.write_str("a f32 number.")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                where
                    V: serde::de::SeqAccess<'a>,
                {
                    let x = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                    let y = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                    Ok(Vec2::new(x, y))
                }
            }


            const FIELDS: &'static [&'static str] = &["x", "y"];
            deserializer.deserialize_struct("Vec2", FIELDS, VectorVisitor)

        }
    }
}


#[cfg(features = "simd")]
pub use vector_simd::*;

#[cfg(not(features = "simd"))]
pub use vector::*;