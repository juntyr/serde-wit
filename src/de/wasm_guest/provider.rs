use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::fmt;

use ::serde::serde_if_integer128;
use scoped_reference::{ScopedBorrowMut, ScopedReference};

mod bindings {
    wit_bindgen::generate!({ world: "serde-deserializer-provider", std_feature });
}

mod export {
    enum GuestsideDeserializerProvider {}

    impl super::bindings::exports::serde::serde::serde_deserializer::Guest
        for GuestsideDeserializerProvider
    {
        type Deserializer = super::GuestsideDeserializerProvider;
        type DeError = super::DeError;
        type SeqAccess = super::GuestsideSeqAccessProvider;
        type MapAccess = super::GuestsideMapAccessProvider;
        type EnumAccess = super::GuestsideEnumAccessProvider;
        type VariantAccess = super::GuestsideVariantAccessProvider;
    }

    super::bindings::export!(GuestsideDeserializerProvider with_types_in super::bindings);
}

use crate::{
    any::Any,
    intern::{intern_str_list, intern_string},
    wit_to_usize,
};

pub struct GuestsideDeserializerProvider {
    deserializer: Box<dyn ErasedDeserializer>,
    is_human_readable: bool,
    _scope: ScopedBorrowMut<()>,
}

impl GuestsideDeserializerProvider {
    #[must_use]
    pub fn with_new<'a, 'de, D: ::serde::de::Deserializer<'de> + 'a, F: FnOnce(Self) -> Q, Q>(
        deserializer: D,
        inner: F,
    ) -> Q {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let deserializer: Box<dyn ErasedDeserializer + 'a> = Box::new(deserializer);
            let deserializer: Box<dyn ErasedDeserializer + 'static> =
                unsafe { core::mem::transmute(deserializer) };

            inner(Self {
                is_human_readable: deserializer.erased_is_human_readable(),
                deserializer,
                _scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }
}

trait WrapDeResult {
    type Ok;

    fn wrap(self)
        -> Result<Self::Ok, bindings::exports::serde::serde::serde_deserializer::DeError>;
}

impl WrapDeResult for Result<DeValue, DeError> {
    type Ok = bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle;

    fn wrap(
        self,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        match self {
            Ok(ok) => Ok(
                bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle {
                    owned_handle: core::mem::ManuallyDrop::new(ok.value).handle(),
                },
            ),
            Err(error) => {
                Err(bindings::exports::serde::serde::serde_deserializer::DeError::new(error))
            }
        }
    }
}

impl WrapDeResult for Result<Option<DeValue>, DeError> {
    type Ok = Option<bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle>;

    fn wrap(
        self,
    ) -> Result<
        Option<bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle>,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        match self {
            Ok(None) => Ok(None),
            Ok(Some(ok)) => Ok(Some(
                bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle {
                    owned_handle: core::mem::ManuallyDrop::new(ok.value).handle(),
                },
            )),
            Err(error) => {
                Err(bindings::exports::serde::serde::serde_deserializer::DeError::new(error))
            }
        }
    }
}

impl WrapDeResult for Result<(), DeError> {
    type Ok = ();

    fn wrap(self) -> Result<(), bindings::exports::serde::serde::serde_deserializer::DeError> {
        self.map_err(bindings::exports::serde::serde::serde_deserializer::DeError::new)
    }
}

impl bindings::exports::serde::serde::serde_deserializer::GuestDeserializer
    for GuestsideDeserializerProvider
{
    fn deserialize_any(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_any(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_bool(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_bool(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_i8(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_i8(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_i16(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_i16(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_i32(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_i32(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_i64(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_i64(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_i128(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_i128(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_u8(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_u8(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_u16(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_u16(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_u32(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_u32(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_u64(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_u64(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_u128(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_u128(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_f32(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_f32(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_f64(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_f64(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_char(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_char(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_str(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_str(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_string(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_string(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_bytes(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_bytes(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_byte_buf(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_byte_buf(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_option(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_option(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_unit(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_unit(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_unit_struct(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        name: String,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_unit_struct(intern_string(name), VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_newtype_struct(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        name: String,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_newtype_struct(intern_string(name), VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_seq(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_seq(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_tuple(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        len: bindings::serde::serde::serde_types::Usize,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_tuple(wit_to_usize(len.val), VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_tuple_struct(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        name: String,
        len: bindings::serde::serde::serde_types::Usize,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_tuple_struct(
                intern_string(name),
                wit_to_usize(len.val),
                VisitableVisitor { visitor },
            )
            .wrap()
    }

    fn deserialize_map(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_map(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_struct(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        name: String,
        fields: Vec<String>,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let fields = fields.into_iter().map(intern_string).collect();

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_struct(
                intern_string(name),
                intern_str_list(fields),
                VisitableVisitor { visitor },
            )
            .wrap()
    }

    fn deserialize_enum(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        name: String,
        variants: Vec<String>,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let variants = variants.into_iter().map(intern_string).collect();

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_enum(
                intern_string(name),
                intern_str_list(variants),
                VisitableVisitor { visitor },
            )
            .wrap()
    }

    fn deserialize_identifier(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_identifier(VisitableVisitor { visitor })
            .wrap()
    }

    fn deserialize_ignored_any(
        this: bindings::exports::serde::serde::serde_deserializer::Deserializer,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { deserializer, .. } = this.into_inner();
        deserializer
            .erased_deserialize_ignored_any(VisitableVisitor { visitor })
            .wrap()
    }

    fn is_human_readable(
        this: bindings::exports::serde::serde::serde_deserializer::DeserializerBorrow,
    ) -> bool {
        this.get::<Self>().is_human_readable
    }
}

trait ErasedDeserializer {
    fn erased_deserialize_any(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_bool(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_u8(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_u16(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_u32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_u64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    serde_if_integer128! {
        fn erased_deserialize_u128(self: Box<Self>, visitor: VisitableVisitor) -> Result<DeValue, DeError>;
    }
    fn erased_deserialize_i8(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_i16(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_i32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_i64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    serde_if_integer128! {
        fn erased_deserialize_i128(self: Box<Self>, visitor: VisitableVisitor) -> Result<DeValue, DeError>;
    }
    fn erased_deserialize_f32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_f64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_char(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_str(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_string(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_bytes(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_byte_buf(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_option(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_unit(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_unit_struct(
        self: Box<Self>,
        name: &'static str,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_newtype_struct(
        self: Box<Self>,
        name: &'static str,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_seq(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_tuple(
        self: Box<Self>,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_tuple_struct(
        self: Box<Self>,
        name: &'static str,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_map(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_struct(
        self: Box<Self>,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_identifier(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_enum(
        self: Box<Self>,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_deserialize_ignored_any(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_is_human_readable(&self) -> bool;
}

impl<'de, T: ::serde::de::Deserializer<'de>> ErasedDeserializer for T {
    fn erased_deserialize_any(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_any(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_bool(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_bool(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_u8(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_u8(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_u16(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_u16(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_u32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_u32(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_u64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_u64(visitor).map_err(DeError::wrap)
    }

    serde_if_integer128! {
        fn erased_deserialize_u128(self:Box<Self>,visitor:VisitableVisitor) -> Result<DeValue,DeError> {
            self.deserialize_u128(visitor).map_err(DeError::wrap)
        }
    }

    fn erased_deserialize_i8(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_i8(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_i16(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_i16(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_i32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_i32(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_i64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_i64(visitor).map_err(DeError::wrap)
    }

    serde_if_integer128! {
        fn erased_deserialize_i128(self:Box<Self>,visitor:VisitableVisitor) -> Result<DeValue,DeError> {
            self.deserialize_i128(visitor).map_err(DeError::wrap)
        }
    }

    fn erased_deserialize_f32(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_f32(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_f64(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_f64(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_char(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_char(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_str(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_str(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_string(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_string(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_bytes(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_bytes(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_byte_buf(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_byte_buf(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_option(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_option(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_unit(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_unit(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_unit_struct(
        self: Box<Self>,
        name: &'static str,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_unit_struct(name, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_newtype_struct(
        self: Box<Self>,
        name: &'static str,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_newtype_struct(name, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_seq(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_seq(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_tuple(
        self: Box<Self>,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_tuple(len, visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_tuple_struct(
        self: Box<Self>,
        name: &'static str,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_tuple_struct(name, len, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_map(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_map(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_struct(
        self: Box<Self>,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_struct(name, fields, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_identifier(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_identifier(visitor).map_err(DeError::wrap)
    }

    fn erased_deserialize_enum(
        self: Box<Self>,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_enum(name, variants, visitor)
            .map_err(DeError::wrap)
    }

    fn erased_deserialize_ignored_any(
        self: Box<Self>,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.deserialize_ignored_any(visitor).map_err(DeError::wrap)
    }

    fn erased_is_human_readable(&self) -> bool {
        self.is_human_readable()
    }
}

trait ErasedSeqAccess {
    fn erased_next_element_seed(
        &mut self,
        seed: DeserializableDeserializeSeed,
    ) -> Result<Option<DeValue>, DeError>;

    fn erased_size_hint(&self) -> Option<usize>;
}

impl<'de, T: ::serde::de::SeqAccess<'de>> ErasedSeqAccess for T {
    fn erased_next_element_seed(
        &mut self,
        seed: DeserializableDeserializeSeed,
    ) -> Result<Option<DeValue>, DeError> {
        self.next_element_seed(seed).map_err(DeError::wrap)
    }

    fn erased_size_hint(&self) -> Option<usize> {
        self.size_hint()
    }
}

pub struct GuestsideSeqAccessProvider {
    seq_access: Box<dyn ErasedSeqAccess>,
    _scope: ScopedBorrowMut<()>,
}

impl GuestsideSeqAccessProvider {
    #[must_use]
    pub fn with_new<'a, 'de, D: ::serde::de::SeqAccess<'de> + 'a, F: FnOnce(Self) -> Q, Q>(
        seq_access: D,
        inner: F,
    ) -> Q {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let seq_access: Box<dyn ErasedSeqAccess + 'a> = Box::new(seq_access);
            let seq_access: Box<dyn ErasedSeqAccess + 'static> =
                unsafe { core::mem::transmute(seq_access) };

            inner(Self {
                seq_access,
                _scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }
}

impl bindings::exports::serde::serde::serde_deserializer::GuestSeqAccess
    for GuestsideSeqAccessProvider
{
    fn next_element_seed(
        mut this: bindings::exports::serde::serde::serde_deserializer::SeqAccess,
        seed: bindings::exports::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle,
    ) -> (
        bindings::exports::serde::serde::serde_deserializer::SeqAccess,
        Result<
            Option<bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle>,
            bindings::exports::serde::serde::serde_deserializer::DeError,
        >,
    ) {
        // TODO: Safety
        let seed = unsafe {
            bindings::serde::serde::serde_deserialize::DeserializeSeed::from_handle(
                seed.owned_handle,
            )
        };

        let result = this
            .get_mut::<Self>()
            .seq_access
            .erased_next_element_seed(DeserializableDeserializeSeed {
                deserialize_seed: seed,
            })
            .wrap();

        (this, result)
    }

    fn size_hint(
        this: bindings::exports::serde::serde::serde_deserializer::SeqAccessBorrow,
    ) -> Option<bindings::serde::serde::serde_types::Usize> {
        let size_hint = this.get::<Self>().seq_access.erased_size_hint()?;

        u32::try_from(size_hint)
            .ok()
            .map(|size_hint| bindings::serde::serde::serde_types::Usize { val: size_hint })
    }
}

trait ErasedMapAccess {
    fn erased_next_key_seed(
        &mut self,
        seed: DeserializableDeserializeSeed,
    ) -> Result<Option<DeValue>, DeError>;
    fn erased_next_value_seed(
        &mut self,
        seed: DeserializableDeserializeSeed,
    ) -> Result<DeValue, DeError>;
    fn erased_size_hint(&self) -> Option<usize>;
}

impl<'de, T: ::serde::de::MapAccess<'de>> ErasedMapAccess for T {
    fn erased_next_key_seed(
        &mut self,
        seed: DeserializableDeserializeSeed,
    ) -> Result<Option<DeValue>, DeError> {
        self.next_key_seed(seed).map_err(DeError::wrap)
    }

    fn erased_next_value_seed(
        &mut self,
        seed: DeserializableDeserializeSeed,
    ) -> Result<DeValue, DeError> {
        self.next_value_seed(seed).map_err(DeError::wrap)
    }

    fn erased_size_hint(&self) -> Option<usize> {
        self.size_hint()
    }
}

pub struct GuestsideMapAccessProvider {
    map_access: Box<dyn ErasedMapAccess>,
    _scope: ScopedBorrowMut<()>,
}

impl GuestsideMapAccessProvider {
    #[must_use]
    pub fn with_new<'a, 'de, D: ::serde::de::MapAccess<'de> + 'a, F: FnOnce(Self) -> Q, Q>(
        map_access: D,
        inner: F,
    ) -> Q {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let map_access: Box<dyn ErasedMapAccess + 'a> = Box::new(map_access);
            let map_access: Box<dyn ErasedMapAccess + 'static> =
                unsafe { core::mem::transmute(map_access) };

            inner(Self {
                map_access,
                _scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }
}

impl bindings::exports::serde::serde::serde_deserializer::GuestMapAccess
    for GuestsideMapAccessProvider
{
    fn next_key_seed(
        mut this: bindings::exports::serde::serde::serde_deserializer::MapAccess,
        seed: bindings::exports::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle,
    ) -> (
        bindings::exports::serde::serde::serde_deserializer::MapAccess,
        Result<
            Option<bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle>,
            bindings::exports::serde::serde::serde_deserializer::DeError,
        >,
    ) {
        // TODO: Safety
        let seed = unsafe {
            bindings::serde::serde::serde_deserialize::DeserializeSeed::from_handle(
                seed.owned_handle,
            )
        };

        let result = this
            .get_mut::<Self>()
            .map_access
            .erased_next_key_seed(DeserializableDeserializeSeed {
                deserialize_seed: seed,
            })
            .wrap();

        (this, result)
    }

    fn next_value_seed(
        mut this: bindings::exports::serde::serde::serde_deserializer::MapAccess,
        seed: bindings::exports::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle,
    ) -> (
        bindings::exports::serde::serde::serde_deserializer::MapAccess,
        Result<
            bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
            bindings::exports::serde::serde::serde_deserializer::DeError,
        >,
    ) {
        // TODO: Safety
        let seed = unsafe {
            bindings::serde::serde::serde_deserialize::DeserializeSeed::from_handle(
                seed.owned_handle,
            )
        };

        let result = this
            .get_mut::<Self>()
            .map_access
            .erased_next_value_seed(DeserializableDeserializeSeed {
                deserialize_seed: seed,
            })
            .wrap();

        (this, result)
    }

    fn size_hint(
        this: bindings::exports::serde::serde::serde_deserializer::MapAccessBorrow,
    ) -> Option<bindings::serde::serde::serde_types::Usize> {
        let size_hint = this.get::<Self>().map_access.erased_size_hint()?;

        u32::try_from(size_hint)
            .ok()
            .map(|size_hint| bindings::serde::serde::serde_types::Usize { val: size_hint })
    }
}

trait ErasedEnumAccess {
    fn erased_variant_seed(
        self: Box<Self>,
        seed: DeserializableDeserializeSeed,
        scope: ScopedBorrowMut<()>,
    ) -> Result<(DeValue, GuestsideVariantAccessProvider), DeError>;
}

impl<'de, T: ::serde::de::EnumAccess<'de>> ErasedEnumAccess for T {
    fn erased_variant_seed(
        self: Box<Self>,
        seed: DeserializableDeserializeSeed,
        scope: ScopedBorrowMut<()>,
    ) -> Result<(DeValue, GuestsideVariantAccessProvider), DeError> {
        match self.variant_seed(seed) {
            Ok((value, variant_access)) => {
                let variant_access: Box<dyn ErasedVariantAccess + '_> = Box::new(variant_access);
                let variant_access: Box<dyn ErasedVariantAccess + 'static> =
                    unsafe { core::mem::transmute(variant_access) };

                let variant_access = GuestsideVariantAccessProvider {
                    variant_access,
                    _scope: scope,
                };

                Ok((value, variant_access))
            }
            Err(err) => Err(DeError::wrap(err)),
        }
    }
}

pub struct GuestsideEnumAccessProvider {
    enum_access: Box<dyn ErasedEnumAccess>,
    scope: ScopedBorrowMut<()>,
}

impl GuestsideEnumAccessProvider {
    #[must_use]
    pub fn with_new<'a, 'de, D: ::serde::de::EnumAccess<'de> + 'a, F: FnOnce(Self) -> Q, Q>(
        enum_access: D,
        inner: F,
    ) -> Q {
        #[allow(clippy::let_unit_value)]
        let mut scope = ();
        let mut scope = ScopedReference::new_mut(&mut scope);

        let result = {
            let enum_access: Box<dyn ErasedEnumAccess + 'a> = Box::new(enum_access);
            let enum_access: Box<dyn ErasedEnumAccess + 'static> =
                unsafe { core::mem::transmute(enum_access) };

            inner(Self {
                enum_access,
                scope: scope.borrow_mut(),
            })
        };

        // Abort if there are any outstanding, soon dangling, scoped borrows
        core::mem::drop(scope);

        result
    }
}

impl bindings::exports::serde::serde::serde_deserializer::GuestEnumAccess
    for GuestsideEnumAccessProvider
{
    fn variant_seed(
        this: bindings::exports::serde::serde::serde_deserializer::EnumAccess,
        seed: bindings::exports::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle,
    ) -> Result<
        (
            bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
            bindings::exports::serde::serde::serde_deserializer::VariantAccess,
        ),
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let seed = unsafe {
            bindings::serde::serde::serde_deserialize::DeserializeSeed::from_handle(
                seed.owned_handle,
            )
        };

        let Self { enum_access, scope } = this.into_inner();

        let result = enum_access.erased_variant_seed(
            DeserializableDeserializeSeed {
                deserialize_seed: seed,
            },
            scope,
        );

        match result {
            Ok((value, variant)) => Ok((
                bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle {
                    owned_handle: core::mem::ManuallyDrop::new(value.value).handle(),
                },
                bindings::exports::serde::serde::serde_deserializer::VariantAccess::new(variant),
            )),
            Err(error) => {
                Err(bindings::exports::serde::serde::serde_deserializer::DeError::new(error))
            }
        }
    }
}

trait ErasedVariantAccess {
    fn erased_unit_variant(self: Box<Self>) -> Result<(), DeError>;
    fn erased_newtype_variant_seed(
        self: Box<Self>,
        seed: DeserializableDeserializeSeed,
    ) -> Result<DeValue, DeError>;
    fn erased_tuple_variant(
        self: Box<Self>,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
    fn erased_struct_variant(
        self: Box<Self>,
        fields: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError>;
}

impl<'de, T: ::serde::de::VariantAccess<'de>> ErasedVariantAccess for T {
    fn erased_unit_variant(self: Box<Self>) -> Result<(), DeError> {
        self.unit_variant().map_err(DeError::wrap)
    }

    fn erased_newtype_variant_seed(
        self: Box<Self>,
        seed: DeserializableDeserializeSeed,
    ) -> Result<DeValue, DeError> {
        self.newtype_variant_seed(seed).map_err(DeError::wrap)
    }

    fn erased_tuple_variant(
        self: Box<Self>,
        len: usize,
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.tuple_variant(len, visitor).map_err(DeError::wrap)
    }

    fn erased_struct_variant(
        self: Box<Self>,
        fields: &'static [&'static str],
        visitor: VisitableVisitor,
    ) -> Result<DeValue, DeError> {
        self.struct_variant(fields, visitor).map_err(DeError::wrap)
    }
}

pub struct GuestsideVariantAccessProvider {
    variant_access: Box<dyn ErasedVariantAccess>,
    _scope: ScopedBorrowMut<()>,
}

impl bindings::exports::serde::serde::serde_deserializer::GuestVariantAccess
    for GuestsideVariantAccessProvider
{
    fn unit_variant(
        this: bindings::exports::serde::serde::serde_deserializer::VariantAccess,
    ) -> Result<(), bindings::exports::serde::serde::serde_deserializer::DeError> {
        let Self { variant_access, .. } = this.into_inner();
        variant_access.erased_unit_variant().wrap()
    }

    fn newtype_variant_seed(
        this: bindings::exports::serde::serde::serde_deserializer::VariantAccess,
        seed: bindings::exports::serde::serde::serde_deserializer::OwnedDeserializeSeedHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let seed = unsafe {
            bindings::serde::serde::serde_deserialize::DeserializeSeed::from_handle(
                seed.owned_handle,
            )
        };

        let Self { variant_access, .. } = this.into_inner();

        variant_access
            .erased_newtype_variant_seed(DeserializableDeserializeSeed {
                deserialize_seed: seed,
            })
            .wrap()
    }

    fn tuple_variant(
        this: bindings::exports::serde::serde::serde_deserializer::VariantAccess,
        len: bindings::serde::serde::serde_types::Usize,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { variant_access, .. } = this.into_inner();

        variant_access
            .erased_tuple_variant(wit_to_usize(len.val), VisitableVisitor { visitor })
            .wrap()
    }

    fn struct_variant(
        this: bindings::exports::serde::serde::serde_deserializer::VariantAccess,
        fields: Vec<String>,
        visitor: bindings::exports::serde::serde::serde_deserializer::OwnedVisitorHandle,
    ) -> Result<
        bindings::exports::serde::serde::serde_deserializer::OwnedDeValueHandle,
        bindings::exports::serde::serde::serde_deserializer::DeError,
    > {
        // TODO: Safety
        let visitor = unsafe {
            bindings::serde::serde::serde_deserialize::Visitor::from_handle(visitor.owned_handle)
        };

        let Self { variant_access, .. } = this.into_inner();

        let fields = fields.into_iter().map(intern_string).collect();

        variant_access
            .erased_struct_variant(intern_str_list(fields), VisitableVisitor { visitor })
            .wrap()
    }
}

struct DeserializableDeserializeSeed {
    deserialize_seed: bindings::serde::serde::serde_deserialize::DeserializeSeed,
}

impl<'de> ::serde::de::DeserializeSeed<'de> for DeserializableDeserializeSeed {
    type Value = DeValue;

    fn deserialize<D: ::serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        unwrap_de_error(GuestsideDeserializerProvider::with_new(
            deserializer,
            |deserializer| {
                let deserializer =
                    bindings::exports::serde::serde::serde_deserializer::Deserializer::new(
                        deserializer,
                    );

                bindings::serde::serde::serde_deserialize::DeserializeSeed::deserialize(
                    self.deserialize_seed,
                    bindings::serde::serde::serde_deserialize::OwnedDeserializerHandle {
                        owned_handle: core::mem::ManuallyDrop::new(deserializer).handle(),
                    },
                )
            },
        ))
    }
}

struct VisitableVisitor {
    visitor: bindings::serde::serde::serde_deserialize::Visitor,
}

fn unwrap_de_error<E: ::serde::de::Error>(
    result: Result<
        bindings::serde::serde::serde_deserialize::DeValue,
        bindings::serde::serde::serde_deserialize::OwnedDeErrorHandle,
    >,
) -> Result<DeValue, E> {
    match result {
        Ok(value) => Ok(DeValue { value }),
        Err(err) => {
            // TODO: Safety
            let err = unsafe {
                bindings::exports::serde::serde::serde_deserializer::DeError::from_handle(
                    err.owned_handle,
                )
            };

            let DeError { inner: err } = err.into_inner();

            let err = match err {
                // Safety: TODO
                DeErrorVariants::Error { err, .. } => err,
                DeErrorVariants::Custom(msg) => return Err(::serde::de::Error::custom(msg)),
                DeErrorVariants::InvalidType { unexp, exp } => {
                    return Err(::serde::de::Error::invalid_type(
                        translate_serde_de_unexpected(&unexp),
                        &&*exp,
                    ))
                }
                DeErrorVariants::InvalidValue { unexp, exp } => {
                    return Err(::serde::de::Error::invalid_value(
                        translate_serde_de_unexpected(&unexp),
                        &&*exp,
                    ))
                }
                DeErrorVariants::InvalidLength { len, exp } => {
                    return Err(::serde::de::Error::invalid_length(
                        wit_to_usize(len),
                        &&*exp,
                    ))
                }
                DeErrorVariants::UnknownVariant { variant, expected } => {
                    return Err(::serde::de::Error::unknown_variant(&variant, expected))
                }
                DeErrorVariants::UnknownField { field, expected } => {
                    return Err(::serde::de::Error::unknown_field(&field, expected))
                }
                DeErrorVariants::MissingField { field } => {
                    return Err(::serde::de::Error::missing_field(field))
                }
                DeErrorVariants::DuplicateField { field } => {
                    return Err(::serde::de::Error::duplicate_field(field))
                }
            };

            // TODO: Safety
            let Some(err): Option<E> = (unsafe { err.take() }) else {
                return Err(::serde::de::Error::custom(
                    "bug: Deserializer::Error type mismatch across the wit boundary",
                ));
            };
            Err(err)
        }
    }
}

impl<'de> ::serde::de::Visitor<'de> for VisitableVisitor {
    type Value = DeValue;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&bindings::serde::serde::serde_deserialize::Visitor::expecting(&self.visitor))
    }

    fn visit_bool<E: ::serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_bool(self.visitor, v),
        )
    }

    fn visit_i8<E: ::serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_i8(self.visitor, v),
        )
    }

    fn visit_i16<E: ::serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_i16(self.visitor, v),
        )
    }

    fn visit_i32<E: ::serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_i32(self.visitor, v),
        )
    }

    fn visit_i64<E: ::serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_i64(self.visitor, v),
        )
    }

    serde_if_integer128! {
        fn visit_i128<E: ::serde::de::Error>(self, v: i128) -> Result<Self::Value, E> {
            let bytes = v.to_le_bytes();

            let le_hi = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let le_lo = [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ];

            unwrap_de_error(bindings::serde::serde::serde_deserialize::Visitor::visit_i128(self.visitor, bindings::serde::serde::serde_types::S128 {
                le_hi: u64::from_le_bytes(le_hi),
                le_lo: u64::from_le_bytes(le_lo),
            }))
        }
    }

    fn visit_u8<E: ::serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_u8(self.visitor, v),
        )
    }

    fn visit_u16<E: ::serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_u16(self.visitor, v),
        )
    }

    fn visit_u32<E: ::serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_u32(self.visitor, v),
        )
    }

    fn visit_u64<E: ::serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_u64(self.visitor, v),
        )
    }

    serde_if_integer128! {
        fn visit_u128<E: ::serde::de::Error>(self, v: u128) -> Result<Self::Value, E> {
            let bytes = v.to_le_bytes();

            let le_hi = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            let le_lo = [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ];

            unwrap_de_error(bindings::serde::serde::serde_deserialize::Visitor::visit_u128(self.visitor, bindings::serde::serde::serde_types::U128 {
                le_hi: u64::from_le_bytes(le_hi),
                le_lo: u64::from_le_bytes(le_lo),
            }))
        }
    }

    fn visit_f32<E: ::serde::de::Error>(self, v: f32) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_f32(self.visitor, v),
        )
    }

    fn visit_f64<E: ::serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_f64(self.visitor, v),
        )
    }

    fn visit_char<E: ::serde::de::Error>(self, v: char) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_char(self.visitor, v),
        )
    }

    fn visit_str<E: ::serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_string(self.visitor, v),
        )
    }

    fn visit_borrowed_str<E: ::serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_string(self.visitor, v),
        )
    }

    fn visit_string<E: ::serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_string(self.visitor, &v),
        )
    }

    fn visit_bytes<E: ::serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_byte_buf(self.visitor, v),
        )
    }

    fn visit_borrowed_bytes<E: ::serde::de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_byte_buf(self.visitor, v),
        )
    }

    fn visit_byte_buf<E: ::serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_byte_buf(self.visitor, &v),
        )
    }

    fn visit_none<E: ::serde::de::Error>(self) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_none(self.visitor),
        )
    }

    fn visit_some<D: ::serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        unwrap_de_error(GuestsideDeserializerProvider::with_new(
            deserializer,
            |deserializer| {
                let deserializer =
                    bindings::exports::serde::serde::serde_deserializer::Deserializer::new(
                        deserializer,
                    );

                bindings::serde::serde::serde_deserialize::Visitor::visit_some(
                    self.visitor,
                    bindings::serde::serde::serde_deserialize::OwnedDeserializerHandle {
                        owned_handle: core::mem::ManuallyDrop::new(deserializer).handle(),
                    },
                )
            },
        ))
    }

    fn visit_unit<E: ::serde::de::Error>(self) -> Result<Self::Value, E> {
        unwrap_de_error(
            bindings::serde::serde::serde_deserialize::Visitor::visit_unit(self.visitor),
        )
    }

    fn visit_newtype_struct<D: ::serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        unwrap_de_error(GuestsideDeserializerProvider::with_new(
            deserializer,
            |deserializer| {
                let deserializer =
                    bindings::exports::serde::serde::serde_deserializer::Deserializer::new(
                        deserializer,
                    );

                bindings::serde::serde::serde_deserialize::Visitor::visit_newtype_struct(
                    self.visitor,
                    bindings::serde::serde::serde_deserialize::OwnedDeserializerHandle {
                        owned_handle: core::mem::ManuallyDrop::new(deserializer).handle(),
                    },
                )
            },
        ))
    }

    fn visit_seq<A: ::serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        unwrap_de_error(GuestsideSeqAccessProvider::with_new(seq, |seq| {
            let seq = bindings::exports::serde::serde::serde_deserializer::SeqAccess::new(seq);

            bindings::serde::serde::serde_deserialize::Visitor::visit_seq(
                self.visitor,
                bindings::serde::serde::serde_deserialize::OwnedSeqAccessHandle {
                    owned_handle: core::mem::ManuallyDrop::new(seq).handle(),
                },
            )
        }))
    }

    fn visit_map<A: ::serde::de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        unwrap_de_error(GuestsideMapAccessProvider::with_new(
            map,
            |map: GuestsideMapAccessProvider| {
                let map = bindings::exports::serde::serde::serde_deserializer::MapAccess::new(map);

                bindings::serde::serde::serde_deserialize::Visitor::visit_map(
                    self.visitor,
                    bindings::serde::serde::serde_deserialize::OwnedMapAccessHandle {
                        owned_handle: core::mem::ManuallyDrop::new(map).handle(),
                    },
                )
            },
        ))
    }

    fn visit_enum<A: ::serde::de::EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        unwrap_de_error(GuestsideEnumAccessProvider::with_new(
            data,
            |data: GuestsideEnumAccessProvider| {
                let data =
                    bindings::exports::serde::serde::serde_deserializer::EnumAccess::new(data);

                bindings::serde::serde::serde_deserialize::Visitor::visit_enum(
                    self.visitor,
                    bindings::serde::serde::serde_deserialize::OwnedEnumAccessHandle {
                        owned_handle: core::mem::ManuallyDrop::new(data).handle(),
                    },
                )
            },
        ))
    }
}

struct DeValue {
    value: bindings::serde::serde::serde_deserialize::DeValue,
}

pub struct DeError {
    inner: DeErrorVariants,
}

enum DeErrorVariants {
    Error {
        err: Any,
        display: String,
        debug: String,
    },
    Custom(String),
    InvalidType {
        unexp: bindings::exports::serde::serde::serde_deserializer::Unexpected,
        exp: String,
    },
    InvalidValue {
        unexp: bindings::exports::serde::serde::serde_deserializer::Unexpected,
        exp: String,
    },
    InvalidLength {
        len: u32,
        exp: String,
    },
    UnknownVariant {
        variant: String,
        expected: &'static [&'static str],
    },
    UnknownField {
        field: String,
        expected: &'static [&'static str],
    },
    MissingField {
        field: &'static str,
    },
    DuplicateField {
        field: &'static str,
    },
}

fn translate_serde_de_unexpected(
    unexp: &bindings::exports::serde::serde::serde_deserializer::Unexpected,
) -> ::serde::de::Unexpected {
    match unexp {
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Bool(v) => {
            ::serde::de::Unexpected::Bool(*v)
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Unsigned(v) => {
            ::serde::de::Unexpected::Unsigned(*v)
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Signed(v) => {
            ::serde::de::Unexpected::Signed(*v)
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Float(v) => {
            ::serde::de::Unexpected::Float(*v)
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Char(v) => {
            ::serde::de::Unexpected::Char(*v)
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Str(v) => {
            ::serde::de::Unexpected::Str(v)
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Bytes(v) => {
            ::serde::de::Unexpected::Bytes(v)
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Unit => {
            ::serde::de::Unexpected::Unit
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Option => {
            ::serde::de::Unexpected::Option
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::NewtypeStruct => {
            ::serde::de::Unexpected::NewtypeStruct
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Seq => {
            ::serde::de::Unexpected::Seq
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Map => {
            ::serde::de::Unexpected::Map
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Enum => {
            ::serde::de::Unexpected::Enum
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::UnitVariant => {
            ::serde::de::Unexpected::UnitVariant
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::NewtypeVariant => {
            ::serde::de::Unexpected::NewtypeVariant
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::TupleVariant => {
            ::serde::de::Unexpected::TupleVariant
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::StructVariant => {
            ::serde::de::Unexpected::StructVariant
        }
        bindings::exports::serde::serde::serde_deserializer::Unexpected::Other(v) => {
            ::serde::de::Unexpected::Other(v)
        }
    }
}

impl DeError {
    fn wrap<T: ::serde::de::Error>(err: T) -> Self {
        let display = format!("{err}");
        let debug = format!("{err:?}");

        // Safety: TODO
        Self {
            inner: DeErrorVariants::Error {
                err: unsafe { Any::new(err) },
                display,
                debug,
            },
        }
    }
}

impl bindings::exports::serde::serde::serde_deserializer::GuestDeError for DeError {
    fn display(this: bindings::exports::serde::serde::serde_deserializer::DeErrorBorrow) -> String {
        match &this.get::<Self>().inner {
            DeErrorVariants::Error { display, .. } => String::from(display),
            DeErrorVariants::Custom(msg) => String::from(msg),
            DeErrorVariants::InvalidType { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("invalid type: {unexp}, expected {exp}")
            }
            DeErrorVariants::InvalidValue { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("invalid value: {unexp}, expected {exp}")
            }
            DeErrorVariants::InvalidLength { len, exp } => {
                format!("invalid length {len}, expected {exp}")
            }
            DeErrorVariants::UnknownVariant { variant, expected } => {
                format!(
                    "unknown variant `{variant}`, {}",
                    ExpectedOneOf {
                        expected,
                        kinds: "variants"
                    }
                )
            }
            DeErrorVariants::UnknownField { field, expected } => {
                format!(
                    "unknown field `{field}`, {}",
                    ExpectedOneOf {
                        expected,
                        kinds: "fields"
                    }
                )
            }
            DeErrorVariants::MissingField { field } => {
                format!("missing field `{field}`")
            }
            DeErrorVariants::DuplicateField { field } => {
                format!("duplicate field `{field}`")
            }
        }
    }

    fn debug(this: bindings::exports::serde::serde::serde_deserializer::DeErrorBorrow) -> String {
        match &this.get::<Self>().inner {
            DeErrorVariants::Error { debug, .. } => {
                format!("serde_wit::de::Error {{ err: {debug} }}")
            }
            DeErrorVariants::Custom(msg) => {
                format!("serde_wit::de::Error {{ err: Custom({msg}) }}")
            }
            DeErrorVariants::InvalidType { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("serde_wit::de::Error {{ err: InvalidType {{ unexp: {unexp:?}, exp: {exp:?} }} }}")
            }
            DeErrorVariants::InvalidValue { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("serde_wit::de::Error {{ err: InvalidValue {{ unexp: {unexp:?}, exp: {exp:?} }} }}")
            }
            DeErrorVariants::InvalidLength { len, exp } => {
                format!(
                    "serde_wit::de::Error {{ err: InvalidLength {{ len: {len}, exp: {exp:?} }} }}"
                )
            }
            DeErrorVariants::UnknownVariant { variant, expected } => {
                format!(
                    "serde_wit::de::Error {{ err: UnknownVariant {{ variant: {variant:?}, expected: {expected:?} }} }}"
                )
            }
            DeErrorVariants::UnknownField { field, expected } => {
                format!(
                    "serde_wit::de::Error {{ err: UnknownField {{ field: {field:?}, expected: {expected:?} }} }}"
                )
            }
            DeErrorVariants::MissingField { field } => {
                format!("serde_wit::de::Error {{ err: MissingField {{ field: {field:?} }} }}")
            }
            DeErrorVariants::DuplicateField { field } => {
                format!("serde_wit::de::Error {{ err: DuplicateField {{ field: {field:?} }} }}")
            }
        }
    }

    fn custom(msg: String) -> bindings::exports::serde::serde::serde_deserializer::DeError {
        bindings::exports::serde::serde::serde_deserializer::DeError::new(Self {
            inner: DeErrorVariants::Custom(msg),
        })
    }

    fn invalid_type(
        unexp: bindings::exports::serde::serde::serde_deserializer::Unexpected,
        exp: String,
    ) -> bindings::exports::serde::serde::serde_deserializer::DeError {
        bindings::exports::serde::serde::serde_deserializer::DeError::new(Self {
            inner: DeErrorVariants::InvalidType { unexp, exp },
        })
    }

    fn invalid_value(
        unexp: bindings::exports::serde::serde::serde_deserializer::Unexpected,
        exp: String,
    ) -> bindings::exports::serde::serde::serde_deserializer::DeError {
        bindings::exports::serde::serde::serde_deserializer::DeError::new(Self {
            inner: DeErrorVariants::InvalidValue { unexp, exp },
        })
    }

    fn invalid_length(
        len: bindings::serde::serde::serde_types::Usize,
        exp: String,
    ) -> bindings::exports::serde::serde::serde_deserializer::DeError {
        bindings::exports::serde::serde::serde_deserializer::DeError::new(Self {
            inner: DeErrorVariants::InvalidLength { len: len.val, exp },
        })
    }

    fn unknown_variant(
        variant: String,
        expected: Vec<String>,
    ) -> bindings::exports::serde::serde::serde_deserializer::DeError {
        let expected = expected.into_iter().map(intern_string).collect();

        bindings::exports::serde::serde::serde_deserializer::DeError::new(Self {
            inner: DeErrorVariants::UnknownVariant {
                variant,
                expected: intern_str_list(expected),
            },
        })
    }

    fn unknown_field(
        field: String,
        expected: Vec<String>,
    ) -> bindings::exports::serde::serde::serde_deserializer::DeError {
        let expected = expected.into_iter().map(intern_string).collect();

        bindings::exports::serde::serde::serde_deserializer::DeError::new(Self {
            inner: DeErrorVariants::UnknownField {
                field,
                expected: intern_str_list(expected),
            },
        })
    }

    fn missing_field(
        field: String,
    ) -> bindings::exports::serde::serde::serde_deserializer::DeError {
        bindings::exports::serde::serde::serde_deserializer::DeError::new(Self {
            inner: DeErrorVariants::MissingField {
                field: intern_string(field),
            },
        })
    }

    fn duplicate_field(
        field: String,
    ) -> bindings::exports::serde::serde::serde_deserializer::DeError {
        bindings::exports::serde::serde::serde_deserializer::DeError::new(Self {
            inner: DeErrorVariants::DuplicateField {
                field: intern_string(field),
            },
        })
    }
}

struct ExpectedOneOf {
    expected: &'static [&'static str],
    kinds: &'static str,
}

impl fmt::Display for ExpectedOneOf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.expected {
            [] => write!(fmt, "there are no {}", self.kinds),
            [one] => write!(fmt, "expected `{one}`"),
            [one, two] => write!(fmt, "expected `{one}` or `{two}`"),
            _ => {
                write!(fmt, "expected one of ")?;
                for (i, alt) in self.expected.iter().enumerate() {
                    if i > 0 {
                        write!(fmt, ", ")?;
                    }
                    write!(fmt, "`{alt}`")?;
                }
                Ok(())
            }
        }
    }
}
