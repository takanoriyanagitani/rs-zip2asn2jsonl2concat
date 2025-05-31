use std::fs::File;

use std::io;

use io::Read;

use io::Seek;

use io::BufWriter;
use io::Write;

use zip::ZipArchive;
use zip::read::ZipFile;

use der::asn1::OctetStringRef;

#[derive(der::Sequence)]
pub struct SimpleZipItems<'a> {
    /// Gzipped names.
    pub names: OctetStringRef<'a>,

    /// Gzipped jsonl.
    pub jsonl: OctetStringRef<'a>,
}

impl<'a> SimpleZipItems<'a> {
    pub fn from_der_bytes(der: &'a [u8]) -> Result<Self, io::Error> {
        der::Decode::from_der(der).map_err(io::Error::other)
    }
}

impl<'a> SimpleZipItems<'a> {
    pub fn as_gzipped_names_bytes(&self) -> &[u8] {
        self.names.as_bytes()
    }

    pub fn as_gzipped_jsonl_bytes(&self) -> &[u8] {
        self.jsonl.as_bytes()
    }
}

impl<'a> SimpleZipItems<'a> {
    pub fn jsonl2zcat2writer<W>(&self, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        let gzipped_jsonl_bytes: &[u8] = self.as_gzipped_jsonl_bytes();
        let mut dec = flate2::bufread::GzDecoder::new(gzipped_jsonl_bytes);
        io::copy(&mut dec, wtr)?;
        wtr.flush()
    }
}

impl<'a> SimpleZipItems<'a> {
    pub fn read2jsonl2zcat2writer<R, W>(
        rdr: &mut R,
        buf: &'a mut Vec<u8>,
        wtr: &mut W,
    ) -> Result<(), io::Error>
    where
        R: Read,
        W: Write,
    {
        buf.clear();
        rdr.read_to_end(buf)?;
        let s: &[u8] = buf;
        let me: Self = Self::from_der_bytes(s)?;
        me.jsonl2zcat2writer(wtr)
    }
}

impl<'a> SimpleZipItems<'a> {
    pub fn zfile2jsonl2zcat2writer<R, W>(
        rdr: &mut ZipFile<R>,
        buf: &'a mut Vec<u8>,
        wtr: &mut W,
    ) -> Result<(), io::Error>
    where
        R: Read,
        W: Write,
    {
        Self::read2jsonl2zcat2writer(rdr, buf, wtr)
    }
}

pub fn zip2jsonl2zcat2writer<R, W>(za: &mut ZipArchive<R>, wtr: &mut W) -> Result<(), io::Error>
where
    R: Read + Seek,
    W: Write,
{
    let mut buf: Vec<u8> = vec![];
    let sz: usize = za.len();
    for ix in 0..sz {
        let mut zfile = za.by_index(ix)?;
        SimpleZipItems::zfile2jsonl2zcat2writer(&mut zfile, &mut buf, wtr)?;
    }
    wtr.flush()
}

pub fn zipfile2jsonl2zcat2stdout(zipfile: File) -> Result<(), io::Error> {
    let mut za: ZipArchive<_> = ZipArchive::new(zipfile)?;
    let o = io::stdout();
    let mut ol = o.lock();
    {
        let mut bw = BufWriter::new(&mut ol);
        zip2jsonl2zcat2writer(&mut za, &mut bw)?;
    }
    ol.flush()
}
