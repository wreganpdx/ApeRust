#[doc = "text input\n\nAn object used for text input. Adds support for text input and input\nmethods to applications. A text_input object is created from a\nwl_text_input_manager and corresponds typically to a text entry in an\napplication.\n\nRequests are used to activate/deactivate the text_input object and set\nstate information like surrounding and selected text or the content type.\nThe information about entered text is sent to the text_input object via\nthe pre-edit and commit events. Using this interface removes the need\nfor applications to directly process hardware key events and compose text\nout of them.\n\nText is generally UTF-8 encoded, indices and lengths are in bytes.\n\nSerials are used to synchronize the state between the text input and\nan input method. New serials are sent by the text input in the\ncommit_state request and are used by the input method to indicate\nthe known text input state in events like preedit_string, commit_string,\nand keysym. The text input can then ignore events from the input method\nwhich are based on an outdated state (for example after a reset).\n\nWarning! The protocol described in this file is experimental and\nbackward incompatible changes may be made. Backward compatible changes\nmay be added together with the corresponding interface version bump.\nBackward incompatible changes are done by bumping the version number in\nthe protocol and interface names and resetting the interface version.\nOnce the protocol is to be declared stable, the 'z' prefix and the\nversion number in the protocol and interface names are removed and the\ninterface version number is reset."]
pub mod zwp_text_input_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    #[doc = "content hint\n\nContent hint is a bitmask to allow to modify the behavior of the text\ninput."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum ContentHint {
        #[doc = "no special behaviour"]
        None = 0,
        #[doc = "auto completion, correction and capitalization"]
        Default = 7,
        #[doc = "hidden and sensitive text"]
        Password = 192,
        #[doc = "suggest word completions"]
        AutoCompletion = 1,
        #[doc = "suggest word corrections"]
        AutoCorrection = 2,
        #[doc = "switch to uppercase letters at the start of a sentence"]
        AutoCapitalization = 4,
        #[doc = "prefer lowercase letters"]
        Lowercase = 8,
        #[doc = "prefer uppercase letters"]
        Uppercase = 16,
        #[doc = "prefer casing for titles and headings (can be language dependent)"]
        Titlecase = 32,
        #[doc = "characters should be hidden"]
        HiddenText = 64,
        #[doc = "typed text should not be stored"]
        SensitiveData = 128,
        #[doc = "just latin characters should be entered"]
        Latin = 256,
        #[doc = "the text input is multiline"]
        Multiline = 512,
    }
    impl ContentHint {
        pub fn from_raw(n: u32) -> Option<ContentHint> {
            match n {
                0 => Some(ContentHint::None),
                7 => Some(ContentHint::Default),
                192 => Some(ContentHint::Password),
                1 => Some(ContentHint::AutoCompletion),
                2 => Some(ContentHint::AutoCorrection),
                4 => Some(ContentHint::AutoCapitalization),
                8 => Some(ContentHint::Lowercase),
                16 => Some(ContentHint::Uppercase),
                32 => Some(ContentHint::Titlecase),
                64 => Some(ContentHint::HiddenText),
                128 => Some(ContentHint::SensitiveData),
                256 => Some(ContentHint::Latin),
                512 => Some(ContentHint::Multiline),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "content purpose\n\nThe content purpose allows to specify the primary purpose of a text\ninput.\n\nThis allows an input method to show special purpose input panels with\nextra characters or to disallow some characters."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum ContentPurpose {
        #[doc = "default input, allowing all characters"]
        Normal = 0,
        #[doc = "allow only alphabetic characters"]
        Alpha = 1,
        #[doc = "allow only digits"]
        Digits = 2,
        #[doc = "input a number (including decimal separator and sign)"]
        Number = 3,
        #[doc = "input a phone number"]
        Phone = 4,
        #[doc = "input an URL"]
        Url = 5,
        #[doc = "input an email address"]
        Email = 6,
        #[doc = "input a name of a person"]
        Name = 7,
        #[doc = "input a password (combine with password or sensitive_data hint)"]
        Password = 8,
        #[doc = "input a date"]
        Date = 9,
        #[doc = "input a time"]
        Time = 10,
        #[doc = "input a date and time"]
        Datetime = 11,
        #[doc = "input for a terminal"]
        Terminal = 12,
    }
    impl ContentPurpose {
        pub fn from_raw(n: u32) -> Option<ContentPurpose> {
            match n {
                0 => Some(ContentPurpose::Normal),
                1 => Some(ContentPurpose::Alpha),
                2 => Some(ContentPurpose::Digits),
                3 => Some(ContentPurpose::Number),
                4 => Some(ContentPurpose::Phone),
                5 => Some(ContentPurpose::Url),
                6 => Some(ContentPurpose::Email),
                7 => Some(ContentPurpose::Name),
                8 => Some(ContentPurpose::Password),
                9 => Some(ContentPurpose::Date),
                10 => Some(ContentPurpose::Time),
                11 => Some(ContentPurpose::Datetime),
                12 => Some(ContentPurpose::Terminal),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum PreeditStyle {
        #[doc = "default style for composing text"]
        Default = 0,
        #[doc = "style should be the same as in non-composing text"]
        None = 1,
        Active = 2,
        Inactive = 3,
        Highlight = 4,
        Underline = 5,
        Selection = 6,
        Incorrect = 7,
    }
    impl PreeditStyle {
        pub fn from_raw(n: u32) -> Option<PreeditStyle> {
            match n {
                0 => Some(PreeditStyle::Default),
                1 => Some(PreeditStyle::None),
                2 => Some(PreeditStyle::Active),
                3 => Some(PreeditStyle::Inactive),
                4 => Some(PreeditStyle::Highlight),
                5 => Some(PreeditStyle::Underline),
                6 => Some(PreeditStyle::Selection),
                7 => Some(PreeditStyle::Incorrect),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum TextDirection {
        #[doc = "automatic text direction based on text and language"]
        Auto = 0,
        #[doc = "left-to-right"]
        Ltr = 1,
        #[doc = "right-to-left"]
        Rtl = 2,
    }
    impl TextDirection {
        pub fn from_raw(n: u32) -> Option<TextDirection> {
            match n {
                0 => Some(TextDirection::Auto),
                1 => Some(TextDirection::Ltr),
                2 => Some(TextDirection::Rtl),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub enum Request {
        #[doc = "request activation\n\nRequests the text_input object to be activated (typically when the\ntext entry gets focus).\n\nThe seat argument is a wl_seat which maintains the focus for this\nactivation. The surface argument is a wl_surface assigned to the\ntext_input object and tracked for focus lost. The enter event\nis emitted on successful activation."]
        Activate {
            seat: Proxy<super::wl_seat::WlSeat>,
            surface: Proxy<super::wl_surface::WlSurface>,
        },
        #[doc = "request deactivation\n\nRequests the text_input object to be deactivated (typically when the\ntext entry lost focus). The seat argument is a wl_seat which was used\nfor activation."]
        Deactivate { seat: Proxy<super::wl_seat::WlSeat> },
        #[doc = "show input panels\n\nRequests input panels (virtual keyboard) to show."]
        ShowInputPanel,
        #[doc = "hide input panels\n\nRequests input panels (virtual keyboard) to hide."]
        HideInputPanel,
        #[doc = "reset\n\nShould be called by an editor widget when the input state should be\nreset, for example after the text was changed outside of the normal\ninput method flow."]
        Reset,
        #[doc = "sets the surrounding text\n\nSets the plain surrounding text around the input position. Text is\nUTF-8 encoded. Cursor is the byte offset within the\nsurrounding text. Anchor is the byte offset of the\nselection anchor within the surrounding text. If there is no selected\ntext anchor, then it is the same as cursor."]
        SetSurroundingText {
            text: String,
            cursor: u32,
            anchor: u32,
        },
        #[doc = "set content purpose and hint\n\nSets the content purpose and content hint. While the purpose is the\nbasic purpose of an input field, the hint flags allow to modify some\nof the behavior.\n\nWhen no content type is explicitly set, a normal content purpose with\ndefault hints (auto completion, auto correction, auto capitalization)\nshould be assumed."]
        SetContentType { hint: u32, purpose: u32 },
        #[doc = ""]
        SetCursorRectangle {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "sets preferred language\n\nSets a specific language. This allows for example a virtual keyboard to\nshow a language specific layout. The \"language\" argument is an RFC-3066\nformat language tag.\n\nIt could be used for example in a word processor to indicate the\nlanguage of the currently edited document or in an instant message\napplication which tracks languages of contacts."]
        SetPreferredLanguage { language: String },
        #[doc = ""]
        CommitState { serial: u32 },
        #[doc = ""]
        InvokeAction { button: u32, index: u32 },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "activate",
                since: 1,
                signature: &[super::ArgumentType::Object, super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "deactivate",
                since: 1,
                signature: &[super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "show_input_panel",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "hide_input_panel",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "reset",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "set_surrounding_text",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
            },
            super::MessageDesc {
                name: "set_content_type",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "set_cursor_rectangle",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
            },
            super::MessageDesc {
                name: "set_preferred_language",
                since: 1,
                signature: &[super::ArgumentType::Str],
            },
            super::MessageDesc {
                name: "commit_state",
                since: 1,
                signature: &[super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "invoke_action",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Activate { .. } => 0,
                Request::Deactivate { .. } => 1,
                Request::ShowInputPanel => 2,
                Request::HideInputPanel => 3,
                Request::Reset => 4,
                Request::SetSurroundingText { .. } => 5,
                Request::SetContentType { .. } => 6,
                Request::SetCursorRectangle { .. } => 7,
                Request::SetPreferredLanguage { .. } => 8,
                Request::CommitState { .. } => 9,
                Request::InvokeAction { .. } => 10,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Activate { seat, surface } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![Argument::Object(seat.id()), Argument::Object(surface.id())],
                },
                Request::Deactivate { seat } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::Object(seat.id())],
                },
                Request::ShowInputPanel => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: vec![],
                },
                Request::HideInputPanel => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: vec![],
                },
                Request::Reset => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: vec![],
                },
                Request::SetSurroundingText {
                    text,
                    cursor,
                    anchor,
                } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: vec![
                        Argument::Str(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(text.into())
                        }),
                        Argument::Uint(cursor),
                        Argument::Uint(anchor),
                    ],
                },
                Request::SetContentType { hint, purpose } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: vec![Argument::Uint(hint), Argument::Uint(purpose)],
                },
                Request::SetCursorRectangle {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 7,
                    args: vec![
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
                },
                Request::SetPreferredLanguage { language } => Message {
                    sender_id: sender_id,
                    opcode: 8,
                    args: vec![Argument::Str(unsafe {
                        ::std::ffi::CString::from_vec_unchecked(language.into())
                    })],
                },
                Request::CommitState { serial } => Message {
                    sender_id: sender_id,
                    opcode: 9,
                    args: vec![Argument::Uint(serial)],
                },
                Request::InvokeAction { button, index } => Message {
                    sender_id: sender_id,
                    opcode: 10,
                    args: vec![Argument::Uint(button), Argument::Uint(index)],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Activate { seat, surface } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::Deactivate { seat } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
                Request::ShowInputPanel => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                }
                Request::HideInputPanel => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                }
                Request::Reset => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(4, &mut _args_array)
                }
                Request::SetSurroundingText {
                    text,
                    cursor,
                    anchor,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(text).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].u = cursor;
                    _args_array[2].u = anchor;
                    f(5, &mut _args_array)
                }
                Request::SetContentType { hint, purpose } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = hint;
                    _args_array[1].u = purpose;
                    f(6, &mut _args_array)
                }
                Request::SetCursorRectangle {
                    x,
                    y,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(7, &mut _args_array)
                }
                Request::SetPreferredLanguage { language } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(language).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(8, &mut _args_array)
                }
                Request::CommitState { serial } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(9, &mut _args_array)
                }
                Request::InvokeAction { button, index } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = button;
                    _args_array[1].u = index;
                    f(10, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "enter event\n\nNotify the text_input object when it received focus. Typically in\nresponse to an activate request."]
        Enter {
            surface: Proxy<super::wl_surface::WlSurface>,
        },
        #[doc = "leave event\n\nNotify the text_input object when it lost focus. Either in response\nto a deactivate request or when the assigned surface lost focus or was\ndestroyed."]
        Leave,
        #[doc = "modifiers map\n\nTransfer an array of 0-terminated modifier names. The position in\nthe array is the index of the modifier as used in the modifiers\nbitmask in the keysym event."]
        ModifiersMap { map: Vec<u8> },
        #[doc = "state of the input panel\n\nNotify when the visibility state of the input panel changed."]
        InputPanelState { state: u32 },
        #[doc = "pre-edit\n\nNotify when a new composing text (pre-edit) should be set around the\ncurrent cursor position. Any previously set composing text should\nbe removed.\n\nThe commit text can be used to replace the preedit text on reset\n(for example on unfocus).\n\nThe text input should also handle all preedit_style and preedit_cursor\nevents occurring directly before preedit_string."]
        PreeditString {
            serial: u32,
            text: String,
            commit: String,
        },
        #[doc = "pre-edit styling\n\nSets styling information on composing text. The style is applied for\nlength bytes from index relative to the beginning of the composing\ntext (as byte offset). Multiple styles can\nbe applied to a composing text by sending multiple preedit_styling\nevents.\n\nThis event is handled as part of a following preedit_string event."]
        PreeditStyling { index: u32, length: u32, style: u32 },
        #[doc = "pre-edit cursor\n\nSets the cursor position inside the composing text (as byte\noffset) relative to the start of the composing text. When index is a\nnegative number no cursor is shown.\n\nThis event is handled as part of a following preedit_string event."]
        PreeditCursor { index: i32 },
        #[doc = "commit\n\nNotify when text should be inserted into the editor widget. The text to\ncommit could be either just a single character after a key press or the\nresult of some composing (pre-edit). It could also be an empty text\nwhen some text should be removed (see delete_surrounding_text) or when\nthe input cursor should be moved (see cursor_position).\n\nAny previously set composing text should be removed."]
        CommitString { serial: u32, text: String },
        #[doc = "set cursor to new position\n\nNotify when the cursor or anchor position should be modified.\n\nThis event should be handled as part of a following commit_string\nevent."]
        CursorPosition { index: i32, anchor: i32 },
        #[doc = "delete surrounding text\n\nNotify when the text around the current cursor position should be\ndeleted.\n\nIndex is relative to the current cursor (in bytes).\nLength is the length of deleted text (in bytes).\n\nThis event should be handled as part of a following commit_string\nevent."]
        DeleteSurroundingText { index: i32, length: u32 },
        #[doc = "keysym\n\nNotify when a key event was sent. Key events should not be used\nfor normal text input operations, which should be done with\ncommit_string, delete_surrounding_text, etc. The key event follows\nthe wl_keyboard key event convention. Sym is an XKB keysym, state a\nwl_keyboard key_state. Modifiers are a mask for effective modifiers\n(where the modifier indices are set by the modifiers_map event)"]
        Keysym {
            serial: u32,
            time: u32,
            sym: u32,
            state: u32,
            modifiers: u32,
        },
        #[doc = "language\n\nSets the language of the input text. The \"language\" argument is an\nRFC-3066 format language tag."]
        Language { serial: u32, language: String },
        #[doc = "text direction\n\nSets the text direction of input text.\n\nIt is mainly needed for showing an input cursor on the correct side of\nthe editor when there is no input done yet and making sure neutral\ndirection text is laid out properly."]
        TextDirection { serial: u32, direction: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "enter",
                since: 1,
                signature: &[super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "leave",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "modifiers_map",
                since: 1,
                signature: &[super::ArgumentType::Array],
            },
            super::MessageDesc {
                name: "input_panel_state",
                since: 1,
                signature: &[super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "preedit_string",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Str,
                    super::ArgumentType::Str,
                ],
            },
            super::MessageDesc {
                name: "preedit_styling",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
            },
            super::MessageDesc {
                name: "preedit_cursor",
                since: 1,
                signature: &[super::ArgumentType::Int],
            },
            super::MessageDesc {
                name: "commit_string",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Str],
            },
            super::MessageDesc {
                name: "cursor_position",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
            },
            super::MessageDesc {
                name: "delete_surrounding_text",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "keysym",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
            },
            super::MessageDesc {
                name: "language",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Str],
            },
            super::MessageDesc {
                name: "text_direction",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Enter { .. } => 0,
                Event::Leave => 1,
                Event::ModifiersMap { .. } => 2,
                Event::InputPanelState { .. } => 3,
                Event::PreeditString { .. } => 4,
                Event::PreeditStyling { .. } => 5,
                Event::PreeditCursor { .. } => 6,
                Event::CommitString { .. } => 7,
                Event::CursorPosition { .. } => 8,
                Event::DeleteSurroundingText { .. } => 9,
                Event::Keysym { .. } => 10,
                Event::Language { .. } => 11,
                Event::TextDirection { .. } => 12,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Enter {
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Event::Leave),
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::ModifiersMap {
                        map: {
                            if let Some(Argument::Array(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::InputPanelState {
                        state: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::PreeditString {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        text: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        commit: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                5 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::PreeditStyling {
                        index: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        length: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        style: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                6 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::PreeditCursor {
                        index: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                7 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::CommitString {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        text: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                8 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::CursorPosition {
                        index: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        anchor: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                9 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::DeleteSurroundingText {
                        index: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        length: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                10 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Keysym {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        sym: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        state: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        modifiers: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                11 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Language {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        language: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                12 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::TextDirection {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        direction: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Enter {
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[0].o as *mut _,
                        ),
                    })
                }
                1 => Ok(Event::Leave),
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::ModifiersMap {
                        map: {
                            let array = &*_args[0].a;
                            ::std::slice::from_raw_parts(array.data as *const u8, array.size)
                                .to_owned()
                        },
                    })
                }
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::InputPanelState { state: _args[0].u })
                }
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::PreeditString {
                        serial: _args[0].u,
                        text: ::std::ffi::CStr::from_ptr(_args[1].s)
                            .to_string_lossy()
                            .into_owned(),
                        commit: ::std::ffi::CStr::from_ptr(_args[2].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::PreeditStyling {
                        index: _args[0].u,
                        length: _args[1].u,
                        style: _args[2].u,
                    })
                }
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::PreeditCursor { index: _args[0].i })
                }
                7 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::CommitString {
                        serial: _args[0].u,
                        text: ::std::ffi::CStr::from_ptr(_args[1].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                8 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::CursorPosition {
                        index: _args[0].i,
                        anchor: _args[1].i,
                    })
                }
                9 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::DeleteSurroundingText {
                        index: _args[0].i,
                        length: _args[1].u,
                    })
                }
                10 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::Keysym {
                        serial: _args[0].u,
                        time: _args[1].u,
                        sym: _args[2].u,
                        state: _args[3].u,
                        modifiers: _args[4].u,
                    })
                }
                11 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Language {
                        serial: _args[0].u,
                        language: ::std::ffi::CStr::from_ptr(_args[1].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                12 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::TextDirection {
                        serial: _args[0].u,
                        direction: _args[1].u,
                    })
                }
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    pub struct ZwpTextInputV1;
    impl Interface for ZwpTextInputV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_text_input_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_text_input_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "request activation\n\nRequests the text_input object to be activated (typically when the\ntext entry gets focus).\n\nThe seat argument is a wl_seat which maintains the focus for this\nactivation. The surface argument is a wl_surface assigned to the\ntext_input object and tracked for focus lost. The enter event\nis emitted on successful activation."]
        fn activate(
            &self,
            seat: &Proxy<super::wl_seat::WlSeat>,
            surface: &Proxy<super::wl_surface::WlSurface>,
        ) -> ();
        #[doc = "request deactivation\n\nRequests the text_input object to be deactivated (typically when the\ntext entry lost focus). The seat argument is a wl_seat which was used\nfor activation."]
        fn deactivate(&self, seat: &Proxy<super::wl_seat::WlSeat>) -> ();
        #[doc = "show input panels\n\nRequests input panels (virtual keyboard) to show."]
        fn show_input_panel(&self) -> ();
        #[doc = "hide input panels\n\nRequests input panels (virtual keyboard) to hide."]
        fn hide_input_panel(&self) -> ();
        #[doc = "reset\n\nShould be called by an editor widget when the input state should be\nreset, for example after the text was changed outside of the normal\ninput method flow."]
        fn reset(&self) -> ();
        #[doc = "sets the surrounding text\n\nSets the plain surrounding text around the input position. Text is\nUTF-8 encoded. Cursor is the byte offset within the\nsurrounding text. Anchor is the byte offset of the\nselection anchor within the surrounding text. If there is no selected\ntext anchor, then it is the same as cursor."]
        fn set_surrounding_text(&self, text: String, cursor: u32, anchor: u32) -> ();
        #[doc = "set content purpose and hint\n\nSets the content purpose and content hint. While the purpose is the\nbasic purpose of an input field, the hint flags allow to modify some\nof the behavior.\n\nWhen no content type is explicitly set, a normal content purpose with\ndefault hints (auto completion, auto correction, auto capitalization)\nshould be assumed."]
        fn set_content_type(&self, hint: u32, purpose: u32) -> ();
        #[doc = ""]
        fn set_cursor_rectangle(&self, x: i32, y: i32, width: i32, height: i32) -> ();
        #[doc = "sets preferred language\n\nSets a specific language. This allows for example a virtual keyboard to\nshow a language specific layout. The \"language\" argument is an RFC-3066\nformat language tag.\n\nIt could be used for example in a word processor to indicate the\nlanguage of the currently edited document or in an instant message\napplication which tracks languages of contacts."]
        fn set_preferred_language(&self, language: String) -> ();
        #[doc = ""]
        fn commit_state(&self, serial: u32) -> ();
        #[doc = ""]
        fn invoke_action(&self, button: u32, index: u32) -> ();
    }
    impl RequestsTrait for Proxy<ZwpTextInputV1> {
        fn activate(
            &self,
            seat: &Proxy<super::wl_seat::WlSeat>,
            surface: &Proxy<super::wl_surface::WlSurface>,
        ) -> () {
            let msg = Request::Activate {
                seat: seat.clone(),
                surface: surface.clone(),
            };
            self.send(msg);
        }
        fn deactivate(&self, seat: &Proxy<super::wl_seat::WlSeat>) -> () {
            let msg = Request::Deactivate { seat: seat.clone() };
            self.send(msg);
        }
        fn show_input_panel(&self) -> () {
            let msg = Request::ShowInputPanel;
            self.send(msg);
        }
        fn hide_input_panel(&self) -> () {
            let msg = Request::HideInputPanel;
            self.send(msg);
        }
        fn reset(&self) -> () {
            let msg = Request::Reset;
            self.send(msg);
        }
        fn set_surrounding_text(&self, text: String, cursor: u32, anchor: u32) -> () {
            let msg = Request::SetSurroundingText {
                text: text,
                cursor: cursor,
                anchor: anchor,
            };
            self.send(msg);
        }
        fn set_content_type(&self, hint: u32, purpose: u32) -> () {
            let msg = Request::SetContentType {
                hint: hint,
                purpose: purpose,
            };
            self.send(msg);
        }
        fn set_cursor_rectangle(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::SetCursorRectangle {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }
        fn set_preferred_language(&self, language: String) -> () {
            let msg = Request::SetPreferredLanguage { language: language };
            self.send(msg);
        }
        fn commit_state(&self, serial: u32) -> () {
            let msg = Request::CommitState { serial: serial };
            self.send(msg);
        }
        fn invoke_action(&self, button: u32, index: u32) -> () {
            let msg = Request::InvokeAction {
                button: button,
                index: index,
            };
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ACTIVATE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DEACTIVATE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SHOW_INPUT_PANEL_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_HIDE_INPUT_PANEL_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RESET_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_SURROUNDING_TEXT_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_CONTENT_TYPE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_CURSOR_RECTANGLE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_PREFERRED_LANGUAGE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COMMIT_STATE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_INVOKE_ACTION_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ENTER_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LEAVE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MODIFIERS_MAP_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_INPUT_PANEL_STATE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PREEDIT_STRING_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PREEDIT_STYLING_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PREEDIT_CURSOR_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_COMMIT_STRING_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CURSOR_POSITION_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DELETE_SURROUNDING_TEXT_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_KEYSYM_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LANGUAGE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_TEXT_DIRECTION_SINCE: u16 = 1u16;
}
#[doc = "text input manager\n\nA factory for text_input objects. This object is a global singleton."]
pub mod zwp_text_input_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "create text input\n\nCreates a new text_input object."]
        CreateTextInput {
            id: Proxy<super::zwp_text_input_v1::ZwpTextInputV1>,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "create_text_input",
            since: 1,
            signature: &[super::ArgumentType::NewId],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::CreateTextInput { .. } => 0,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwp_text_input_v1::ZwpTextInputV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::CreateTextInput { id } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![Argument::NewId(id.id())],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::CreateTextInput { id } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    pub struct ZwpTextInputManagerV1;
    impl Interface for ZwpTextInputManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_text_input_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_text_input_manager_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "create text input\n\nCreates a new text_input object."]
        fn create_text_input<F>(
            &self,
            implementor: F,
        ) -> Result<Proxy<super::zwp_text_input_v1::ZwpTextInputV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_text_input_v1::ZwpTextInputV1>,
            ) -> Proxy<super::zwp_text_input_v1::ZwpTextInputV1>;
    }
    impl RequestsTrait for Proxy<ZwpTextInputManagerV1> {
        fn create_text_input<F>(
            &self,
            implementor: F,
        ) -> Result<Proxy<super::zwp_text_input_v1::ZwpTextInputV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_text_input_v1::ZwpTextInputV1>,
            ) -> Proxy<super::zwp_text_input_v1::ZwpTextInputV1>,
        {
            let msg = Request::CreateTextInput {
                id: self.child_placeholder(),
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_TEXT_INPUT_SINCE: u16 = 1u16;
}