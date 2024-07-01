// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use dav_server::fs::{DavDirEntry, DavMetaData};
use futures::StreamExt;
use futures::{FutureExt, Stream};
use opendal::Operator;
use opendal::{Entry, Lister};
use std::pin::Pin;
use std::task::Poll::Ready;
use std::task::{ready, Context, Poll};

use super::metadata::OpendalMetaData;
use super::utils::*;

/// OpendalStream is a stream of `DavDirEntry` that is used to list the contents of a directory.
pub struct OpendalStream {
    op: Operator,
    lister: Lister,
}

impl OpendalStream {
    /// Create a new opendal stream.
    pub fn new(op: Operator, lister: Lister) -> Self {
        OpendalStream { op, lister }
    }
}

impl Stream for OpendalStream {
    type Item = Box<dyn DavDirEntry>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let dav_stream = self.get_mut();
        match ready!(dav_stream.lister.poll_next_unpin(cx)) {
            Some(entry) => {
                let webdav_entry = OpendalDirEntry::new(dav_stream.op.clone(), entry.unwrap());
                Ready(Some(Box::new(webdav_entry) as Box<dyn DavDirEntry>))
            }
            None => Ready(None),
        }
    }
}

/// OpendalDirEntry is a `DavDirEntry` implementation for opendal.
pub struct OpendalDirEntry {
    op: Operator,
    dir_entry: Entry,
}

impl OpendalDirEntry {
    /// Create a new opendal dir entry.
    pub fn new(op: Operator, dir_entry: Entry) -> Self {
        OpendalDirEntry { dir_entry, op }
    }
}

impl DavDirEntry for OpendalDirEntry {
    fn name(&self) -> Vec<u8> {
        self.dir_entry.name().as_bytes().to_vec()
    }

    fn metadata(&self) -> dav_server::fs::FsFuture<Box<dyn DavMetaData>> {
        async move {
            self.op
                .stat(self.dir_entry.path())
                .await
                .map(|metadata| Box::new(OpendalMetaData::new(metadata)) as Box<dyn DavMetaData>)
                .map_err(convert_error)
        }
        .boxed()
    }
}
