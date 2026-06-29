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

use std::ffi::{CStr, c_char};
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, FindWindowW, GetMessageW, MSG, PEEK_MESSAGE_REMOVE_TYPE, PM_REMOVE,
    PeekMessageW, TranslateMessage, WM_QUIT,
};
use windows_core::{HSTRING, PCWSTR};

#[derive(Clone, Debug)]
pub(crate) struct SafeModuleHandle(HMODULE);

impl SafeModuleHandle {
    /**
    创建新实例。
    */
    pub(crate) fn new(h_module: HMODULE) -> Self {
        Self(h_module)
    }
}

unsafe impl Send for SafeModuleHandle {}
unsafe impl Sync for SafeModuleHandle {}

impl std::ops::Deref for SafeModuleHandle {
    type Target = HMODULE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/**
Расширение для работы со строками.
*/
pub(crate) trait StringExt {
    /**
    Преобразовать в обычную строку Rust (`String`).
    */
    #[allow(unused)]
    fn to_string(self) -> String;

    /**
    Преобразовать в строку UTF-16.
    */
    fn to_string_utf16(self) -> String;
}

impl StringExt for *const u8 {
    fn to_string(self) -> String {
        unsafe {
            // Интерпретируем указатель как C-строку и конвертируем в Rust String
            CStr::from_ptr(self as *const c_char)
                .to_str()
                .unwrap_or("")
                .to_string()
        }
    }

    fn to_string_utf16(self) -> String {
        // Преобразуем указатель на u8 в указатель на u16 и вызываем соответствующий метод
        (self as *const u16).to_string_utf16()
    }
}

impl StringExt for *const u16 {
    fn to_string(self) -> String {
        self.to_string_utf16()
    }

    fn to_string_utf16(self) -> String {
        unsafe {
            // Используем PCWSTR из windows crate для конвертации в HSTRING и в String
            PCWSTR(self).to_hstring().to_string_lossy()
        }
    }
}

impl StringExt for &[u16] {
    fn to_string(self) -> String {
        self.to_string_utf16()
    }

    fn to_string_utf16(self) -> String {
        // Ищем нулевой терминатор в срезе
        let Some(pos) = self.iter().position(|x| *x == 0) else {
            return String::new();
        };
        // Преобразуем срез UTF-16 в Rust String
        String::from_utf16_lossy(&self[..pos])
    }
}

//noinspection SpellCheckingInspection
/**
Обрабатывает входящие неотложные сообщения, проверяет очередь сообщений потока на наличие
опубликованных сообщений и, если они есть, извлекает их.
`msg` - структура сообщения, которая получает информацию о сообщениях из очереди потока.
`h_wnd` - дескриптор окна, сообщения которого нужно проверить. Окно должно принадлежать текущему потоку.
         Если h_wnd равен NULL, PeekMessage извлекает сообщения для любого окна, принадлежащего текущему потоку,
         а также любые сообщения в очереди текущего потока со значением h_wnd равным NULL (см. структуру MSG).
         Таким образом, если h_wnd равен NULL, обрабатываются как оконные сообщения, так и сообщения потока.
         Если h_wnd равен -1, PeekMessage проверяет только сообщения в очереди текущего потока со значением h_wnd равным NULL,
         то есть сообщения потока, опубликованные через PostMessage или PostThreadMessage, когда параметр h_wnd равен NULL.
`msg_filter_min` - значение первого сообщения в диапазоне проверяемых сообщений.
                  Используйте WM_KEYFIRST (0x0100) для указания первого сообщения клавиатуры
                  или WM_MOUSEFIRST (0x0200) для указания первого сообщения мыши.
                  Если и msg_filter_min, и msg_filter_max равны нулю, PeekMessage возвращает все доступные сообщения
                  (т.е. без фильтрации по диапазону).
`msg_filter_max` - значение последнего сообщения в диапазоне проверяемых сообщений.
                  Используйте WM_KEYLAST для указания последнего сообщения клавиатуры,
                  WM_MOUSELAST для указания последнего сообщения мыши.
                  Если и msg_filter_min, и msg_filter_max равны нулю, PeekMessage возвращает все доступные сообщения
                  (т.е. без фильтрации по диапазону).
`remove_msg` - определяет, как обрабатывать сообщения. Этот параметр может принимать одно или несколько следующих значений:
              - PM_NOREMOVE: после обработки сообщение не удаляется из очереди.
              - PM_REMOVE: после обработки сообщение удаляется из очереди.
              - PM_NOYIELD: предотвращает освобождение системой любых потоков, ожидающих перехода вызывающего потока в состояние простоя
                           (см. WaitForInputIdle). Комбинируйте это значение с PM_NOREMOVE или PM_REMOVE.
              По умолчанию обрабатываются все типы сообщений. Чтобы указать, что следует обрабатывать только определенные сообщения,
              укажите одно или несколько из следующих значений:
              - PM_QS_INPUT: обрабатывать сообщения мыши и клавиатуры.
              - PM_QS_PAINT: обрабатывать сообщения перерисовки.
              - PM_QS_POSTMESSAGE: обрабатывать все опубликованные сообщения, включая таймеры и горячие клавиши.
              - PM_QS_SENDMESSAGE: обрабатывать все отправленные сообщения.
*/
pub(crate) fn peek_message(
    msg: &mut MSG,
    h_wnd: Option<HWND>,
    msg_filter_min: u32,
    msg_filter_max: u32,
    remove_msg: PEEK_MESSAGE_REMOVE_TYPE,
) -> bool {
    unsafe { PeekMessageW(msg, h_wnd, msg_filter_min, msg_filter_max, remove_msg) }.as_bool()
}

/**
Обрабатывает все ожидающие сообщения текущего потока.
*/
pub(crate) fn pump_waiting_messages() {
    let mut msg = MSG::default();
    while peek_message(&mut msg, None, 0, 0, PM_REMOVE) {
        unsafe {
            DispatchMessageW(&msg);
            TranslateMessage(&msg).as_bool();
        }
    }
}

//noinspection SpellCheckingInspection
/**
Извлекает сообщения из очереди сообщений вызывающего потока. Функция обрабатывает входящие отправленные сообщения,
пока не появятся опубликованные сообщения для извлечения.
В отличие от GetMessage, функция PeekMessage не ожидает публикации сообщений перед возвратом.
`msg` - структура сообщения, которая получает информацию о сообщениях из очереди потока.
`h_wnd` - дескриптор окна, сообщения которого нужно получить. Окно должно принадлежать текущему потоку.
         Если h_wnd равен NULL, GetMessage извлекает сообщения для любого окна, принадлежащего текущему потоку,
         а также любые сообщения в очереди текущего потока со значением h_wnd равным NULL.
         Таким образом, если h_wnd равен NULL, обрабатываются как оконные сообщения, так и сообщения потока.
         Если h_wnd равен -1, GetMessage извлекает только сообщения в очереди текущего потока со значением h_wnd равным NULL,
         то есть сообщения потока, опубликованные через PostMessage (когда параметр h_wnd равен NULL) или PostThreadMessage.
`msg_filter_min` - целочисленное значение минимального номера сообщения для извлечения.
                  Используйте WM_KEYFIRST (0x0100) для указания первого сообщения клавиатуры
                  или WM_MOUSEFIRST (0x0200) для указания первого сообщения мыши.
                  Используйте WM_INPUT здесь и в msg_filter_max, чтобы указать только сообщения WM_INPUT.
                  Если и msg_filter_min, и msg_filter_max равны нулю, GetMessage возвращает все доступные сообщения
                  (т.е. без фильтрации по диапазону).
`msg_filter_max` - целочисленное значение максимального номера сообщения для извлечения.
                  Используйте WM_KEYLAST для указания последнего сообщения клавиатуры,
                  WM_MOUSELAST для указания последнего сообщения мыши.
                  Используйте WM_INPUT здесь и в msg_filter_min, чтобы указать только сообщения WM_INPUT.
                  Если и msg_filter_min, и msg_filter_max равны нулю, GetMessage возвращает все доступные сообщения
                  (т.е. без фильтрации по диапазону).
*/
pub(crate) fn get_message(
    msg: &mut MSG,
    h_wnd: Option<HWND>,
    msg_filter_min: u32,
    msg_filter_max: u32,
) -> bool {
    unsafe { GetMessageW(msg, h_wnd, msg_filter_min, msg_filter_max) }.as_bool()
}

//noinspection SpellCheckingInspection
/**
Создает цикл обработки оконных сообщений в текущем потоке до получения сообщения WM_QUIT.
`slot` - функция обратного вызова, которая уведомляет о получении сообщения,
         позволяя выполнить пользовательскую обработку сообщения.
*/
pub fn message_loop(slot: impl Fn(&MSG)) {
    let mut msg = MSG::default();
    while get_message(&mut msg, None, 0, 0) != false {
        if msg.message == WM_QUIT {
            break;
        }
        slot(&msg);
        unsafe {
            DispatchMessageW(&msg);
            TranslateMessage(&msg).as_bool();
        }
    }
}

/**
Находит окно верхнего уровня, имя класса и заголовок которого соответствуют указанным строкам.
Эта функция не выполняет поиск среди дочерних окон.
`class_name` - Строка, указывающая имя класса, или атом (atom), который идентифицирует строку имени класса.
              Если этот параметр является атомом, он должен быть глобальным атомом, созданным ранее с помощью функции GlobalAddAtom.
              Атом (16-битное значение) должен быть помещен в младший байт параметра `class_name`,
              а старший байт `class_name` должен быть обнулен.
              Если параметр равен NULL, будут найдены все окна, соответствующие параметру `window_name`.
`window_name` - Строка, указывающая имя окна (заголовок окна).
               Если этот параметр равен NULL, соответствуют все заголовки окон.
*/
pub fn find_window(class_name: Option<&str>, window_name: Option<&str>) -> HWND {
    unsafe {
        match (class_name, window_name) {
            (Some(c), Some(w)) => FindWindowW(&HSTRING::from(c), &HSTRING::from(w)),
            (Some(c), None) => FindWindowW(&HSTRING::from(c), None),
            (None, Some(w)) => FindWindowW(None, &HSTRING::from(w)),
            _ => FindWindowW(None, None),
        }
        .unwrap_or(Default::default())
    }
}
