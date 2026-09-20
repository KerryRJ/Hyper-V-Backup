use crate::infra::VirtualDiskRange;
use crate::model::Error;
use fastcdc::v2020::FastCDC;
use futures::{Stream, StreamExt};
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

const READ_WINDOW_SIZE: usize = 64 * 1024 * 1024;
const FASTCDC_MIN_SIZE: usize = 1024 * 1024;
const FASTCDC_AVERAGE_SIZE: usize = 4 * 1024 * 1024;
const FASTCDC_MAX_SIZE: usize = 16 * 1024 * 1024;

pub(crate) async fn copy_ranges_to_file<S>(source_path: &Path, destination_path: &Path, source_length: u64, mut ranges: S) -> Result<(), Error>
where
    S: Stream<Item = std::result::Result<VirtualDiskRange, windows::core::Error>>,
{
    let mut ranges = Box::pin(ranges);
    let mut source = tokio::fs::File::open(source_path).await?;
    let mut destination = tokio::fs::File::create(destination_path).await?;
    destination.set_len(source_length).await?;

    while let Some(range) = ranges.next().await {
        let range = range?;
        let mut range_offset = 0u64;

        while range_offset < range.byte_length {
            let window_length = (range.byte_length - range_offset).min(READ_WINDOW_SIZE as u64) as usize;
            let mut window = vec![0u8; window_length];
            source.seek(std::io::SeekFrom::Start(range.byte_offset + range_offset)).await?;
            source.read_exact(&mut window).await?;

            for chunk in FastCDC::new(&window, FASTCDC_MIN_SIZE, FASTCDC_AVERAGE_SIZE, FASTCDC_MAX_SIZE) {
                let destination_offset = range.byte_offset + range_offset + chunk.offset as u64;
                destination.seek(std::io::SeekFrom::Start(destination_offset)).await?;
                destination.write_all(&window[chunk.offset..chunk.offset + chunk.length]).await?;
            }

            range_offset += window_length as u64;
        }
    }

    destination.flush().await?;
    Ok(())
}
