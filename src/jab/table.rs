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
    context::AccessibleContext,
    jab_lib::{
        packages::{AccessibleContext as AC, AccessibleTableCellInfo, AccessibleTableInfo},
        JabLib,
    },
};

#[derive(Debug)]
pub struct AccessibleTable<'lib> {
    _lib: &'lib JabLib,
    _vm_id: i32,
    _table: AccessibleTableInfo,
    _caption: AccessibleContext<'lib>,
    _summary: AccessibleContext<'lib>,
}

impl<'lib> AccessibleTable<'lib> {
    /**
     * Создать новый экземпляр.
     * `lib` - ссылка на библиотеку.
     * `vm_id` - идентификатор виртуальной машины.
     * `ac` - исходный объект контекста.
     */
    pub(crate) fn new(lib: &'lib JabLib, vm_id: i32, ac: AC) -> Option<Self> {
        let Some(table) = lib.get_accessible_table_info(vm_id, ac) else {
            return None;
        };
        return Some(Self {
            _lib: &lib,
            _vm_id: vm_id,
            _table: table.clone(),
            _caption: AccessibleContext::new(&lib, vm_id, table.caption),
            _summary: AccessibleContext::new(&lib, vm_id, table.summary),
        });
    }

    /**
     * Получить объект заголовка.
     */
    pub fn get_caption(&'_ self) -> &'_ AccessibleContext<'_> {
        &self._caption
    }

    /**
     * Получить объект сводки.
     */
    pub fn get_summary(&'_ self) -> &'_ AccessibleContext<'_> {
        &self._summary
    }

    /**
     * Вернуть описание указанного столбца в таблице. Индексы столбцов начинаются с нуля.
     * `column` - индекс столбца.
     */
    pub fn get_column_description(&self, column: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(ac) = self._lib.get_accessible_table_column_description(
            self._vm_id,
            self._table.accessibleContext,
            column,
        ) {
            return Some(AccessibleContext::new(self._lib, self._vm_id, ac));
        }
        None
    }

    /**
     * Вернуть описание указанной строки в таблице. Индексы строк начинаются с нуля.
     * `row` - индекс строки.
     */
    pub fn get_row_description(&self, row: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(ac) = self._lib.get_accessible_table_row_description(
            self._vm_id,
            self._table.accessibleContext,
            row,
        ) {
            return Some(AccessibleContext::new(self._lib, self._vm_id, ac));
        }
        None
    }

    /**
     * Вернуть заголовок строк таблицы в виде объекта таблицы.
     */
    pub fn get_row_header(&self) -> Option<AccessibleTable<'lib>> {
        let Some(info) = self
            ._lib
            .get_accessible_table_row_header(self._vm_id, self._table.accessibleContext)
        else {
            return None;
        };
        Some(Self {
            _lib: self._lib,
            _vm_id: self._vm_id,
            _table: info.clone(),
            _caption: AccessibleContext::new(self._lib, self._vm_id, info.caption),
            _summary: AccessibleContext::new(self._lib, self._vm_id, info.summary),
        })
    }

    /**
     * Вернуть заголовок столбцов таблицы в виде объекта таблицы.
     */
    pub fn get_column_header(&self) -> Option<AccessibleTable<'lib>> {
        let Some(info) = self
            ._lib
            .get_accessible_table_column_header(self._vm_id, self._table.accessibleContext)
        else {
            return None;
        };
        Some(Self {
            _lib: self._lib,
            _vm_id: self._vm_id,
            _table: info.clone(),
            _caption: AccessibleContext::new(self._lib, self._vm_id, info.caption),
            _summary: AccessibleContext::new(self._lib, self._vm_id, info.summary),
        })
    }

    /**
     * Проверить, выбрана ли указанная строка в таблице. Если выбрана, вернуть true.
     * `row` - индекс строки.
     */
    pub fn is_row_selected(&self, row: i32) -> bool {
        self._lib
            .is_accessible_table_row_selected(self._vm_id, self._table.accessibleTable, row)
    }

    /**
     * Проверить, выбран ли указанный столбец в таблице. Если выбран, вернуть true.
     * `column` - индекс столбца.
     */
    pub fn is_column_selected(&self, column: i32) -> bool {
        self._lib.is_accessible_table_column_selected(
            self._vm_id,
            self._table.accessibleTable,
            column,
        )
    }

    /**
     * Вернуть информацию о ячейке таблицы по указанным строке и столбцу. Индексы строк и столбцов начинаются с нуля.
     * `row` - индекс строки.
     * `column` - индекс столбца.
     */
    pub fn get_cell(&'_ self, row: i32, column: i32) -> Option<AccessibleTableCell<'_>> {
        let Some(info) = self._lib.get_accessible_table_cell_info(
            self._vm_id,
            self._table.accessibleTable,
            row,
            column,
        ) else {
            return None;
        };
        Some(AccessibleTableCell {
            _lib: &self._lib,
            _vm_id: self._vm_id,
            _info: info,
        })
    }

    /**
     * Вернуть номер столбца для ячейки по указанному индексу. Индексы начинаются с нуля.
     * `index` - индекс.
     */
    pub fn get_column(&self, index: i32) -> i32 {
        self._lib
            .get_accessible_table_column(self._vm_id, self._table.accessibleTable, index)
    }

    /**
     * Вернуть номер строки для ячейки по указанному индексу. Индексы начинаются с нуля.
     * `index` - индекс.
     */
    pub fn get_row(&self, index: i32) -> i32 {
        self._lib
            .get_accessible_table_row(self._vm_id, self._table.accessibleTable, index)
    }

    /**
     * Вернуть количество выбранных столбцов в таблице.
     */
    pub fn get_column_selection_count(&self) -> i32 {
        self._lib
            .get_accessible_table_column_selection_count(self._vm_id, self._table.accessibleTable)
    }

    /**
     * Вернуть количество выбранных строк в таблице.
     */
    pub fn get_row_selection_count(&self) -> i32 {
        self._lib
            .get_accessible_table_row_selection_count(self._vm_id, self._table.accessibleTable)
    }

    /**
     * Вернуть индекс для указанной строки и столбца в таблице. Индексы начинаются с нуля.
     * `row` - индекс строки.
     * `column` - индекс столбца.
     */
    pub fn get_index(&self, row: i32, column: i32) -> i32 {
        self._lib
            .get_accessible_table_index(self._vm_id, self._table.accessibleTable, row, column)
    }

    /**
     * Вернуть массив индексов выбранных столбцов, начиная с нуля.
     * `count` - длина массива.
     */
    pub fn get_column_selections(&self, count: i32) -> Vec<i32> {
        if let Some(v) = self._lib.get_accessible_table_column_selections(
            self._vm_id,
            self._table.accessibleTable,
            count,
        ) {
            return v;
        }
        vec![]
    }

    /**
     * Вернуть массив индексов выбранных строк, начиная с нуля.
     * `count` - длина массива.
     */
    pub fn get_row_selections(&self, count: i32) -> Vec<i32> {
        if let Some(v) = self._lib.get_accessible_table_row_selections(
            self._vm_id,
            self._table.accessibleTable,
            count,
        ) {
            return v;
        }
        vec![]
    }
}

impl<'lib> Drop for AccessibleTable<'lib> {
    fn drop(&mut self) {
        self._lib
            .release_java_object(self._vm_id, self._table.accessibleContext);
    }
}

#[derive(Debug)]
pub struct AccessibleTableCell<'lib> {
    _lib: &'lib JabLib,
    _vm_id: i32,
    _info: AccessibleTableCellInfo,
}

impl<'lib> AccessibleTableCell<'lib> {
    /**
     * Получить объект контекста.
     */
    pub fn get_context(&self) -> AccessibleContext<'lib> {
        AccessibleContext::new(self._lib, self._vm_id, self._info.accessibleContext)
    }

    /**
     * Получить индекс ячейки.
     */
    pub fn get_index(&self) -> i32 {
        self._info.index
    }

    /**
     * Получить индекс столбца ячейки.
     */
    pub fn get_column(&self) -> i32 {
        self._info.column
    }

    /**
     * Получить индекс строки ячейки.
     */
    pub fn get_row(&self) -> i32 {
        self._info.row
    }

    /**
     * Получить количество строк, занимаемых ячейкой.
     */
    pub fn get_row_extent(&self) -> i32 {
        self._info.rowExtent
    }

    /**
     * Получить количество столбцов, занимаемых ячейкой.
     */
    pub fn get_column_extent(&self) -> i32 {
        self._info.columnExtent
    }

    /**
     * Проверить, выбрана ли ячейка.
     */
    pub fn is_selected(&self) -> bool {
        self._info.isSelected != 0
    }
}

impl<'lib> Drop for AccessibleTableCell<'lib> {
    fn drop(&mut self) {
        self._lib
            .release_java_object(self._vm_id, self._info.accessibleContext)
    }
}
