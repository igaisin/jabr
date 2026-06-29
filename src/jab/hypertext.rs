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
    hyperlink::AccessibleHyperlink,
    jab_lib::{
        packages::{AccessibleContext as AC, AccessibleHypertextInfo},
        JabLib,
    },
};
use std::fmt::{Debug, Formatter};

pub struct AccessibleHypertext<'lib> {
    _lib: &'lib JabLib,
    _ac: AC,
    _vm_id: i32,
    _info: AccessibleHypertextInfo,
}

impl<'lib> AccessibleHypertext<'lib> {
    /**
     * Создать новый экземпляр.
     * `lib` - ссылка на библиотеку.
     * `vm_id` - идентификатор виртуальной машины.
     * `ac` - контекст доступности.
     * `info` - информация о гипертексте.
     */
    pub(crate) fn new(
        lib: &'lib JabLib,
        vm_id: i32,
        ac: AC,
        info: AccessibleHypertextInfo,
    ) -> Self {
        Self {
            _lib: lib,
            _ac: ac,
            _vm_id: vm_id,
            _info: info,
        }
    }

    /**
     * Получить все объекты гиперссылок.
     */
    pub fn get_links(&self) -> Vec<AccessibleHyperlink<'lib>> {
        let mut v = vec![];
        for i in 0..self._info.linkCount {
            v.push(AccessibleHyperlink::new(
                self._lib,
                self._vm_id,
                self._ac,
                self._info.links[i as usize].clone(),
            ));
        }
        v
    }

    /**
     * Вернуть n-ую гиперссылку в документе. В случае ошибки вернуть None.
     * `index` - индекс.
     */
    pub fn get_hyperlink(&self, index: i32) -> Option<AccessibleHypertext<'lib>> {
        let Some(link) =
            self._lib
                .get_accessible_hyperlink(self._vm_id, self._info.accessibleHypertext, index)
        else {
            return None;
        };
        Some(Self::new(self._lib, self._vm_id, self._ac, link))
    }

    /**
     * Вернуть количество гиперссылок в компоненте. В случае ошибки вернуть -1.
     */
    pub fn get_hyperlink_count(&self) -> i32 {
        self._lib
            .get_accessible_hyperlink_count(self._vm_id, self._info.accessibleHypertext)
    }

    /**
     * Вернуть индекс гиперссылки, связанной с индексом символа в документе. В случае ошибки вернуть -1.
     * `index` - индекс.
     */
    pub fn get_link_index(&self, index: i32) -> i32 {
        self._lib.get_accessible_hypertext_link_index(
            self._vm_id,
            self._info.accessibleHypertext,
            index,
        )
    }
}

impl<'lib> Drop for AccessibleHypertext<'lib> {
    fn drop(&mut self) {
        self._lib
            .release_java_object(self._vm_id, self._info.accessibleHypertext)
    }
}

impl<'lib> Debug for AccessibleHypertext<'lib> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccessibleHypertext(link_count:{})",
            self._info.linkCount
        )
    }
}
