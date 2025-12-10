// Copyright © 2024 Nathaniel Hardesty
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! # STREAM 6: DATA COLLECTION
//! **Based on SEMI E5§10.10**
//!
//! ---------------------------------------------------------------------------
//!
//! [Message]s which deal with in-process measurement and equipment
//! monitoring.
//!
//! ---------------------------------------------------------------------------
//!
//! [Message]: crate::Message

use crate::*;
use crate::Error::*;
use crate::items::*;

/// ## S6F0
///
/// **Abort Transaction**
///
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// Used in lieu of an expected reply to abort a transaction.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// Header only.
pub struct Abort;
message_headeronly!{Abort, false, 6, 0}

/// ## S6F11
///
/// **Event Report**
///
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY REQUIRED**
///
/// ---------------------------------------------------------------------------
///
/// Unsolicited event report from equipment.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - 3
///    1. [DATAID]
///    2. [CEID]
///    3. List - N
///       - List - 2
///          1. [RPTID]
///          2. List - M
///             - [V]
///
/// N is the number of reports.
///
/// M is the number of variables in a report.
///
/// [DATAID]: DataID
/// [CEID]:   CollectionEventID
/// [RPTID]:  ReportID
/// [V]:      Item
pub struct EventReport(pub (DataID, CollectionEventID, VecList<(ReportID, VecList<Item>)>));
message_data!{EventReport, true, 6, 11}

/// ## S6F12
///
/// **Event Report Acknowledge**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// Acknowledge event report.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - [ACKC6]
///
/// [ACKC6]: AcknowledgeCode6
pub struct EventReportAcknowledge(pub AcknowledgeCode6);
message_data!{EventReportAcknowledge, false, 6, 12}

/// ## S6F15
///
/// **Event Report Request**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
///
/// ---------------------------------------------------------------------------
///
/// Request to send the data associated with a specific collection event.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - [CEID]
///
/// [CEID]: CollectionEventID
pub struct EventReportRequest(pub CollectionEventID);
message_data!{EventReportRequest, true, 6, 15}

/// ## S6F16
///
/// **Event Report Data**
///
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// Event report data in response to S6F15 request.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - 3
///    1. [DATAID]
///    2. [CEID]
///    3. List - N
///       - List - 2
///          1. [RPTID]
///          2. List - M
///             - [V]
///
/// N is the number of reports.
///
/// M is the number of variables in a report.
///
/// [DATAID]: DataID
/// [CEID]:   CollectionEventID
/// [RPTID]:  ReportID
/// [V]:      Item
pub struct EventReportData(pub (DataID, CollectionEventID, VecList<(ReportID, VecList<Item>)>));
message_data!{EventReportData, false, 6, 16}
