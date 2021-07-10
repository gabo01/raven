use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sled::Db as DBHandle;
use std::io;
use std::marker::PhantomData;
use std::path::Path;
use uuid::Uuid;

pub struct Database {
    handle: DBHandle,
}

impl Database {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let path = path.as_ref();
        let handle = sled::open(path).map_err(|err| {
            match err {
                sled::Error::Io(err) => err,
                _ => unreachable!("A non I/O related error was found while creating the database. Such error should not be possible. Please report the incident.")
            }
        })?;
        Ok(Self { handle })
    }

    pub fn delete(&mut self) -> Result<(), ()> {
        todo!();
    }
}

pub struct DBId<T> {
    id: Uuid,
    marker: PhantomData<T>,
}

impl<T> Serialize for DBId<T> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.id.serialize(ser)
    }
}

impl<'de, T> Deserialize<'de> for DBId<T> {
    fn deserialize<D: Deserializer<'de>>(de: D) -> std::result::Result<Self, D::Error> {
        Ok(Self {
            id: Uuid::deserialize(de)?,
            marker: PhantomData,
        })
    }
}