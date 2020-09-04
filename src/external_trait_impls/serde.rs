mod size_hint {
    use core::cmp;

    /// This presumably exists to prevent denial of service attacks.
    ///
    /// Original discussion: https://github.com/serde-rs/serde/issues/1114.
    #[cfg_attr(feature = "inline-more", inline)]
    pub(super) fn cautious(hint: Option<usize>) -> usize {
        cmp::min(hint.unwrap_or(0), 4096)
    }
}

mod map {
    use core::fmt;
    use core::marker::PhantomData;
    use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
    use serde::ser::{Serialize, Serializer};

    use crate::{AutoHash, AutoHashMap};

    use super::size_hint;

    impl<K, V> Serialize for AutoHashMap<K, V>
    where
        K: Serialize + Eq + AutoHash,
        V: Serialize,
    {
        #[cfg_attr(feature = "inline-more", inline)]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_map(self)
        }
    }

    impl<'de, K, V> Deserialize<'de> for AutoHashMap<K, V>
    where
        K: Deserialize<'de> + Eq + AutoHash,
        V: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct MapVisitor<K, V> {
                marker: PhantomData<AutoHashMap<K, V>>,
            }

            impl<'de, K, V> Visitor<'de> for MapVisitor<K, V>
            where
                K: Deserialize<'de> + Eq + AutoHash,
                V: Deserialize<'de>,
            {
                type Value = AutoHashMap<K, V>;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str("a map")
                }

                #[cfg_attr(feature = "inline-more", inline)]
                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
                {
                    let mut values =
                        AutoHashMap::with_capacity(size_hint::cautious(map.size_hint()));

                    while let Some((key, value)) = map.next_entry()? {
                        values.insert(key, value);
                    }

                    Ok(values)
                }
            }

            let visitor = MapVisitor {
                marker: PhantomData,
            };
            deserializer.deserialize_map(visitor)
        }
    }
}

mod set {
    use core::fmt;
    use core::marker::PhantomData;
    use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
    use serde::ser::{Serialize, Serializer};

    use crate::{AutoHash, AutoHashSet};

    use super::size_hint;

    impl<T> Serialize for AutoHashSet<T>
    where
        T: Serialize + Eq + AutoHash,
    {
        #[cfg_attr(feature = "inline-more", inline)]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(self)
        }
    }

    impl<'de, T> Deserialize<'de> for AutoHashSet<T>
    where
        T: Deserialize<'de> + Eq + AutoHash,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct SeqVisitor<T> {
                marker: PhantomData<AutoHashSet<T>>,
            }

            impl<'de, T> Visitor<'de> for SeqVisitor<T>
            where
                T: Deserialize<'de> + Eq + AutoHash,
            {
                type Value = AutoHashSet<T>;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str("a sequence")
                }

                #[cfg_attr(feature = "inline-more", inline)]
                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    let mut values =
                        AutoHashSet::with_capacity(size_hint::cautious(seq.size_hint()));

                    while let Some(value) = seq.next_element()? {
                        values.insert(value);
                    }

                    Ok(values)
                }
            }

            let visitor = SeqVisitor {
                marker: PhantomData,
            };
            deserializer.deserialize_seq(visitor)
        }

        fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
        where
            D: Deserializer<'de>,
        {
            struct SeqInPlaceVisitor<'a, T>(&'a mut AutoHashSet<T>);

            impl<'a, 'de, T> Visitor<'de> for SeqInPlaceVisitor<'a, T>
            where
                T: Deserialize<'de> + Eq + AutoHash,
            {
                type Value = ();

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str("a sequence")
                }

                #[cfg_attr(feature = "inline-more", inline)]
                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    self.0.clear();
                    self.0.reserve(size_hint::cautious(seq.size_hint()));

                    while let Some(value) = seq.next_element()? {
                        self.0.insert(value);
                    }

                    Ok(())
                }
            }

            deserializer.deserialize_seq(SeqInPlaceVisitor(place))
        }
    }
}

mod wrappers {
    use core::hash::{Hash, Hasher};
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};

    use crate::wrappers::*;

    impl Serialize for U64Hash {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for U64Hash {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            u64::deserialize(deserializer).map(U64Hash)
        }
    }

    impl<T, H> Serialize for AutoHashed<T, H>
    where
        T: Serialize,
    {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.value.serialize(serializer)
        }
    }

    impl<'de, T, H> Deserialize<'de> for AutoHashed<T, H>
    where
        T: Deserialize<'de>,
    {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            T::deserialize(deserializer).map(Self::from)
        }
    }

    impl<T, H> Serialize for MemoHashed<T, H>
    where
        T: Serialize,
    {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.value.serialize(serializer)
        }
    }

    impl<'de, T, H> Deserialize<'de> for MemoHashed<T, H>
    where
        T: Deserialize<'de> + Hash,
        H: Hasher + Default,
    {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            T::deserialize(deserializer).map(Self::from)
        }
    }
}
