/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 * Modified by igaisin 2025
 */

use crate::jab::jab_lib::packages::JObject64;

/**
 * Заголовочный файл, определяющий typedefs обратных вызовов для Windows процедур,
 * которые вызываются из Java (реагирование на события и т.д.).
 * */

pub(crate) type AccessBridgePropertyChangeFp = extern "system" fn(
    i32,        /*vmID*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*property*/
    *const u16, /*oldValue*/
    *const u16, /*newValue*/
);

pub(crate) type AccessBridgeJavaShutdownFp = extern "system" fn(i32 /*vm_id*/);

pub(crate) type AccessBridgeFocusGainedFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeFocusLostFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);

pub(crate) type AccessBridgeCaretUpdateFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);

pub(crate) type AccessBridgeMouseClickedFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMouseEnteredFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMouseExitedFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMousePressedFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMouseReleasedFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);

pub(crate) type AccessBridgeMenuCanceledFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMenuDeselectedFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMenuSelectedFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePopupMenuCanceledFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePopupMenuWillBecomeInvisibleFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePopupMenuWillBecomeVisibleFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);

pub(crate) type AccessBridgePropertyNameChangeFp = extern "system" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*oldName*/
    *const u16, /*newName*/
);
pub(crate) type AccessBridgePropertyDescriptionChangeFp = extern "system" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*oldDescription*/
    *const u16, /*newDescription*/
);
pub(crate) type AccessBridgePropertyStateChangeFp = extern "system" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*oldState*/
    *const u16, /*newState*/
);
pub(crate) type AccessBridgePropertyValueChangeFp = extern "system" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*oldValue*/
    *const u16, /*newValue*/
);
pub(crate) type AccessBridgePropertySelectionChangeFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePropertyTextChangeFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePropertyCaretChangeFp = extern "system" fn(
    i32,       /*vm_id*/
    JObject64, /*event*/
    JObject64, /*source*/
    i32,       /*oldPosition*/
    i32,       /*newPosition*/
);
pub(crate) type AccessBridgePropertyVisibleDataChangeFp =
    extern "system" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePropertyChildChangeFp = extern "system" fn(
    i32,       /*vm_id*/
    JObject64, /*event*/
    JObject64, /*source*/
    JObject64, /*oldChild*/
    JObject64, /*newChild*/
);
pub(crate) type AccessBridgePropertyActiveDescendentChangeFp = extern "system" fn(
    i32,       /*vm_id*/
    JObject64, /*event*/
    JObject64, /*source*/
    JObject64, /*oldActiveDescendent*/
    JObject64, /*newActiveDescendent*/
);

pub(crate) type AccessBridgePropertyTableModelChangeFp = extern "system" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*src*/
    *const u16, /*oldValue*/
    *const u16, /*newValue*/
);
