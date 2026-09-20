use std::ffi::OsStr;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;
use tokio::task::JoinHandle;

use super::{VirtualDiskRange, VirtualDisk};

pub(super) struct ChangedRangeStream<'a> {
    disk: &'a VirtualDisk,
    change_tracking_id: Vec<u16>,
    byte_length: u64,
    byte_offset: u64,
    ranges: Vec<VirtualDiskRange>,
    range_index: usize,
    finished: bool,
    worker: Option<JoinHandle<std::result::Result<(Vec<VirtualDiskRange>, u64), windows::core::Error>>>,
}

impl<'a> ChangedRangeStream<'a> {
    pub(super) fn new(disk: &'a VirtualDisk, change_tracking_id: &OsStr, byte_length: u64) -> Self {
        Self {
            disk,
            change_tracking_id: super::wide_path(change_tracking_id),
            byte_length,
            byte_offset: 0,
            ranges: Vec::new(),
            range_index: 0,
            finished: byte_length == 0,
            worker: None,
        }
    }
}

impl Stream for ChangedRangeStream<'_> {
    type Item = std::result::Result<VirtualDiskRange, windows::core::Error>;

    fn poll_next(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if self.range_index < self.ranges.len() {
                let range = self.ranges[self.range_index].clone();
                self.range_index += 1;
                return Poll::Ready(Some(Ok(range)));
            }

            if self.finished {
                return Poll::Ready(None);
            }

            if let Some(worker) = self.worker.as_mut() {
                match Pin::new(worker).poll(_context) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(result) => {
                        self.worker = None;
                        match result {
                            Ok(Ok((ranges, processed_length))) => {
                                self.ranges = ranges;
                                self.range_index = 0;
                                self.byte_offset = self.byte_offset.saturating_add(processed_length);
                                self.finished = processed_length == 0 || self.byte_offset >= self.byte_length;
                            }
                            Ok(Err(error)) => {
                                self.finished = true;
                                return Poll::Ready(Some(Err(error)));
                            }
                            Err(error) => {
                                self.finished = true;
                                return Poll::Ready(Some(Err(windows::core::Error::new(
                                    windows::core::HRESULT(0x80004005u32 as i32),
                                    format!("change query worker failed: {error}"),
                                ))));
                            }
                        }
                    }
                }
                continue;
            }

            let handle = match self.disk.duplicate_handle() {
                Ok(handle) => handle,
                Err(error) => {
                    self.finished = true;
                    return Poll::Ready(Some(Err(error)));
                }
            };
            let change_tracking_id = self.change_tracking_id.clone();
            let byte_offset = self.byte_offset;
            let byte_length = self.byte_length - byte_offset;
            self.worker = Some(tokio::task::spawn_blocking(move || {
                super::query_virtual_disk_range_changes(handle, &change_tracking_id, byte_offset, byte_length)
            }));
        }
    }
}
