use rkyv::rancor::Error as RkyvError;
use rkyv::{Archive, Deserialize};
use rkyv::api::high::HighDeserializer;
use rkyv::api::high::HighValidator;
use rkyv::bytecheck::CheckBytes;

use bytes::Bytes;

use crate::types::PurrStep;
use crate::types::InsertPosition;

use crate::cat_malloc::purr_train::*;
use crate::cat_malloc::train_route::*;
use crate::cat_malloc::train_types::*;

use super::dispatcher_types::*;


impl<T, U, S> PurrTrain<T, U, S> 
where 
    T: PurrStep + Archive,
    T::Archived: 
        for<'a> CheckBytes<HighValidator<'a, RkyvError>> 
            + Deserialize<T, HighDeserializer<RkyvError>>,
    U: PurrRule + Archive,
    U::Archived: 
        for<'a> CheckBytes<HighValidator<'a, RkyvError>> 
            + Deserialize<U, HighDeserializer<RkyvError>>,
    S: PurrTrack<RouteBox<T, U>>
{
    pub fn deserialize<'a>(bytes: &'a Bytes) -> Result<impl Iterator<Item = RouteBox<T, U>>, PurrError> where T: 'a, U: 'a {
        let archived  = rkyv::access::<rkyv::Archived<Vec<RouteBox<T, U>>>, RkyvError>(bytes)
            .map_err(|e| PurrError::Internal(e.to_string()))?;
        let de_archived = archived.iter().map(|archived_state| {
            rkyv::deserialize::<RouteBox<T, U>, RkyvError>(archived_state).unwrap()
        });
        Ok(de_archived)
    }
    
    pub fn attach_bytes(&mut self, bytes: Bytes) -> Result<(), PurrError> {
        let de_archived = Self::deserialize(&bytes)?;
        self.line.tr_extend(de_archived);
        Ok(())
    }

    pub fn replace_bytes(&mut self, bytes: Bytes) -> Result<(), PurrError> {
        self.line.tr_clear();
        self.attach_bytes(bytes)
    }

    pub fn reroute_at_bytes(&mut self, bytes: Bytes, insert_position: InsertPosition) -> Result<(), PurrError> {
        let cursor_pos = self.line.tr_get_cursor();
        let mut index = match insert_position {
            InsertPosition::Forward => cursor_pos,
            InsertPosition::Index(i) => i + cursor_pos
        };
        index = index.min(self.line.tr_len());

        let de_archived = Self::deserialize(&bytes)?;
        self.line.tr_splice(index..index, de_archived);
        Ok(())
    }
}
