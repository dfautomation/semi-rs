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

//! # STREAM 5: EXCEPTION HANDLING
//! **Based on SEMI E5§10.9**
//!
//! ---------------------------------------------------------------------------
//!
//! [Message]s which deal with binary and analog equipment exceptions.
//!
//! Exceptions are classified into two categories: Errors and Alarms
//!
//! ---------------------------------------------------------------------------
//!
//! [Message]s S5F1 through S5F8 provide basic alarm messages, which may
//! be divided into the following categories:
//!
//! - Personal Safety - Condition may be dangerous to people.
//! - Equipment Safety - Condition may harm equipment.
//! - Parameter Control Warning - Parameter variation outside of preset
//!   limits - may harm product.
//! - Parameter Control Error - Parameter variation outside of reasonable
//!   control limits - may indicate an equipment malfunction.
//! - Irrecoverable Error - Intervention required before normal use of
//!   equipment can resume.
//! - Equipment Status Warning - An unexpected condition has occurred, but
//!   operation can continue.
//! - Attention Flags - A signal from a process program indicating that a
//!   particular step has been reached.
//! - Data Integrity - A condition which may cause loss of data; usually
//!   related to [Stream 6].
//!
//! It will be the equipment's responsibility to categorize alarms.
//!
//! Some alarm conditions may cause more than one type of alarm to be issued.
//!
//! ---------------------------------------------------------------------------
//!
//! [Message]s S5F9 through S5F15 provide extended capabilities for
//! exception handling.
//!
//! ---------------------------------------------------------------------------
//!
//! [Message]: crate::Message
//! [Stream 6]: crate::messages::s6

use crate::*;
use crate::Error::*;
use crate::items::*;

/// ## S5F0
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
message_headeronly!{Abort, false, 5, 0}

/// ## S5F1
///
/// **Alarm Report Send**
///
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY REQUIRED**
///
/// ---------------------------------------------------------------------------
///
/// Notification that an alarm has occurred.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - 3
///    1. [ALCD]
///    2. [ALID]
///    3. [ALTX]
///
/// [ALCD]: AlarmCode
/// [ALID]: AlarmID
/// [ALTX]: AlarmText
pub struct AlarmReportSend(pub (AlarmCode, AlarmID, AlarmText));
message_data!{AlarmReportSend, true, 5, 1}

/// ## S5F2
///
/// **Alarm Report Acknowledge**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// Acknowledge alarm report.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - [ACKC5]
///
/// [ACKC5]: AcknowledgeCode5
pub struct AlarmReportAcknowledge(pub AcknowledgeCode5);
message_data!{AlarmReportAcknowledge, false, 5, 2}

/// ## S5F3
///
/// **Enable/Disable Alarm Send**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
///
/// ---------------------------------------------------------------------------
///
/// Request to enable or disable a specific alarm.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - 2
///    1. [ALED]
///    2. [ALID]
///
/// [ALED]: AlarmEnableDisable
/// [ALID]: AlarmID
pub struct EnableDisableAlarmSend(pub (AlarmEnableDisable, AlarmID));
message_data!{EnableDisableAlarmSend, true, 5, 3}

/// ## S5F3
///
/// **Enable/Disable All Alarm Send**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
///
/// ---------------------------------------------------------------------------
///
/// Request to enable or disable all alarms.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - 2
///    1. [ALED]
///    2. List - 0
///
/// Zero-length N means to enable/disable all alarms.
///
/// [ALED]: AlarmEnableDisable
/// [ALID]: AlarmID
///
/// Note: User need to manually validate empty list, VecList<AlarmID> is a placeholder for now.
pub struct EnableDisableAllAlarmSend(pub (AlarmEnableDisable, AllAlarmID));
message_data!{EnableDisableAllAlarmSend, true, 5, 3}

/// ## S5F4
///
/// **Enable/Disable Alarm Acknowledge**
///
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// Acknowledge enable/disable alarm request.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - [ACKC5]
///
/// [ACKC5]: AcknowledgeCode5
pub struct EnableDisableAlarmAcknowledge(pub AcknowledgeCode5);
message_data!{EnableDisableAlarmAcknowledge, false, 5, 4}

/// ## S5F5
///
/// **List Alarms Request**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
///
/// ---------------------------------------------------------------------------
///
/// Request a list of alarms.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - N
///    - [ALID]
///
/// N is the number of alarm IDs.
///
/// Zero-length N means to report all enabled alarms.
///
/// [ALID]: AlarmID
pub struct ListAlarmsRequest(pub VecList<AlarmID>);
message_data!{ListAlarmsRequest, true, 5, 5}

/// ## S5F6
///
/// **List Alarms Data**
///
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// List of requested alarms.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - N
///    - List - 3
///       1. [ALCD]
///       2. [ALID]
///       3. [ALTX]
///
/// N is the number of alarms.
///
/// [ALCD]: AlarmCode
/// [ALID]: AlarmID
/// [ALTX]: AlarmText
pub struct ListAlarmsData(pub VecList<(AlarmCode, AlarmID, AlarmText)>);
message_data!{ListAlarmsData, false, 5, 6}

/// ## S5F7
///
/// **List Enabled Alarms Request**
///
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
///
/// ---------------------------------------------------------------------------
///
/// Request a list of all enabled alarms.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// Header only.
pub struct ListEnabledAlarmsRequest;
message_headeronly!{ListEnabledAlarmsRequest, true, 5, 7}

/// ## S5F8
///
/// **List Enabled Alarms Data**
///
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
///
/// ---------------------------------------------------------------------------
///
/// List of all enabled alarms.
///
/// ---------------------------------------------------------------------------
///
/// #### Structure
///
/// - List - N
///    - List - 3
///       1. [ALCD]
///       2. [ALID]
///       3. [ALTX]
///
/// N is the number of enabled alarms.
///
/// [ALCD]: AlarmCode
/// [ALID]: AlarmID
/// [ALTX]: AlarmText
pub struct ListEnabledAlarmsData(pub VecList<(AlarmCode, AlarmID, AlarmText)>);
message_data!{ListEnabledAlarmsData, false, 5, 8}
