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

pub(crate) mod callbacks;
pub(crate) mod packages;

use crate::jab::jab_lib::{
    callbacks::{
        AccessBridgeCaretUpdateFp, AccessBridgeFocusGainedFp, AccessBridgeFocusLostFp,
        AccessBridgeJavaShutdownFp, AccessBridgeMenuCanceledFp, AccessBridgeMenuDeselectedFp,
        AccessBridgeMenuSelectedFp, AccessBridgeMouseClickedFp, AccessBridgeMouseEnteredFp,
        AccessBridgeMouseExitedFp, AccessBridgeMousePressedFp, AccessBridgeMouseReleasedFp,
        AccessBridgePopupMenuCanceledFp, AccessBridgePopupMenuWillBecomeInvisibleFp,
        AccessBridgePopupMenuWillBecomeVisibleFp, AccessBridgePropertyActiveDescendentChangeFp,
        AccessBridgePropertyCaretChangeFp, AccessBridgePropertyChangeFp,
        AccessBridgePropertyChildChangeFp, AccessBridgePropertyDescriptionChangeFp,
        AccessBridgePropertyNameChangeFp, AccessBridgePropertySelectionChangeFp,
        AccessBridgePropertyStateChangeFp, AccessBridgePropertyTableModelChangeFp,
        AccessBridgePropertyTextChangeFp, AccessBridgePropertyValueChangeFp,
        AccessBridgePropertyVisibleDataChangeFp,
    },
    packages::{
        AccessBridgeVersionInfo, AccessibleActions, AccessibleActionsToDo, AccessibleContext,
        AccessibleContextInfo, AccessibleHyperlink, AccessibleHypertext, AccessibleHypertextInfo,
        AccessibleIcons, AccessibleKeyBindings, AccessibleRelationSetInfo, AccessibleSelection,
        AccessibleTable, AccessibleTableCellInfo, AccessibleTableInfo, AccessibleText,
        AccessibleTextAttributesInfo, AccessibleTextInfo, AccessibleTextItemsInfo,
        AccessibleTextRectInfo, AccessibleTextSelectionInfo, AccessibleValue, BOOL, JInt, JObject,
        JObject64, JavaObject, VisibleChildrenInfo,
    },
};
use crate::utils::SafeModuleHandle;
use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};
// use win_wrap::common::{FARPROC, HWND, Result, free_library, get_proc_address, load_library};
use windows::{
    Win32::Foundation::{FARPROC, FreeLibrary, HMODULE, HWND, S_FALSE},
    Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW},
    core::{Error as WinError, HSTRING, PCSTR},
};

static CUSTOM_SEARCH_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
static LIBRARY_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

#[cfg(all(target_arch = "x86"))]
const JAB_LIB_NAME: &str = "WindowsAccessBridge-32.dll";
#[cfg(all(target_arch = "x86_64"))]
const JAB_LIB_NAME: &str = "WindowsAccessBridge-64.dll";

/// Установить пользовательскую директорию для поиска
pub fn set_custom_directory(directory: PathBuf) {
    if CUSTOM_SEARCH_DIRECTORY.set(directory).is_err() {
        eprintln!("Директория поиска уже задана, и изменить её нельзя");
    }
}

/// Найти путь к библиотеке
pub(crate) fn find_library_path() -> Option<PathBuf> {
    // 1. Проверяем пользовательскую директорию (приоритет №1)
    if let Some(custom_dir) = CUSTOM_SEARCH_DIRECTORY.get() {
        let path = custom_dir.join(JAB_LIB_NAME);
        if path.exists() {
            return Some(path);
        }
    }

    // 2. Проверяем переменную окружения JAVA_HOME
    if let Ok(java_home) = env::var("JAVA_HOME") {
        let path = Path::new(&java_home).join("bin").join(JAB_LIB_NAME);
        if path.exists() {
            return Some(path);
        }
    }

    // 3. Фоллбэк: ищем в C:\Program Files\Java любую папку, начинающуюся с jre или jdk
    // Это корректная замена неработающему wildcard "jre1.8.0_*"
    let default_java_dir = PathBuf::from("C:\\Program Files\\Java");
    if default_java_dir.exists() {
        if let Ok(entries) = fs::read_dir(&default_java_dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();
                if name.starts_with("jre") || name.starts_with("jdk") {
                    let path = entry.path().join("bin").join(JAB_LIB_NAME);
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

/// Установить динамическую библиотеку для программы
pub fn is_setup_library() -> Result<(), String> {
    if let Some(path) = find_library_path() {
        LIBRARY_DIRECTORY
            .set(path.parent().unwrap().to_path_buf())
            .map_err(|_| "Не удалось установить путь к библиотеке".to_string())?;
        println!(
            "Библиотека установлена: {}",
            LIBRARY_DIRECTORY.get().unwrap().display()
        );
        Ok(())
    } else {
        Err(format!(
            "Не удалось найти библиотеку `{}` в указанной директории",
            JAB_LIB_NAME
        ))
    }
}

macro_rules! call_proc {
    ($module:expr,$name:ident,$def:ty,$($arg:expr),*) => {{
        let f = get_proc_address($module, stringify!($name));
        if !f.is_none() {
            unsafe {
                let r = (&*((&f) as *const FARPROC as *const $def)) ($($arg),*);
                Some(r)
            }
        } else {
            None
        }
    }};
}

macro_rules! jab {
    // Макрос для вызова функции Windows_run
    ($module:expr,windows_run) => {
        call_proc!($module, Windows_run, extern "system" fn() -> BOOL,)
    };
    // Макрос для вызова функции isJavaWindow
    ($module:expr,is_java_window,$h_wnd:expr) => {
        call_proc!(
            $module,
            isJavaWindow,
            extern "system" fn(HWND) -> BOOL,
            $h_wnd
        )
    };
    // Макрос для вызова функции getAccessibleContextFromHWND
    ($module:expr,get_accessible_context_from_hwnd,$target:expr,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getAccessibleContextFromHWND,
            extern "system" fn(HWND, *mut i32, *mut AccessibleContext) -> BOOL,
            $target,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции getHWNDFromAccessibleContext
    ($module:expr,get_hwnd__from_accessible_context,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getHWNDFromAccessibleContext,
            extern "system" fn(i32, AccessibleContext) -> HWND,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции releaseJavaObject
    ($module:expr,release_java_object,$vm_id:expr,$object:expr) => {
        call_proc!(
            $module,
            releaseJavaObject,
            extern "system" fn(i32, JavaObject),
            $vm_id,
            $object
        )
    };
    // Макрос для вызова функции getVersionInfo
    ($module:expr,get_version_info,$vm_id:expr,$info:expr) => {
        call_proc!(
            $module,
            getVersionInfo,
            extern "system" fn(i32, *mut AccessBridgeVersionInfo) -> BOOL,
            $vm_id,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleContextAt
    ($module:expr,get_accessible_context_at,$vm_id:expr,$parent:expr,$x:expr,$y:expr,$ac:expr) => {
        call_proc!(
            $module,
            getAccessibleContextAt,
            extern "system" fn(i32, AccessibleContext, JInt, JInt, *mut AccessibleContext) -> BOOL,
            $vm_id,
            $parent,
            $x,
            $y,
            $ac
        )
    };
    // Макрос для вызова функции getAccessibleContextWithFocus
    ($module:expr,get_accessible_context_with_focus,$window:expr,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getAccessibleContextWithFocus,
            extern "system" fn(HWND, *mut i32, *mut AccessibleContext) -> BOOL,
            $window,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции getAccessibleContextInfo
    ($module:expr,get_accessible_context_info,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleContextInfo,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleContextInfo) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleChildFromContext
    ($module:expr,get_accessible_child_from_context,$vm_id:expr,$ac:expr,$index:expr) => {
        call_proc!(
            $module,
            getAccessibleChildFromContext,
            extern "system" fn(i32, AccessibleContext, JInt) -> AccessibleContext,
            $vm_id,
            $ac,
            $index
        )
    };
    // Макрос для вызова функции getAccessibleParentFromContext
    ($module:expr,get_accessible_parent_from_context,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getAccessibleParentFromContext,
            extern "system" fn(i32, AccessibleContext) -> AccessibleContext,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции isSameObject
    ($module:expr,is_same_object,$vm_id:expr,$obj1:expr,$obj2:expr) => {
        call_proc!(
            $module,
            isSameObject,
            extern "system" fn(i32, JObject64, JObject64) -> BOOL,
            $vm_id,
            $obj1,
            $obj2
        )
    };
    // Макрос для вызова функции getParentWithRole
    ($module:expr,get_parent_with_role,$vm_id:expr,$ac:expr,$role:expr) => {
        call_proc!(
            $module,
            getParentWithRole,
            extern "system" fn(i32, AccessibleContext, *const u16) -> AccessibleContext,
            $vm_id,
            $ac,
            $role
        )
    };
    // Макрос для вызова функции getParentWithRoleElseRoot
    ($module:expr,get_parent_with_role_else_root,$vm_id:expr,$ac:expr,$role:expr) => {
        call_proc!(
            $module,
            getParentWithRoleElseRoot,
            extern "system" fn(i32, AccessibleContext, *const u16) -> AccessibleContext,
            $vm_id,
            $ac,
            $role
        )
    };
    // Макрос для вызова функции getTopLevelObject
    ($module:expr,get_top_level_object,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getTopLevelObject,
            extern "system" fn(i32, AccessibleContext) -> AccessibleContext,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции getObjectDepth
    ($module:expr,get_object_depth,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getObjectDepth,
            extern "system" fn(i32, AccessibleContext) -> i32,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции getActiveDescendent
    ($module:expr,get_active_descendent,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getActiveDescendent,
            extern "system" fn(i32, AccessibleContext) -> AccessibleContext,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции requestFocus
    ($module:expr,request_focus,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            requestFocus,
            extern "system" fn(i32, AccessibleContext) -> BOOL,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции getVisibleChildrenCount
    ($module:expr,get_visible_children_count,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getVisibleChildrenCount,
            extern "system" fn(i32, AccessibleContext) -> i32,
            $vm_id,
            $ac
        )
    };
    // Макрос для вызова функции getVisibleChildren
    ($module:expr,get_visible_children,$vm_id:expr,$ac:expr,$start:expr,$info:expr) => {
        call_proc!(
            $module,
            getVisibleChildren,
            extern "system" fn(i32, AccessibleContext, i32, *mut VisibleChildrenInfo) -> BOOL,
            $vm_id,
            $ac,
            $start,
            $info
        )
    };
    // Макрос для вызова функции getEventsWaiting
    ($module:expr,get_events_waiting) => {
        call_proc!($module, getEventsWaiting, extern "system" fn() -> i32,)
    };
    // Макрос для вызова функции getAccessibleActions
    ($module:expr,get_accessible_actions,$vm_id:expr,$ac:expr,$actions:expr) => {
        call_proc!(
            $module,
            getAccessibleActions,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleActions) -> BOOL,
            $vm_id,
            $ac,
            $actions
        )
    };
    // Макрос для вызова функции getCaretLocation
    ($module:expr,get_caret_location,$vm_id:expr,$ac:expr,$info:expr,$index:expr) => {
        call_proc!(
            $module,
            getCaretLocation,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleTextRectInfo, JInt) -> BOOL,
            $vm_id,
            $ac,
            $info,
            $index
        )
    };
    // Макрос для вызова функции setCaretPosition
    ($module:expr,set_caret_position,$vm_id:expr,$ac:expr,$position:expr) => {
        call_proc!(
            $module,
            setCaretPosition,
            extern "system" fn(i32, AccessibleContext, i32) -> BOOL,
            $vm_id,
            $ac,
            $position
        )
    };
    // Макрос для вызова функции getTextAttributesInRange
    ($module:expr,get_text_attributes_in_range,$vm_id:expr,$ac:expr,$start_index:expr,$end_index:expr,$info:expr,$len:expr) => {
        call_proc!(
            $module,
            getTextAttributesInRange,
            extern "system" fn(
                i32,
                AccessibleContext,
                i32,
                i32,
                *mut AccessibleTextAttributesInfo,
                *mut i16,
            ) -> BOOL,
            $vm_id,
            $ac,
            $start_index,
            $end_index,
            $info,
            $len
        )
    };
    // Макрос для вызова функции getAccessibleRelationSet
    ($module:expr,get_accessible_relation_set,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleRelationSet,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleRelationSetInfo) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleKeyBindings
    ($module:expr,get_accessible_key_bindings,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleKeyBindings,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleKeyBindings) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleIcons
    ($module:expr,get_accessible_icons,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleIcons,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleIcons) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleTableRowHeader
    ($module:expr,get_accessible_table_row_header,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleTableRowHeader,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleTableInfo) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleTableColumnHeader
    ($module:expr,get_accessible_table_column_header,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleTableColumnHeader,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleTableInfo) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleTableColumnDescription
    ($module:expr,get_accessible_table_column_description,$vm_id:expr,$ac:expr,$column:expr) => {
        call_proc!(
            $module,
            getAccessibleTableColumnDescription,
            extern "system" fn(i32, AccessibleContext, JInt) -> AccessibleContext,
            $vm_id,
            $ac,
            $column
        )
    };
    // Макрос для вызова функции getAccessibleTableRowDescription
    ($module:expr,get_accessible_table_row_description,$vm_id:expr,$ac:expr,$row:expr) => {
        call_proc!(
            $module,
            getAccessibleTableRowDescription,
            extern "system" fn(i32, AccessibleContext, JInt) -> AccessibleContext,
            $vm_id,
            $ac,
            $row
        )
    };
    // Макрос для вызова функции selectTextRange
    ($module:expr,select_text_range,$vm_id:expr,$ac:expr,$start_index:expr,$end_index:expr) => {
        call_proc!(
            $module,
            selectTextRange,
            extern "system" fn(i32, AccessibleContext, JInt, JInt) -> BOOL,
            $vm_id,
            $ac,
            $start_index,
            $end_index
        )
    };
    // Макрос для вызова функции getAccessibleTableInfo
    ($module:expr,get_accessible_table_info,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleTableInfo,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleTableInfo) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    // Макрос для вызова функции getVirtualAccessibleName
    ($module:expr,get_virtual_accessible_name,$vm_id:expr,$ac:expr,$name:expr,$len:expr) => {
        call_proc!(
            $module,
            getVirtualAccessibleName,
            extern "system" fn(i32, AccessibleContext, *mut u16, i32) -> BOOL,
            $vm_id,
            $ac,
            $name,
            $len
        )
    };
    // Макрос для вызова функции getAccessibleHypertext
    ($module:expr,get_accessible_hypertext,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleHypertext,
            extern "system" fn(i32, AccessibleContext, *mut AccessibleHypertextInfo) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleHypertextExt
    ($module:expr,get_accessible_hypertext_ext,$vm_id:expr,$ac:expr,$start_index:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleHypertextExt,
            extern "system" fn(i32, AccessibleContext, JInt, *mut AccessibleHypertextInfo) -> BOOL,
            $vm_id,
            $ac,
            $start_index,
            $info
        )
    };
    // Макрос для вызова функции doAccessibleActions
    ($module:expr,do_accessible_actions,$vm_id:expr,$ac:expr,$actions_to_do:expr,$failure:expr) => {
        call_proc!(
            $module,
            doAccessibleActions,
            extern "system" fn(
                i32,
                AccessibleContext,
                *const AccessibleActionsToDo,
                *mut JInt,
            ) -> BOOL,
            $vm_id,
            $ac,
            $actions_to_do,
            $failure
        )
    };
    // Макрос для вызова функции setTextContents
    ($module:expr,set_text_contents,$vm_id:expr,$ac:expr,$text:expr) => {
        call_proc!(
            $module,
            setTextContents,
            extern "system" fn(i32, AccessibleContext, *const u16) -> BOOL,
            $vm_id,
            $ac,
            $text
        )
    };
    // Макрос для вызова функции setCaretUpdateFP
    ($module:expr,set_caret_update_fp,$cb:expr) => {
        call_proc!(
            $module,
            setCaretUpdateFP,
            extern "system" fn(AccessBridgeCaretUpdateFp),
            $cb
        )
    };
    // Макрос для вызова функции setFocusGainedFP
    ($module:expr,set_focus_gained_fp,$cb:expr) => {
        call_proc!(
            $module,
            setFocusGainedFP,
            extern "system" fn(AccessBridgeFocusGainedFp),
            $cb
        )
    };
    // Макрос для вызова функции setFocusLostFP
    ($module:expr,set_focus_lost_fp,$cb:expr) => {
        call_proc!(
            $module,
            setFocusLostFP,
            extern "system" fn(AccessBridgeFocusLostFp),
            $cb
        )
    };
    // Макрос для вызова функции setJavaShutdownFP
    ($module:expr,set_java_shutdown_fp,$cb:expr) => {
        call_proc!(
            $module,
            setJavaShutdownFP,
            extern "system" fn(AccessBridgeJavaShutdownFp),
            $cb
        )
    };
    // Макрос для вызова функции setMenuCanceledFP
    ($module:expr,set_menu_canceled_fp,$cb:expr) => {
        call_proc!(
            $module,
            setMenuCanceledFP,
            extern "system" fn(AccessBridgeMenuCanceledFp),
            $cb
        )
    };
    // Макрос для вызова функции setMenuDeselectedFP
    ($module:expr,set_menu_deselected_fp,$cb:expr) => {
        call_proc!(
            $module,
            setMenuDeselectedFP,
            extern "system" fn(AccessBridgeMenuDeselectedFp),
            $cb
        )
    };
    // Макрос для вызова функции setMenuSelectedFP
    ($module:expr,set_menu_selected_fp,$cb:expr) => {
        call_proc!(
            $module,
            setMenuSelectedFP,
            extern "system" fn(AccessBridgeMenuSelectedFp),
            $cb
        )
    };
    // Макрос для вызова функции setMouseClickedFP
    ($module:expr,set_mouse_clicked_fp,$cb:expr) => {
        call_proc!(
            $module,
            setMouseClickedFP,
            extern "system" fn(AccessBridgeMouseClickedFp),
            $cb
        )
    };
    // Макрос для вызова функции setMouseEnteredFP
    ($module:expr,set_mouse_entered_fp,$cb:expr) => {
        call_proc!(
            $module,
            setMouseEnteredFP,
            extern "system" fn(AccessBridgeMouseEnteredFp),
            $cb
        )
    };
    // Макрос для вызова функции setMouseExitedFP
    ($module:expr,set_mouse_exited_fp,$cb:expr) => {
        call_proc!(
            $module,
            setMouseExitedFP,
            extern "system" fn(AccessBridgeMouseExitedFp),
            $cb
        )
    };
    // Макрос для вызова функции setMousePressedFP
    ($module:expr,set_mouse_pressed_fp,$cb:expr) => {
        call_proc!(
            $module,
            setMousePressedFP,
            extern "system" fn(AccessBridgeMousePressedFp),
            $cb
        )
    };
    // Макрос для вызова функции setMouseReleasedFP
    ($module:expr,set_mouse_released_fp,$cb:expr) => {
        call_proc!(
            $module,
            setMouseReleasedFP,
            extern "system" fn(AccessBridgeMouseReleasedFp),
            $cb
        )
    };
    // Макрос для вызова функции setPopupMenuCanceledFP
    ($module:expr,set_popup_menu_canceled_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPopupMenuCanceledFP,
            extern "system" fn(AccessBridgePopupMenuCanceledFp),
            $cb
        )
    };
    // Макрос для вызова функции setPopupMenuWillBecomeInvisibleFP
    ($module:expr,set_popup_menu_will_become_invisible_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPopupMenuWillBecomeInvisibleFP,
            extern "system" fn(AccessBridgePopupMenuWillBecomeInvisibleFp),
            $cb
        )
    };
    // Макрос для вызова функции setPopupMenuWillBecomeVisibleFP
    ($module:expr,set_popup_menu_will_become_visible_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPopupMenuWillBecomeVisibleFP,
            extern "system" fn(AccessBridgePopupMenuWillBecomeVisibleFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyActiveDescendentChangeFP
    ($module:expr,set_property_active_descendent_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyActiveDescendentChangeFP,
            extern "system" fn(AccessBridgePropertyActiveDescendentChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyCaretChangeFP
    ($module:expr,set_property_caret_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyCaretChangeFP,
            extern "system" fn(AccessBridgePropertyCaretChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyChangeFP
    ($module:expr,set_property_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyChangeFP,
            extern "system" fn(AccessBridgePropertyChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyChildChangeFP
    ($module:expr,set_property_child_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyChildChangeFP,
            extern "system" fn(AccessBridgePropertyChildChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyDescriptionChangeFP
    ($module:expr,set_property_description_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyDescriptionChangeFP,
            extern "system" fn(AccessBridgePropertyDescriptionChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyNameChangeFP
    ($module:expr,set_property_name_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyNameChangeFP,
            extern "system" fn(AccessBridgePropertyNameChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertySelectionChangeFP
    ($module:expr,set_property_selection_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertySelectionChangeFP,
            extern "system" fn(AccessBridgePropertySelectionChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyStateChangeFP
    ($module:expr,set_property_state_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyStateChangeFP,
            extern "system" fn(AccessBridgePropertyStateChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyTableModelChangeFP
    ($module:expr,set_property_table_model_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyTableModelChangeFP,
            extern "system" fn(AccessBridgePropertyTableModelChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyTextChangeFP
    ($module:expr,set_property_text_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyTextChangeFP,
            extern "system" fn(AccessBridgePropertyTextChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyValueChangeFP
    ($module:expr,set_property_value_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyValueChangeFP,
            extern "system" fn(AccessBridgePropertyValueChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции setPropertyVisibleDataChangeFP
    ($module:expr,set_property_visible_data_change_fp,$cb:expr) => {
        call_proc!(
            $module,
            setPropertyVisibleDataChangeFP,
            extern "system" fn(AccessBridgePropertyVisibleDataChangeFp),
            $cb
        )
    };
    // Макрос для вызова функции activateAccessibleHyperlink
    ($module:expr,activate_accessible_hyperlink,$vm_id:expr,$ac:expr,$link:expr) => {
        call_proc!(
            $module,
            activateAccessibleHyperlink,
            extern "system" fn(i32, AccessibleContext, AccessibleHyperlink) -> BOOL,
            $vm_id,
            $ac,
            $link
        )
    };
    // Макрос для вызова функции addAccessibleSelectionFromContext
    ($module:expr,add_accessible_selection_from_context,$vm_id:expr,$as:expr,$index:expr) => {
        call_proc!(
            $module,
            addAccessibleSelectionFromContext,
            extern "system" fn(i32, AccessibleSelection, i32),
            $vm_id,
            $as,
            $index
        )
    };
    // Макрос для вызова функции removeAccessibleSelectionFromContext
    ($module:expr,remove_accessible_selection_from_context,$vm_id:expr,$as:expr,$index:expr) => {
        call_proc!(
            $module,
            removeAccessibleSelectionFromContext,
            extern "system" fn(i32, AccessibleSelection, i32),
            $vm_id,
            $as,
            $index
        )
    };
    // Макрос для вызова функции clearAccessibleSelectionFromContext
    ($module:expr,clear_accessible_selection_from_context,$vm_id:expr,$as:expr) => {
        call_proc!(
            $module,
            clearAccessibleSelectionFromContext,
            extern "system" fn(i32, AccessibleSelection),
            $vm_id,
            $as
        )
    };
    // Макрос для вызова функции selectAllAccessibleSelectionFromContext
    ($module:expr,select_all_accessible_selection_from_context,$vm_id:expr,$as:expr) => {
        call_proc!(
            $module,
            selectAllAccessibleSelectionFromContext,
            extern "system" fn(i32, AccessibleSelection),
            $vm_id,
            $as
        )
    };
    // Макрос для вызова функции getAccessibleHyperlink
    ($module:expr,get_accessible_hyperlink,$vm_id:expr,$ah:expr,$index:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleHyperlink,
            extern "system" fn(
                i32,
                AccessibleHypertext,
                JInt,
                *mut AccessibleHypertextInfo,
            ) -> BOOL,
            $vm_id,
            $ah,
            $index,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleHyperlinkCount
    ($module:expr,get_accessible_hyperlink_count,$vm_id:expr,$ah:expr) => {
        call_proc!(
            $module,
            getAccessibleHyperlinkCount,
            extern "system" fn(i32, AccessibleHypertext) -> JInt,
            $vm_id,
            $ah
        )
    };
    // Макрос для вызова функции getAccessibleHypertextLinkIndex
    ($module:expr,get_accessible_hypertext_link_index,$vm_id:expr,$ah:expr,$index:expr) => {
        call_proc!(
            $module,
            getAccessibleHypertextLinkIndex,
            extern "system" fn(i32, AccessibleHypertext, JInt) -> JInt,
            $vm_id,
            $ah,
            $index
        )
    };
    // Макрос для вызова функции getAccessibleSelectionCountFromContext
    ($module:expr,get_accessible_selection_count_from_context,$vm_id:expr,$as:expr) => {
        call_proc!(
            $module,
            getAccessibleSelectionCountFromContext,
            extern "system" fn(i32, AccessibleSelection) -> i32,
            $vm_id,
            $as
        )
    };
    // Макрос для вызова функции getAccessibleSelectionFromContext
    ($module:expr,get_accessible_selection_from_context,$vm_id:expr,$as:expr,$index:expr) => {
        call_proc!(
            $module,
            getAccessibleSelectionFromContext,
            extern "system" fn(i32, AccessibleSelection, i32) -> JObject,
            $vm_id,
            $as,
            $index
        )
    };
    // Макрос для вызова функции isAccessibleChildSelectedFromContext
    ($module:expr,is_accessible_child_selected_from_context,$vm_id:expr,$as:expr,$index:expr) => {
        call_proc!(
            $module,
            isAccessibleChildSelectedFromContext,
            extern "system" fn(i32, AccessibleSelection, i32) -> BOOL,
            $vm_id,
            $as,
            $index
        )
    };
    // Макрос для вызова функции isAccessibleTableRowSelected
    ($module:expr,is_accessible_table_row_selected,$vm_id:expr,$at:expr,$row:expr) => {
        call_proc!(
            $module,
            isAccessibleTableRowSelected,
            extern "system" fn(i32, AccessibleTable, JInt) -> BOOL,
            $vm_id,
            $at,
            $row
        )
    };
    // Макрос для вызова функции isAccessibleTableColumnSelected
    ($module:expr,is_accessible_table_column_selected,$vm_id:expr,$at:expr,$column:expr) => {
        call_proc!(
            $module,
            isAccessibleTableColumnSelected,
            extern "system" fn(i32, AccessibleTable, JInt) -> BOOL,
            $vm_id,
            $at,
            $column
        )
    };
    // Макрос для вызова функции getAccessibleTableCellInfo
    ($module:expr,get_accessible_table_cell_info,$vm_id:expr,$at:expr,$row:expr,$column:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleTableCellInfo,
            extern "system" fn(
                i32,
                AccessibleTable,
                JInt,
                JInt,
                *mut AccessibleTableCellInfo,
            ) -> BOOL,
            $vm_id,
            $at,
            $row,
            $column,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleTableColumn
    ($module:expr,get_accessible_table_column,$vm_id:expr,$at:expr,$index:expr) => {
        call_proc!(
            $module,
            getAccessibleTableColumn,
            extern "system" fn(i32, AccessibleTable, JInt) -> JInt,
            $vm_id,
            $at,
            $index
        )
    };
    // Макрос для вызова функции getAccessibleTableRow
    ($module:expr,get_accessible_table_row,$vm_id:expr,$at:expr,$index:expr) => {
        call_proc!(
            $module,
            getAccessibleTableRow,
            extern "system" fn(i32, AccessibleTable, JInt) -> JInt,
            $vm_id,
            $at,
            $index
        )
    };
    // Макрос для вызова функции getAccessibleTableColumnSelectionCount
    ($module:expr,get_accessible_table_column_selection_count,$vm_id:expr,$at:expr) => {
        call_proc!(
            $module,
            getAccessibleTableColumnSelectionCount,
            extern "system" fn(i32, AccessibleTable) -> JInt,
            $vm_id,
            $at
        )
    };
    // Макрос для вызова функции getAccessibleTableRowSelectionCount
    ($module:expr,get_accessible_table_row_selection_count,$vm_id:expr,$at:expr) => {
        call_proc!(
            $module,
            getAccessibleTableRowSelectionCount,
            extern "system" fn(i32, AccessibleTable) -> JInt,
            $vm_id,
            $at
        )
    };
    // Макрос для вызова функции getAccessibleTableIndex
    ($module:expr,get_accessible_table_index,$vm_id:expr,$at:expr,$row:expr,$column:expr) => {
        call_proc!(
            $module,
            getAccessibleTableIndex,
            extern "system" fn(i32, AccessibleTable, JInt, JInt) -> JInt,
            $vm_id,
            $at,
            $row,
            $column
        )
    };
    // Макрос для вызова функции getAccessibleTableColumnSelections
    ($module:expr,get_accessible_table_column_selections,$vm_id:expr,$at:expr,$count:expr,$selections:expr) => {
        call_proc!(
            $module,
            getAccessibleTableColumnSelections,
            extern "system" fn(i32, AccessibleTable, JInt, *mut JInt) -> BOOL,
            $vm_id,
            $at,
            $count,
            $selections
        )
    };
    // Макрос для вызова функции getAccessibleTableRowSelections
    ($module:expr,get_accessible_table_row_selections,$vm_id:expr,$at:expr,$count:expr,$selections:expr) => {
        call_proc!(
            $module,
            getAccessibleTableRowSelections,
            extern "system" fn(i32, AccessibleTable, JInt, *mut JInt) -> BOOL,
            $vm_id,
            $at,
            $count,
            $selections
        )
    };
    // Макрос для вызова функции getAccessibleTextSelectionInfo
    ($module:expr,get_accessible_text_selection_info,$vm_id:expr,$at:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleTextSelectionInfo,
            extern "system" fn(i32, AccessibleText, *mut AccessibleTextSelectionInfo) -> BOOL,
            $vm_id,
            $at,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleTextInfo
    ($module:expr,get_accessible_text_info,$vm_id:expr,$at:expr,$info:expr,$x:expr,$y:expr) => {
        call_proc!(
            $module,
            getAccessibleTextInfo,
            extern "system" fn(i32, AccessibleText, *mut AccessibleTextInfo, JInt, JInt) -> BOOL,
            $vm_id,
            $at,
            $info,
            $x,
            $y
        )
    };
    // Макрос для вызова функции getAccessibleTextAttributes
    ($module:expr,get_accessible_text_attributes,$vm_id:expr,$at:expr,$index:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleTextAttributes,
            extern "system" fn(
                i32,
                AccessibleText,
                JInt,
                *mut AccessibleTextAttributesInfo,
            ) -> *const u8,
            $vm_id,
            $at,
            $index,
            $info
        )
    };
    // Макрос для вызова функции getAccessibleTextItems
    ($module:expr,get_accessible_text_items,$vm_id:expr,$at:expr,$info:expr,$index:expr) => {
        call_proc!(
            $module,
            getAccessibleTextItems,
            extern "system" fn(i32, AccessibleText, *mut AccessibleTextItemsInfo, JInt) -> BOOL,
            $vm_id,
            $at,
            $info,
            $index
        )
    };
    // Макрос для вызова функции getAccessibleTextLineBounds
    ($module:expr,get_accessible_text_line_bounds,$vm_id:expr,$at:expr,$index:expr,$start_index:expr,$end_index:expr) => {
        call_proc!(
            $module,
            getAccessibleTextLineBounds,
            extern "system" fn(i32, AccessibleText, JInt, *mut JInt, *mut JInt) -> BOOL,
            $vm_id,
            $at,
            $index,
            $start_index,
            $end_index
        )
    };
    // Макрос для вызова функции getAccessibleTextRange
    ($module:expr,get_accessible_text_range,$vm_id:expr,$at:expr,$start_index:expr,$end_index:expr,$text:expr,$len:expr) => {
        call_proc!(
            $module,
            getAccessibleTextRange,
            extern "system" fn(i32, AccessibleText, JInt, JInt, *mut u16, i16) -> BOOL,
            $vm_id,
            $at,
            $start_index,
            $end_index,
            $text,
            $len
        )
    };
    // Макрос для вызова функции getAccessibleTextRect
    ($module:expr,get_accessible_text_rect,$vm_id:expr,$at:expr,$info:expr,$index:expr) => {
        call_proc!(
            $module,
            getAccessibleTextRect,
            extern "system" fn(i32, AccessibleText, *mut AccessibleTextRectInfo, JInt) -> BOOL,
            $vm_id,
            $at,
            $info,
            $index
        )
    };
    // Макрос для вызова функции getCurrentAccessibleValueFromContext
    ($module:expr,get_current_accessible_value_from_context,$vm_id:expr,$av:expr,$value:expr,$len:expr) => {
        call_proc!(
            $module,
            getCurrentAccessibleValueFromContext,
            extern "system" fn(i32, AccessibleValue, *mut u16, i16) -> BOOL,
            $vm_id,
            $av,
            $value,
            $len
        )
    };
    // Макрос для вызова функции getMaximumAccessibleValueFromContext
    ($module:expr,get_maximum_accessible_value_from_context,$vm_id:expr,$av:expr,$value:expr,$len:expr) => {
        call_proc!(
            $module,
            getMaximumAccessibleValueFromContext,
            extern "system" fn(i32, AccessibleValue, *mut u16, i16) -> BOOL,
            $vm_id,
            $av,
            $value,
            $len
        )
    };
    // Макрос для вызова функции getMinimumAccessibleValueFromContext
    ($module:expr,get_minimum_accessible_value_from_context,$vm_id:expr,$av:expr,$value:expr,$len:expr) => {
        call_proc!(
            $module,
            getMinimumAccessibleValueFromContext,
            extern "system" fn(i32, AccessibleValue, *mut u16, i16) -> BOOL,
            $vm_id,
            $av,
            $value,
            $len
        )
    };
}

/**
Извлекает адрес экспортированной функции (также известной как процедура) или переменной из указанной динамически подключаемой библиотеки (DLL).
`h_module` - Дескриптор модуля DLL, содержащего функцию или переменную. Этот дескриптор возвращается функциями LoadLibrary, LoadLibraryEx, LoadPackagedLibrary или GetModuleHandle.
            Функция GetProcAddress не извлекает адреса из модулей, загруженных с флагом LOAD_LIBRARY_AS_DATAFILE. Подробнее см. в документации LoadLibraryEx.
`proc_name` - Имя функции или переменной, либо порядковое значение функции. Если этот параметр является порядковым значением, оно должно находиться в младшем слове (low-order word);
            старшее слово (high-order word) должно быть равно нулю.
*/
pub fn get_proc_address(h_module: HMODULE, proc_name: &str) -> FARPROC {
    let name = CString::new(proc_name).unwrap();
    unsafe { GetProcAddress(h_module, PCSTR::from_raw(name.as_ptr().cast())) }
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct JabLib {
    h_module: SafeModuleHandle,
}

#[allow(dead_code)]
impl JabLib {
    //noinspection RsUnresolvedPath
    //noinspection SpellCheckingInspection
    pub(crate) fn new(lib_path: PathBuf) -> windows::core::Result<Self> {
        // Безопасное преобразование в UTF-16 для Windows API
        let wide_path: Vec<u16> = lib_path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let h_module = unsafe { LoadLibraryW(windows::core::PCWSTR(wide_path.as_ptr())) }
            .map_err(|e| WinError::new(S_FALSE, &format!("Ошибка LoadLibraryW: {}", e)))?;

        let safe_handle = SafeModuleHandle::new(h_module);

        let res = jab!(*safe_handle, windows_run);
        if res.is_none() {
            return Err(WinError::new(
                S_FALSE,
                "Не удается инициализировать JAB (windows_run не найден).",
            ));
        }

        Ok(Self {
            h_module: safe_handle,
        })
    }

    /**
    Проверяет, реализует ли данное окно Java Accessibility API.
    */
    pub(crate) fn is_java_window(&self, h_wnd: HWND) -> bool {
        jab!(*self.h_module, is_java_window, h_wnd).unwrap_or(0) != 0
    }

    /**
    Получает AccessibleContext и vmID для данного окна. Многие функции Java Access Bridge требуют значения AccessibleContext и vmID.
    `target` Целевое окно.
    */
    pub(crate) fn get_accessible_context_from_hwnd(
        &self,
        target: HWND,
    ) -> Option<(i32, AccessibleContext)> {
        let (mut context, mut vm_id) = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_context_from_hwnd,
            target,
            &mut vm_id,
            &mut context
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some((vm_id, context))
    }

    /**
    Возвращает HWND из AccessibleContext верхнего уровня окна.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_hwnd_from_accessible_context(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> HWND {
        jab!(*self.h_module, get_hwnd__from_accessible_context, vm_id, ac)
            .unwrap_or(HWND::default())
    }

    /**
    Освобождает память, используемую объектом Java, где object - это объект, возвращенный Java Access Bridge. Java Access Bridge автоматически поддерживает ссылки на все объекты Java, которые он возвращает вам в JVM, чтобы они не были собраны мусором. Чтобы предотвратить утечки памяти, вызывайте их после завершения работы с объектами Java, возвращенными Java Access Bridge.
    `object` Объект Java.
    */
    pub(crate) fn release_java_object(&self, vm_id: i32, object: JavaObject) {
        jab!(*self.h_module, release_java_object, vm_id, object);
    }

    /**
    Получает информацию о версии экземпляра Java Access Bridge, используемого приложением. Вы можете использовать эту информацию, чтобы определить доступные функции вашей версии Java Access Bridge. Примечание: чтобы определить версию JVM, вам нужно передать действительный vm_id; в противном случае возвращается только версия файла WindowsAccessBridge.DLL, к которому подключено приложение.
    `vm_id` Идентификатор виртуальной машины.
    */
    pub(crate) fn get_version_info(&self, vm_id: i32) -> Option<AccessBridgeVersionInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(*self.h_module, get_version_info, vm_id, &mut info).unwrap_or(0) == 0 {
            return None;
        }
        Some(info)
    }

    /**
    Запрашивает объект AccessibleContext окна или объекта под указателем мыши.
    `parent` Родительский объект.
    `x` X координата.
    `y` Y координата.
    */
    pub(crate) fn get_accessible_context_at(
        &self,
        vm_id: i32,
        parent: AccessibleContext,
        x: JInt,
        y: JInt,
    ) -> Option<AccessibleContext> {
        let mut ac = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_context_at,
            vm_id,
            parent,
            x,
            y,
            &mut ac
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(ac)
    }

    //noinspection StructuralWrap
    /**
    Запрашивает объект AccessibleContext окна или объекта с фокусом.
    `window` Дескриптор окна для запроса.
    */
    pub(crate) fn get_accessible_context_with_focus(
        &self,
        window: HWND,
    ) -> Option<(i32, AccessibleContext)> {
        let (mut vm_id, mut ac) = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_context_with_focus,
            window,
            &mut vm_id,
            &mut ac
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some((vm_id, ac))
    }

    /**
    Запрашивает объект AccessibleContextInfo из объекта AccessibleContext.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_context_info(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_context_info,
            vm_id,
            ac,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Возвращает объект, представляющий n-й дочерний объект данного объекта AccessibleContext, где n указан значением index.
    `ac` Контекст доступности.
    `vm_id` Идентификатор виртуальной машины.
    `index` Индекс дочернего объекта.
    */
    pub(crate) fn get_accessible_child_from_context(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        index: JInt,
    ) -> Option<AccessibleContext> {
        jab!(
            *self.h_module,
            get_accessible_child_from_context,
            vm_id,
            ac,
            index
        )
    }

    /**
    Возвращает объект, представляющий родительский объект данного объекта AccessibleContext.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_parent_from_context(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContext> {
        jab!(
            *self.h_module,
            get_accessible_parent_from_context,
            vm_id,
            ac
        )
    }

    /**
    Возвращает, ссылаются ли два объекта на один и тот же объект.
    `vm_id` Идентификатор виртуальной машины.
    `obj1` Объект 1.
    `obj2` Объект 2.
    */
    pub(crate) fn is_same_object(&self, vm_id: i32, obj1: JObject64, obj2: JObject64) -> bool {
        jab!(*self.h_module, is_same_object, vm_id, obj1, obj2).unwrap_or(0) != 0
    }

    /**
    Возвращает объект AccessibleContext с указанной ролью, который является предком данного объекта. Роль является одной из строк ролей, определенных в структуре данных Java Access Bridge API. Если объект предка с указанной ролью не существует, возвращается (AccessibleContext)0.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `role` Строка роли.
    */
    pub(crate) fn get_parent_with_role(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        role: &str,
    ) -> Option<AccessibleContext> {
        jab!(
            *self.h_module,
            get_parent_with_role,
            vm_id,
            ac,
            HSTRING::from(role).as_ptr()
        )
    }

    /**
    Возвращает объект AccessibleContext с указанной ролью, который является предком данного объекта. Если объект с указанной ролью не существует, возвращается объект верхнего уровня Java окна. В случае ошибки возвращается (AccessibleContext)0.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `role` Строка роли.
    */
    pub(crate) fn get_parent_with_role_else_root(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        role: &str,
    ) -> Option<AccessibleContext> {
        jab!(
            *self.h_module,
            get_parent_with_role_else_root,
            vm_id,
            ac,
            HSTRING::from(role).as_ptr()
        )
    }

    /**
    Возвращает объект AccessibleContext верхнего уровня Java окна. Это то же самое, что и AccessibleContext, полученный из get_accessible_context_from_hwnd для этого окна. В случае ошибки возвращается (AccessibleContext)0.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_top_level_object(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContext> {
        jab!(*self.h_module, get_top_level_object, vm_id, ac)
    }

    //noinspection StructuralWrap
    /**
    Возвращает глубину объекта в иерархии объектов. Глубина объекта на вершине иерархии объектов равна 0. В случае ошибки возвращается -1.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_object_depth(&self, vm_id: i32, ac: AccessibleContext) -> i32 {
        jab!(*self.h_module, get_object_depth, vm_id, ac).unwrap_or(-1)
    }

    /**
    Возвращает объект AccessibleContext текущего ActiveDescendent объекта. Этот метод предполагает, что ActiveDescendent - это компонент, который в настоящее время выбран в контейнерном объекте. В случае ошибки или отсутствия выбора возвращается (AccessibleContext)0.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_active_descendent(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContext> {
        jab!(*self.h_module, get_active_descendent, vm_id, ac)
    }

    /**
    Запросить фокус для компонента. Возвращает true, если успешно.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn request_focus(&self, vm_id: i32, ac: AccessibleContext) -> bool {
        jab!(*self.h_module, request_focus, vm_id, ac).unwrap_or(0) != 0
    }

    //noinspection StructuralWrap
    /**
    Возвращает количество видимых дочерних компонентов. В случае ошибки возвращается -1.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_visible_children_count(&self, vm_id: i32, ac: AccessibleContext) -> i32 {
        jab!(*self.h_module, get_visible_children_count, vm_id, ac).unwrap_or(-1)
    }

    /**
    Получает видимых дочерних компонентов для AccessibleContext.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `start_index` Начальный индекс.
    */
    pub(crate) fn get_visible_children(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        start_index: i32,
    ) -> Option<VisibleChildrenInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_visible_children,
            vm_id,
            ac,
            start_index,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Получает количество ожидающих событий.
    */
    pub(crate) fn get_events_waiting(&self) -> i32 {
        jab!(*self.h_module, get_events_waiting).unwrap_or(0)
    }

    /**
    Возвращает список действий, которые может выполнить компонент.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_actions(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleActions> {
        let mut actions = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_actions,
            vm_id,
            ac,
            &mut actions
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(actions)
    }

    /**
    Получает местоположение каретки текста.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `index` Индекс.
    */
    pub(crate) fn get_caret_location(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        index: JInt,
    ) -> Option<AccessibleTextRectInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_caret_location,
            vm_id,
            ac,
            &mut info,
            index
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Установить позицию каретки в тексте. Возвращает true, если успешно.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `position` Позиция каретки.
    */
    pub(crate) fn set_caret_position(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        position: i32,
    ) -> bool {
        jab!(*self.h_module, set_caret_position, vm_id, ac, position).unwrap_or(0) != 0
    }

    /**
    Получает атрибуты текста между двумя индексами. Список атрибутов включает текст в начальном индексе и текст в конечном индексе.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `start_index` Начальный индекс.
    `end_index` Конечный индекс.
    */
    pub(crate) fn get_text_attributes_in_range(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        start_index: i32,
        end_index: i32,
    ) -> Option<(AccessibleTextAttributesInfo, i16)> {
        let (mut info, mut len) = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_text_attributes_in_range,
            vm_id,
            ac,
            start_index,
            end_index,
            &mut info,
            &mut len
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some((info, len))
    }

    /**
    Возвращает информацию о наборе отношений объекта.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_relation_set(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleRelationSetInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_relation_set,
            vm_id,
            ac,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Возвращает список привязок клавиш, связанных с компонентом.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_key_bindings(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleKeyBindings> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_key_bindings,
            vm_id,
            ac,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Возвращает список значков, связанных с компонентом.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_icons(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleIcons> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(*self.h_module, get_accessible_icons, vm_id, ac, &mut info).unwrap_or(0) == 0 {
            return None;
        }
        Some(info)
    }

    /**
    Возвращает заголовок строки таблицы для указанной таблицы в виде таблицы.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_table_row_header(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleTableInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_table_row_header,
            vm_id,
            ac,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Возвращает заголовок столбца таблицы для указанной таблицы в виде таблицы.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_table_column_header(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleTableInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_table_column_header,
            vm_id,
            ac,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Возвращает описание указанного столбца в указанной таблице. Описание столбца начинается с нуля.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `column` Индекс столбца.
    */
    pub(crate) fn get_accessible_table_column_description(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        column: JInt,
    ) -> Option<AccessibleContext> {
        jab!(
            *self.h_module,
            get_accessible_table_column_description,
            vm_id,
            ac,
            column
        )
    }

    /**
    Возвращает описание указанной строки в указанной таблице. Описание строки начинается с нуля.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `row` Индекс строки.
    */
    pub(crate) fn get_accessible_table_row_description(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        row: JInt,
    ) -> Option<AccessibleContext> {
        jab!(
            *self.h_module,
            get_accessible_table_row_description,
            vm_id,
            ac,
            row
        )
    }

    /**
    Выбирает текст между двумя индексами. Выбор включает текст в начальном индексе и текст в конечном индексе. Возвращает, успешно ли выполнено.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `start_index` Начальный индекс.
    `end_index` Конечный индекс.
    */
    pub(crate) fn select_text_range(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        start_index: JInt,
        end_index: JInt,
    ) -> bool {
        jab!(
            *self.h_module,
            select_text_range,
            vm_id,
            ac,
            start_index,
            end_index
        )
        .unwrap_or(0)
            != 0
    }

    /**
    Получает информацию о таблице, такую как заголовок, сводка, количество строк и столбцов, и AccessibleTable.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_table_info(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleTableInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_table_info,
            vm_id,
            ac,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Получает виртуальное имя компонента на основе алгоритма JAWS. Возвращает, успешно ли выполнено. Bug ID 4916682-реализация политики виртуального имени JAWS
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `len` Длина имени.
    */
    pub(crate) fn get_virtual_accessible_name(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        len: i32,
    ) -> Option<Vec<u16>> {
        let mut name = Vec::new();
        for _ in 0..len {
            name.push(0);
        }
        if jab!(
            *self.h_module,
            get_virtual_accessible_name,
            vm_id,
            ac,
            name.as_mut_ptr(),
            len
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(name)
    }

    /**
    Возвращает информацию о гипертексте, связанную с компонентом.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    */
    pub(crate) fn get_accessible_hypertext(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleHypertextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_hypertext,
            vm_id,
            ac,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Перебирает гиперссылки в компоненте. Возвращает информацию о гипертексте компонента, начиная с индекса гиперссылки start_index. Для каждого вызова этого метода возвращаемый объект AccessibleHypertextInfo не превышает MAX_HYPERLINKS. В случае ошибки возвращается None.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `start_index` Начальный индекс.
    */
    pub(crate) fn get_accessible_hypertext_ext(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        start_index: JInt,
    ) -> Option<AccessibleHypertextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_hypertext_ext,
            vm_id,
            ac,
            start_index,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Запрашивает выполнение списка доступных действий компонента. Если все действия выполнены, возвращает true. Если первое запрошенное действие не удалось, возвращает false, в этом случае "failure" содержит индекс неудачного действия.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `actions_to_do` Список действий для выполнения.
    */
    pub(crate) fn do_accessible_actions(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        actions_to_do: *const AccessibleActionsToDo,
    ) -> (bool, JInt) {
        let mut failure = unsafe { std::mem::zeroed() };
        (
            jab!(
                *self.h_module,
                do_accessible_actions,
                vm_id,
                ac,
                actions_to_do,
                &mut failure
            )
            .unwrap_or(0)
                != 0,
            failure,
        )
    }

    /**
    Устанавливает содержимое редактируемого текста. AccessibleContext должен реализовывать AccessibleEditableText и быть редактируемым. Максимальная длина текста, которую можно установить, составляет MAX_STRING_SIZE-1. Возвращает, успешно ли выполнено.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `text` Текстовое содержимое.
    */
    pub(crate) fn set_text_contents(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        text: *const u16,
    ) -> bool {
        jab!(*self.h_module, set_text_contents, vm_id, ac, text).unwrap_or(0) != 0
    }

    /**
    Установить обработчик для обновления каретки.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_caret_update_fp(&self, cb: AccessBridgeCaretUpdateFp) {
        jab!(*self.h_module, set_caret_update_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для получения фокуса.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_focus_gained_fp(&self, cb: AccessBridgeFocusGainedFp) {
        jab!(*self.h_module, set_focus_gained_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для потери фокуса.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_focus_lost_fp(&self, cb: AccessBridgeFocusLostFp) {
        jab!(*self.h_module, set_focus_lost_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для завершения работы JVM.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_java_shutdown_fp(&self, cb: AccessBridgeJavaShutdownFp) {
        jab!(*self.h_module, set_java_shutdown_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для отмены меню.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_menu_canceled_fp(&self, cb: AccessBridgeMenuCanceledFp) {
        jab!(*self.h_module, set_menu_canceled_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для отмены выбора меню.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_menu_deselected_fp(&self, cb: AccessBridgeMenuDeselectedFp) {
        jab!(*self.h_module, set_menu_deselected_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для выбора меню.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_menu_selected_fp(&self, cb: AccessBridgeMenuSelectedFp) {
        jab!(*self.h_module, set_menu_selected_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для щелчка мыши.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_mouse_clicked_fp(&self, cb: AccessBridgeMouseClickedFp) {
        jab!(*self.h_module, set_mouse_clicked_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для входа мыши.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_mouse_entered_fp(&self, cb: AccessBridgeMouseEnteredFp) {
        jab!(*self.h_module, set_mouse_entered_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для выхода мыши.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_mouse_exited_fp(&self, cb: AccessBridgeMouseExitedFp) {
        jab!(*self.h_module, set_mouse_exited_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для нажатия мыши.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_mouse_pressed_fp(&self, cb: AccessBridgeMousePressedFp) {
        jab!(*self.h_module, set_mouse_pressed_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для отпускания мыши.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_mouse_released_fp(&self, cb: AccessBridgeMouseReleasedFp) {
        jab!(*self.h_module, set_mouse_released_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для отмены всплывающего меню.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_popup_menu_canceled_fp(&self, cb: AccessBridgePopupMenuCanceledFp) {
        jab!(*self.h_module, set_popup_menu_canceled_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для скрытия всплывающего меню.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_popup_menu_will_become_invisible_fp(
        &self,
        cb: AccessBridgePopupMenuWillBecomeInvisibleFp,
    ) {
        jab!(*self.h_module, set_popup_menu_will_become_invisible_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для отображения всплывающего меню.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_popup_menu_will_become_visible_fp(
        &self,
        cb: AccessBridgePopupMenuWillBecomeVisibleFp,
    ) {
        jab!(*self.h_module, set_popup_menu_will_become_visible_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения активного потомка свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_active_descendent_change_fp(
        &self,
        cb: AccessBridgePropertyActiveDescendentChangeFp,
    ) {
        jab!(*self.h_module, set_property_active_descendent_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения каретки свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_caret_change_fp(&self, cb: AccessBridgePropertyCaretChangeFp) {
        jab!(*self.h_module, set_property_caret_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_change_fp(&self, cb: AccessBridgePropertyChangeFp) {
        jab!(*self.h_module, set_property_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения дочернего свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_child_change_fp(&self, cb: AccessBridgePropertyChildChangeFp) {
        jab!(*self.h_module, set_property_child_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения описания свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_description_change_fp(
        &self,
        cb: AccessBridgePropertyDescriptionChangeFp,
    ) {
        jab!(*self.h_module, set_property_description_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения имени свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_name_change_fp(&self, cb: AccessBridgePropertyNameChangeFp) {
        jab!(*self.h_module, set_property_name_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения выбора свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_selection_change_fp(
        &self,
        cb: AccessBridgePropertySelectionChangeFp,
    ) {
        jab!(*self.h_module, set_property_selection_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения состояния свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_state_change_fp(&self, cb: AccessBridgePropertyStateChangeFp) {
        jab!(*self.h_module, set_property_state_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения модели таблицы свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_table_model_change_fp(
        &self,
        cb: AccessBridgePropertyTableModelChangeFp,
    ) {
        jab!(*self.h_module, set_property_table_model_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения текста свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_text_change_fp(&self, cb: AccessBridgePropertyTextChangeFp) {
        jab!(*self.h_module, set_property_text_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения значения свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_value_change_fp(&self, cb: AccessBridgePropertyValueChangeFp) {
        jab!(*self.h_module, set_property_value_change_fp, cb).unwrap_or(())
    }

    /**
    Устанавливает обработчик для изменения видимых данных свойства.
    `cb` Функция, принимающая событие.
    */
    pub(crate) fn set_property_visible_data_change_fp(
        &self,
        cb: AccessBridgePropertyVisibleDataChangeFp,
    ) {
        jab!(*self.h_module, set_property_visible_data_change_fp, cb).unwrap_or(())
    }

    /**
    Запрашивает активацию гиперссылки.
    `vm_id` Идентификатор виртуальной машины.
    `ac` Контекст доступности.
    `link` Объект гиперссылки.
    */
    pub(crate) fn activate_accessible_hyperlink(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        link: AccessibleHyperlink,
    ) -> bool {
        jab!(
            *self.h_module,
            activate_accessible_hyperlink,
            vm_id,
            ac,
            link
        )
        .unwrap_or(0)
            != 0
    }

    /**
    Добавляет элемент в выбор. Если флаг AccessibleSelection в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleSelection. Поддержка AccessibleSelection - это первое место, где можно манипулировать пользовательским интерфейсом, добавляя и удаляя элементы из выбора (а не запрашивая). Некоторые функции используют индексы в координатах дочерних элементов, а другие - в координатах выбора. Например, добавление в выбор и удаление из него осуществляется путем передачи индекса дочернего элемента (например, добавление четвертого дочернего элемента в выбор). С другой стороны, перечисление выбранных дочерних объектов выполняется в координатах выбора (например, получение AccessibleContext первого выбранного объекта).
    `vm_id` Идентификатор виртуальной машины.
    `as` Контекст доступности.
    `index` Индекс.
    */
    pub(crate) fn add_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
        index: i32,
    ) {
        jab!(
            *self.h_module,
            add_accessible_selection_from_context,
            vm_id,
            r#as,
            index
        )
        .unwrap_or(())
    }

    /**
    Удаляет элемент из выбора. Если флаг AccessibleSelection в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleSelection. Поддержка AccessibleSelection - это первое место, где можно манипулировать пользовательским интерфейсом, добавляя и удаляя элементы из выбора (а не запрашивая). Некоторые функции используют индексы в координатах дочерних элементов, а другие - в координатах выбора. Например, добавление в выбор и удаление из него осуществляется путем передачи индекса дочернего элемента (например, добавление четвертого дочернего элемента в выбор). С другой стороны, перечисление выбранных дочерних объектов выполняется в координатах выбора (например, получение AccessibleContext первого выбранного объекта).
    `vm_id` Идентификатор виртуальной машины.
    `as` Контекст доступности.
    `index` Индекс.
    */
    pub(crate) fn remove_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
        index: i32,
    ) {
        jab!(
            *self.h_module,
            remove_accessible_selection_from_context,
            vm_id,
            r#as,
            index
        )
        .unwrap_or(())
    }

    /**
    Очищает выбор. Если флаг AccessibleSelection в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleSelection. Поддержка AccessibleSelection - это первое место, где можно манипулировать пользовательским интерфейсом, добавляя и удаляя элементы из выбора (а не запрашивая). Некоторые функции используют индексы в координатах дочерних элементов, а другие - в координатах выбора. Например, добавление в выбор и удаление из него осуществляется путем передачи индекса дочернего элемента (например, добавление четвертого дочернего элемента в выбор). С другой стороны, перечисление выбранных дочерних объектов выполняется в координатах выбора (например, получение AccessibleContext первого выбранного объекта).
    `vm_id` Идентификатор виртуальной машины.
    `as` Контекст доступности.
    */
    pub(crate) fn clear_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
    ) {
        jab!(
            *self.h_module,
            clear_accessible_selection_from_context,
            vm_id,
            r#as
        )
        .unwrap_or(())
    }

    /**
    Выбирает все. Если флаг AccessibleSelection в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleSelection. Поддержка AccessibleSelection - это первое место, где можно манипулировать пользовательским интерфейсом, добавляя и удаляя элементы из выбора (а не запрашивая). Некоторые функции используют индексы в координатах дочерних элементов, а другие - в координатах выбора. Например, добавление в выбор и удаление из него осуществляется путем передачи индекса дочернего элемента (например, добавление четвертого дочернего элемента в выбор). С другой стороны, перечисление выбранных дочерних объектов выполняется в координатах выбора (например, получение AccessibleContext первого выбранного объекта).
    `vm_id` Идентификатор виртуальной машины.
    `as` Контекст доступности.
    */
    pub(crate) fn select_all_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
    ) {
        jab!(
            *self.h_module,
            select_all_accessible_selection_from_context,
            vm_id,
            r#as
        )
        .unwrap_or(())
    }

    /**
    Возвращает n-ю гиперссылку в документе. Соответствует AccessibleHypertext.getLink. В случае ошибки возвращается None.
    `vm_id` Идентификатор виртуальной машины.
    `ah` Контекст доступности гипертекста.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_hyperlink(
        &self,
        vm_id: i32,
        ah: AccessibleContext,
        index: JInt,
    ) -> Option<AccessibleHypertextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_hyperlink,
            vm_id,
            ah,
            index,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Возвращает количество гиперссылок в компоненте. Соответствует AccessibleHypertext.getLinkCount. В случае ошибки возвращается -1.
    `vm_id` Идентификатор виртуальной машины.
    `ah` Контекст доступности гипертекста.
    */
    pub(crate) fn get_accessible_hyperlink_count(
        &self,
        vm_id: i32,
        ah: AccessibleHypertext,
    ) -> JInt {
        jab!(*self.h_module, get_accessible_hyperlink_count, vm_id, ah).unwrap_or(-1)
    }

    /**
    Возвращает индекс в массиве гиперссылок, связанный с индексом символа в документе. Соответствует AccessibleHypertext.getLinkIndex. В случае ошибки возвращается -1.
    `vm_id` Идентификатор виртуальной машины.
    `ah` Контекст доступности гипертекста.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_hypertext_link_index(
        &self,
        vm_id: i32,
        ah: AccessibleHypertext,
        index: JInt,
    ) -> JInt {
        jab!(
            *self.h_module,
            get_accessible_hypertext_link_index,
            vm_id,
            ah,
            index
        )
        .unwrap_or(-1)
    }

    /**
    Получает количество объектов в выборе.
    `vm_id` Идентификатор виртуальной машины.
    `as` Контекст доступности выбора.
    */
    pub(crate) fn get_accessible_selection_count_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
    ) -> i32 {
        jab!(
            *self.h_module,
            get_accessible_selection_count_from_context,
            vm_id,
            r#as
        )
        .unwrap_or(-1)
    }

    /**
    Получает объект в выборе.
    `vm_id` Идентификатор виртуальной машины.
    `as` Контекст доступности выбора.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
        index: i32,
    ) -> Option<JObject> {
        jab!(
            *self.h_module,
            get_accessible_selection_from_context,
            vm_id,
            r#as,
            index
        )
    }

    /**
    Определяет, выбран ли объект.
    `vm_id` Идентификатор виртуальной машины.
    `as` Контекст доступности выбора.
    `index` Индекс дочернего объекта.
    */
    pub(crate) fn is_accessible_child_selected_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
        index: i32,
    ) -> bool {
        jab!(
            *self.h_module,
            is_accessible_child_selected_from_context,
            vm_id,
            r#as,
            index
        )
        .unwrap_or(0)
            != 0
    }

    /**
    Определяет, выбрана ли строка таблицы. Возвращает true, если указанный ряд, начиная с нуля, выбран.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    `row` Индекс строки.
    */
    pub(crate) fn is_accessible_table_row_selected(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        row: JInt,
    ) -> bool {
        jab!(
            *self.h_module,
            is_accessible_table_row_selected,
            vm_id,
            at,
            row
        )
        .unwrap_or(0)
            != 0
    }

    /**
    Определяет, выбран ли столбец таблицы. Возвращает true, если указанный столбец, начиная с нуля, выбран.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    `column` Индекс столбца.
    */
    pub(crate) fn is_accessible_table_column_selected(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        column: JInt,
    ) -> bool {
        jab!(
            *self.h_module,
            is_accessible_table_column_selected,
            vm_id,
            at,
            column
        )
        .unwrap_or(0)
            != 0
    }

    /**
    Возвращает информацию о ячейке таблицы для указанной строки и столбца. Строка и столбец начинаются с нуля.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    `row` Индекс строки.
    `column` Индекс столбца.
    */
    pub(crate) fn get_accessible_table_cell_info(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        row: JInt,
        column: JInt,
    ) -> Option<AccessibleTableCellInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_table_cell_info,
            vm_id,
            at,
            row,
            column,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Возвращает номер столбца для ячейки в указанном индексе ячейки. Эти значения начинаются с нуля.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_table_column(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        index: JInt,
    ) -> JInt {
        jab!(
            *self.h_module,
            get_accessible_table_column,
            vm_id,
            at,
            index
        )
        .unwrap_or(0)
    }

    /**
    Возвращает номер строки для ячейки в указанном индексе ячейки. Эти значения начинаются с нуля.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_table_row(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        index: JInt,
    ) -> JInt {
        jab!(*self.h_module, get_accessible_table_row, vm_id, at, index).unwrap_or(0)
    }

    /**
    Возвращает количество выбранных столбцов в таблице.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    */
    pub(crate) fn get_accessible_table_column_selection_count(
        &self,
        vm_id: i32,
        at: AccessibleTable,
    ) -> JInt {
        jab!(
            *self.h_module,
            get_accessible_table_column_selection_count,
            vm_id,
            at
        )
        .unwrap_or(0)
    }

    /**
    Возвращает количество выбранных строк в таблице.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    */
    pub(crate) fn get_accessible_table_row_selection_count(
        &self,
        vm_id: i32,
        at: AccessibleTable,
    ) -> JInt {
        jab!(
            *self.h_module,
            get_accessible_table_row_selection_count,
            vm_id,
            at
        )
        .unwrap_or(0)
    }

    /**
    Возвращает индекс для указанной строки и столбца в таблице. Эти значения начинаются с нуля.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    `row` Индекс строки.
    `column` Индекс столбца.
    */
    pub(crate) fn get_accessible_table_index(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        row: JInt,
        column: JInt,
    ) -> JInt {
        jab!(
            *self.h_module,
            get_accessible_table_index,
            vm_id,
            at,
            row,
            column
        )
        .unwrap_or(0)
    }

    /**
    Возвращает массив индексов выбранных столбцов, начиная с нуля.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    `count` Длина массива.
    */
    pub(crate) fn get_accessible_table_column_selections(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        count: JInt,
    ) -> Option<Vec<JInt>> {
        let mut arr = Vec::new();
        for _ in 0..count {
            arr.push(0);
        }
        if jab!(
            *self.h_module,
            get_accessible_table_column_selections,
            vm_id,
            at,
            count,
            arr.as_mut_ptr()
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        return Some(arr);
    }

    /**
    Возвращает массив индексов выбранных строк, начиная с нуля.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности таблицы.
    `count` Длина массива.
    */
    pub(crate) fn get_accessible_table_row_selections(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        count: JInt,
    ) -> Option<Vec<JInt>> {
        let mut arr = Vec::new();
        for _ in 0..count {
            arr.push(0);
        }
        if jab!(
            *self.h_module,
            get_accessible_table_row_selections,
            vm_id,
            at,
            count,
            arr.as_mut_ptr()
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        return Some(arr);
    }

    /**
    Получает информацию о выборе текста. Если флаг AccessibleText в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleText. Файл AccessBridgePackages.h определяет значения структур, используемых в этих функциях. Java Access Bridge API описывает их обратные вызовы.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности текста.
    */
    pub(crate) fn get_accessible_text_selection_info(
        &self,
        vm_id: i32,
        at: AccessibleText,
    ) -> Option<AccessibleTextSelectionInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_text_selection_info,
            vm_id,
            at,
            &mut info
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Получает информацию о тексте. Если флаг AccessibleText в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleText. Файл AccessBridgePackages.h определяет значения структур, используемых в этих функциях. Java Access Bridge API описывает их обратные вызовы.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности текста.
    `x` X координата.
    `y` Y координата.
    */
    pub(crate) fn get_accessible_text_info(
        &self,
        vm_id: i32,
        at: AccessibleText,
        x: JInt,
        y: JInt,
    ) -> Option<AccessibleTextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_text_info,
            vm_id,
            at,
            &mut info,
            x,
            y
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Получает атрибуты текста. Если флаг AccessibleText в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleText. Файл AccessBridgePackages.h определяет значения структур, используемых в этих функциях. Java Access Bridge API описывает их обратные вызовы.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности текста.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_text_attributes(
        &self,
        vm_id: i32,
        at: AccessibleText,
        index: JInt,
    ) -> (*const u8, AccessibleTextAttributesInfo) {
        let mut info = unsafe { std::mem::zeroed() };
        let char = jab!(
            *self.h_module,
            get_accessible_text_attributes,
            vm_id,
            at,
            index,
            &mut info
        )
        .unwrap_or(std::ptr::null());
        (char, info)
    }

    /**
    Получает элементы текста. Если флаг AccessibleText в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleText. Файл AccessBridgePackages.h определяет значения структур, используемых в этих функциях. Java Access Bridge API описывает их обратные вызовы.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности текста.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_text_items(
        &self,
        vm_id: i32,
        at: AccessibleText,
        index: JInt,
    ) -> Option<AccessibleTextItemsInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_text_items,
            vm_id,
            at,
            &mut info,
            index
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Получает границы строки текста. Если флаг AccessibleText в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleText. Файл AccessBridgePackages.h определяет значения структур, используемых в этих функциях. Java Access Bridge API описывает их обратные вызовы.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности текста.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_text_line_bounds(
        &self,
        vm_id: i32,
        at: AccessibleText,
        index: JInt,
    ) -> Option<(JInt, JInt)> {
        let (mut start, mut end) = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_text_line_bounds,
            vm_id,
            at,
            index,
            &mut start,
            &mut end
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some((start, end))
    }

    /**
    Получает диапазон текста. Если флаг AccessibleText в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleText. Файл AccessBridgePackages.h определяет значения структур, используемых в этих функциях. Java Access Bridge API описывает их обратные вызовы.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности текста.
    `start_index` Начальный индекс.
    `end_index` Конечный индекс.
    `len` Длина.
    */
    pub(crate) fn get_accessible_text_range(
        &self,
        vm_id: i32,
        at: AccessibleText,
        start_index: JInt,
        end_index: JInt,
        len: i16,
    ) -> Option<Vec<u16>> {
        let mut text = Vec::new();
        for _ in 0..len {
            text.push(0);
        }
        if jab!(
            *self.h_module,
            get_accessible_text_range,
            vm_id,
            at,
            start_index,
            end_index,
            text.as_mut_ptr(),
            len
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(text)
    }

    /**
    Получает прямоугольную область текста. Если флаг AccessibleText в структуре данных AccessibleContextInfo установлен в TRUE, то в AccessibleContext содержится информация о AccessibleText. Файл AccessBridgePackages.h определяет значения структур, используемых в этих функциях. Java Access Bridge API описывает их обратные вызовы.
    `vm_id` Идентификатор виртуальной машины.
    `at` Контекст доступности текста.
    `index` Индекс.
    */
    pub(crate) fn get_accessible_text_rect(
        &self,
        vm_id: i32,
        at: AccessibleText,
        index: JInt,
    ) -> Option<AccessibleTextRectInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            *self.h_module,
            get_accessible_text_rect,
            vm_id,
            at,
            &mut info,
            index
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
    Получает текущее значение. Если флаг AccessibleValue в структуре данных AccessibleContextInfo установлен в TRUE, то в объекте AccessibleContext содержится информация о AccessibleValue. Возвращаемое значение является строкой (char*value), так как невозможно заранее определить, является ли значение целым числом, числом с плавающей запятой или другим объектом, являющимся подклассом java.lang.Number в языке Java.
    `vm_id` Идентификатор виртуальной машины.
    `av` Контекст доступности значения.
    `len` Длина текста.
    */
    pub(crate) fn get_current_accessible_value_from_context(
        &self,
        vm_id: i32,
        av: AccessibleValue,
        len: i16,
    ) -> Option<Vec<u16>> {
        let mut value = Vec::new();
        for _ in 0..len {
            value.push(0);
        }
        if jab!(
            *self.h_module,
            get_current_accessible_value_from_context,
            vm_id,
            av,
            value.as_mut_ptr(),
            len
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(value)
    }

    /**
    Получает максимальное значение. Если флаг AccessibleValue в структуре данных AccessibleContextInfo установлен в TRUE, то в объекте AccessibleContext содержится информация о AccessibleValue. Возвращаемое значение является строкой (char*value), так как невозможно заранее определить, является ли значение целым числом, числом с плавающей запятой или другим объектом, являющимся подклассом java.lang.Number в языке Java.
    `vm_id` Идентификатор виртуальной машины.
    `av` Контекст доступности значения.
    `len` Длина текста.
    */
    pub(crate) fn get_maximum_accessible_value_from_context(
        &self,
        vm_id: i32,
        av: AccessibleValue,
        len: i16,
    ) -> Option<Vec<u16>> {
        let mut value = Vec::new();
        for _ in 0..len {
            value.push(0);
        }
        if jab!(
            *self.h_module,
            get_maximum_accessible_value_from_context,
            vm_id,
            av,
            value.as_mut_ptr(),
            len
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(value)
    }

    /**
    Получает минимальное значение. Если флаг AccessibleValue в структуре данных AccessibleContextInfo установлен в TRUE, то в объекте AccessibleContext содержится информация о AccessibleValue. Возвращаемое значение является строкой (char*value), так как невозможно заранее определить, является ли значение целым числом, числом с плавающей запятой или другим объектом, являющимся подклассом java.lang.Number в языке Java.
    `vm_id` Идентификатор виртуальной машины.
    `av` Контекст доступности значения.
    `len` Длина текста.
    */
    pub(crate) fn get_minimum_accessible_value_from_context(
        &self,
        vm_id: i32,
        av: AccessibleValue,
        len: i16,
    ) -> Option<Vec<u16>> {
        let mut value = Vec::new();
        for _ in 0..len {
            value.push(0);
        }
        if jab!(
            *self.h_module,
            get_minimum_accessible_value_from_context,
            vm_id,
            av,
            value.as_mut_ptr(),
            len
        )
        .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(value)
    }
}

impl Drop for JabLib {
    fn drop(&mut self) {
        if self.h_module.is_invalid() {
            return;
        }
        unsafe { FreeLibrary(*self.h_module).unwrap_or(()) };
    }
}
