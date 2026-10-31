//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/scan/reader.md
//! @prompt-hash 0950db90
//! @layer L3
//! @updated 2026-10-28

// Direct borrowed input avoids the generic std Read/Seek retry adapter.
use zune_core::bytestream::{ZByteIoError, ZByteReaderTrait, ZSeekFrom};
pub(super) struct BorrowedJpegReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}
impl<'a> BorrowedJpegReader<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
    #[inline]
    fn peek(&self, out: &mut [u8]) -> usize {
        let remaining = self.bytes.get(self.pos..).unwrap_or_default();
        let n = out.len().min(remaining.len());
        out[..n].copy_from_slice(&remaining[..n]);
        n
    }
}
impl ZByteReaderTrait for BorrowedJpegReader<'_> {
    #[inline]
    fn read_byte_no_error(&mut self) -> u8 {
        match self.bytes.get(self.pos) {
            Some(&b) => {
                self.pos += 1;
                b
            }
            None => 0,
        }
    }
    #[inline]
    fn read_exact_bytes(&mut self, out: &mut [u8]) -> Result<(), ZByteIoError> {
        let n = self.peek(out);
        if n != out.len() {
            return Err(ZByteIoError::NotEnoughBytes(n, out.len()));
        }
        self.pos += n;
        Ok(())
    }
    #[inline]
    fn read_bytes(&mut self, out: &mut [u8]) -> Result<usize, ZByteIoError> {
        let n = self.peek(out);
        self.pos += n;
        Ok(n)
    }
    #[inline]
    fn peek_bytes(&mut self, out: &mut [u8]) -> Result<usize, ZByteIoError> {
        Ok(self.peek(out))
    }
    #[inline]
    fn peek_exact_bytes(&mut self, out: &mut [u8]) -> Result<(), ZByteIoError> {
        let n = self.peek(out);
        if n != out.len() {
            Err(ZByteIoError::NotEnoughBytes(n, out.len()))
        } else {
            Ok(())
        }
    }
    #[inline]
    fn z_seek(&mut self, from: ZSeekFrom) -> Result<u64, ZByteIoError> {
        let (base, delta) = match from {
            ZSeekFrom::Start(n) => {
                let n = usize::try_from(n)?;
                self.pos = n;
                return Ok(n as u64);
            }
            ZSeekFrom::Current(n) => (self.pos, n),
            ZSeekFrom::End(n) => (self.bytes.len(), n),
        };
        let n = base
            .checked_add_signed(isize::try_from(delta)?)
            .ok_or(ZByteIoError::SeekError("Negative or overflowing seek"))?;
        self.pos = n;
        Ok(n as u64)
    }
    #[inline]
    fn is_eof(&mut self) -> Result<bool, ZByteIoError> {
        Ok(self.pos >= self.bytes.len())
    }
    #[inline]
    fn z_position(&mut self) -> Result<u64, ZByteIoError> {
        Ok(self.pos as u64)
    }
    fn read_remaining(&mut self, out: &mut Vec<u8>) -> Result<usize, ZByteIoError> {
        let bytes = self.bytes.get(self.pos..).unwrap_or_default();
        out.extend_from_slice(bytes);
        let n = bytes.len();
        self.pos += n;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::BorrowedJpegReader;
    use zune_core::bytestream::{ZByteReaderTrait, ZCursor, ZSeekFrom};

    #[test]
    fn exact_eof_copies_prefix_and_preserves_position() {
        let mut r = BorrowedJpegReader::new(&[1, 2, 3]);
        let mut out = [9; 5];
        assert!(r.read_exact_bytes(&mut out).is_err());
        assert_eq!(out, [1, 2, 3, 9, 9]);
        assert_eq!(r.z_position().unwrap(), 0);
        r.read_exact_bytes(&mut out[..3]).unwrap();
        assert_eq!(r.z_position().unwrap(), 3);
        assert!(r.read_exact_bytes(&mut out[..1]).is_err());
        assert_eq!(r.z_position().unwrap(), 3);
        r.read_exact_bytes(&mut []).unwrap();
    }

    #[test]
    fn peeks_and_partial_reads_match_std_adapter() {
        let mut r = BorrowedJpegReader::new(&[4, 5, 6]);
        let mut old = ZCursor::new([4, 5, 6]);
        for n in [0, 1, 5, 3, 4] {
            let (mut a, mut b) = ([9; 5], [9; 5]);
            assert_eq!(
                r.peek_bytes(&mut a[..n]).unwrap(),
                old.peek_bytes(&mut b[..n]).unwrap()
            );
            assert_eq!(a, b);
            assert_eq!(
                r.peek_exact_bytes(&mut a[..n]).is_ok(),
                old.peek_exact_bytes(&mut b[..n]).is_ok()
            );
            assert_eq!(a, b);
            assert_eq!(r.z_position().unwrap(), old.z_position().unwrap());
            assert_eq!(
                r.read_bytes(&mut a[..n]).unwrap(),
                old.read_bytes(&mut b[..n]).unwrap()
            );
            assert_eq!(a, b);
            assert_eq!(r.z_position().unwrap(), old.z_position().unwrap());
        }
    }

    #[test]
    fn byte_eof_and_remaining_suffix_keep_contract() {
        let mut r = BorrowedJpegReader::new(&[1, 2, 3]);
        assert_eq!(r.read_byte_no_error(), 1);
        let mut out = vec![9];
        assert_eq!(r.read_remaining(&mut out).unwrap(), 2);
        assert_eq!(out, [9, 2, 3]);
        assert_eq!(r.read_byte_no_error(), 0);
        assert_eq!(r.z_position().unwrap(), 3);
        r.z_seek(ZSeekFrom::Start(8)).unwrap();
        assert_eq!(r.read_remaining(&mut out).unwrap(), 0);
        assert_eq!(r.read_byte_no_error(), 0);
        assert_eq!(r.z_position().unwrap(), 8);
        assert!(r.is_eof().unwrap());
    }

    #[test]
    fn seeks_match_received_adapter() {
        let bytes = [1; 8];
        let mut r = BorrowedJpegReader::new(&bytes);
        let mut old = ZCursor::new(bytes);
        for seek in [
            ZSeekFrom::Start(3),
            ZSeekFrom::Current(-2),
            ZSeekFrom::End(-1),
            ZSeekFrom::End(5),
            ZSeekFrom::Current(-99),
            ZSeekFrom::Start(0),
        ] {
            assert_eq!(r.z_seek(seek).map_err(|_| ()), old.z_seek(seek).map_err(|_| ()));
            assert_eq!(r.z_position().unwrap(), old.z_position().unwrap());
            assert_eq!(r.is_eof().unwrap(), old.is_eof().unwrap());
        }
    }

    #[test]
    fn extreme_seek_and_read_never_wrap() {
        let mut r = BorrowedJpegReader::new(&[1, 2]);
        let max = usize::MAX as u64;
        r.z_seek(ZSeekFrom::Start(max)).unwrap();
        assert!(r.z_seek(ZSeekFrom::Current(1)).is_err());
        assert_eq!(r.z_position().unwrap(), max);
        let mut out = [9; 8];
        assert_eq!(r.read_bytes(&mut out).unwrap(), 0);
        assert!(r.read_exact_bytes(&mut out).is_err());
        assert!(r.peek_exact_bytes(&mut out).is_err());
        assert_eq!(r.read_byte_no_error(), 0);
        assert_eq!(out, [9; 8]);
        assert_eq!(r.z_position().unwrap(), max);
        assert!(r.z_seek(ZSeekFrom::End(i64::MIN)).is_err());
        assert_eq!(r.z_position().unwrap(), max);
    }

    #[test]
    fn seeded_mixed_operations_match_std_adapter() {
        let bytes: Vec<u8> = (0..251).map(|n| (n * 37) as u8).collect();
        let mut r = BorrowedJpegReader::new(&bytes);
        let mut old = ZCursor::new(bytes.as_slice());
        let mut seed = 0x1423_6875_u64;
        for _ in 0..1500 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let n = (seed >> 32) as usize % 33;
            let (mut a, mut b) = ([0xa5; 32], [0xa5; 32]);
            match seed % 8 {
                0 => assert_eq!(
                    r.read_exact_bytes(&mut a[..n]).is_ok(),
                    old.read_exact_bytes(&mut b[..n]).is_ok()
                ),
                1 => assert_eq!(
                    r.peek_exact_bytes(&mut a[..n]).is_ok(),
                    old.peek_exact_bytes(&mut b[..n]).is_ok()
                ),
                2 => assert_eq!(
                    r.read_bytes(&mut a[..n]).unwrap(),
                    old.read_bytes(&mut b[..n]).unwrap()
                ),
                3 => assert_eq!(
                    r.peek_bytes(&mut a[..n]).unwrap(),
                    old.peek_bytes(&mut b[..n]).unwrap()
                ),
                4 => {
                    let seek = ZSeekFrom::Start((seed >> 40) % 512);
                    assert_eq!(
                        r.z_seek(seek).map_err(|_| ()),
                        old.z_seek(seek).map_err(|_| ())
                    );
                }
                5 => {
                    let seek = ZSeekFrom::Current(((seed >> 24) % 193) as i64 - 96);
                    assert_eq!(
                        r.z_seek(seek).map_err(|_| ()),
                        old.z_seek(seek).map_err(|_| ())
                    );
                }
                6 => {
                    let seek = ZSeekFrom::End(((seed >> 24) % 193) as i64 - 96);
                    assert_eq!(
                        r.z_seek(seek).map_err(|_| ()),
                        old.z_seek(seek).map_err(|_| ())
                    );
                }
                _ => assert_eq!(r.read_byte_no_error(), old.read_byte_no_error()),
            }
            assert_eq!(a, b);
            assert_eq!(r.z_position().unwrap(), old.z_position().unwrap());
            assert_eq!(r.is_eof().unwrap(), old.is_eof().unwrap());
        }
    }
}
