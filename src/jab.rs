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

#![allow(non_upper_case_globals)]

pub(crate) mod jab_lib;

pub mod callback;
pub mod context;
pub mod hyperlink;
pub mod hypertext;
pub mod key_binding;
pub mod relation;
pub mod role;
pub mod table;
pub mod text;
pub mod version;

use crate::{
    find_library_path,
    jab::{
        callback::{AccessibleCallback, AccessibleContextType},
        context::AccessibleContext,
        jab_lib::{JabLib, packages::JObject64},
    },
    utils::{StringExt, pump_waiting_messages},
};
use std::sync::{Arc, LazyLock, Mutex};
use windows::Win32::Foundation::HWND;

static LIB: LazyLock<JabLib> = LazyLock::new(|| {
    #[cfg(target_arch = "x86_64")]
    let path = find_library_path("WindowsAccessBridge-64.dll").unwrap_or_default();
    #[cfg(target_arch = "x86")]
    let path = find_library_path("WindowsAccessBridge-32.dll").unwrap_or_default();

    pump_waiting_messages();
    JabLib::new(Some(path)).unwrap()
});

static FUNCS: Mutex<Vec<AccessibleCallback>> = Mutex::new(vec![]);

#[derive(Debug)]
pub struct Jab {
    _lib: &'static JabLib,
}

impl Jab {
    /**
     * Создать новый экземпляр.
     */
    pub fn new() -> Self {
        Self { _lib: &*LIB }
    }

    /**
     * Получить контекст из объекта фокуса на окне.
     * `target` - это дескриптор целевого окна.
     */
    pub fn get_context_from_hwnd(&'_ self, target: HWND) -> Option<AccessibleContext<'_>> {
        AccessibleContext::from_hwnd(&self._lib, target)
    }

    /**
     * Получить контекст из окна.
     * `h_wnd` - это дескриптор родительского окна.
     */
    pub fn get_context_with_focus(&'_ self, h_wnd: HWND) -> Option<AccessibleContext<'_>> {
        AccessibleContext::from_focus(&self._lib, h_wnd)
    }

    /**
     * Определить, является ли окно Java-окном.
     * `h_wnd` - это дескриптор окна.
     */
    pub fn is_java_window(&self, h_wnd: HWND) -> bool {
        pump_waiting_messages();
        self._lib.is_java_window(h_wnd)
    }

    /**
     * Получить количество событий, ожидающих выполнения.
     */
    pub fn get_events_waiting(&self) -> i32 {
        self._lib.get_events_waiting()
    }

    /**
     * Удалить всех слушателей.
     */
    pub fn remove_all_listeners(&self) {
        let mut lock = FUNCS.lock().unwrap();
        lock.clear();
    }
}

unsafe impl Send for Jab {}
unsafe impl Sync for Jab {}

macro_rules! add_event_fp {
    (general,$lib:expr,$store:expr,$cb_name:ident,$func_name:ident,$type:path,$origin_name:ident,$doc:literal) => {
        extern "system" fn $cb_name(vm_id: i32, event: JObject64, source: JObject64) {
            let source = Arc::new(AccessibleContext::new(&*$lib, vm_id, source));
            $lib.release_java_object(vm_id, event);

            let lock = $store.lock().unwrap();
            lock.iter().for_each(move |cb| {
                if let $type(f) = cb {
                    f(source.clone());
                }
            });
        }
        impl Jab {
            #[doc=concat!("Добавить ", $doc," слушатель\n","`func` - функция или замыкание для слушателя.")]
            pub fn $func_name(&self, func: impl Fn(AccessibleContextType) + Sync + Send + 'static) {
                static $origin_name: std::sync::Once = std::sync::Once::new();
                $origin_name.call_once(|| self._lib.$origin_name($cb_name));

                let mut lock = $store.lock().unwrap();
                lock.push($type(Box::new(func)));
            }
        }
    };

        (property_change,$lib:expr,$store:expr,$doc:literal) => {
            extern "system" fn cb_property_change(vm_id: i32, event: JObject64, source: JObject64,property:*const u16,old_value:*const u16,new_value: *const u16) {
                let source = Arc::new(AccessibleContext::new(&*$lib, vm_id, source));
                let property2 = property.to_string_utf16();
                let old_value2 = old_value.to_string_utf16();
                let new_value2 = new_value.to_string_utf16();
                $lib.release_java_object(vm_id, property as JObject64);
                $lib.release_java_object(vm_id, old_value as JObject64);
                $lib.release_java_object(vm_id, new_value as JObject64);
                $lib.release_java_object(vm_id, event);

                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let AccessibleCallback::PropertyChange(f) = cb {
                        f(source.clone(), property2.clone(),old_value2.clone(),new_value2.clone());
                    }
                });
            }
            impl Jab {
                #[doc=concat!("Добавить ", $doc," слушатель\n","`func` - функция или замыкание для слушателя.")]
                pub fn add_on_property_change_listener(&self, func: impl Fn(AccessibleContextType, String,String,String) + Sync + Send + 'static) {
                    static set_property_change_fp: std::sync::Once = std::sync::Once::new();
                    set_property_change_fp.call_once(|| self._lib.set_property_change_fp(cb_property_change));

                    let mut lock = $store.lock().unwrap();
                    lock.push(AccessibleCallback::PropertyChange(Box::new(func)));
                }
            }
        };

        (property_x_change,$lib:expr,$store:expr,$cb_name:ident,$func_name:ident,$type:path,$origin_name:ident,$doc:literal) => {
            extern "system" fn $cb_name(vm_id: i32, event: JObject64, source: JObject64,old_value:*const u16,new_value: *const u16) {
                let source = Arc::new(AccessibleContext::new(&*$lib, vm_id, source));
                let old_value2 = old_value.to_string_utf16();
                let new_value2 = new_value.to_string_utf16();
                $lib.release_java_object(vm_id, old_value as JObject64);
                $lib.release_java_object(vm_id, new_value as JObject64);

                $lib.release_java_object(vm_id, event);

                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let $type(f) = cb {
                        f(source.clone(), old_value2.clone(),new_value2.clone());
                    }
                });
            }
            impl Jab {
                #[doc=concat!("Добавить ", $doc," слушатель\n","`func` - функция или замыкание для слушателя.")]
                pub fn $func_name(&self, func: impl Fn(AccessibleContextType, String,String) + Sync + Send + 'static) {
                    static $origin_name: std::sync::Once = std::sync::Once::new();
                    $origin_name.call_once(|| self._lib.$origin_name($cb_name));

                    let mut lock = $store.lock().unwrap();
                    lock.push($type(Box::new(func)));
                }
            }
        };

        (property_caret_change,$lib:expr,$store:expr,$doc:literal) => {
            extern "system" fn cb_property_caret_change(vm_id: i32, event: JObject64, source: JObject64,old_value:i32,new_value: i32) {
                let source = Arc::new(AccessibleContext::new(&*$lib, vm_id, source));
                $lib.release_java_object(vm_id, event);

                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let AccessibleCallback::PropertyCaretChange(f) = cb {
                        f(source.clone(), old_value,new_value);
                    }
                });
            }
            impl Jab {
                #[doc=concat!("Добавить ", $doc," слушатель\n","`func` - функция или замыкание для слушателя.")]
                pub fn add_on_property_caret_change_listener(&self, func: impl Fn(AccessibleContextType, i32,i32) + Sync + Send + 'static) {
                    static set_property_caret_change_fp: std::sync::Once = std::sync::Once::new();
                    set_property_caret_change_fp.call_once(|| self._lib.set_property_caret_change_fp(cb_property_caret_change));

                    let mut lock = $store.lock().unwrap();
                    lock.push(AccessibleCallback::PropertyCaretChange(Box::new(func)));
                }
            }
        };

        (property_y_change,$lib:expr,$store:expr,$cb_name:ident,$func_name:ident,$type:path,$origin_name:ident,$doc:literal) => {
            extern "system" fn $cb_name(vm_id: i32, event: JObject64, source: JObject64,old_value:JObject64,new_value: JObject64) {
                let source = Arc::new(AccessibleContext::new(&*$lib, vm_id, source));
                let old_value = Arc::new(AccessibleContext::new(&*$lib,vm_id,old_value));
                let new_value = Arc::new(AccessibleContext::new(&*$lib,vm_id,new_value));
                $lib.release_java_object(vm_id, event);

                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let $type(f) = cb {
                        f(source.clone(), old_value.clone(),new_value.clone());
                    }
                });
            }
            impl Jab {
                #[doc=concat!("Добавить ", $doc," слушатель\n","`func` - функция или замыкание для слушателя.")]
                pub fn $func_name(&self, func: impl Fn(AccessibleContextType, AccessibleContextType, AccessibleContextType) + Sync + Send + 'static) {
                    static $origin_name: std::sync::Once = std::sync::Once::new();
                    $origin_name.call_once(|| self._lib.$origin_name($cb_name));

                    let mut lock = $store.lock().unwrap();
                    lock.push($type(Box::new(func)));
                }
            }
        };

        (java_shutdown,$lib:expr,$store:expr,$doc:literal) => {
            extern "system" fn cb_java_shutdown(vm_id: i32) {
                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let AccessibleCallback::JavaShutdown(f) = cb {
                        f(vm_id);
                    }
                });
            }
            impl Jab {
                #[doc=concat!("Добавить ", $doc," слушатель\n","`func` - функция или замыкание для слушателя.")]
                pub fn add_on_java_shutdown_listener(&self, func: impl Fn(i32) + Sync + Send + 'static) {
                    static set_java_shutdown_fp: std::sync::Once = std::sync::Once::new();
                    set_java_shutdown_fp.call_once(|| self._lib.set_java_shutdown_fp(cb_java_shutdown));

                    let mut lock = $store.lock().unwrap();
                    lock.push(AccessibleCallback::JavaShutdown(Box::new(func)));
                }
            }
        };
}

// Добавить слушателей событий для различных событий доступности
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_caret_update,
    add_on_caret_update_listener,
    AccessibleCallback::CaretUpdate,
    set_caret_update_fp,
    "изменение позиции каретки"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_focus_gained,
    add_on_focus_gained_listener,
    AccessibleCallback::FocusGained,
    set_focus_gained_fp,
    "получение фокуса"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_mouse_clicked,
    add_on_mouse_clicked_listener,
    AccessibleCallback::MouseClicked,
    set_mouse_clicked_fp,
    "щелчок мыши"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_mouse_entered,
    add_on_mouse_entered_listener,
    AccessibleCallback::MouseEntered,
    set_mouse_entered_fp,
    "вход мыши"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_mouse_exited,
    add_on_mouse_exited_listener,
    AccessibleCallback::MouseExited,
    set_mouse_exited_fp,
    "выход мыши"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_mouse_pressed,
    add_on_mouse_pressed_listener,
    AccessibleCallback::MousePressed,
    set_mouse_pressed_fp,
    "нажатие мыши"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_mouse_released,
    add_on_mouse_released_listener,
    AccessibleCallback::MouseReleased,
    set_mouse_released_fp,
    "отпускание мыши"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_menu_canceled,
    add_on_menu_canceled_listener,
    AccessibleCallback::MenuCanceled,
    set_menu_canceled_fp,
    "отмена меню"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_menu_deselected,
    add_on_menu_deselected_listener,
    AccessibleCallback::MenuDeselected,
    set_menu_deselected_fp,
    "снятие выделения меню"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_menu_selected,
    add_on_menu_selected_listener,
    AccessibleCallback::MenuSelected,
    set_menu_selected_fp,
    "выбор меню"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_popup_menu_canceled,
    add_on_popup_menu_canceled_listener,
    AccessibleCallback::PopupMenuCanceled,
    set_popup_menu_canceled_fp,
    "отмена всплывающего меню"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_popup_menu_will_become_invisible,
    add_on_popup_menu_will_become_invisible_listener,
    AccessibleCallback::PopupMenuWillBecomeInvisible,
    set_popup_menu_will_become_invisible_fp,
    "всплывающее меню станет невидимым"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_popup_menu_will_become_visible,
    add_on_popup_menu_will_become_visible_listener,
    AccessibleCallback::PopupMenuWillBecomeVisible,
    set_popup_menu_will_become_visible_fp,
    "всплывающее меню станет видимым"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_property_selection_change,
    add_on_property_selection_change_listener,
    AccessibleCallback::PropertySelectionChange,
    set_property_selection_change_fp,
    "изменение выбора свойства"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_property_text_change,
    add_on_property_text_change_listener,
    AccessibleCallback::PropertyTextChange,
    set_property_text_change_fp,
    "изменение текста свойства"
);
add_event_fp!(
    general,
    LIB,
    FUNCS,
    cb_property_visible_data_change,
    add_on_property_visible_data_change_listener,
    AccessibleCallback::PropertyVisibleDataChange,
    set_property_visible_data_change_fp,
    "изменение видимых данных свойства"
);
add_event_fp!(property_change, LIB, FUNCS, "изменение свойства");
add_event_fp!(
    property_x_change,
    LIB,
    FUNCS,
    cb_property_name_change,
    add_on_property_name_change_listener,
    AccessibleCallback::PropertyNameChange,
    set_property_name_change_fp,
    "изменение имени свойства"
);
add_event_fp!(
    property_x_change,
    LIB,
    FUNCS,
    cb_property_description_change,
    add_on_property_description_change_listener,
    AccessibleCallback::PropertyDescriptionChange,
    set_property_description_change_fp,
    "изменение описания свойства"
);
add_event_fp!(
    property_x_change,
    LIB,
    FUNCS,
    cb_property_state_change,
    add_on_property_state_change_listener,
    AccessibleCallback::PropertyStateChange,
    set_property_state_change_fp,
    "изменение состояния свойства"
);
add_event_fp!(
    property_x_change,
    LIB,
    FUNCS,
    cb_property_value_change,
    add_on_property_value_change_listener,
    AccessibleCallback::PropertyValueChange,
    set_property_value_change_fp,
    "изменение значения свойства"
);
add_event_fp!(
    property_caret_change,
    LIB,
    FUNCS,
    "изменение позиции каретки свойства"
);
add_event_fp!(
    property_y_change,
    LIB,
    FUNCS,
    cb_property_child_change,
    add_on_property_child_change_listener,
    AccessibleCallback::PropertyChildChange,
    set_property_child_change_fp,
    "изменение дочернего свойства"
);
add_event_fp!(
    property_y_change,
    LIB,
    FUNCS,
    cb_property_active_descendent_change,
    add_on_property_active_descendent_change_listener,
    AccessibleCallback::PropertyActiveDescendentChange,
    set_property_active_descendent_change_fp,
    "изменение активного потомка свойства"
);
add_event_fp!(
    property_x_change,
    LIB,
    FUNCS,
    cb_property_table_model_change,
    add_on_property_table_model_change_listener,
    AccessibleCallback::PropertyTableModelChange,
    set_property_table_model_change_fp,
    "изменение модели таблицы свойства"
);
add_event_fp!(java_shutdown, LIB, FUNCS, "завершение работы Java");
