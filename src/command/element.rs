/// 액션 분류의 엘리먼트([`Button`](crate::element::action::Button), [`Link`](crate::element::action::Link))를 위한 명령
pub mod action;

/// complex 분류의 엘리먼트([`SapTable`](crate::element::complex::SapTable))를 위한 명령
pub mod complex;

/// 선택 분류의 엘리먼트([`ListBox`](crate::element::selection::list_box::ListBox), [`ComboBox`](crate::element::selection::ComboBox))를 위한 명령
pub mod selection;

/// 레이아웃 분류의 엘리먼트를 위한 명령
pub mod layout;

/// 텍스트 분류의 엘리먼트([`Caption`](crate::element::text::Caption), [`InputField`](crate::element::text::InputField), [`Label`](crate::element::text::Label), [`TextView`](crate::element::text::TextView)를 위한 명령)
pub mod text;

/// 시스템 분류의 엘리먼트([`ClientInspector`](crate::element::system::ClientInspector), [`Custom`](system::Custom), [`LoadingPlaceholder`](crate::element::system::LoadingPlaceholder))를 위한 명령
pub mod system;
