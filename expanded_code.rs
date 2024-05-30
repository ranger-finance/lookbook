#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use core::cell::RefCell;
use dioxus::prelude::*;
pub use lookbook_macros::preview;
mod control {
    use dioxus::prelude::*;
    use serde::{Deserialize, Serialize};
    /// A controllable property.
    pub trait Control: Sized {
        type State;
        /// Create the initial state.
        fn state(default: Option<impl Into<Self>>) -> Self::State;
        /// Convert the current state to `Self`.
        fn from_state(state: Signal<&Self::State>) -> Self;
        /// Render the controller element.
        fn control(name: &'static str, state: Signal<Self::State>) -> Element;
    }
    impl Control for &str {
        type State = String;
        fn state(default: Option<impl Into<Self>>) -> Self::State {
            default.map(Into::into).map(String::from).unwrap_or_default()
        }
        fn from_state(state: Signal<&Self::State>) -> Self {
            let value_ref = &state.read();
            value_ref.as_str()
        }
        fn control(_name: &'static str, mut state: Signal<Self::State>) -> Element {
            let value = "{state.read()}";
            Some({
                static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                    name: "src/control.rs:37:9:2805",
                    roots: &[
                        dioxus_core::TemplateNode::Dynamic {
                            id: 0usize,
                        },
                    ],
                    node_paths: &[&[0u8]],
                    attr_paths: &[],
                };
                {
                    let __vnodes = dioxus_core::VNode::new(
                        None,
                        TEMPLATE,
                        Box::new([
                            dioxus_core::DynamicNode::Component({
                                use dioxus_core::prelude::Properties;
                                (fc_to_builder(Input)
                                    .value(value)
                                    .oninput(move |event: FormEvent| {
                                        let val = event.data.value().clone();
                                        *state.write() = val;
                                    })
                                    .build())
                                    .into_vcomponent(Input, "Input")
                            }),
                        ]),
                        Box::new([]),
                    );
                    __vnodes
                }
            })
        }
    }
    pub struct Json<T>(pub T);
    #[automatically_derived]
    impl<T: ::core::default::Default> ::core::default::Default for Json<T> {
        #[inline]
        fn default() -> Json<T> {
            Json(::core::default::Default::default())
        }
    }
    impl<T> From<T> for Json<T> {
        fn from(value: T) -> Self {
            Self(value)
        }
    }
    impl<T> Control for Json<T>
    where
        T: 'static + Clone + Default + Deserialize<'static> + Serialize,
    {
        type State = T;
        fn state(default: Option<impl Into<Self>>) -> Self::State {
            default.map(Into::into).unwrap_or_default().0
        }
        fn from_state(state: Signal<&Self::State>) -> Self {
            Self(state.read().clone())
        }
        fn control(_name: &'static str, mut state: Signal<Self::State>) -> Element {
            let value = "{serde_json::to_string(&*state.read()).unwrap().as_str()}";
            Some({
                static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                    name: "src/control.rs:95:9:4412",
                    roots: &[
                        dioxus_core::TemplateNode::Dynamic {
                            id: 0usize,
                        },
                    ],
                    node_paths: &[&[0u8]],
                    attr_paths: &[],
                };
                {
                    let __vnodes = dioxus_core::VNode::new(
                        None,
                        TEMPLATE,
                        Box::new([
                            dioxus_core::DynamicNode::Component({
                                use dioxus_core::prelude::Properties;
                                (fc_to_builder(Input)
                                    .value(value)
                                    .oninput(move |_event: FormEvent| {
                                        let value = "{event.data.value().clone().as_str()}";
                                        if let Ok(new_state) = serde_json::from_str::<T>(value) {
                                            *state.write() = new_state;
                                        }
                                    })
                                    .build())
                                    .into_vcomponent(Input, "Input")
                            }),
                        ]),
                        Box::new([]),
                    );
                    __vnodes
                }
            })
        }
    }
    ///Properties for the [`Input`] component.
    #[allow(non_camel_case_types)]
    struct InputProps {
        value: &'static str,
        oninput: EventHandler<FormEvent>,
    }
    impl InputProps {
        /**
Create a builder for building `InputProps`.
On the builder, call `.value(...)`, `.oninput(...)` to set the values of the fields.
Finally, call `.build()` to create the instance of `InputProps`.
                    */
        #[allow(dead_code, clippy::type_complexity)]
        fn builder() -> InputPropsBuilder<((), ())> {
            InputPropsBuilder {
                owner: Owner::default(),
                fields: ((), ()),
                _phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    struct InputPropsBuilder<TypedBuilderFields> {
        owner: Owner,
        fields: TypedBuilderFields,
        _phantom: (),
    }
    impl dioxus_core::prelude::Properties for InputProps
    where
        Self: Clone,
    {
        type Builder = InputPropsBuilder<((), ())>;
        fn builder() -> Self::Builder {
            InputProps::builder()
        }
        fn memoize(&mut self, new: &Self) -> bool {
            let equal = self == new;
            self.oninput.__set(new.oninput.__take());
            if !equal {
                let new_clone = new.clone();
                self.value = new_clone.value;
            }
            equal
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub trait InputPropsBuilder_Optional<T> {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
    }
    impl<T> InputPropsBuilder_Optional<T> for () {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
            default()
        }
    }
    impl<T> InputPropsBuilder_Optional<T> for (T,) {
        fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
            self.0
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__oninput> InputPropsBuilder<((), __oninput)> {
        #[allow(clippy::type_complexity)]
        pub fn value(
            self,
            value: &'static str,
        ) -> InputPropsBuilder<((&'static str,), __oninput)> {
            let value = (value,);
            let (_, oninput) = self.fields;
            InputPropsBuilder {
                owner: self.owner,
                fields: (value, oninput),
                _phantom: self._phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum InputPropsBuilder_Error_Repeated_field_value {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__oninput> InputPropsBuilder<((&'static str,), __oninput)> {
        #[deprecated(note = "Repeated field value")]
        #[allow(clippy::type_complexity)]
        pub fn value(
            self,
            _: InputPropsBuilder_Error_Repeated_field_value,
        ) -> InputPropsBuilder<((&'static str,), __oninput)> {
            self
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__value> InputPropsBuilder<(__value, ())> {
        #[allow(clippy::type_complexity)]
        pub fn oninput<__Marker>(
            self,
            oninput: impl dioxus_core::prelude::SuperInto<
                EventHandler<FormEvent>,
                __Marker,
            >,
        ) -> InputPropsBuilder<(__value, (EventHandler<FormEvent>,))> {
            let oninput = (
                with_owner(
                    self.owner.clone(),
                    move || dioxus_core::prelude::SuperInto::super_into(oninput),
                ),
            );
            let (value, _) = self.fields;
            InputPropsBuilder {
                owner: self.owner,
                fields: (value, oninput),
                _phantom: self._phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum InputPropsBuilder_Error_Repeated_field_oninput {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__value> InputPropsBuilder<(__value, (EventHandler<FormEvent>,))> {
        #[deprecated(note = "Repeated field oninput")]
        #[allow(clippy::type_complexity)]
        pub fn oninput<__Marker>(
            self,
            _: InputPropsBuilder_Error_Repeated_field_oninput,
        ) -> InputPropsBuilder<(__value, (EventHandler<FormEvent>,))> {
            self
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum InputPropsBuilder_Error_Missing_required_field_value {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl<__oninput> InputPropsBuilder<((), __oninput)> {
        #[deprecated(note = "Missing required field value")]
        pub fn build(
            self,
            _: InputPropsBuilder_Error_Missing_required_field_value,
        ) -> InputProps {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                const fn panic_cold_explicit() -> ! {
                    ::core::panicking::panic_explicit()
                }
                panic_cold_explicit();
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum InputPropsBuilder_Error_Missing_required_field_oninput {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl InputPropsBuilder<((&'static str,), ())> {
        #[deprecated(note = "Missing required field oninput")]
        pub fn build(
            self,
            _: InputPropsBuilder_Error_Missing_required_field_oninput,
        ) -> InputProps {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                const fn panic_cold_explicit() -> ! {
                    ::core::panicking::panic_explicit()
                }
                panic_cold_explicit();
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    struct InputPropsWithOwner {
        inner: InputProps,
        owner: Owner,
    }
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl ::core::clone::Clone for InputPropsWithOwner {
        #[inline]
        fn clone(&self) -> InputPropsWithOwner {
            InputPropsWithOwner {
                inner: ::core::clone::Clone::clone(&self.inner),
                owner: ::core::clone::Clone::clone(&self.owner),
            }
        }
    }
    impl PartialEq for InputPropsWithOwner {
        fn eq(&self, other: &Self) -> bool {
            self.inner.eq(&other.inner)
        }
    }
    impl InputPropsWithOwner {
        /// Create a component from the props.
        pub fn into_vcomponent<M: 'static>(
            self,
            render_fn: impl dioxus_core::prelude::ComponentFunction<InputProps, M>,
            component_name: &'static str,
        ) -> dioxus_core::VComponent {
            use dioxus_core::prelude::ComponentFunction;
            dioxus_core::VComponent::new(
                move |wrapper: Self| render_fn.rebuild(wrapper.inner),
                self,
                component_name,
            )
        }
    }
    impl dioxus_core::prelude::Properties for InputPropsWithOwner {
        type Builder = ();
        fn builder() -> Self::Builder {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn memoize(&mut self, new: &Self) -> bool {
            self.inner.memoize(&new.inner)
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl InputPropsBuilder<((&'static str,), (EventHandler<FormEvent>,))> {
        pub fn build(self) -> InputPropsWithOwner {
            let (value, oninput) = self.fields;
            let value = value.0;
            let oninput = oninput.0;
            InputPropsWithOwner {
                inner: InputProps { value, oninput },
                owner: self.owner,
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for InputProps {
        #[inline]
        fn clone(&self) -> InputProps {
            InputProps {
                value: ::core::clone::Clone::clone(&self.value),
                oninput: ::core::clone::Clone::clone(&self.oninput),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for InputProps {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for InputProps {
        #[inline]
        fn eq(&self, other: &InputProps) -> bool {
            self.value == other.value && self.oninput == other.oninput
        }
    }
    #[allow(non_snake_case)]
    fn Input(mut __props: InputProps) -> Element {
        let InputProps { mut value, mut oninput } = __props;
        {
            Some({
                static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                    name: "src/control.rs:113:5:4951",
                    roots: &[
                        dioxus_core::TemplateNode::Element {
                            tag: dioxus_elements::input::TAG_NAME,
                            namespace: dioxus_elements::input::NAME_SPACE,
                            attrs: &[
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::input::border.0,
                                    namespace: dioxus_elements::input::border.1,
                                    value: "2px solid #e7e7e7",
                                },
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::input::padding.0,
                                    namespace: dioxus_elements::input::padding.1,
                                    value: "10px",
                                },
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::input::outline.0,
                                    namespace: dioxus_elements::input::outline.1,
                                    value: "none",
                                },
                                dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::input::background.0,
                                    namespace: dioxus_elements::input::background.1,
                                    value: "none",
                                },
                                dioxus_core::TemplateAttribute::Dynamic {
                                    id: 0usize,
                                },
                                dioxus_core::TemplateAttribute::Dynamic {
                                    id: 1usize,
                                },
                            ],
                            children: &[],
                        },
                    ],
                    node_paths: &[],
                    attr_paths: &[&[0u8], &[0u8]],
                };
                {
                    let __vnodes = dioxus_core::VNode::new(
                        None,
                        TEMPLATE,
                        Box::new([]),
                        Box::new([
                            Box::new([
                                dioxus_core::Attribute::new(
                                    dioxus_elements::input::value.0,
                                    (value).to_string().to_string(),
                                    dioxus_elements::input::value.1,
                                    dioxus_elements::input::value.2,
                                ),
                            ]),
                            Box::new([
                                dioxus_elements::events::oninput(move |event| {
                                    oninput.call(event)
                                }),
                            ]),
                        ]),
                    );
                    __vnodes
                }
            })
        }
    }
}
pub use control::{Control, Json};
mod ui {
    mod look {
        use crate::ui::pane::VerticalPane;
        use dioxus::prelude::*;
        ///Properties for the [`Look`] component.
        #[allow(non_camel_case_types)]
        pub struct LookProps {
            pub name: &'static str,
            pub docs: &'static str,
            pub controls: Element,
            pub children: Element,
        }
        impl LookProps {
            /**
Create a builder for building `LookProps`.
On the builder, call `.name(...)`, `.docs(...)`, `.controls(...)`, `.children(...)`(optional) to set the values of the fields.
Finally, call `.build()` to create the instance of `LookProps`.
                    */
            #[allow(dead_code, clippy::type_complexity)]
            pub fn builder() -> LookPropsBuilder<((), (), (), ())> {
                LookPropsBuilder {
                    fields: ((), (), (), ()),
                    _phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub struct LookPropsBuilder<TypedBuilderFields> {
            fields: TypedBuilderFields,
            _phantom: (),
        }
        impl dioxus_core::prelude::Properties for LookProps
        where
            Self: Clone,
        {
            type Builder = LookPropsBuilder<((), (), (), ())>;
            fn builder() -> Self::Builder {
                LookProps::builder()
            }
            fn memoize(&mut self, new: &Self) -> bool {
                let equal = self == new;
                if !equal {
                    let new_clone = new.clone();
                    self.name = new_clone.name;
                    self.docs = new_clone.docs;
                    self.controls = new_clone.controls;
                    self.children = new_clone.children;
                }
                equal
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub trait LookPropsBuilder_Optional<T> {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
        }
        impl<T> LookPropsBuilder_Optional<T> for () {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
                default()
            }
        }
        impl<T> LookPropsBuilder_Optional<T> for (T,) {
            fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
                self.0
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __children,
            __controls,
            __docs,
        > LookPropsBuilder<((), __docs, __controls, __children)> {
            #[allow(clippy::type_complexity)]
            pub fn name(
                self,
                name: &'static str,
            ) -> LookPropsBuilder<((&'static str,), __docs, __controls, __children)> {
                let name = (name,);
                let (_, docs, controls, children) = self.fields;
                LookPropsBuilder {
                    fields: (name, docs, controls, children),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookPropsBuilder_Error_Repeated_field_name {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __children,
            __controls,
            __docs,
        > LookPropsBuilder<((&'static str,), __docs, __controls, __children)> {
            #[deprecated(note = "Repeated field name")]
            #[allow(clippy::type_complexity)]
            pub fn name(
                self,
                _: LookPropsBuilder_Error_Repeated_field_name,
            ) -> LookPropsBuilder<((&'static str,), __docs, __controls, __children)> {
                self
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __children,
            __controls,
            __name,
        > LookPropsBuilder<(__name, (), __controls, __children)> {
            #[allow(clippy::type_complexity)]
            pub fn docs(
                self,
                docs: &'static str,
            ) -> LookPropsBuilder<(__name, (&'static str,), __controls, __children)> {
                let docs = (docs,);
                let (name, _, controls, children) = self.fields;
                LookPropsBuilder {
                    fields: (name, docs, controls, children),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookPropsBuilder_Error_Repeated_field_docs {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __children,
            __controls,
            __name,
        > LookPropsBuilder<(__name, (&'static str,), __controls, __children)> {
            #[deprecated(note = "Repeated field docs")]
            #[allow(clippy::type_complexity)]
            pub fn docs(
                self,
                _: LookPropsBuilder_Error_Repeated_field_docs,
            ) -> LookPropsBuilder<(__name, (&'static str,), __controls, __children)> {
                self
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __children,
            __docs,
            __name,
        > LookPropsBuilder<(__name, __docs, (), __children)> {
            #[allow(clippy::type_complexity)]
            pub fn controls(
                self,
                controls: Element,
            ) -> LookPropsBuilder<(__name, __docs, (Element,), __children)> {
                let controls = (controls,);
                let (name, docs, _, children) = self.fields;
                LookPropsBuilder {
                    fields: (name, docs, controls, children),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookPropsBuilder_Error_Repeated_field_controls {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __children,
            __docs,
            __name,
        > LookPropsBuilder<(__name, __docs, (Element,), __children)> {
            #[deprecated(note = "Repeated field controls")]
            #[allow(clippy::type_complexity)]
            pub fn controls(
                self,
                _: LookPropsBuilder_Error_Repeated_field_controls,
            ) -> LookPropsBuilder<(__name, __docs, (Element,), __children)> {
                self
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __controls,
            __docs,
            __name,
        > LookPropsBuilder<(__name, __docs, __controls, ())> {
            #[allow(clippy::type_complexity)]
            pub fn children(
                self,
                children: Element,
            ) -> LookPropsBuilder<(__name, __docs, __controls, (Element,))> {
                let children = (children,);
                let (name, docs, controls, _) = self.fields;
                LookPropsBuilder {
                    fields: (name, docs, controls, children),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookPropsBuilder_Error_Repeated_field_children {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __controls,
            __docs,
            __name,
        > LookPropsBuilder<(__name, __docs, __controls, (Element,))> {
            #[deprecated(note = "Repeated field children")]
            #[allow(clippy::type_complexity)]
            pub fn children(
                self,
                _: LookPropsBuilder_Error_Repeated_field_children,
            ) -> LookPropsBuilder<(__name, __docs, __controls, (Element,))> {
                self
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookPropsBuilder_Error_Missing_required_field_name {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl<
            __children,
            __controls,
            __docs,
        > LookPropsBuilder<((), __docs, __controls, __children)> {
            #[deprecated(note = "Missing required field name")]
            pub fn build(
                self,
                _: LookPropsBuilder_Error_Missing_required_field_name,
            ) -> LookProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookPropsBuilder_Error_Missing_required_field_docs {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl<
            __children,
            __controls,
        > LookPropsBuilder<((&'static str,), (), __controls, __children)> {
            #[deprecated(note = "Missing required field docs")]
            pub fn build(
                self,
                _: LookPropsBuilder_Error_Missing_required_field_docs,
            ) -> LookProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookPropsBuilder_Error_Missing_required_field_controls {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl<
            __children,
        > LookPropsBuilder<((&'static str,), (&'static str,), (), __children)> {
            #[deprecated(note = "Missing required field controls")]
            pub fn build(
                self,
                _: LookPropsBuilder_Error_Missing_required_field_controls,
            ) -> LookProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __children: LookPropsBuilder_Optional<Element>,
        > LookPropsBuilder<((&'static str,), (&'static str,), (Element,), __children)> {
            pub fn build(self) -> LookProps {
                let (name, docs, controls, children) = self.fields;
                let name = name.0;
                let docs = docs.0;
                let controls = controls.0;
                let children = LookPropsBuilder_Optional::into_value(
                    children,
                    || ::core::default::Default::default(),
                );
                LookProps {
                    name,
                    docs,
                    controls,
                    children,
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for LookProps {
            #[inline]
            fn clone(&self) -> LookProps {
                LookProps {
                    name: ::core::clone::Clone::clone(&self.name),
                    docs: ::core::clone::Clone::clone(&self.docs),
                    controls: ::core::clone::Clone::clone(&self.controls),
                    children: ::core::clone::Clone::clone(&self.children),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::StructuralPartialEq for LookProps {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for LookProps {
            #[inline]
            fn eq(&self, other: &LookProps) -> bool {
                self.name == other.name && self.docs == other.docs
                    && self.controls == other.controls && self.children == other.children
            }
        }
        #[allow(non_snake_case)]
        /**# Props
*For details, see the [props struct definition](LookProps).**/
        ///- [`docs`](LookProps::docs) : `&'static str`
        ///- [`controls`](LookProps::controls) : `Element`
        ///- [`children`](LookProps::children) : `Element`
        pub fn Look(mut __props: LookProps) -> Element {
            let LookProps { mut name, mut docs, mut controls, mut children } = __props;
            {
                let top = Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/look.rs:12:15:5636",
                        roots: &[
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::padding.0,
                                        namespace: dioxus_elements::div::padding.1,
                                        value: "20px",
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::Element {
                                        tag: dioxus_elements::h2::TAG_NAME,
                                        namespace: dioxus_elements::h2::NAME_SPACE,
                                        attrs: &[],
                                        children: &[
                                            dioxus_core::TemplateNode::DynamicText {
                                                id: 0usize,
                                            },
                                        ],
                                    },
                                ],
                            },
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 0usize,
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::display.0,
                                        namespace: dioxus_elements::div::display.1,
                                        value: "flex",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::flex_direction.0,
                                        namespace: dioxus_elements::div::flex_direction.1,
                                        value: "column",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::justify_content.0,
                                        namespace: dioxus_elements::div::justify_content.1,
                                        value: "center",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::align_items.0,
                                        namespace: dioxus_elements::div::align_items.1,
                                        value: "center",
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::Dynamic {
                                        id: 1usize,
                                    },
                                ],
                            },
                        ],
                        node_paths: &[&[0u8, 0u8, 0u8], &[1u8, 0u8]],
                        attr_paths: &[&[1u8]],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Text(
                                    dioxus_core::VText::new((name).to_string().to_string()),
                                ),
                                {
                                    let ___nodes = ({ children }).into_dyn_node();
                                    ___nodes
                                },
                            ]),
                            Box::new([
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::flex.0,
                                        1,
                                        dioxus_elements::div::flex.1,
                                        dioxus_elements::div::flex.2,
                                    ),
                                ]),
                            ]),
                        );
                        __vnodes
                    }
                });
                let bottom = Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/look.rs:28:18:6021",
                        roots: &[
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 0usize,
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::display.0,
                                        namespace: dioxus_elements::div::display.1,
                                        value: "flex",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::flex_direction.0,
                                        namespace: dioxus_elements::div::flex_direction.1,
                                        value: "column",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::overflow_y.0,
                                        namespace: dioxus_elements::div::overflow_y.1,
                                        value: "auto",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::gap.0,
                                        namespace: dioxus_elements::div::gap.1,
                                        value: "20px",
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::Element {
                                        tag: dioxus_elements::table::TAG_NAME,
                                        namespace: dioxus_elements::table::NAME_SPACE,
                                        attrs: &[
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::table::text_align.0,
                                                namespace: dioxus_elements::table::text_align.1,
                                                value: "left",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::table::border_collapse.0,
                                                namespace: dioxus_elements::table::border_collapse.1,
                                                value: "collapse",
                                            },
                                        ],
                                        children: &[
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::tr::TAG_NAME,
                                                namespace: dioxus_elements::tr::NAME_SPACE,
                                                attrs: &[
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::tr::height.0,
                                                        namespace: dioxus_elements::tr::height.1,
                                                        value: "60px",
                                                    },
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::tr::color.0,
                                                        namespace: dioxus_elements::tr::color.1,
                                                        value: "#777",
                                                    },
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::tr::border_bottom.0,
                                                        namespace: dioxus_elements::tr::border_bottom.1,
                                                        value: "2px solid #e7e7e7",
                                                    },
                                                ],
                                                children: &[
                                                    dioxus_core::TemplateNode::Element {
                                                        tag: dioxus_elements::th::TAG_NAME,
                                                        namespace: dioxus_elements::th::NAME_SPACE,
                                                        attrs: &[
                                                            dioxus_core::TemplateAttribute::Static {
                                                                name: dioxus_elements::th::padding_left.0,
                                                                namespace: dioxus_elements::th::padding_left.1,
                                                                value: "20px",
                                                            },
                                                        ],
                                                        children: &[
                                                            dioxus_core::TemplateNode::Text {
                                                                text: "Name",
                                                            },
                                                        ],
                                                    },
                                                    dioxus_core::TemplateNode::Element {
                                                        tag: dioxus_elements::th::TAG_NAME,
                                                        namespace: dioxus_elements::th::NAME_SPACE,
                                                        attrs: &[],
                                                        children: &[
                                                            dioxus_core::TemplateNode::Text {
                                                                text: "Type",
                                                            },
                                                        ],
                                                    },
                                                    dioxus_core::TemplateNode::Element {
                                                        tag: dioxus_elements::th::TAG_NAME,
                                                        namespace: dioxus_elements::th::NAME_SPACE,
                                                        attrs: &[],
                                                        children: &[
                                                            dioxus_core::TemplateNode::Text {
                                                                text: "Description",
                                                            },
                                                        ],
                                                    },
                                                    dioxus_core::TemplateNode::Element {
                                                        tag: dioxus_elements::th::TAG_NAME,
                                                        namespace: dioxus_elements::th::NAME_SPACE,
                                                        attrs: &[],
                                                        children: &[
                                                            dioxus_core::TemplateNode::Text {
                                                                text: "Default",
                                                            },
                                                        ],
                                                    },
                                                    dioxus_core::TemplateNode::Element {
                                                        tag: dioxus_elements::th::TAG_NAME,
                                                        namespace: dioxus_elements::th::NAME_SPACE,
                                                        attrs: &[
                                                            dioxus_core::TemplateAttribute::Static {
                                                                name: dioxus_elements::th::padding_right.0,
                                                                namespace: dioxus_elements::th::padding_right.1,
                                                                value: "20px",
                                                            },
                                                        ],
                                                        children: &[
                                                            dioxus_core::TemplateNode::Text {
                                                                text: "Controls",
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                            dioxus_core::TemplateNode::Dynamic {
                                                id: 0usize,
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                        node_paths: &[&[0u8, 0u8, 1u8]],
                        attr_paths: &[&[0u8]],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                {
                                    let ___nodes = ({ controls }).into_dyn_node();
                                    ___nodes
                                },
                            ]),
                            Box::new([
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::flex.0,
                                        1,
                                        dioxus_elements::div::flex.1,
                                        dioxus_elements::div::flex.2,
                                    ),
                                ]),
                            ]),
                        );
                        __vnodes
                    }
                });
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/look.rs:43:5:6594",
                        roots: &[
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 0usize,
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::display.0,
                                        namespace: dioxus_elements::div::display.1,
                                        value: "flex",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::flex_direction.0,
                                        namespace: dioxus_elements::div::flex_direction.1,
                                        value: "column",
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::Dynamic {
                                        id: 0usize,
                                    },
                                ],
                            },
                        ],
                        node_paths: &[&[0u8, 0u8]],
                        attr_paths: &[&[0u8]],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Component({
                                    use dioxus_core::prelude::Properties;
                                    (fc_to_builder(VerticalPane)
                                        .top(top)
                                        .bottom(bottom)
                                        .build())
                                        .into_vcomponent(VerticalPane, "VerticalPane")
                                }),
                            ]),
                            Box::new([
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::flex.0,
                                        1,
                                        dioxus_elements::div::flex.1,
                                        dioxus_elements::div::flex.2,
                                    ),
                                ]),
                            ]),
                        );
                        __vnodes
                    }
                })
            }
        }
    }
    pub use look::Look;
    mod look_book {
        use crate::{prefixed_route::use_prefix, register, PrefixedRoute, Preview, HOME};
        use dioxus::prelude::*;
        ///Properties for the [`LookBook`] component.
        #[allow(non_camel_case_types)]
        pub struct LookBookProps<
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > {
            pub previews: I,
            pub home: Component,
            pub prefix: Option<&'static str>,
        }
        impl<
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookProps<I> {
            /**
Create a builder for building `LookBookProps`.
On the builder, call `.previews(...)`, `.home(...)`, `.prefix(...)`(optional) to set the values of the fields.
Finally, call `.build()` to create the instance of `LookBookProps`.
                    */
            #[allow(dead_code, clippy::type_complexity)]
            pub fn builder() -> LookBookPropsBuilder<((), (), ()), I> {
                LookBookPropsBuilder {
                    fields: ((), (), ()),
                    _phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub struct LookBookPropsBuilder<
            TypedBuilderFields,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > {
            fields: TypedBuilderFields,
            _phantom: (::core::marker::PhantomData<I>),
        }
        impl<
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > dioxus_core::prelude::Properties for LookBookProps<I>
        where
            Self: Clone,
        {
            type Builder = LookBookPropsBuilder<((), (), ()), I>;
            fn builder() -> Self::Builder {
                LookBookProps::builder()
            }
            fn memoize(&mut self, new: &Self) -> bool {
                let equal = self == new;
                if !equal {
                    let new_clone = new.clone();
                    self.previews = new_clone.previews;
                    self.home = new_clone.home;
                    self.prefix = new_clone.prefix;
                }
                equal
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub trait LookBookPropsBuilder_Optional<T> {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
        }
        impl<T> LookBookPropsBuilder_Optional<T> for () {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
                default()
            }
        }
        impl<T> LookBookPropsBuilder_Optional<T> for (T,) {
            fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
                self.0
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __prefix,
            __home,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<((), __home, __prefix), I> {
            #[allow(clippy::type_complexity)]
            pub fn previews(
                self,
                previews: I,
            ) -> LookBookPropsBuilder<((I,), __home, __prefix), I> {
                let previews = (previews,);
                let (_, home, prefix) = self.fields;
                LookBookPropsBuilder {
                    fields: (previews, home, prefix),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookBookPropsBuilder_Error_Repeated_field_previews {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __prefix,
            __home,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<((I,), __home, __prefix), I> {
            #[deprecated(note = "Repeated field previews")]
            #[allow(clippy::type_complexity)]
            pub fn previews(
                self,
                _: LookBookPropsBuilder_Error_Repeated_field_previews,
            ) -> LookBookPropsBuilder<((I,), __home, __prefix), I> {
                self
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __prefix,
            __previews,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<(__previews, (), __prefix), I> {
            #[allow(clippy::type_complexity)]
            pub fn home(
                self,
                home: Component,
            ) -> LookBookPropsBuilder<(__previews, (Component,), __prefix), I> {
                let home = (home,);
                let (previews, _, prefix) = self.fields;
                LookBookPropsBuilder {
                    fields: (previews, home, prefix),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookBookPropsBuilder_Error_Repeated_field_home {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __prefix,
            __previews,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<(__previews, (Component,), __prefix), I> {
            #[deprecated(note = "Repeated field home")]
            #[allow(clippy::type_complexity)]
            pub fn home(
                self,
                _: LookBookPropsBuilder_Error_Repeated_field_home,
            ) -> LookBookPropsBuilder<(__previews, (Component,), __prefix), I> {
                self
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __home,
            __previews,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<(__previews, __home, ()), I> {
            #[allow(clippy::type_complexity)]
            pub fn prefix<__Marker>(
                self,
                prefix: impl dioxus_core::prelude::SuperInto<
                    Option<&'static str>,
                    __Marker,
                >,
            ) -> LookBookPropsBuilder<(__previews, __home, (Option<&'static str>,)), I> {
                let prefix = (dioxus_core::prelude::SuperInto::super_into(prefix),);
                let (previews, home, _) = self.fields;
                LookBookPropsBuilder {
                    fields: (previews, home, prefix),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookBookPropsBuilder_Error_Repeated_field_prefix {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __home,
            __previews,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<(__previews, __home, (Option<&'static str>,)), I> {
            #[deprecated(note = "Repeated field prefix")]
            #[allow(clippy::type_complexity)]
            pub fn prefix<__Marker>(
                self,
                _: LookBookPropsBuilder_Error_Repeated_field_prefix,
            ) -> LookBookPropsBuilder<(__previews, __home, (Option<&'static str>,)), I> {
                self
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookBookPropsBuilder_Error_Missing_required_field_previews {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl<
            __prefix,
            __home,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<((), __home, __prefix), I> {
            #[deprecated(note = "Missing required field previews")]
            pub fn build(
                self,
                _: LookBookPropsBuilder_Error_Missing_required_field_previews,
            ) -> LookBookProps<I> {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum LookBookPropsBuilder_Error_Missing_required_field_home {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl<
            __prefix,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<((I,), (), __prefix), I> {
            #[deprecated(note = "Missing required field home")]
            pub fn build(
                self,
                _: LookBookPropsBuilder_Error_Missing_required_field_home,
            ) -> LookBookProps<I> {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<
            __prefix: LookBookPropsBuilder_Optional<Option<&'static str>>,
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > LookBookPropsBuilder<((I,), (Component,), __prefix), I> {
            pub fn build(self) -> LookBookProps<I> {
                let (previews, home, prefix) = self.fields;
                let previews = previews.0;
                let home = home.0;
                let prefix = LookBookPropsBuilder_Optional::into_value(
                    prefix,
                    || ::core::default::Default::default(),
                );
                LookBookProps {
                    previews,
                    home,
                    prefix,
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<
            I: ::core::clone::Clone + IntoIterator<Item = Preview> + Clone + 'static
                + std::cmp::PartialEq,
        > ::core::clone::Clone for LookBookProps<I> {
            #[inline]
            fn clone(&self) -> LookBookProps<I> {
                LookBookProps {
                    previews: ::core::clone::Clone::clone(&self.previews),
                    home: ::core::clone::Clone::clone(&self.home),
                    prefix: ::core::clone::Clone::clone(&self.prefix),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        > ::core::marker::StructuralPartialEq for LookBookProps<I> {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<
            I: ::core::cmp::PartialEq + IntoIterator<Item = Preview> + Clone + 'static
                + std::cmp::PartialEq,
        > ::core::cmp::PartialEq for LookBookProps<I> {
            #[inline]
            fn eq(&self, other: &LookBookProps<I>) -> bool {
                self.previews == other.previews && self.home == other.home
                    && self.prefix == other.prefix
            }
        }
        #[allow(non_snake_case)]
        /**# Props
*For details, see the [props struct definition](LookBookProps).**/
        ///- [`home`](LookBookProps::home) : `Component`
        ///- [`prefix`](LookBookProps::prefix) : `Option<&'static str>`
        pub fn LookBook<
            I: IntoIterator<Item = Preview> + Clone + 'static + std::cmp::PartialEq,
        >(mut __props: LookBookProps<I>) -> Element {
            let LookBookProps { mut previews, mut home, mut prefix } = __props;
            {
                use_future(move || {
                    for preview in previews.clone() {
                        register(preview.name, preview.component)
                    }
                    HOME.try_with(|cell| *cell.borrow_mut() = Some(home)).unwrap();
                    async move {}
                });
                use_prefix(prefix);
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/look_book.rs:25:5:7367",
                        roots: &[
                            dioxus_core::TemplateNode::Dynamic {
                                id: 0usize,
                            },
                        ],
                        node_paths: &[&[0u8]],
                        attr_paths: &[],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Component({
                                    use dioxus_core::prelude::Properties;
                                    (fc_to_builder(Router::<PrefixedRoute>).build())
                                        .into_vcomponent(Router::<PrefixedRoute>, "Router")
                                }),
                            ]),
                            Box::new([]),
                        );
                        __vnodes
                    }
                })
            }
        }
    }
    pub use look_book::LookBook;
    mod pane {
        use dioxus::prelude::*;
        use dioxus_resize_observer::use_size;
        use dioxus_use_mounted::use_mounted;
        ///Properties for the [`HorizontalPane`] component.
        #[allow(non_camel_case_types)]
        pub struct HorizontalPaneProps {
            pub left: Element,
            pub right: Element,
        }
        impl HorizontalPaneProps {
            /**
Create a builder for building `HorizontalPaneProps`.
On the builder, call `.left(...)`, `.right(...)` to set the values of the fields.
Finally, call `.build()` to create the instance of `HorizontalPaneProps`.
                    */
            #[allow(dead_code, clippy::type_complexity)]
            pub fn builder() -> HorizontalPanePropsBuilder<((), ())> {
                HorizontalPanePropsBuilder {
                    fields: ((), ()),
                    _phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub struct HorizontalPanePropsBuilder<TypedBuilderFields> {
            fields: TypedBuilderFields,
            _phantom: (),
        }
        impl dioxus_core::prelude::Properties for HorizontalPaneProps
        where
            Self: Clone,
        {
            type Builder = HorizontalPanePropsBuilder<((), ())>;
            fn builder() -> Self::Builder {
                HorizontalPaneProps::builder()
            }
            fn memoize(&mut self, new: &Self) -> bool {
                let equal = self == new;
                if !equal {
                    let new_clone = new.clone();
                    self.left = new_clone.left;
                    self.right = new_clone.right;
                }
                equal
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub trait HorizontalPanePropsBuilder_Optional<T> {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
        }
        impl<T> HorizontalPanePropsBuilder_Optional<T> for () {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
                default()
            }
        }
        impl<T> HorizontalPanePropsBuilder_Optional<T> for (T,) {
            fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
                self.0
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__right> HorizontalPanePropsBuilder<((), __right)> {
            #[allow(clippy::type_complexity)]
            pub fn left(
                self,
                left: Element,
            ) -> HorizontalPanePropsBuilder<((Element,), __right)> {
                let left = (left,);
                let (_, right) = self.fields;
                HorizontalPanePropsBuilder {
                    fields: (left, right),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum HorizontalPanePropsBuilder_Error_Repeated_field_left {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__right> HorizontalPanePropsBuilder<((Element,), __right)> {
            #[deprecated(note = "Repeated field left")]
            #[allow(clippy::type_complexity)]
            pub fn left(
                self,
                _: HorizontalPanePropsBuilder_Error_Repeated_field_left,
            ) -> HorizontalPanePropsBuilder<((Element,), __right)> {
                self
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__left> HorizontalPanePropsBuilder<(__left, ())> {
            #[allow(clippy::type_complexity)]
            pub fn right(
                self,
                right: Element,
            ) -> HorizontalPanePropsBuilder<(__left, (Element,))> {
                let right = (right,);
                let (left, _) = self.fields;
                HorizontalPanePropsBuilder {
                    fields: (left, right),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum HorizontalPanePropsBuilder_Error_Repeated_field_right {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__left> HorizontalPanePropsBuilder<(__left, (Element,))> {
            #[deprecated(note = "Repeated field right")]
            #[allow(clippy::type_complexity)]
            pub fn right(
                self,
                _: HorizontalPanePropsBuilder_Error_Repeated_field_right,
            ) -> HorizontalPanePropsBuilder<(__left, (Element,))> {
                self
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum HorizontalPanePropsBuilder_Error_Missing_required_field_left {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl<__right> HorizontalPanePropsBuilder<((), __right)> {
            #[deprecated(note = "Missing required field left")]
            pub fn build(
                self,
                _: HorizontalPanePropsBuilder_Error_Missing_required_field_left,
            ) -> HorizontalPaneProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum HorizontalPanePropsBuilder_Error_Missing_required_field_right {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl HorizontalPanePropsBuilder<((Element,), ())> {
            #[deprecated(note = "Missing required field right")]
            pub fn build(
                self,
                _: HorizontalPanePropsBuilder_Error_Missing_required_field_right,
            ) -> HorizontalPaneProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl HorizontalPanePropsBuilder<((Element,), (Element,))> {
            pub fn build(self) -> HorizontalPaneProps {
                let (left, right) = self.fields;
                let left = left.0;
                let right = right.0;
                HorizontalPaneProps { left, right }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for HorizontalPaneProps {
            #[inline]
            fn clone(&self) -> HorizontalPaneProps {
                HorizontalPaneProps {
                    left: ::core::clone::Clone::clone(&self.left),
                    right: ::core::clone::Clone::clone(&self.right),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::StructuralPartialEq for HorizontalPaneProps {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for HorizontalPaneProps {
            #[inline]
            fn eq(&self, other: &HorizontalPaneProps) -> bool {
                self.left == other.left && self.right == other.right
            }
        }
        #[allow(non_snake_case)]
        pub fn HorizontalPane(mut __props: HorizontalPaneProps) -> Element {
            let HorizontalPaneProps { mut left, mut right } = __props;
            {
                let mut width = use_signal(|| 250.);
                let mut is_dragging = use_signal(|| false);
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/pane.rs:10:5:7736",
                        roots: &[
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::position.0,
                                        namespace: dioxus_elements::div::position.1,
                                        value: "relative",
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 0usize,
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::display.0,
                                        namespace: dioxus_elements::div::display.1,
                                        value: "flex",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::flex_direction.0,
                                        namespace: dioxus_elements::div::flex_direction.1,
                                        value: "row",
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 1usize,
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 2usize,
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 3usize,
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::Element {
                                        tag: dioxus_elements::div::TAG_NAME,
                                        namespace: dioxus_elements::div::NAME_SPACE,
                                        attrs: &[
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::display.0,
                                                namespace: dioxus_elements::div::display.1,
                                                value: "flex",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::flex_direction.0,
                                                namespace: dioxus_elements::div::flex_direction.1,
                                                value: "row",
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 4usize,
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::overflow.0,
                                                namespace: dioxus_elements::div::overflow.1,
                                                value: "auto",
                                            },
                                        ],
                                        children: &[
                                            dioxus_core::TemplateNode::Dynamic {
                                                id: 0usize,
                                            },
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::div::TAG_NAME,
                                                namespace: dioxus_elements::div::NAME_SPACE,
                                                attrs: &[
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::div::height.0,
                                                        namespace: dioxus_elements::div::height.1,
                                                        value: "100%",
                                                    },
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::div::padding.0,
                                                        namespace: dioxus_elements::div::padding.1,
                                                        value: "0 5px",
                                                    },
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::div::margin.0,
                                                        namespace: dioxus_elements::div::margin.1,
                                                        value: "0 -5px",
                                                    },
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::div::cursor.0,
                                                        namespace: dioxus_elements::div::cursor.1,
                                                        value: "ew-resize",
                                                    },
                                                    dioxus_core::TemplateAttribute::Dynamic {
                                                        id: 5usize,
                                                    },
                                                ],
                                                children: &[
                                                    dioxus_core::TemplateNode::Element {
                                                        tag: dioxus_elements::div::TAG_NAME,
                                                        namespace: dioxus_elements::div::NAME_SPACE,
                                                        attrs: &[
                                                            dioxus_core::TemplateAttribute::Static {
                                                                name: dioxus_elements::div::width.0,
                                                                namespace: dioxus_elements::div::width.1,
                                                                value: "2px",
                                                            },
                                                            dioxus_core::TemplateAttribute::Static {
                                                                name: dioxus_elements::div::height.0,
                                                                namespace: dioxus_elements::div::height.1,
                                                                value: "100%",
                                                            },
                                                            dioxus_core::TemplateAttribute::Static {
                                                                name: dioxus_elements::div::background.0,
                                                                namespace: dioxus_elements::div::background.1,
                                                                value: "#ccc",
                                                            },
                                                        ],
                                                        children: &[],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                    dioxus_core::TemplateNode::Dynamic {
                                        id: 1usize,
                                    },
                                ],
                            },
                        ],
                        node_paths: &[&[0u8, 0u8, 0u8], &[0u8, 1u8]],
                        attr_paths: &[
                            &[0u8],
                            &[0u8],
                            &[0u8],
                            &[0u8],
                            &[0u8, 0u8],
                            &[0u8, 0u8, 1u8],
                        ],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                {
                                    let ___nodes = ({ left }).into_dyn_node();
                                    ___nodes
                                },
                                {
                                    let ___nodes = ({ right }).into_dyn_node();
                                    ___nodes
                                },
                            ]),
                            Box::new([
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::flex.0,
                                        1,
                                        dioxus_elements::div::flex.1,
                                        dioxus_elements::div::flex.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_elements::events::onmouseup(move |_| {
                                        is_dragging.set(false)
                                    }),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::prevent_default.0,
                                        if *is_dragging.peek() {
                                            "onmousedown onmousemove"
                                        } else {
                                            ""
                                        },
                                        dioxus_elements::div::prevent_default.1,
                                        dioxus_elements::div::prevent_default.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_elements::events::onmousemove(move |event| {
                                        if *is_dragging.peek() {
                                            width.set(event.data.client_coordinates().x)
                                        }
                                    }),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::width.0,
                                        format_args!("{0}px", width).to_string(),
                                        dioxus_elements::div::width.1,
                                        dioxus_elements::div::width.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_elements::events::onmousedown(move |_| {
                                        is_dragging.set(true)
                                    }),
                                ]),
                            ]),
                        );
                        __vnodes
                    }
                })
            }
        }
        ///Properties for the [`VerticalPane`] component.
        #[allow(non_camel_case_types)]
        pub struct VerticalPaneProps {
            pub top: Element,
            pub bottom: Element,
        }
        impl VerticalPaneProps {
            /**
Create a builder for building `VerticalPaneProps`.
On the builder, call `.top(...)`, `.bottom(...)` to set the values of the fields.
Finally, call `.build()` to create the instance of `VerticalPaneProps`.
                    */
            #[allow(dead_code, clippy::type_complexity)]
            pub fn builder() -> VerticalPanePropsBuilder<((), ())> {
                VerticalPanePropsBuilder {
                    fields: ((), ()),
                    _phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub struct VerticalPanePropsBuilder<TypedBuilderFields> {
            fields: TypedBuilderFields,
            _phantom: (),
        }
        impl dioxus_core::prelude::Properties for VerticalPaneProps
        where
            Self: Clone,
        {
            type Builder = VerticalPanePropsBuilder<((), ())>;
            fn builder() -> Self::Builder {
                VerticalPaneProps::builder()
            }
            fn memoize(&mut self, new: &Self) -> bool {
                let equal = self == new;
                if !equal {
                    let new_clone = new.clone();
                    self.top = new_clone.top;
                    self.bottom = new_clone.bottom;
                }
                equal
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub trait VerticalPanePropsBuilder_Optional<T> {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
        }
        impl<T> VerticalPanePropsBuilder_Optional<T> for () {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
                default()
            }
        }
        impl<T> VerticalPanePropsBuilder_Optional<T> for (T,) {
            fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
                self.0
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__bottom> VerticalPanePropsBuilder<((), __bottom)> {
            #[allow(clippy::type_complexity)]
            pub fn top(
                self,
                top: Element,
            ) -> VerticalPanePropsBuilder<((Element,), __bottom)> {
                let top = (top,);
                let (_, bottom) = self.fields;
                VerticalPanePropsBuilder {
                    fields: (top, bottom),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum VerticalPanePropsBuilder_Error_Repeated_field_top {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__bottom> VerticalPanePropsBuilder<((Element,), __bottom)> {
            #[deprecated(note = "Repeated field top")]
            #[allow(clippy::type_complexity)]
            pub fn top(
                self,
                _: VerticalPanePropsBuilder_Error_Repeated_field_top,
            ) -> VerticalPanePropsBuilder<((Element,), __bottom)> {
                self
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__top> VerticalPanePropsBuilder<(__top, ())> {
            #[allow(clippy::type_complexity)]
            pub fn bottom(
                self,
                bottom: Element,
            ) -> VerticalPanePropsBuilder<(__top, (Element,))> {
                let bottom = (bottom,);
                let (top, _) = self.fields;
                VerticalPanePropsBuilder {
                    fields: (top, bottom),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum VerticalPanePropsBuilder_Error_Repeated_field_bottom {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__top> VerticalPanePropsBuilder<(__top, (Element,))> {
            #[deprecated(note = "Repeated field bottom")]
            #[allow(clippy::type_complexity)]
            pub fn bottom(
                self,
                _: VerticalPanePropsBuilder_Error_Repeated_field_bottom,
            ) -> VerticalPanePropsBuilder<(__top, (Element,))> {
                self
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum VerticalPanePropsBuilder_Error_Missing_required_field_top {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl<__bottom> VerticalPanePropsBuilder<((), __bottom)> {
            #[deprecated(note = "Missing required field top")]
            pub fn build(
                self,
                _: VerticalPanePropsBuilder_Error_Missing_required_field_top,
            ) -> VerticalPaneProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum VerticalPanePropsBuilder_Error_Missing_required_field_bottom {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl VerticalPanePropsBuilder<((Element,), ())> {
            #[deprecated(note = "Missing required field bottom")]
            pub fn build(
                self,
                _: VerticalPanePropsBuilder_Error_Missing_required_field_bottom,
            ) -> VerticalPaneProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl VerticalPanePropsBuilder<((Element,), (Element,))> {
            pub fn build(self) -> VerticalPaneProps {
                let (top, bottom) = self.fields;
                let top = top.0;
                let bottom = bottom.0;
                VerticalPaneProps { top, bottom }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for VerticalPaneProps {
            #[inline]
            fn clone(&self) -> VerticalPaneProps {
                VerticalPaneProps {
                    top: ::core::clone::Clone::clone(&self.top),
                    bottom: ::core::clone::Clone::clone(&self.bottom),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::StructuralPartialEq for VerticalPaneProps {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for VerticalPaneProps {
            #[inline]
            fn eq(&self, other: &VerticalPaneProps) -> bool {
                self.top == other.top && self.bottom == other.bottom
            }
        }
        #[allow(non_snake_case)]
        pub fn VerticalPane(mut __props: VerticalPaneProps) -> Element {
            let VerticalPaneProps { mut top, mut bottom } = __props;
            {
                let container_ref = use_mounted();
                let container_size = use_size(container_ref);
                let mut height = use_signal(|| 0.);
                let mut count = use_signal(|| 0);
                use_future(move || {
                    if *count.peek() <= 2 {
                        height.set(height / 2.);
                        count += 1;
                    }
                    async {}
                });
                let mut is_dragging = use_signal(|| false);
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/pane.rs:59:5:9202",
                        roots: &[
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::position.0,
                                        namespace: dioxus_elements::div::position.1,
                                        value: "relative",
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 0usize,
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::display.0,
                                        namespace: dioxus_elements::div::display.1,
                                        value: "flex",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::flex_direction.0,
                                        namespace: dioxus_elements::div::flex_direction.1,
                                        value: "column",
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 1usize,
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::Element {
                                        tag: dioxus_elements::div::TAG_NAME,
                                        namespace: dioxus_elements::div::NAME_SPACE,
                                        attrs: &[
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::position.0,
                                                namespace: dioxus_elements::div::position.1,
                                                value: "absolute",
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 2usize,
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::width.0,
                                                namespace: dioxus_elements::div::width.1,
                                                value: "100%",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::height.0,
                                                namespace: dioxus_elements::div::height.1,
                                                value: "100%",
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 3usize,
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 4usize,
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 5usize,
                                            },
                                        ],
                                        children: &[],
                                    },
                                    dioxus_core::TemplateNode::Dynamic {
                                        id: 0usize,
                                    },
                                    dioxus_core::TemplateNode::Element {
                                        tag: dioxus_elements::div::TAG_NAME,
                                        namespace: dioxus_elements::div::NAME_SPACE,
                                        attrs: &[
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::width.0,
                                                namespace: dioxus_elements::div::width.1,
                                                value: "100%",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::padding.0,
                                                namespace: dioxus_elements::div::padding.1,
                                                value: "5px 0",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::margin.0,
                                                namespace: dioxus_elements::div::margin.1,
                                                value: "-5px 0",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::cursor.0,
                                                namespace: dioxus_elements::div::cursor.1,
                                                value: "ns-resize",
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 6usize,
                                            },
                                        ],
                                        children: &[
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::div::TAG_NAME,
                                                namespace: dioxus_elements::div::NAME_SPACE,
                                                attrs: &[
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::div::height.0,
                                                        namespace: dioxus_elements::div::height.1,
                                                        value: "2px",
                                                    },
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::div::width.0,
                                                        namespace: dioxus_elements::div::width.1,
                                                        value: "100%",
                                                    },
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::div::background.0,
                                                        namespace: dioxus_elements::div::background.1,
                                                        value: "#ccc",
                                                    },
                                                ],
                                                children: &[],
                                            },
                                        ],
                                    },
                                    dioxus_core::TemplateNode::Element {
                                        tag: dioxus_elements::div::TAG_NAME,
                                        namespace: dioxus_elements::div::NAME_SPACE,
                                        attrs: &[
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::display.0,
                                                namespace: dioxus_elements::div::display.1,
                                                value: "flex",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::flex_direction.0,
                                                namespace: dioxus_elements::div::flex_direction.1,
                                                value: "column",
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 7usize,
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::overflow.0,
                                                namespace: dioxus_elements::div::overflow.1,
                                                value: "auto",
                                            },
                                        ],
                                        children: &[
                                            dioxus_core::TemplateNode::Dynamic {
                                                id: 1usize,
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                        node_paths: &[&[0u8, 1u8], &[0u8, 3u8, 0u8]],
                        attr_paths: &[
                            &[0u8],
                            &[0u8],
                            &[0u8, 0u8],
                            &[0u8, 0u8],
                            &[0u8, 0u8],
                            &[0u8, 0u8],
                            &[0u8, 2u8],
                            &[0u8, 3u8],
                        ],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                {
                                    let ___nodes = ({ top }).into_dyn_node();
                                    ___nodes
                                },
                                {
                                    let ___nodes = ({ bottom }).into_dyn_node();
                                    ___nodes
                                },
                            ]),
                            Box::new([
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::flex.0,
                                        1,
                                        dioxus_elements::div::flex.1,
                                        dioxus_elements::div::flex.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_elements::events::onmounted(move |event| {
                                        container_ref.onmounted(event)
                                    }),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::display.0,
                                        if *is_dragging.peek() { "block" } else { "none" },
                                        dioxus_elements::div::display.1,
                                        dioxus_elements::div::display.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_elements::events::onmouseup(move |_| {
                                        is_dragging.set(false)
                                    }),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::prevent_default.0,
                                        if *is_dragging.peek() {
                                            "onmousedown onmousemove"
                                        } else {
                                            ""
                                        },
                                        dioxus_elements::div::prevent_default.1,
                                        dioxus_elements::div::prevent_default.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_elements::events::onmousemove(move |event| {
                                        height
                                            .set(
                                                container_size.height() - event.data.client_coordinates().y,
                                            )
                                    }),
                                ]),
                                Box::new([
                                    dioxus_elements::events::onmousedown(move |_| {
                                        is_dragging.set(true)
                                    }),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::height.0,
                                        format_args!("{0}px", height).to_string(),
                                        dioxus_elements::div::height.1,
                                        dioxus_elements::div::height.2,
                                    ),
                                ]),
                            ]),
                        );
                        __vnodes
                    }
                })
            }
        }
    }
    mod wrap {
        use crate::{
            prefixed_route::PrefixedRoute, ui::pane::HorizontalPane, Route, CONTEXT,
        };
        use dioxus::prelude::*;
        #[allow(non_snake_case)]
        /// The main application wrap component.
        pub fn Wrap() -> Element {
            {
                let mut query = use_signal(|| String::new());
                let elements = use_memo(move || {
                    CONTEXT
                        .try_with(|cx| {
                            cx.borrow()
                                .iter()
                                .filter(|(name, _)| {
                                    name.to_lowercase().contains(&query.read().to_lowercase())
                                })
                                .copied()
                                .collect::<Vec<_>>()
                        })
                        .unwrap()
                });
                let navigator = use_navigator();
                let left = Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/wrap.rs:24:16:11104",
                        roots: &[
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 0usize,
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::display.0,
                                        namespace: dioxus_elements::div::display.1,
                                        value: "flex",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::flex_direction.0,
                                        namespace: dioxus_elements::div::flex_direction.1,
                                        value: "column",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::gap.0,
                                        namespace: dioxus_elements::div::gap.1,
                                        value: "10px",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::width.0,
                                        namespace: dioxus_elements::div::width.1,
                                        value: "200px",
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 1usize,
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::padding.0,
                                        namespace: dioxus_elements::div::padding.1,
                                        value: "10px 20px",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::font_size.0,
                                        namespace: dioxus_elements::div::font_size.1,
                                        value: "14px",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::background.0,
                                        namespace: dioxus_elements::div::background.1,
                                        value: "#eeeeee",
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::Element {
                                        tag: dioxus_elements::div::TAG_NAME,
                                        namespace: dioxus_elements::div::NAME_SPACE,
                                        attrs: &[
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::display.0,
                                                namespace: dioxus_elements::div::display.1,
                                                value: "flex",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::flex_direction.0,
                                                namespace: dioxus_elements::div::flex_direction.1,
                                                value: "row",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::align_items.0,
                                                namespace: dioxus_elements::div::align_items.1,
                                                value: "center",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::justify_content.0,
                                                namespace: dioxus_elements::div::justify_content.1,
                                                value: "space-between",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::div::margin.0,
                                                namespace: dioxus_elements::div::margin.1,
                                                value: "20px 0",
                                            },
                                        ],
                                        children: &[
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::h1::TAG_NAME,
                                                namespace: dioxus_elements::h1::NAME_SPACE,
                                                attrs: &[
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::h1::cursor.0,
                                                        namespace: dioxus_elements::h1::cursor.1,
                                                        value: "pointer",
                                                    },
                                                    dioxus_core::TemplateAttribute::Static {
                                                        name: dioxus_elements::h1::margin.0,
                                                        namespace: dioxus_elements::h1::margin.1,
                                                        value: "0",
                                                    },
                                                    dioxus_core::TemplateAttribute::Dynamic {
                                                        id: 2usize,
                                                    },
                                                ],
                                                children: &[
                                                    dioxus_core::TemplateNode::Text {
                                                        text: "Lookbook",
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                    dioxus_core::TemplateNode::Element {
                                        tag: dioxus_elements::input::TAG_NAME,
                                        namespace: dioxus_elements::input::NAME_SPACE,
                                        attrs: &[
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::input::placeholder.0,
                                                namespace: dioxus_elements::input::placeholder.1,
                                                value: "Search",
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 3usize,
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::input::width.0,
                                                namespace: dioxus_elements::input::width.1,
                                                value: "100%",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::input::margin.0,
                                                namespace: dioxus_elements::input::margin.1,
                                                value: "5px",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::input::margin_bottom.0,
                                                namespace: dioxus_elements::input::margin_bottom.1,
                                                value: "20px",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::input::padding.0,
                                                namespace: dioxus_elements::input::padding.1,
                                                value: "10px",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::input::border.0,
                                                namespace: dioxus_elements::input::border.1,
                                                value: "2px solid #999",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::input::outline.0,
                                                namespace: dioxus_elements::input::outline.1,
                                                value: "none",
                                            },
                                            dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::input::background.0,
                                                namespace: dioxus_elements::input::background.1,
                                                value: "none",
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 4usize,
                                            },
                                            dioxus_core::TemplateAttribute::Dynamic {
                                                id: 5usize,
                                            },
                                        ],
                                        children: &[],
                                    },
                                    dioxus_core::TemplateNode::Dynamic {
                                        id: 0usize,
                                    },
                                ],
                            },
                        ],
                        node_paths: &[&[0u8, 2u8]],
                        attr_paths: &[
                            &[0u8],
                            &[0u8],
                            &[0u8, 0u8, 0u8],
                            &[0u8, 1u8],
                            &[0u8, 1u8],
                            &[0u8, 1u8],
                        ],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                {
                                    let ___nodes = ({
                                        elements
                                            .read()
                                            .iter()
                                            .map(move |(name, _)| {
                                                Some({
                                                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                                                        name: "src/ui/wrap.rs:65:62:12511",
                                                        roots: &[
                                                            dioxus_core::TemplateNode::Dynamic {
                                                                id: 0usize,
                                                            },
                                                        ],
                                                        node_paths: &[&[0u8]],
                                                        attr_paths: &[],
                                                    };
                                                    {
                                                        let __vnodes = dioxus_core::VNode::new(
                                                            None,
                                                            TEMPLATE,
                                                            Box::new([
                                                                dioxus_core::DynamicNode::Component({
                                                                    use dioxus_core::prelude::Properties;
                                                                    (fc_to_builder(NavItem)
                                                                        .route(Route::ComponentScreen {
                                                                            name: name.to_string(),
                                                                        })
                                                                        .label((name).to_string())
                                                                        .build())
                                                                        .into_vcomponent(NavItem, "NavItem")
                                                                }),
                                                            ]),
                                                            Box::new([]),
                                                        );
                                                        __vnodes
                                                    }
                                                })
                                            })
                                    })
                                        .into_dyn_node();
                                    ___nodes
                                },
                            ]),
                            Box::new([
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::flex.0,
                                        1,
                                        dioxus_elements::div::flex.1,
                                        dioxus_elements::div::flex.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::margin.0,
                                        0,
                                        dioxus_elements::div::margin.1,
                                        dioxus_elements::div::margin.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_elements::events::onclick(move |_| {
                                        navigator.push(Route::Home);
                                    }),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::input::value.0,
                                        (query).to_string().to_string(),
                                        dioxus_elements::input::value.1,
                                        dioxus_elements::input::value.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::input::font_size.0,
                                        14.,
                                        dioxus_elements::input::font_size.1,
                                        dioxus_elements::input::font_size.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_elements::events::oninput(move |event: FormEvent| {
                                        query.set(event.value().clone())
                                    }),
                                ]),
                            ]),
                        );
                        __vnodes
                    }
                });
                let right = Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/wrap.rs:70:17:12657",
                        roots: &[
                            dioxus_core::TemplateNode::Dynamic {
                                id: 0usize,
                            },
                        ],
                        node_paths: &[&[0u8]],
                        attr_paths: &[],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Component({
                                    use dioxus_core::prelude::Properties;
                                    (fc_to_builder(Outlet::<PrefixedRoute>).build())
                                        .into_vcomponent(Outlet::<PrefixedRoute>, "Outlet")
                                }),
                            ]),
                            Box::new([]),
                        );
                        __vnodes
                    }
                });
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/wrap.rs:72:5:12726",
                        roots: &[
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::position.0,
                                        namespace: dioxus_elements::div::position.1,
                                        value: "absolute",
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 0usize,
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 1usize,
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::width.0,
                                        namespace: dioxus_elements::div::width.1,
                                        value: "100vw",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::height.0,
                                        namespace: dioxus_elements::div::height.1,
                                        value: "100vh",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::display.0,
                                        namespace: dioxus_elements::div::display.1,
                                        value: "flex",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::flex_direction.0,
                                        namespace: dioxus_elements::div::flex_direction.1,
                                        value: "row",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::font_family.0,
                                        namespace: dioxus_elements::div::font_family.1,
                                        value: "sans-serif",
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 2usize,
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 3usize,
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::Dynamic {
                                        id: 0usize,
                                    },
                                ],
                            },
                        ],
                        node_paths: &[&[0u8, 0u8]],
                        attr_paths: &[&[0u8], &[0u8], &[0u8], &[0u8]],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Component({
                                    use dioxus_core::prelude::Properties;
                                    (fc_to_builder(HorizontalPane)
                                        .left(left)
                                        .right(right)
                                        .build())
                                        .into_vcomponent(HorizontalPane, "HorizontalPane")
                                }),
                            ]),
                            Box::new([
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::top.0,
                                        0,
                                        dioxus_elements::div::top.1,
                                        dioxus_elements::div::top.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::left.0,
                                        0,
                                        dioxus_elements::div::left.1,
                                        dioxus_elements::div::left.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::margin.0,
                                        0,
                                        dioxus_elements::div::margin.1,
                                        dioxus_elements::div::margin.2,
                                    ),
                                ]),
                                Box::new([
                                    dioxus_core::Attribute::new(
                                        dioxus_elements::div::padding.0,
                                        0,
                                        dioxus_elements::div::padding.1,
                                        dioxus_elements::div::padding.2,
                                    ),
                                ]),
                            ]),
                        );
                        __vnodes
                    }
                })
            }
        }
        ///Properties for the [`NavItem`] component.
        #[allow(non_camel_case_types)]
        struct NavItemProps {
            route: Route,
            label: String,
        }
        impl NavItemProps {
            /**
Create a builder for building `NavItemProps`.
On the builder, call `.route(...)`, `.label(...)` to set the values of the fields.
Finally, call `.build()` to create the instance of `NavItemProps`.
                    */
            #[allow(dead_code, clippy::type_complexity)]
            fn builder() -> NavItemPropsBuilder<((), ())> {
                NavItemPropsBuilder {
                    fields: ((), ()),
                    _phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        struct NavItemPropsBuilder<TypedBuilderFields> {
            fields: TypedBuilderFields,
            _phantom: (),
        }
        impl dioxus_core::prelude::Properties for NavItemProps
        where
            Self: Clone,
        {
            type Builder = NavItemPropsBuilder<((), ())>;
            fn builder() -> Self::Builder {
                NavItemProps::builder()
            }
            fn memoize(&mut self, new: &Self) -> bool {
                let equal = self == new;
                if !equal {
                    let new_clone = new.clone();
                    self.route = new_clone.route;
                    self.label = new_clone.label;
                }
                equal
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub trait NavItemPropsBuilder_Optional<T> {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
        }
        impl<T> NavItemPropsBuilder_Optional<T> for () {
            fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
                default()
            }
        }
        impl<T> NavItemPropsBuilder_Optional<T> for (T,) {
            fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
                self.0
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__label> NavItemPropsBuilder<((), __label)> {
            #[allow(clippy::type_complexity)]
            pub fn route(
                self,
                route: Route,
            ) -> NavItemPropsBuilder<((Route,), __label)> {
                let route = (route,);
                let (_, label) = self.fields;
                NavItemPropsBuilder {
                    fields: (route, label),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum NavItemPropsBuilder_Error_Repeated_field_route {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__label> NavItemPropsBuilder<((Route,), __label)> {
            #[deprecated(note = "Repeated field route")]
            #[allow(clippy::type_complexity)]
            pub fn route(
                self,
                _: NavItemPropsBuilder_Error_Repeated_field_route,
            ) -> NavItemPropsBuilder<((Route,), __label)> {
                self
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__route> NavItemPropsBuilder<(__route, ())> {
            #[allow(clippy::type_complexity)]
            pub fn label(
                self,
                label: impl ::core::fmt::Display,
            ) -> NavItemPropsBuilder<(__route, (String,))> {
                let label = (label.to_string(),);
                let (route, _) = self.fields;
                NavItemPropsBuilder {
                    fields: (route, label),
                    _phantom: self._phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum NavItemPropsBuilder_Error_Repeated_field_label {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl<__route> NavItemPropsBuilder<(__route, (String,))> {
            #[deprecated(note = "Repeated field label")]
            #[allow(clippy::type_complexity)]
            pub fn label(
                self,
                _: NavItemPropsBuilder_Error_Repeated_field_label,
            ) -> NavItemPropsBuilder<(__route, (String,))> {
                self
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum NavItemPropsBuilder_Error_Missing_required_field_route {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl<__label> NavItemPropsBuilder<((), __label)> {
            #[deprecated(note = "Missing required field route")]
            pub fn build(
                self,
                _: NavItemPropsBuilder_Error_Missing_required_field_route,
            ) -> NavItemProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub enum NavItemPropsBuilder_Error_Missing_required_field_label {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        impl NavItemPropsBuilder<((Route,), ())> {
            #[deprecated(note = "Missing required field label")]
            pub fn build(
                self,
                _: NavItemPropsBuilder_Error_Missing_required_field_label,
            ) -> NavItemProps {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        impl NavItemPropsBuilder<((Route,), (String,))> {
            pub fn build(self) -> NavItemProps {
                let (route, label) = self.fields;
                let route = route.0;
                let label = label.0;
                NavItemProps { route, label }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for NavItemProps {
            #[inline]
            fn clone(&self) -> NavItemProps {
                NavItemProps {
                    route: ::core::clone::Clone::clone(&self.route),
                    label: ::core::clone::Clone::clone(&self.label),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::StructuralPartialEq for NavItemProps {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for NavItemProps {
            #[inline]
            fn eq(&self, other: &NavItemProps) -> bool {
                self.route == other.route && self.label == other.label
            }
        }
        #[allow(non_snake_case)]
        /// Navigation rail item component.
        fn NavItem(mut __props: NavItemProps) -> Element {
            let NavItemProps { mut route, mut label } = __props;
            {
                let navigator = use_navigator();
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/ui/wrap.rs:100:5:13456",
                        roots: &[
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::div::TAG_NAME,
                                namespace: dioxus_elements::div::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::padding.0,
                                        namespace: dioxus_elements::div::padding.1,
                                        value: "10px 15px",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::div::cursor.0,
                                        namespace: dioxus_elements::div::cursor.1,
                                        value: "pointer",
                                    },
                                    dioxus_core::TemplateAttribute::Dynamic {
                                        id: 0usize,
                                    },
                                ],
                                children: &[
                                    dioxus_core::TemplateNode::DynamicText {
                                        id: 0usize,
                                    },
                                ],
                            },
                        ],
                        node_paths: &[&[0u8, 0u8]],
                        attr_paths: &[&[0u8]],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Text(
                                    dioxus_core::VText::new(
                                        (label.as_str()).to_string().to_string(),
                                    ),
                                ),
                            ]),
                            Box::new([
                                Box::new([
                                    dioxus_elements::events::onclick(move |_| {
                                        navigator.push(PrefixedRoute(route.clone()));
                                    }),
                                ]),
                            ]),
                        );
                        __vnodes
                    }
                })
            }
        }
    }
    pub use wrap::Wrap;
}
use ui::Wrap;
pub use ui::{Look, LookBook};
mod prefixed_route {
    use crate::Route;
    use core::cell::RefCell;
    use dioxus::prelude::*;
    use std::fmt;
    const PREFIX: ::std::thread::LocalKey<RefCell<&'static str>> = {
        #[inline]
        fn __init() -> RefCell<&'static str> {
            RefCell::new("")
        }
        #[inline]
        unsafe fn __getit(
            init: ::std::option::Option<
                &mut ::std::option::Option<RefCell<&'static str>>,
            >,
        ) -> ::std::option::Option<&'static RefCell<&'static str>> {
            #[thread_local]
            static __KEY: ::std::thread::local_impl::Key<RefCell<&'static str>> = ::std::thread::local_impl::Key::<
                RefCell<&'static str>,
            >::new();
            unsafe {
                __KEY
                    .get(move || {
                        if let ::std::option::Option::Some(init) = init {
                            if let ::std::option::Option::Some(value) = init.take() {
                                return value;
                            }
                            if true {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "internal error: entered unreachable code: {0}",
                                            format_args!("missing default value"),
                                        ),
                                    );
                                };
                            }
                        }
                        __init()
                    })
            }
        }
        unsafe { ::std::thread::LocalKey::new(__getit) }
    };
    pub fn use_prefix(prefix: Option<&'static str>) {
        use_future(move || {
            if let Some(prefix) = prefix {
                PREFIX.try_with(|cell| *cell.borrow_mut() = prefix).unwrap();
            }
            async move {}
        });
    }
    pub struct PrefixedRoute(pub(crate) Route);
    #[automatically_derived]
    impl ::core::clone::Clone for PrefixedRoute {
        #[inline]
        fn clone(&self) -> PrefixedRoute {
            PrefixedRoute(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PrefixedRoute {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PrefixedRoute {
        #[inline]
        fn eq(&self, other: &PrefixedRoute) -> bool {
            self.0 == other.0
        }
    }
    pub struct PrefixError;
    impl fmt::Display for PrefixError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("DummyError")
        }
    }
    impl dioxus::prelude::Routable for PrefixedRoute {
        const SITE_MAP: &'static [dioxus::prelude::SiteMapSegment] = &[];
        fn render<'a>(&self, level: usize) -> dioxus::prelude::Element {
            self.0.render(level)
        }
        fn static_routes() -> Vec<Self> {
            Route::static_routes().into_iter().map(PrefixedRoute).collect()
        }
    }
    impl std::str::FromStr for PrefixedRoute {
        type Err = PrefixError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let prefix = &*PREFIX.try_with(|cell| *cell.borrow()).unwrap();
            if s.is_empty() || s.starts_with(prefix) {
                let route = s[prefix.len()..].parse::<Route>().map_err(|_| PrefixError)?;
                Ok(PrefixedRoute(route))
            } else {
                Err(PrefixError)
            }
        }
    }
    impl fmt::Display for PrefixedRoute {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let prefix = &*PREFIX.try_with(|cell| *cell.borrow()).unwrap();
            f.write_str(&prefix)?;
            self.0.fmt(f)
        }
    }
}
pub(crate) use prefixed_route::PrefixedRoute;
#[doc(hidden)]
pub struct Preview {
    name: &'static str,
    component: Component,
}
#[automatically_derived]
impl ::core::clone::Clone for Preview {
    #[inline]
    fn clone(&self) -> Preview {
        let _: ::core::clone::AssertParamIsClone<&'static str>;
        let _: ::core::clone::AssertParamIsClone<Component>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Preview {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Preview {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Preview {
    #[inline]
    fn eq(&self, other: &Preview) -> bool {
        self.name == other.name && self.component == other.component
    }
}
impl Preview {
    pub const fn new(name: &'static str, component: Component) -> Self {
        Self { name, component }
    }
}
const CONTEXT: ::std::thread::LocalKey<RefCell<Vec<(&'static str, Component)>>> = {
    #[inline]
    fn __init() -> RefCell<Vec<(&'static str, Component)>> {
        RefCell::new(Vec::new())
    }
    #[inline]
    unsafe fn __getit(
        init: ::std::option::Option<
            &mut ::std::option::Option<RefCell<Vec<(&'static str, Component)>>>,
        >,
    ) -> ::std::option::Option<&'static RefCell<Vec<(&'static str, Component)>>> {
        #[thread_local]
        static __KEY: ::std::thread::local_impl::Key<
            RefCell<Vec<(&'static str, Component)>>,
        > = ::std::thread::local_impl::Key::<
            RefCell<Vec<(&'static str, Component)>>,
        >::new();
        unsafe {
            __KEY
                .get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        }
                        if true {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("missing default value"),
                                    ),
                                );
                            };
                        }
                    }
                    __init()
                })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
const HOME: ::std::thread::LocalKey<RefCell<Option<Component>>> = {
    #[inline]
    fn __init() -> RefCell<Option<Component>> {
        RefCell::new(None)
    }
    #[inline]
    unsafe fn __getit(
        init: ::std::option::Option<
            &mut ::std::option::Option<RefCell<Option<Component>>>,
        >,
    ) -> ::std::option::Option<&'static RefCell<Option<Component>>> {
        #[thread_local]
        static __KEY: ::std::thread::local_impl::Key<RefCell<Option<Component>>> = ::std::thread::local_impl::Key::<
            RefCell<Option<Component>>,
        >::new();
        unsafe {
            __KEY
                .get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        }
                        if true {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("missing default value"),
                                    ),
                                );
                            };
                        }
                    }
                    __init()
                })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
fn register(name: &'static str, component: Component) {
    CONTEXT.try_with(|cx| cx.borrow_mut().push((name, component))).unwrap();
}
enum Route {
    #[layout(Wrap)]
    #[route("/")]
    Home,
    #[route("/:name")]
    ComponentScreen { name: String },
}
#[automatically_derived]
impl ::core::clone::Clone for Route {
    #[inline]
    fn clone(&self) -> Route {
        match self {
            Route::Home => Route::Home,
            Route::ComponentScreen { name: __self_0 } => {
                Route::ComponentScreen {
                    name: ::core::clone::Clone::clone(__self_0),
                }
            }
        }
    }
}
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum HomeParseError {
    ExtraSegments(String),
    StaticSegment0ParseError(String),
}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::fmt::Debug for HomeParseError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            HomeParseError::ExtraSegments(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ExtraSegments",
                    &__self_0,
                )
            }
            HomeParseError::StaticSegment0ParseError(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "StaticSegment0ParseError",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::marker::StructuralPartialEq for HomeParseError {}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::cmp::PartialEq for HomeParseError {
    #[inline]
    fn eq(&self, other: &HomeParseError) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    HomeParseError::ExtraSegments(__self_0),
                    HomeParseError::ExtraSegments(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                (
                    HomeParseError::StaticSegment0ParseError(__self_0),
                    HomeParseError::StaticSegment0ParseError(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
impl std::fmt::Display for HomeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExtraSegments(segments) => {
                f.write_fmt(
                    format_args!("Found additional trailing segments: {0}", segments),
                )?
            }
            Self::StaticSegment0ParseError(found) => {
                f.write_fmt(
                    format_args!(
                        "Static segment \'{0}\' did not match instead found \'{1}\'",
                        "",
                        found,
                    ),
                )?
            }
        }
        Ok(())
    }
}
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum ComponentScreenParseError {
    ExtraSegments(String),
    nameParseError(<String as dioxus_router::routable::FromRouteSegment>::Err),
    nameMissingError,
}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::fmt::Debug for ComponentScreenParseError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ComponentScreenParseError::ExtraSegments(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ExtraSegments",
                    &__self_0,
                )
            }
            ComponentScreenParseError::nameParseError(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "nameParseError",
                    &__self_0,
                )
            }
            ComponentScreenParseError::nameMissingError => {
                ::core::fmt::Formatter::write_str(f, "nameMissingError")
            }
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::marker::StructuralPartialEq for ComponentScreenParseError {}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::cmp::PartialEq for ComponentScreenParseError {
    #[inline]
    fn eq(&self, other: &ComponentScreenParseError) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    ComponentScreenParseError::ExtraSegments(__self_0),
                    ComponentScreenParseError::ExtraSegments(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                (
                    ComponentScreenParseError::nameParseError(__self_0),
                    ComponentScreenParseError::nameParseError(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => true,
            }
    }
}
impl std::fmt::Display for ComponentScreenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExtraSegments(segments) => {
                f.write_fmt(
                    format_args!("Found additional trailing segments: {0}", segments),
                )?
            }
            Self::nameParseError(err) => {
                f.write_fmt(
                    format_args!(
                        "Dynamic segment \'({0}:{1})\' did not match: {2}",
                        "name",
                        "String",
                        err,
                    ),
                )?
            }
            Self::nameMissingError => {
                f.write_fmt(
                    format_args!(
                        "Dynamic segment \'({0}:{1})\' was missing",
                        "name",
                        "String",
                    ),
                )?
            }
        }
        Ok(())
    }
}
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum RouteMatchError {
    Home(HomeParseError),
    ComponentScreen(ComponentScreenParseError),
}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::fmt::Debug for RouteMatchError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            RouteMatchError::Home(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Home", &__self_0)
            }
            RouteMatchError::ComponentScreen(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ComponentScreen",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::marker::StructuralPartialEq for RouteMatchError {}
#[automatically_derived]
#[allow(non_camel_case_types)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl ::core::cmp::PartialEq for RouteMatchError {
    #[inline]
    fn eq(&self, other: &RouteMatchError) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (RouteMatchError::Home(__self_0), RouteMatchError::Home(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (
                    RouteMatchError::ComponentScreen(__self_0),
                    RouteMatchError::ComponentScreen(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
impl std::fmt::Display for RouteMatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Home(err) => {
                f.write_fmt(
                    format_args!(
                        "Route \'{0}\' (\'{1}\') did not match:\n{2}",
                        "Home",
                        "/",
                        err,
                    ),
                )?
            }
            Self::ComponentScreen(err) => {
                f.write_fmt(
                    format_args!(
                        "Route \'{0}\' (\'{1}\') did not match:\n{2}",
                        "ComponentScreen",
                        "/:name",
                        err,
                    ),
                )?
            }
        }
        Ok(())
    }
}
impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(unused)]
        match self {
            Self::Home {} => {
                f.write_fmt(format_args!("/{0}", ""))?;
            }
            Self::ComponentScreen { name } => {
                f.write_fmt(format_args!("/{0}", name))?;
            }
        }
        Ok(())
    }
}
impl dioxus_router::routable::Routable for Route
where
    Self: Clone,
{
    const SITE_MAP: &'static [dioxus_router::routable::SiteMapSegment] = &[
        dioxus_router::routable::SiteMapSegment {
            segment_type: dioxus_router::routable::SegmentType::Static(""),
            children: &[],
        },
        dioxus_router::routable::SiteMapSegment {
            segment_type: dioxus_router::routable::SegmentType::Dynamic("name"),
            children: &[],
        },
    ];
    fn render(&self, level: usize) -> ::dioxus::prelude::Element {
        let myself = self.clone();
        match (level, myself) {
            #[allow(unused)]
            (0usize, Self::Home { .. }) => {
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/lib.rs:41:17:908",
                        roots: &[
                            dioxus_core::TemplateNode::Dynamic {
                                id: 0usize,
                            },
                        ],
                        node_paths: &[&[0u8]],
                        attr_paths: &[],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Component({
                                    use dioxus_core::prelude::Properties;
                                    (fc_to_builder(Wrap).build()).into_vcomponent(Wrap, "Wrap")
                                }),
                            ]),
                            Box::new([]),
                        );
                        __vnodes
                    }
                })
            }
            #[allow(unused)]
            (1usize, Self::Home {}) => {
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/lib.rs:41:17:937",
                        roots: &[
                            dioxus_core::TemplateNode::Dynamic {
                                id: 0usize,
                            },
                        ],
                        node_paths: &[&[0u8]],
                        attr_paths: &[],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Component({
                                    use dioxus_core::prelude::Properties;
                                    (fc_to_builder(Home).build()).into_vcomponent(Home, "Home")
                                }),
                            ]),
                            Box::new([]),
                        );
                        __vnodes
                    }
                })
            }
            #[allow(unused)]
            (0usize, Self::ComponentScreen { name, .. }) => {
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/lib.rs:41:17:908",
                        roots: &[
                            dioxus_core::TemplateNode::Dynamic {
                                id: 0usize,
                            },
                        ],
                        node_paths: &[&[0u8]],
                        attr_paths: &[],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Component({
                                    use dioxus_core::prelude::Properties;
                                    (fc_to_builder(Wrap).build()).into_vcomponent(Wrap, "Wrap")
                                }),
                            ]),
                            Box::new([]),
                        );
                        __vnodes
                    }
                })
            }
            #[allow(unused)]
            (1usize, Self::ComponentScreen { name }) => {
                Some({
                    static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                        name: "src/lib.rs:41:17:981",
                        roots: &[
                            dioxus_core::TemplateNode::Dynamic {
                                id: 0usize,
                            },
                        ],
                        node_paths: &[&[0u8]],
                        attr_paths: &[],
                    };
                    {
                        let __vnodes = dioxus_core::VNode::new(
                            None,
                            TEMPLATE,
                            Box::new([
                                dioxus_core::DynamicNode::Component({
                                    use dioxus_core::prelude::Properties;
                                    (fc_to_builder(ComponentScreen).name(name).build())
                                        .into_vcomponent(ComponentScreen, "ComponentScreen")
                                }),
                            ]),
                            Box::new([]),
                        );
                        __vnodes
                    }
                })
            }
            _ => None,
        }
    }
}
impl dioxus_core::ComponentFunction<
    ::std::rc::Rc<::std::cell::Cell<dioxus_router::prelude::RouterConfig<Route>>>,
> for Route {
    fn rebuild(
        &self,
        props: ::std::rc::Rc<
            ::std::cell::Cell<dioxus_router::prelude::RouterConfig<Route>>,
        >,
    ) -> dioxus_core::Element {
        let initial_route = self.clone();
        Some({
            static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                name: "src/lib.rs:41:17:857",
                roots: &[
                    dioxus_core::TemplateNode::Dynamic {
                        id: 0usize,
                    },
                ],
                node_paths: &[&[0u8]],
                attr_paths: &[],
            };
            {
                let __vnodes = dioxus_core::VNode::new(
                    None,
                    TEMPLATE,
                    Box::new([
                        dioxus_core::DynamicNode::Component({
                            use dioxus_core::prelude::Properties;
                            (fc_to_builder(dioxus_router::prelude::Router::<Route>)
                                .config(move || props.take().initial_route(initial_route))
                                .build())
                                .into_vcomponent(
                                    dioxus_router::prelude::Router::<Route>,
                                    "Router",
                                )
                        }),
                    ]),
                    Box::new([]),
                );
                __vnodes
            }
        })
    }
}
impl<'a> core::convert::TryFrom<&'a str> for Route {
    type Error = <Self as std::str::FromStr>::Err;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
impl std::str::FromStr for Route {
    type Err = dioxus_router::routable::RouteParseError<RouteMatchError>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let route = s;
        let (route, _hash) = route.split_once('#').unwrap_or((route, ""));
        let (route, query) = route.split_once('?').unwrap_or((route, ""));
        let query = dioxus_router::exports::urlencoding::decode(query)
            .unwrap_or(query.into());
        let mut segments = route
            .split('/')
            .map(|s| dioxus_router::exports::urlencoding::decode(s).unwrap_or(s.into()));
        if s.starts_with('/') {
            let _ = segments.next();
        } else {
            return Err(dioxus_router::routable::RouteParseError {
                attempted_routes: Vec::new(),
            });
        }
        let mut errors = Vec::new();
        {
            let remaining_segments = segments.clone();
            let mut segments_clone = segments.clone();
            let next_segment = segments_clone.next();
            let next_segment = next_segment.as_deref();
            let segment_after_next = segments_clone.next();
            let segment_after_next = segment_after_next.as_deref();
            match (next_segment, segment_after_next) {
                (None, _) | (Some(""), None) => {
                    return Ok(Route::Home {});
                }
                _ => {
                    let mut trailing = String::new();
                    for seg in remaining_segments {
                        trailing += &*seg;
                        trailing += "/";
                    }
                    trailing.pop();
                    errors
                        .push(
                            RouteMatchError::Home(
                                HomeParseError::ExtraSegments(trailing),
                            ),
                        )
                }
            }
        }
        {
            let mut segments = segments.clone();
            let segment = segments.next();
            let parsed = if let Some(segment) = segment.as_deref() {
                <String as dioxus_router::routable::FromRouteSegment>::from_route_segment(
                        segment,
                    )
                    .map_err(|err| RouteMatchError::ComponentScreen(
                        ComponentScreenParseError::nameParseError(err),
                    ))
            } else {
                Err(
                    RouteMatchError::ComponentScreen(
                        ComponentScreenParseError::nameMissingError,
                    ),
                )
            };
            match parsed {
                Ok(name) => {
                    let remaining_segments = segments.clone();
                    let mut segments_clone = segments.clone();
                    let next_segment = segments_clone.next();
                    let next_segment = next_segment.as_deref();
                    let segment_after_next = segments_clone.next();
                    let segment_after_next = segment_after_next.as_deref();
                    match (next_segment, segment_after_next) {
                        (None, _) | (Some(""), None) => {
                            return Ok(Route::ComponentScreen { name });
                        }
                        _ => {
                            let mut trailing = String::new();
                            for seg in remaining_segments {
                                trailing += &*seg;
                                trailing += "/";
                            }
                            trailing.pop();
                            errors
                                .push(
                                    RouteMatchError::ComponentScreen(
                                        ComponentScreenParseError::ExtraSegments(trailing),
                                    ),
                                )
                        }
                    }
                }
                Err(err) => {
                    errors.push(err);
                }
            }
        }
        Err(dioxus_router::routable::RouteParseError {
            attempted_routes: errors,
        })
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Route {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Route::Home => ::core::fmt::Formatter::write_str(f, "Home"),
            Route::ComponentScreen { name: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ComponentScreen",
                    "name",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Route {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Route {
    #[inline]
    fn eq(&self, other: &Route) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    Route::ComponentScreen { name: __self_0 },
                    Route::ComponentScreen { name: __arg1_0 },
                ) => *__self_0 == *__arg1_0,
                _ => true,
            }
    }
}
#[allow(non_snake_case)]
fn Home() -> Element {
    {
        #[allow(non_snake_case)]
        let Child = HOME.try_with(|cell| cell.borrow().clone().unwrap()).unwrap();
        Some({
            static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                name: "src/lib.rs:56:5:1180",
                roots: &[
                    dioxus_core::TemplateNode::Dynamic {
                        id: 0usize,
                    },
                ],
                node_paths: &[&[0u8]],
                attr_paths: &[],
            };
            {
                let __vnodes = dioxus_core::VNode::new(
                    None,
                    TEMPLATE,
                    Box::new([
                        dioxus_core::DynamicNode::Component({
                            use dioxus_core::prelude::Properties;
                            (fc_to_builder(Child).build())
                                .into_vcomponent(Child, "Child")
                        }),
                    ]),
                    Box::new([]),
                );
                __vnodes
            }
        })
    }
}
///Properties for the [`ComponentScreen`] component.
#[allow(non_camel_case_types)]
struct ComponentScreenProps {
    name: String,
}
impl ComponentScreenProps {
    /**
Create a builder for building `ComponentScreenProps`.
On the builder, call `.name(...)` to set the values of the fields.
Finally, call `.build()` to create the instance of `ComponentScreenProps`.
                    */
    #[allow(dead_code, clippy::type_complexity)]
    fn builder() -> ComponentScreenPropsBuilder<((),)> {
        ComponentScreenPropsBuilder {
            fields: ((),),
            _phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
struct ComponentScreenPropsBuilder<TypedBuilderFields> {
    fields: TypedBuilderFields,
    _phantom: (),
}
impl dioxus_core::prelude::Properties for ComponentScreenProps
where
    Self: Clone,
{
    type Builder = ComponentScreenPropsBuilder<((),)>;
    fn builder() -> Self::Builder {
        ComponentScreenProps::builder()
    }
    fn memoize(&mut self, new: &Self) -> bool {
        let equal = self == new;
        if !equal {
            let new_clone = new.clone();
            self.name = new_clone.name;
        }
        equal
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub trait ComponentScreenPropsBuilder_Optional<T> {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
}
impl<T> ComponentScreenPropsBuilder_Optional<T> for () {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
        default()
    }
}
impl<T> ComponentScreenPropsBuilder_Optional<T> for (T,) {
    fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
        self.0
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl ComponentScreenPropsBuilder<((),)> {
    #[allow(clippy::type_complexity)]
    pub fn name(
        self,
        name: impl ::core::fmt::Display,
    ) -> ComponentScreenPropsBuilder<((String,),)> {
        let name = (name.to_string(),);
        let (_,) = self.fields;
        ComponentScreenPropsBuilder {
            fields: (name,),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum ComponentScreenPropsBuilder_Error_Repeated_field_name {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl ComponentScreenPropsBuilder<((String,),)> {
    #[deprecated(note = "Repeated field name")]
    #[allow(clippy::type_complexity)]
    pub fn name(
        self,
        _: ComponentScreenPropsBuilder_Error_Repeated_field_name,
    ) -> ComponentScreenPropsBuilder<((String,),)> {
        self
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum ComponentScreenPropsBuilder_Error_Missing_required_field_name {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
impl ComponentScreenPropsBuilder<((),)> {
    #[deprecated(note = "Missing required field name")]
    pub fn build(
        self,
        _: ComponentScreenPropsBuilder_Error_Missing_required_field_name,
    ) -> ComponentScreenProps {
        {
            #[cold]
            #[track_caller]
            #[inline(never)]
            const fn panic_cold_explicit() -> ! {
                ::core::panicking::panic_explicit()
            }
            panic_cold_explicit();
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl ComponentScreenPropsBuilder<((String,),)> {
    pub fn build(self) -> ComponentScreenProps {
        let (name,) = self.fields;
        let name = name.0;
        ComponentScreenProps { name }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for ComponentScreenProps {
    #[inline]
    fn clone(&self) -> ComponentScreenProps {
        ComponentScreenProps {
            name: ::core::clone::Clone::clone(&self.name),
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for ComponentScreenProps {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for ComponentScreenProps {
    #[inline]
    fn eq(&self, other: &ComponentScreenProps) -> bool {
        self.name == other.name
    }
}
#[allow(non_snake_case)]
fn ComponentScreen(mut __props: ComponentScreenProps) -> Element {
    let ComponentScreenProps { mut name } = __props;
    {
        #[allow(non_snake_case)]
        let (_name, Child) = CONTEXT
            .try_with(|cx| {
                cx.borrow().iter().find(|(n, _)| *n == name).unwrap().clone()
            })
            .unwrap();
        Some({
            static TEMPLATE: dioxus_core::Template = dioxus_core::Template {
                name: "src/lib.rs:84:5:1811",
                roots: &[
                    dioxus_core::TemplateNode::Dynamic {
                        id: 0usize,
                    },
                ],
                node_paths: &[&[0u8]],
                attr_paths: &[],
            };
            {
                let __vnodes = dioxus_core::VNode::new(
                    None,
                    TEMPLATE,
                    Box::new([
                        dioxus_core::DynamicNode::Component({
                            use dioxus_core::prelude::Properties;
                            (fc_to_builder(Child).build())
                                .into_vcomponent(Child, "Child")
                        }),
                    ]),
                    Box::new([]),
                );
                __vnodes
            }
        })
    }
}
