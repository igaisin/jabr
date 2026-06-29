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

// Этот файл определяет роли, используемые в контексте доступности.
use crate::jab::jab_lib::packages::{
    ACCESSIBLE_ALERT, ACCESSIBLE_AWT_COMPONENT, ACCESSIBLE_CANVAS, ACCESSIBLE_CHECK_BOX,
    ACCESSIBLE_COLOR_CHOOSER, ACCESSIBLE_COLUMN_HEADER, ACCESSIBLE_COMBO_BOX,
    ACCESSIBLE_DATE_EDITOR, ACCESSIBLE_DESKTOP_ICON, ACCESSIBLE_DESKTOP_PANE, ACCESSIBLE_DIALOG,
    ACCESSIBLE_DIRECTORY_PANE, ACCESSIBLE_EDITBAR, ACCESSIBLE_FILE_CHOOSER, ACCESSIBLE_FILLER,
    ACCESSIBLE_FONT_CHOOSER, ACCESSIBLE_FOOTER, ACCESSIBLE_FRAME, ACCESSIBLE_GLASS_PANE,
    ACCESSIBLE_GROUP_BOX, ACCESSIBLE_HEADER, ACCESSIBLE_HYPERLINK, ACCESSIBLE_ICON,
    ACCESSIBLE_INTERNAL_FRAME, ACCESSIBLE_LABEL, ACCESSIBLE_LAYERED_PANE, ACCESSIBLE_LIST,
    ACCESSIBLE_LIST_ITEM, ACCESSIBLE_MENU, ACCESSIBLE_MENU_BAR, ACCESSIBLE_MENU_ITEM,
    ACCESSIBLE_OPTION_PANE, ACCESSIBLE_PAGE_TAB, ACCESSIBLE_PAGE_TAB_LIST, ACCESSIBLE_PANEL,
    ACCESSIBLE_PARAGRAPH, ACCESSIBLE_PASSWORD_TEXT, ACCESSIBLE_POPUP_MENU, ACCESSIBLE_PROGRESS_BAR,
    ACCESSIBLE_PUSH_BUTTON, ACCESSIBLE_RADIO_BUTTON, ACCESSIBLE_ROOT_PANE, ACCESSIBLE_ROW_HEADER,
    ACCESSIBLE_RULER, ACCESSIBLE_SCROLL_BAR, ACCESSIBLE_SCROLL_PANE, ACCESSIBLE_SEPARATOR,
    ACCESSIBLE_SLIDER, ACCESSIBLE_SPIN_BOX, ACCESSIBLE_SPLIT_PANE, ACCESSIBLE_STATUS_BAR,
    ACCESSIBLE_SWING_COMPONENT, ACCESSIBLE_TABLE, ACCESSIBLE_TEXT, ACCESSIBLE_TOGGLE_BUTTON,
    ACCESSIBLE_TOOL_BAR, ACCESSIBLE_TOOL_TIP, ACCESSIBLE_TREE, ACCESSIBLE_UNKNOWN,
    ACCESSIBLE_VIEWPORT, ACCESSIBLE_WINDOW, PROGRESS_MONITOR,
};

/**
 ******************************************************
 *  Доступные Роли
 *      Определяет все AccessibleRoles в Local.US
 ******************************************************
 * */
#[derive(Debug)]
#[allow(dead_code)]
pub enum AccessibleRole {
    /**
     * Объект используется для предупреждения пользователя о чем-то.
     * */
    Alert,

    /**
     * Заголовок для столбца данных.
     * */
    ColumnHeader,

    /**
     * Объект, в который можно рисовать и который используется для захвата
     * событий.
     * см. ACCESSIBLE_FRAME
     * см. ACCESSIBLE_GLASS_PANE
     * см. ACCESSIBLE_LAYERED_PANE
     * */
    Canvas,

    /**
     * Список вариантов, из которых пользователь может выбрать.
     * Также, возможно, позволяет пользователю ввести свой собственный вариант.
     * */
    ComboBox,

    /**
     * Иконизированная внутренняя рамка в DesktopPane.
     * См. ACCESSIBLE_DESKTOP_PANE
     * см. ACCESSIBLE_INTERNAL_FRAME
     * */
    DesktopIcon,

    /**
     * Объект, похожий на рамку, который обрезается панелью рабочего стола.
     * Объекты панели рабочего стола, внутренней рамки и значка рабочего стола
     * часто используются для создания интерфейсов с несколькими документами в
     * приложении.
     * см. ACCESSIBLE_DESKTOP_ICON
     * см. ACCESSIBLE_DESKTOP_PANE
     * см. ACCESSIBLE_FRAME
     * */
    InternalFrame,

    /**
     * Панель, поддерживающая внутренние рамки и
     * иконизированные версии этих внутренних рамок.
     * см. ACCESSIBLE_DESKTOP_ICON
     * см. ACCESSIBLE_INTERNAL_FRAME
     * */
    DesktopPane,

    /**
     * Специализированная панель, основное использование которой внутри DIALOG
     * см. ACCESSIBLE_DIALOG
     * */
    OptionPane,

    /**
     * Окно верхнего уровня без заголовка и границы.
     * см. ACCESSIBLE_FRAME
     * см. ACCESSIBLE_DIALOG
     * */
    Window,

    /**
     * Окно верхнего уровня с заголовком, границей, строкой меню и т.д.
     * Часто используется как основное окно для приложения.
     * см. ACCESSIBLE_DIALOG
     * см. ACCESSIBLE_CANVAS
     * см. ACCESSIBLE_WINDOW
     * */
    Frame,

    /**
     * Окно верхнего уровня с заголовком и границей.
     * Диалоговое окно похоже на рамку, но имеет меньше свойств и часто используется как
     * вторичное окно для приложения.
     * см. ACCESSIBLE_FRAME
     * см. ACCESSIBLE_WINDOW
     * */
    Dialog,

    /**
     * Специализированное диалоговое окно, позволяющее пользователю выбрать цвет.
     * */
    ColorChooser,

    /**
     * Панель, позволяющая пользователю перемещаться по
     * и выбирать содержимое каталога.
     * Может использоваться файловым выборщиком.
     * см. ACCESSIBLE_FILE_CHOOSER
     * */
    DirectoryPane,

    /**
     * Специализированное диалоговое окно, отображающее файлы в каталоге
     * и позволяющее пользователю выбрать файл, просмотреть другой каталог,
     * или указать имя файла.
     * Может использовать панель каталога для отображения содержимого каталога.
     * см. ACCESSIBLE_DIRECTORY_PANE
     * */
    FileChooser,

    /**
     * Объект, заполняющий пространство в пользовательском интерфейсе.
     * Часто используется в интерфейсах для настройки расстояния между компонентами,
     * но не имеет другой цели.
     * */
    Filler,

    /**
     * Гипертекстовая ссылка
     * */
    Hyperlink,

    /**
     * Небольшая картинка фиксированного размера, обычно используемая для украшения компонентов.
     * */
    Icon,

    /**
     * Объект, используемый для представления значка или короткой строки в интерфейсе.
     * */
    Label,

    /**
     * Специализированная панель, имеющая стеклянную панель и многослойную панель в качестве
     * детей.
     * см. ACCESSIBLE_GLASS_PANE
     * см. ACCESSIBLE_LAYERED_PANE
     * */
    RootPane,

    /**
     * Панель, гарантированно рисуемая поверх
     * всех панелей под ней.
     * см. ACCESSIBLE_ROOT_PANE
     * см. ACCESSIBLE_CANVAS
     * */
    GlassPane,

    /**
     * Специализированная панель, позволяющая ее детям рисоваться в слоях,
     * предоставляя форму порядка наложения.
     * Обычно это панель, содержащая строку меню, а также панель, содержащую большинство
     * визуальных компонентов в окне.
     * см. ACCESSIBLE_GLASS_PANE
     * см. ACCESSIBLE_ROOT_PANE
     * */
    LayeredPane,

    /**
     * Объект, представляющий список объектов пользователю и позволяющий
     * пользователю выбрать один или несколько из них.
     * Список обычно содержится в прокручиваемой панели.
     * см. ACCESSIBLE_SCROLL_PANE
     * см. ACCESSIBLE_LIST_ITEM
     * */
    List,

    /**
     * Объект, представляющий элемент в списке.
     * Список обычно содержится в прокручиваемой панели.
     * см. ACCESSIBLE_SCROLL_PANE
     * см. ACCESSIBLE_LIST
     * */
    ListItem,

    /**
     * Объект, обычно рисуемый в верхней части основного диалогового окна
     * приложения, содержащий список меню, из которых пользователь может выбрать.
     * Например, строка меню может содержать меню "Файл", "Правка" и "Справка".
     * см. ACCESSIBLE_MENU
     * см. ACCESSIBLE_POPUP_MENU
     * см. ACCESSIBLE_LAYERED_PANE
     * */
    MenuBar,

    /**
     * Временное окно, обычно используемое для предложения пользователю
     * списка вариантов, и затем скрывающееся, когда пользователь выбирает один из
     * этих вариантов.
     * см. ACCESSIBLE_MENU
     * см. ACCESSIBLE_MENU_ITEM
     * */
    PopupMenu,

    /**
     * Объект, обычно содержащийся в строке меню, содержащий список
     * действий, которые пользователь может выбрать.
     * Меню может иметь любые объекты в качестве детей, но чаще всего это элементы меню, другие меню,
     * или простые объекты, такие как радиокнопки, флажки или
     * разделители.
     * Например, приложение может иметь меню "Правка", содержащее элементы меню "Вырезать" и "Вставить".
     * см. ACCESSIBLE_MENU_BAR
     * см. ACCESSIBLE_MENU_ITEM
     * см. ACCESSIBLE_SEPARATOR
     * см. ACCESSIBLE_RADIO_BUTTON
     * см. ACCESSIBLE_CHECK_BOX
     * см. ACCESSIBLE_POPUP_MENU
     * */
    Menu,

    /**
     * Объект, обычно содержащийся в меню, представляющий действие,
     * которое пользователь может выбрать.
     * Например, элемент меню "Вырезать" в меню "Правка"
     * будет действием, которое пользователь может выбрать для вырезания выделенной области текста в документе.
     * см. ACCESSIBLE_MENU_BAR
     * см. ACCESSIBLE_SEPARATOR
     * см. ACCESSIBLE_POPUP_MENU
     * */
    MenuItem,

    /**
     * Объект, обычно содержащийся в меню для предоставления визуального
     * и логического разделения содержимого в меню.
     * Например, меню "Файл" приложения может содержать элементы меню
     * "Открыть", "Закрыть" и "Выход", и разместит разделитель между
     * элементами меню "Закрыть" и "Выход".
     * см. ACCESSIBLE_MENU
     * см. ACCESSIBLE_MENU_ITEM
     * */
    SEPARATOR,

    /**
     * Объект, представляющий серию панелей (или вкладок страниц), одну за
     * раз, через некоторый механизм, предоставляемый объектом.
     * Наиболее распространенный механизм - это список вкладок в верхней части панели.
     * Дети списка вкладок страницы - это все вкладки страницы.
     * см. ACCESSIBLE_PAGE_TAB
     * */
    PageTabList,

    /**
     * Объект, являющийся ребенком списка вкладок страницы.
     * Его единственный ребенок - это панель, которая будет представлена пользователю, когда пользователь
     * выберет вкладку страницы из списка вкладок в списке вкладок страницы.
     * см. ACCESSIBLE_PAGE_TAB_LIST
     * */
    PageTab,

    /**
     * Общий контейнер, часто используемый для группировки объектов.
     * */
    Panel,

    /**
     * Объект, используемый для указания, сколько задачи выполнено.
     * */
    ProgressBar,

    /**
     * Текстовый объект, используемый для паролей или других мест, где
     * содержимое текста не отображается пользователю явно.
     * */
    PasswordText,

    /**
     * Объект, который пользователь может манипулировать, чтобы заставить приложение что-то сделать.
     * см. ACCESSIBLE_CHECK_BOX
     * см. ACCESSIBLE_TOGGLE_BUTTON
     * см. ACCESSIBLE_RADIO_BUTTON
     * */
    PushButton,

    /**
     * Специализированная кнопка, которую можно отметить или снять отметку, но
     * не предоставляет отдельного индикатора для текущего состояния.
     * см. ACCESSIBLE_PUSH_BUTTON
     * см. ACCESSIBLE_CHECK_BOX
     * см. ACCESSIBLE_RADIO_BUTTON
     * */
    ToggleButton,

    /**
     * Выбор, который можно отметить или снять отметку и предоставляет
     * отдельный индикатор для текущего состояния.
     * см. ACCESSIBLE_PUSH_BUTTON
     * см. ACCESSIBLE_TOGGLE_BUTTON
     * см. ACCESSIBLE_RADIO_BUTTON
     * */
    CheckBox,

    /**
     * Специализированный флажок, который заставит другие радиокнопки в
     * той же группе стать неотмеченными, когда этот флажок отмечен.
     * см. ACCESSIBLE_PUSH_BUTTON
     * см. ACCESSIBLE_TOGGLE_BUTTON
     * см. ACCESSIBLE_CHECK_BOX
     * */
    RadioButton,

    /**
     * Заголовок для строки данных.
     * */
    RowHeader,

    /**
     * Объект, позволяющий пользователю постепенно просматривать большое количество
     * информации.
     * Его дети могут включать полосы прокрутки и видовую область.
     * см. ACCESSIBLE_SCROLL_BAR
     * см. ACCESSIBLE_VIEWPORT
     * */
    ScrollPane,

    /**
     * Объект, обычно используемый для того, чтобы позволить пользователю постепенно просматривать
     * большое количество данных.
     * Обычно используется только прокручиваемой панелью.
     * см. ACCESSIBLE_SCROLL_PANE
     * */
    ScrollBar,

    /**
     * Объект, обычно используемый в прокручиваемой панели.
     * Он представляет собой часть всех данных, которые пользователь может видеть.
     * По мере того, как пользователь манипулирует полосами прокрутки, содержимое видовой области может изменяться.
     * см. ACCESSIBLE_SCROLL_PANE
     * */
    Viewport,

    /**
     * Объект, позволяющий пользователю выбрать из ограниченного диапазона.
     * Например, ползунок может использоваться для выбора числа от 0 до 100.
     * */
    Slider,

    /**
     * Специализированная панель, представляющая две другие панели одновременно.
     * Между двумя панелями находится разделитель, который пользователь может манипулировать, чтобы сделать
     * одну панель больше, а другую панель меньше.
     * */
    SplitPane,

    /**
     * Объект, используемый для представления информации в виде строк и столбцов.
     * Примером может быть приложение для работы с электронными таблицами.
     * */
    Table,

    /**
     * Объект, представляющий текст пользователю.
     * Текст обычно редактируется пользователем в отличие от метки.
     * см. ACCESSIBLE_LABEL
     * */
    Text,

    /**
     * Объект, используемый для представления иерархической информации пользователю.
     * Отдельные узлы в дереве могут быть свернуты и развернуты
     * для предоставления выборочного раскрытия содержимого дерева.
     * */
    Tree,

    /**
     * Панель или палитра, обычно состоящая из кнопок или переключателей.
     * Часто используется для предоставления наиболее часто используемых функций для
     * приложения.
     * */
    ToolBar,

    /**
     * Объект, предоставляющий информацию о другом объекте.
     * Свойство accessibleDescription подсказки часто отображается
     * пользователю в небольшом окне, когда пользователь вызывает
     * наведение мыши на объект, связанный с подсказкой.
     * */
    ToolTip,

    /**
     * Компонент AWT, но больше ничего о нем не известно.
     * см. ACCESSIBLE_SWING_COMPONENT
     * см. ACCESSIBLE_UNKNOWN
     * */
    AwtComponent,

    /**
     * Компонент Swing, но больше ничего о нем не известно.
     * см. ACCESSIBLE_AWT_COMPONENT
     * см. ACCESSIBLE_UNKNOWN
     * */
    SwingComponent,

    /**
     * Объект содержит некоторую доступную информацию, но его роль неизвестна.
     * см. ACCESSIBLE_AWT_COMPONENT
     * см. ACCESSIBLE_SWING_COMPONENT
     * */
    Unknown,

    /**
     * Строка состояния - это простой компонент, который может содержать
     * несколько меток статуса для пользователя.
     * */
    StatusBar,

    /**
     * Редактор даты - это компонент, позволяющий пользователям редактировать
     * объекты java.util.Date и java.util.Time
     * */
    DateEditor,

    /**
     * SpinBox - это простой компонент спиннера, и его основное использование
     * для простых чисел.
     * */
    SpinBox,

    /**
     * FontChooser - это компонент, позволяющий пользователю выбирать различные
     * атрибуты для шрифтов.
     * */
    FontChooser,

    /**
     * GroupBox - это простой контейнер, содержащий границу
     * вокруг него и содержащий компоненты внутри него.
     * */
    GroupBox,

    /**
     * Текстовый заголовок
     * */
    Header,

    /**
     * Текстовый нижний колонтитул
     * */
    Footer,

    /**
     * Текстовый абзац
     * */
    Paragraph,

    /**
     * Линейка - это объект, используемый для измерения расстояния
     * */
    Ruler,

    /**
     * Роль, указывающая, что объект действует как формула для
     * вычисления значения.
     * Примером может быть формула в ячейке электронной таблицы.
     * */
    EditBar,

    /**
     * Роль, указывающая, что объект отслеживает прогресс
     * выполнения какой-либо операции.
     * */
    ProgressMonitor,
}

impl AccessibleRole {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Alert => ACCESSIBLE_ALERT,
            Self::ColumnHeader => ACCESSIBLE_COLUMN_HEADER,
            Self::Canvas => ACCESSIBLE_CANVAS,
            Self::ComboBox => ACCESSIBLE_COMBO_BOX,
            Self::DesktopIcon => ACCESSIBLE_DESKTOP_ICON,
            Self::InternalFrame => ACCESSIBLE_INTERNAL_FRAME,
            Self::DesktopPane => ACCESSIBLE_DESKTOP_PANE,
            Self::OptionPane => ACCESSIBLE_OPTION_PANE,
            Self::Window => ACCESSIBLE_WINDOW,
            Self::Frame => ACCESSIBLE_FRAME,
            Self::Dialog => ACCESSIBLE_DIALOG,
            Self::ColorChooser => ACCESSIBLE_COLOR_CHOOSER,
            Self::DirectoryPane => ACCESSIBLE_DIRECTORY_PANE,
            Self::FileChooser => ACCESSIBLE_FILE_CHOOSER,
            Self::Filler => ACCESSIBLE_FILLER,
            Self::Hyperlink => ACCESSIBLE_HYPERLINK,
            Self::Icon => ACCESSIBLE_ICON,
            Self::Label => ACCESSIBLE_LABEL,
            Self::RootPane => ACCESSIBLE_ROOT_PANE,
            Self::GlassPane => ACCESSIBLE_GLASS_PANE,
            Self::LayeredPane => ACCESSIBLE_LAYERED_PANE,
            Self::List => ACCESSIBLE_LIST,
            Self::ListItem => ACCESSIBLE_LIST_ITEM,
            Self::MenuBar => ACCESSIBLE_MENU_BAR,
            Self::PopupMenu => ACCESSIBLE_POPUP_MENU,
            Self::Menu => ACCESSIBLE_MENU,
            Self::MenuItem => ACCESSIBLE_MENU_ITEM,
            Self::SEPARATOR => ACCESSIBLE_SEPARATOR,
            Self::PageTabList => ACCESSIBLE_PAGE_TAB_LIST,
            Self::PageTab => ACCESSIBLE_PAGE_TAB,
            Self::Panel => ACCESSIBLE_PANEL,
            Self::ProgressBar => ACCESSIBLE_PROGRESS_BAR,
            Self::PasswordText => ACCESSIBLE_PASSWORD_TEXT,
            Self::PushButton => ACCESSIBLE_PUSH_BUTTON,
            Self::ToggleButton => ACCESSIBLE_TOGGLE_BUTTON,
            Self::CheckBox => ACCESSIBLE_CHECK_BOX,
            Self::RadioButton => ACCESSIBLE_RADIO_BUTTON,
            Self::RowHeader => ACCESSIBLE_ROW_HEADER,
            Self::ScrollPane => ACCESSIBLE_SCROLL_PANE,
            Self::ScrollBar => ACCESSIBLE_SCROLL_BAR,
            Self::Viewport => ACCESSIBLE_VIEWPORT,
            Self::Slider => ACCESSIBLE_SLIDER,
            Self::SplitPane => ACCESSIBLE_SPLIT_PANE,
            Self::Table => ACCESSIBLE_TABLE,
            Self::Text => ACCESSIBLE_TEXT,
            Self::Tree => ACCESSIBLE_TREE,
            Self::ToolBar => ACCESSIBLE_TOOL_BAR,
            Self::ToolTip => ACCESSIBLE_TOOL_TIP,
            Self::AwtComponent => ACCESSIBLE_AWT_COMPONENT,
            Self::SwingComponent => ACCESSIBLE_SWING_COMPONENT,
            Self::Unknown => ACCESSIBLE_UNKNOWN,
            Self::StatusBar => ACCESSIBLE_STATUS_BAR,
            Self::DateEditor => ACCESSIBLE_DATE_EDITOR,
            Self::SpinBox => ACCESSIBLE_SPIN_BOX,
            Self::FontChooser => ACCESSIBLE_FONT_CHOOSER,
            Self::GroupBox => ACCESSIBLE_GROUP_BOX,
            Self::Header => ACCESSIBLE_HEADER,
            Self::Footer => ACCESSIBLE_FOOTER,
            Self::Paragraph => ACCESSIBLE_PARAGRAPH,
            Self::Ruler => ACCESSIBLE_RULER,
            Self::EditBar => ACCESSIBLE_EDITBAR,
            Self::ProgressMonitor => PROGRESS_MONITOR,
        }
    }

    pub fn from_str(role: &str) -> Self {
        match role {
            ACCESSIBLE_ALERT => Self::Alert,
            ACCESSIBLE_COLUMN_HEADER => Self::ColumnHeader,
            ACCESSIBLE_CANVAS => Self::Canvas,
            ACCESSIBLE_COMBO_BOX => Self::ComboBox,
            ACCESSIBLE_DESKTOP_ICON => Self::DesktopIcon,
            ACCESSIBLE_INTERNAL_FRAME => Self::InternalFrame,
            ACCESSIBLE_DESKTOP_PANE => Self::DesktopPane,
            ACCESSIBLE_OPTION_PANE => Self::OptionPane,
            ACCESSIBLE_WINDOW => Self::Window,
            ACCESSIBLE_FRAME => Self::Frame,
            ACCESSIBLE_DIALOG => Self::Dialog,
            ACCESSIBLE_COLOR_CHOOSER => Self::ColorChooser,
            ACCESSIBLE_DIRECTORY_PANE => Self::DirectoryPane,
            ACCESSIBLE_FILE_CHOOSER => Self::FileChooser,
            ACCESSIBLE_FILLER => Self::Filler,
            ACCESSIBLE_HYPERLINK => Self::Hyperlink,
            ACCESSIBLE_ICON => Self::Icon,
            ACCESSIBLE_LABEL => Self::Label,
            ACCESSIBLE_ROOT_PANE => Self::RootPane,
            ACCESSIBLE_GLASS_PANE => Self::GlassPane,
            ACCESSIBLE_LAYERED_PANE => Self::LayeredPane,
            ACCESSIBLE_LIST => Self::List,
            ACCESSIBLE_LIST_ITEM => Self::ListItem,
            ACCESSIBLE_MENU_BAR => Self::MenuBar,
            ACCESSIBLE_POPUP_MENU => Self::PopupMenu,
            ACCESSIBLE_MENU => Self::Menu,
            ACCESSIBLE_MENU_ITEM => Self::MenuItem,
            ACCESSIBLE_SEPARATOR => Self::SEPARATOR,
            ACCESSIBLE_PAGE_TAB_LIST => Self::PageTabList,
            ACCESSIBLE_PAGE_TAB => Self::PageTab,
            ACCESSIBLE_PANEL => Self::Panel,
            ACCESSIBLE_PROGRESS_BAR => Self::ProgressBar,
            ACCESSIBLE_PASSWORD_TEXT => Self::PasswordText,
            ACCESSIBLE_PUSH_BUTTON => Self::PushButton,
            ACCESSIBLE_TOGGLE_BUTTON => Self::ToggleButton,
            ACCESSIBLE_CHECK_BOX => Self::CheckBox,
            ACCESSIBLE_RADIO_BUTTON => Self::RadioButton,
            ACCESSIBLE_ROW_HEADER => Self::RowHeader,
            ACCESSIBLE_SCROLL_PANE => Self::ScrollPane,
            ACCESSIBLE_SCROLL_BAR => Self::ScrollBar,
            ACCESSIBLE_VIEWPORT => Self::Viewport,
            ACCESSIBLE_SLIDER => Self::Slider,
            ACCESSIBLE_SPLIT_PANE => Self::SplitPane,
            ACCESSIBLE_TABLE => Self::Table,
            ACCESSIBLE_TEXT => Self::Text,
            ACCESSIBLE_TREE => Self::Tree,
            ACCESSIBLE_TOOL_BAR => Self::ToolBar,
            ACCESSIBLE_TOOL_TIP => Self::ToolTip,
            ACCESSIBLE_AWT_COMPONENT => Self::AwtComponent,
            ACCESSIBLE_SWING_COMPONENT => Self::SwingComponent,
            ACCESSIBLE_UNKNOWN => Self::Unknown,
            ACCESSIBLE_STATUS_BAR => Self::StatusBar,
            ACCESSIBLE_DATE_EDITOR => Self::DateEditor,
            ACCESSIBLE_SPIN_BOX => Self::SpinBox,
            ACCESSIBLE_FONT_CHOOSER => Self::FontChooser,
            ACCESSIBLE_GROUP_BOX => Self::GroupBox,
            ACCESSIBLE_HEADER => Self::Header,
            ACCESSIBLE_FOOTER => Self::Footer,
            ACCESSIBLE_PARAGRAPH => Self::Paragraph,
            ACCESSIBLE_RULER => Self::Ruler,
            ACCESSIBLE_EDITBAR => Self::EditBar,
            PROGRESS_MONITOR => Self::ProgressMonitor,
            _ => Self::Unknown,
        }
    }
}
