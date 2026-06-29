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

use crate::jab::context::AccessibleContext;
use std::sync::Arc;

pub type AccessibleContextType = Arc<AccessibleContext<'static>>;

pub(crate) enum AccessibleCallback {
    CaretUpdate(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    FocusGained(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MouseClicked(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MouseEntered(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MouseExited(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MousePressed(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MouseReleased(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MenuCanceled(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MenuDeselected(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MenuSelected(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PopupMenuCanceled(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PopupMenuWillBecomeInvisible(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PopupMenuWillBecomeVisible(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PropertySelectionChange(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PropertyTextChange(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PropertyVisibleDataChange(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PropertyChange(Box<dyn Fn(AccessibleContextType, String, String, String) + Sync + Send>),
    PropertyNameChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    PropertyDescriptionChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    PropertyStateChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    PropertyValueChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    PropertyCaretChange(Box<dyn Fn(AccessibleContextType, i32, i32) + Sync + Send>),
    PropertyChildChange(
        Box<
            dyn Fn(AccessibleContextType, AccessibleContextType, AccessibleContextType)
                + Sync
                + Send,
        >,
    ),
    PropertyActiveDescendentChange(
        Box<
            dyn Fn(AccessibleContextType, AccessibleContextType, AccessibleContextType)
                + Sync
                + Send,
        >,
    ),
    PropertyTableModelChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    JavaShutdown(Box<dyn Fn(i32) + Sync + Send>),
}
