use std::path::PathBuf;
use thiserror::Error;

/*
** The type used to represent a page number.  The first page in a file
** is called page 1.  0 is used to represent "not a page".
** Using u64 for the love of game Sqlite3 use u32.
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Pgno (u64);
 
#[derive(Error, Debug)]
pub enum PgnoError {
    #[error("page number must be >= 1")]
    ZeroPage,
    #[error("offset computation overflowed")]
    Overflow,
    #[error("page number does not fit into u32")]
    TooLargeForU32,
}

impl Pgno {
    pub fn new(n: u64) -> Result<Self, PgnoError> {
        if n == 0 {
            return Err(PgnoError::ZeroPage)
        }
        Ok (Pgno(n))
    }
    
    pub fn as_u64(self) -> u64 { self.0 }

    /// Convert to on-disk u32 representation (fail if too large).
    pub fn to_u32(self) -> Result<u32, PgnoError> {
        u32::try_from(self.0).map_err(|_| PgnoError::TooLargeForU32)
    }

    /// Compute byte offset = (pgno - 1) * page_size, checked for overflow.
    pub fn offset(self, page_size: usize) -> Result<u64, PgnoError> {
        if self.0 == 0 {
            return Err(PgnoError::ZeroPage);
        }
        let ps = page_size as u64;
        self.0
            .checked_sub(1)
            .and_then(|n| n.checked_mul(ps))
            .ok_or(PgnoError::Overflow)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PagerState { Closed, Open, Reader, Writer, WriterLocked }

pub struct Pager {
    pub pgno: Pgno, //number of pages in the database
    pub page_size: usize,
    pub reserve_byter: usize,
    pub filename: PathBuf,
    pub wal_filename: Option<PathButh>,
    pub read_only: bool,
}
