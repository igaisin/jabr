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

use crate::jab::{
    hypertext::AccessibleHypertext,
    jab_lib::{
        JabLib,
        packages::{
            AccessibleActionInfo, AccessibleActions, AccessibleActionsToDo,
            AccessibleContext as AC, AccessibleContextInfo, JInt, MAX_ACTION_INFO, MAX_STRING_SIZE,
            SHORT_STRING_SIZE,
        },
    },
    key_binding::AccessibleKeyBinding,
    relation::AccessibleRelation,
    role::AccessibleRole,
    table::AccessibleTable,
    text::AccessibleTextAttributes,
    version::AccessBridgeVersionInfo,
};
use crate::utils::StringExt;
use std::{
    cmp::min,
    ffi::{CStr, c_char},
    fmt::{Debug, Formatter},
};
use windows::Win32::Foundation::HWND;

pub struct AccessibleContext<'lib> {
    _lib: &'lib JabLib,
    _ac: AC,
    _vm_id: i32,
    _info: Option<AccessibleContextInfo>,
}

impl<'lib> AccessibleContext<'lib> {
    /**
     * Создать экземпляр.
     * `lib` ссылка на библиотеку.
     * `vm_id` ID виртуальной машины.
     * `ac` исходный объект контекста.
     */
    pub(crate) fn new(lib: &'lib JabLib, vm_id: i32, ac: AC) -> Self {
        Self {
            _lib: &lib,
            _vm_id: vm_id,
            _ac: ac,
            _info: lib.get_accessible_context_info(vm_id, ac),
        }
    }
    pub(crate) fn from_hwnd(lib: &'lib JabLib, h_wnd: HWND) -> Option<Self> {
        if let Some((vm_id, ac)) = lib.get_accessible_context_from_hwnd(h_wnd) {
            return Some(Self::new(&lib, vm_id, ac));
        }
        None
    }

    pub(crate) fn from_focus(lib: &'lib JabLib, h_wnd: HWND) -> Option<AccessibleContext<'lib>> {
        if let Some((vm_id, ac)) = lib.get_accessible_context_with_focus(h_wnd) {
            return Some(Self::new(&lib, vm_id, ac));
        }
        None
    }

    /**
     * Возвращает HWND верхнего уровня окна AccessibleContext.
     */
    pub fn get_hwnd(&self) -> HWND {
        self._lib
            .get_hwnd_from_accessible_context(self._vm_id, self._ac)
    }

    /**
     * Получить уникальный ID.
     */
    pub fn get_unique_id(&self) -> i32 {
        self._ac as i32
    }

    /**
     * Запрос объекта AccessibleContext окна или объекта под указателем мыши.
     * `x` координата X.
     * `y` координата Y.
     */
    pub fn get_at(&self, x: i32, y: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(ac) = self
            ._lib
            .get_accessible_context_at(self._vm_id, self._ac, x, y)
        {
            return Some(Self::new(&self._lib, self._vm_id, ac));
        }
        None
    }

    /**
     * Возвращает объект, представляющий n-й дочерний объект этого контекста, где n указывается значением индекса.
     * `index` индекс дочернего объекта.
     */
    pub fn get_child(&self, index: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(child) =
            self._lib
                .get_accessible_child_from_context(self._vm_id, self._ac, index)
        {
            return Some(Self::new(&self._lib, self._vm_id, child));
        }
        None
    }

    /**
     * Возвращает объект, представляющий родительский объект этого контекста.
     */
    pub fn get_parent(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(parent) = self
            ._lib
            .get_accessible_parent_from_context(self._vm_id, self._ac)
        {
            return Some(Self::new(&self._lib, self._vm_id, parent));
        }
        None
    }

    /**
     * Возвращает AccessibleContext с указанной ролью, который является предком данного объекта. Если предка с указанной ролью нет, возвращает None.
     * `role` роль.
     */
    pub fn get_parent_with_role(&self, role: &AccessibleRole) -> Option<AccessibleContext<'lib>> {
        if let Some(parent) = self
            ._lib
            .get_parent_with_role(self._vm_id, self._ac, role.to_str())
        {
            return Some(Self::new(&self._lib, self._vm_id, parent));
        }
        None
    }

    /**
     * Возвращает AccessibleContext с указанной ролью, который является предком данного объекта. Если предка с указанной ролью нет, возвращает корневой объект Java окна. В случае ошибки возвращает None.
     * `role` роль.
     */
    pub fn get_parent_with_role_else_root(
        &self,
        role: &AccessibleRole,
    ) -> Option<AccessibleContext<'lib>> {
        if let Some(parent_or_root) =
            self._lib
                .get_parent_with_role_else_root(self._vm_id, self._ac, role.to_str())
        {
            return Some(Self::new(&self._lib, self._vm_id, parent_or_root));
        }
        None
    }

    /**
     * Возвращает AccessibleContext верхнего уровня объекта Java окна. Это то же самое, что и AccessibleContext, полученный из from_hwnd. В случае ошибки возвращает None.
     */
    pub fn get_top_level(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(top) = self._lib.get_top_level_object(self._vm_id, self._ac) {
            return Some(Self::new(&self._lib, self._vm_id, top));
        }
        None
    }

    /**
     * Возвращает AccessibleContext текущего ActiveDescendent объекта. Этот метод предполагает, что ActiveDescendent является текущим выбранным компонентом в контейнере. В случае ошибки или отсутствия выбора возвращает None.
     */
    pub fn get_active_descendent(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(descendent) = self._lib.get_active_descendent(self._vm_id, self._ac) {
            return Some(Self::new(&self._lib, self._vm_id, descendent));
        }
        None
    }

    /**
     * Возвращает текущую глубину объекта в иерархии объектов. Глубина объекта на вершине иерархии равна 0. В случае ошибки возвращает -1.
     */
    pub fn get_depth(&self) -> i32 {
        self._lib.get_object_depth(self._vm_id, self._ac)
    }

    /**
     * Получить информацию о версии Java Access Bridge, используемой приложением. Вы можете использовать эту информацию, чтобы определить доступные функции вашей версии Java Access Bridge.
     */
    pub fn get_version(&self) -> Option<AccessBridgeVersionInfo> {
        if let Some(version_info) = self._lib.get_version_info(self._vm_id) {
            return Some(AccessBridgeVersionInfo::from(&version_info));
        }
        None
    }

    /**
     * Получить имя объекта.
     */
    pub fn get_name(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(info.name.to_string_utf16())
    }

    /**
     * Получить описание объекта.
     */
    pub fn get_description(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(info.description.to_string_utf16())
    }

    /**
     * Получить роль объекта.
     */
    pub fn get_role(&self) -> AccessibleRole {
        let Some(ref info) = self._info else {
            return AccessibleRole::Unknown;
        };
        AccessibleRole::from_str(&info.role.to_string_utf16())
    }

    /**
     * Получить состояние объекта.
     */
    pub fn get_states(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(info.states.to_string_utf16())
    }

    /**
     * Получить состояние объекта (описание на английском языке).
     */
    pub fn get_states_en_us(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(info.states_en_US.to_string_utf16())
    }

    /**
     * Получить индекс объекта в родительском объекте.
     */
    pub fn get_index_in_parent(&self) -> i32 {
        let Some(ref info) = self._info else {
            return -1;
        };
        info.indexInParent
    }

    /**
     * Получить количество дочерних объектов.
     */
    pub fn get_child_count(&self) -> i32 {
        let Some(ref info) = self._info else {
            return -1;
        };
        info.childrenCount
    }

    /**
     * Получить прямоугольную границу объекта (левая, верхняя, ширина, высота).
     */
    pub fn get_bound_rectangle(&self) -> Option<(i32, i32, i32, i32)> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some((info.x, info.y, info.width, info.height))
    }

    /**
     * Установить курсор в текстовую позицию. Возвращает успех операции.
     * `position` текстовая позиция.
     */
    pub fn set_caret_position(&self, position: i32) -> bool {
        self._lib
            .set_caret_position(self._vm_id, self._ac, position)
    }

    /**
     * Получить местоположение курсора (левая, верхняя, ширина и высота).
     * `index` индекс символа.
     */
    pub fn get_caret_location(&self, index: i32) -> Option<(i32, i32, i32, i32)> {
        if let Some(location) = self._lib.get_caret_location(self._vm_id, self._ac, index) {
            return Some((location.x, location.y, location.width, location.height));
        }
        None
    }

    /**
     * Возвращает список действий, которые может выполнить компонент.
     */
    pub fn get_actions(&self) -> Vec<String> {
        if let Some(actions) = self._lib.get_accessible_actions(self._vm_id, self._ac) {
            let mut names = vec![];
            for i in 0..actions.actionsCount {
                names.push(actions.actionInfo[i as usize].name.to_string_utf16())
            }
            return names;
        }
        return vec![];
    }

    /**
     * Запрос на выполнение списка доступных действий компонентом. Возвращает TRUE, если все действия выполнены. Возвращает FALSE, если первое запрошенное действие не удалось, в этом случае "failure" содержит индекс неудачного действия.
     * `actions_to_do` список действий для выполнения.
     */
    pub fn do_actions(&self, actions_to_do: &[String]) -> (bool, i32) {
        let mut actions = unsafe { std::mem::zeroed::<AccessibleActions>() };
        actions.actionsCount = min(actions_to_do.len() as JInt, MAX_ACTION_INFO as JInt);
        for i in 0..actions.actionsCount {
            let mut name: [u16; SHORT_STRING_SIZE as usize] = [0; SHORT_STRING_SIZE as usize];
            for (i, x) in actions_to_do[i as usize].encode_utf16().enumerate() {
                name[i] = x;
            }
            actions.actionInfo[i as usize] = AccessibleActionInfo { name };
        }
        self._lib.do_accessible_actions(
            self._vm_id,
            self._ac,
            &AccessibleActionsToDo::from_actions(&actions),
        )
    }
    /**
     * Возвращает информацию о связанных объектах.
     */
    pub fn get_relations(&'_ self) -> Vec<AccessibleRelation<'_>> {
        if let Some(r) = self._lib.get_accessible_relation_set(self._vm_id, self._ac) {
            let mut relations = vec![];
            for i in 0..r.relationCount {
                let item = &r.relations[i as usize];
                let mut targets = vec![];
                for j in 0..item.targetCount {
                    targets.push(Self::new(&self._lib, self._vm_id, item.targets[j as usize]))
                }
                relations.push(AccessibleRelation {
                    key: item.key.to_string_utf16(),
                    targets,
                });
            }
            return relations;
        }
        vec![]
    }

    /**
     * Возвращает список привязок клавиш, связанных с компонентом.
     */
    pub fn get_key_bindings(&self) -> Vec<AccessibleKeyBinding> {
        if let Some(key) = self._lib.get_accessible_key_bindings(self._vm_id, self._ac) {
            let mut keys = vec![];
            for i in 0..key.keyBindingsCount {
                keys.push(AccessibleKeyBinding::from(&key.keyBindingInfo[i as usize]));
            }
            return keys;
        }
        vec![]
    }

    /**
     * Возвращает список иконок, связанных с компонентом.
     * Каждая иконка содержит описание, ширину и высоту.
     */
    pub fn get_icons(&self) -> Vec<(String, i32, i32)> {
        if let Some(icons) = self._lib.get_accessible_icons(self._vm_id, self._ac) {
            let mut ret = vec![];
            for i in 0..icons.iconsCount {
                let item = &icons.iconInfo[i as usize];
                ret.push((item.description.to_string_utf16(), item.width, item.height));
            }
            return ret;
        }
        vec![]
    }

    /**
     * Получить виртуальное имя компонента на основе алгоритма JAWS. Возвращает успех операции.
     * Bug ID 4916682 - Реализация политики JAWS AccessibleName
     * `len` длина имени.
     */
    pub fn get_virtual_name(&self, len: i32) -> Option<String> {
        let Some(name) = self
            ._lib
            .get_virtual_accessible_name(self._vm_id, self._ac, len)
        else {
            return None;
        };
        Some(name.to_string_utf16())
    }

    /**
     * Получить текущее значение.
     * `len` длина строки для хранения значения.
     */
    pub fn get_current_value(&self, len: i32) -> Option<String> {
        let Some(v) =
            self._lib
                .get_current_accessible_value_from_context(self._vm_id, self._ac, len as i16)
        else {
            return None;
        };
        let val = v.to_string_utf16();
        if val.is_empty() {
            return None;
        }
        Some(val)
    }

    /**
     * Получить максимальное значение.
     * `len` длина строки для хранения значения.
     */
    pub fn get_maximum_value(&self, len: i32) -> Option<String> {
        let Some(v) =
            self._lib
                .get_maximum_accessible_value_from_context(self._vm_id, self._ac, len as i16)
        else {
            return None;
        };
        let val = v.to_string_utf16();
        if val.is_empty() {
            return None;
        }
        Some(val)
    }

    /**
     * Получить минимальное значение.
     * `len` длина строки для хранения значения.
     */
    pub fn get_minimum_value(&self, len: i32) -> Option<String> {
        let Some(v) =
            self._lib
                .get_minimum_accessible_value_from_context(self._vm_id, self._ac, len as i16)
        else {
            return None;
        };
        let val = v.to_string_utf16();
        if val.is_empty() {
            return None;
        }
        Some(val)
    }

    /**
     * Получить объект таблицы.
     */
    pub fn get_table(&'_ self) -> Option<AccessibleTable<'_>> {
        AccessibleTable::new(&self._lib, self._vm_id, self._ac)
    }

    /**
     * Получить объект гипертекста.
     */
    pub fn get_hypertext(&'_ self) -> Option<AccessibleHypertext<'_>> {
        let Some(info) = self._lib.get_accessible_hypertext(self._vm_id, self._ac) else {
            return None;
        };
        Some(AccessibleHypertext::new(
            self._lib,
            self._vm_id,
            self._ac,
            info,
        ))
    }

    /**
     * Перебирает гиперссылки в компоненте. Возвращает объект гипертекста компонента, начиная с индекса гиперссылки start_index. В случае ошибки возвращает None.
     * `start_index` начальный индекс.
     */
    pub fn get_hypertext_ext(&'_ self, start_index: i32) -> Option<AccessibleHypertext<'_>> {
        let Some(info) = self
            ._lib
            .get_accessible_hypertext_ext(self._vm_id, self._ac, start_index)
        else {
            return None;
        };
        Some(AccessibleHypertext::new(
            self._lib,
            self._vm_id,
            self._ac,
            info,
        ))
    }

    /**
     * Запрос фокуса для компонента. Возвращает успех операции.
     */
    pub fn request_focus(&self) -> bool {
        self._lib.request_focus(self._vm_id, self._ac)
    }

    /**
     * Возвращает количество видимых дочерних объектов компонента. В случае ошибки возвращает -1.
     */
    pub fn get_visible_child_count(&self) -> i32 {
        self._lib.get_visible_children_count(self._vm_id, self._ac)
    }

    /**
     * Получить видимые дочерние объекты.
     * `start_index` начальный индекс.
     */
    pub fn get_visible_children(&self, start_index: i32) -> Option<Vec<Self>> {
        let Some(info) = self
            ._lib
            .get_visible_children(self._vm_id, self._ac, start_index)
        else {
            return None;
        };
        let mut v = vec![];
        for i in 0..info.returnedChildrenCount {
            v.push(Self::new(self._lib, self._vm_id, info.children[i as usize]));
        }
        Some(v)
    }

    /**
     * Добавить элемент в выделение.
     * `index` индекс.
     */
    pub fn add_selection(&self, index: i32) {
        self._lib
            .add_accessible_selection_from_context(self._vm_id, self._ac, index)
    }

    /**
     * Удалить элемент из выделения.
     * `index` индекс.
     */
    pub fn remove_selection(&self, index: i32) {
        self._lib
            .remove_accessible_selection_from_context(self._vm_id, self._ac, index)
    }

    /**
     * Очистить выделение.
     */
    pub fn clear_selection(&self) {
        self._lib
            .clear_accessible_selection_from_context(self._vm_id, self._ac)
    }

    /**
     * Выбрать все.
     */
    pub fn select_all(&self) {
        self._lib
            .select_all_accessible_selection_from_context(self._vm_id, self._ac)
    }

    /**
     * Получить количество объектов в выделении.
     */
    pub fn get_selection_count(&self) -> i32 {
        self._lib
            .get_accessible_selection_count_from_context(self._vm_id, self._ac)
    }

    /**
     * Получить объект из выделения.
     * `index` индекс.
     */
    pub fn get_selection(&self, index: i32) -> Option<Self> {
        let Some(obj) =
            self._lib
                .get_accessible_selection_from_context(self._vm_id, self._ac, index)
        else {
            return None;
        };
        Some(Self::new(self._lib, self._vm_id, obj as AC))
    }

    /**
     * Проверить, выбран ли дочерний объект.
     * `index` индекс дочернего объекта.
     */
    pub fn is_child_selected(&self, index: i32) -> bool {
        self._lib
            .is_accessible_child_selected_from_context(self._vm_id, self._ac, index)
    }

    /**
     * Проверить, поддерживает ли объект режим выбора.
     */
    pub fn is_supported_selection(&self) -> bool {
        let Some(ref info) = self._info else {
            return false;
        };
        info.accessibleSelection != 0
    }

    /**
     * Выбрать текст в диапазоне между двумя индексами. Выделение включает текст на начальном и конечном индексах. Возвращает успех операции.
     * `start_index` начальный индекс.
     * `end_index` конечный индекс.
     */
    pub fn select_text_range(&self, start_index: i32, end_index: i32) -> bool {
        self._lib
            .select_text_range(self._vm_id, self._ac, start_index, end_index)
    }

    /**
     * Получить атрибуты текста в диапазоне между двумя индексами. Список атрибутов включает текст на начальном и конечном индексах.
     * `start_index` начальная позиция.
     * `end_index` конечная позиция.
     */
    pub fn get_text_attributes_in_range(
        &self,
        start_index: i32,
        end_index: i32,
    ) -> Option<(AccessibleTextAttributes, i16)> {
        let Some(info) =
            self._lib
                .get_text_attributes_in_range(self._vm_id, self._ac, start_index, end_index)
        else {
            return None;
        };
        Some((AccessibleTextAttributes::new(info.0), info.1))
    }

    /**
     * Получить информацию о выделении текста (начальный индекс, конечный индекс и выделенный текст).
     */
    pub fn get_text_selection(&self) -> Option<(i32, i32, String)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self
            ._lib
            .get_accessible_text_selection_info(self._vm_id, self._ac)
        else {
            return None;
        };
        Some((
            info.selectionStartIndex,
            info.selectionEndIndex,
            info.selectedText.to_string_utf16(),
        ))
    }

    /**
     * Получить информацию о тексте (количество символов, индекс курсора, индекс по координатам).
     * `x` координата X.
     * `y` координата Y.
     */
    pub fn get_text_info(&self, x: i32, y: i32) -> Option<(i32, i32, i32)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self
            ._lib
            .get_accessible_text_info(self._vm_id, self._ac, x, y)
        else {
            return None;
        };
        Some((info.charCount, info.caretIndex, info.indexAtPoint))
    }

    /**
     * Получить атрибуты текста.
     * `index` индекс.
     */
    pub fn get_text_attributes(&self, index: i32) -> Option<(AccessibleTextAttributes, String)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let (text, info) = self
            ._lib
            .get_accessible_text_attributes(self._vm_id, self._ac, index);
        unsafe {
            Some((
                AccessibleTextAttributes::new(info),
                CStr::from_ptr(text as *const c_char)
                    .to_string_lossy()
                    .to_string(),
            ))
        }
    }

    /**
     * Получить текстовые элементы (символ, слово и предложение).
     * `index` индекс.
     */
    pub fn get_text_items(&self, index: i32) -> Option<(u16, String, String)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self
            ._lib
            .get_accessible_text_items(self._vm_id, self._ac, index)
        else {
            return None;
        };
        let word = info.word.to_string_utf16();
        let sentence = info.sentence.to_string_utf16();
        Some((info.letter, word, sentence))
    }

    /**
     * Получить диапазон текста.
     * `start_index` начальный индекс.
     * `end_index` конечный индекс.
     */
    pub fn get_text_range(&self, start_index: i32, end_index: i32) -> Option<String> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self._lib.get_accessible_text_range(
            self._vm_id,
            self._ac,
            start_index,
            end_index,
            (end_index - start_index).abs() as i16,
        ) else {
            return None;
        };
        Some(info.to_string_utf16())
    }

    /**
     * Получить границы строки текста.
     * `index` индекс.
     */
    pub fn get_text_line_bounds(&self, index: i32) -> Option<(i32, i32)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        self._lib
            .get_accessible_text_line_bounds(self._vm_id, self._ac, index)
    }

    /**
     * Получить прямоугольную область текста (левая, верхняя, ширина и высота).
     * `index` индекс.
     */
    pub fn get_text_rect(&self, index: i32) -> Option<(i32, i32, i32, i32)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self
            ._lib
            .get_accessible_text_rect(self._vm_id, self._ac, index)
        else {
            return None;
        };
        Some((info.x, info.y, info.width, info.height))
    }

    /**
     * Установить редактируемое текстовое содержимое. Возвращает успех операции.
     * `text` текстовое содержимое.
     */
    pub fn set_text_contents(&self, text: &str) -> bool {
        let mut value: [u16; (MAX_STRING_SIZE - 1) as usize] = [0; (MAX_STRING_SIZE - 1) as usize];
        text.encode_utf16().enumerate().for_each(|(i, c)| {
            if i < (MAX_STRING_SIZE - 1) as usize {
                value[i] = c
            }
        });
        self._lib
            .set_text_contents(self._vm_id, self._ac, value.as_ptr())
    }
}

impl<'lib> PartialEq for AccessibleContext<'lib> {
    fn eq(&self, other: &Self) -> bool {
        self._lib.is_same_object(self._vm_id, self._ac, other._ac)
    }
}

impl<'lib> Eq for AccessibleContext<'lib> {}

impl<'lib> Drop for AccessibleContext<'lib> {
    fn drop(&mut self) {
        self._lib.release_java_object(self._vm_id, self._ac);
    }
}

impl<'lib> Debug for AccessibleContext<'lib> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut info = String::new();
        if let Some(name) = self.get_name() {
            info += format!("name:{},", name).as_str();
        }
        if let Some(description) = self.get_description() {
            info += format!("description:{},", description).as_str();
        }
        let role = self.get_role();
        info += format!("role:{},", role.to_str()).as_str();
        if let Some(states) = self.get_states() {
            info += format!("states:{},", states).as_str();
        }
        write!(f, "AccessibleContext({})", info)
    }
}

unsafe impl<'lib> Send for AccessibleContext<'lib> {}

unsafe impl<'lib> Sync for AccessibleContext<'lib> {}
