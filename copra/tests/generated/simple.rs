// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Simple {
    // message fields
    pub int_val: i32,
    pub bool_val: bool,
    pub str_val: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Simple {}

impl Simple {
    pub fn new() -> Simple {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Simple {
        static mut instance: ::protobuf::lazy::Lazy<Simple> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Simple,
        };
        unsafe {
            instance.get(Simple::new)
        }
    }

    // int32 int_val = 1;

    pub fn clear_int_val(&mut self) {
        self.int_val = 0;
    }

    // Param is passed by value, moved
    pub fn set_int_val(&mut self, v: i32) {
        self.int_val = v;
    }

    pub fn get_int_val(&self) -> i32 {
        self.int_val
    }

    fn get_int_val_for_reflect(&self) -> &i32 {
        &self.int_val
    }

    fn mut_int_val_for_reflect(&mut self) -> &mut i32 {
        &mut self.int_val
    }

    // bool bool_val = 2;

    pub fn clear_bool_val(&mut self) {
        self.bool_val = false;
    }

    // Param is passed by value, moved
    pub fn set_bool_val(&mut self, v: bool) {
        self.bool_val = v;
    }

    pub fn get_bool_val(&self) -> bool {
        self.bool_val
    }

    fn get_bool_val_for_reflect(&self) -> &bool {
        &self.bool_val
    }

    fn mut_bool_val_for_reflect(&mut self) -> &mut bool {
        &mut self.bool_val
    }

    // string str_val = 3;

    pub fn clear_str_val(&mut self) {
        self.str_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_str_val(&mut self, v: ::std::string::String) {
        self.str_val = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_str_val(&mut self) -> &mut ::std::string::String {
        &mut self.str_val
    }

    // Take field
    pub fn take_str_val(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.str_val, ::std::string::String::new())
    }

    pub fn get_str_val(&self) -> &str {
        &self.str_val
    }

    fn get_str_val_for_reflect(&self) -> &::std::string::String {
        &self.str_val
    }

    fn mut_str_val_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.str_val
    }
}

impl ::protobuf::Message for Simple {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.int_val = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.bool_val = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.str_val)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.int_val != 0 {
            my_size += ::protobuf::rt::value_size(1, self.int_val, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bool_val != false {
            my_size += 2;
        }
        if !self.str_val.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.str_val);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.int_val != 0 {
            os.write_int32(1, self.int_val)?;
        }
        if self.bool_val != false {
            os.write_bool(2, self.bool_val)?;
        }
        if !self.str_val.is_empty() {
            os.write_string(3, &self.str_val)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Simple {
    fn new() -> Simple {
        Simple::new()
    }

    fn descriptor_static(_: ::std::option::Option<Simple>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "int_val",
                    Simple::get_int_val_for_reflect,
                    Simple::mut_int_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool_val",
                    Simple::get_bool_val_for_reflect,
                    Simple::mut_bool_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "str_val",
                    Simple::get_str_val_for_reflect,
                    Simple::mut_str_val_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Simple>(
                    "Simple",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Simple {
    fn clear(&mut self) {
        self.clear_int_val();
        self.clear_bool_val();
        self.clear_str_val();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Simple {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Simple {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fcopra/tests/protos/simple.proto\"U\n\x06Simple\x12\x17\n\x07int_va\
    l\x18\x01\x20\x01(\x05R\x06intVal\x12\x19\n\x08bool_val\x18\x02\x20\x01(\
    \x08R\x07boolVal\x12\x17\n\x07str_val\x18\x03\x20\x01(\tR\x06strVal2\x20\
    \n\x04Echo\x12\x18\n\x04echo\x12\x07.Simple\x1a\x07.Simpleb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
