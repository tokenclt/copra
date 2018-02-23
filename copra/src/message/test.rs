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
pub struct TestMessage {
    // message fields
    pub string_field: ::std::string::String,
    pub i32_field: i32,
    pub float_field: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestMessage {}

impl TestMessage {
    pub fn new() -> TestMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestMessage {
        static mut instance: ::protobuf::lazy::Lazy<TestMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestMessage,
        };
        unsafe {
            instance.get(TestMessage::new)
        }
    }

    // string string_field = 1;

    pub fn clear_string_field(&mut self) {
        self.string_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_field(&mut self, v: ::std::string::String) {
        self.string_field = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_field(&mut self) -> &mut ::std::string::String {
        &mut self.string_field
    }

    // Take field
    pub fn take_string_field(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.string_field, ::std::string::String::new())
    }

    pub fn get_string_field(&self) -> &str {
        &self.string_field
    }

    fn get_string_field_for_reflect(&self) -> &::std::string::String {
        &self.string_field
    }

    fn mut_string_field_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.string_field
    }

    // int32 i32_field = 2;

    pub fn clear_i32_field(&mut self) {
        self.i32_field = 0;
    }

    // Param is passed by value, moved
    pub fn set_i32_field(&mut self, v: i32) {
        self.i32_field = v;
    }

    pub fn get_i32_field(&self) -> i32 {
        self.i32_field
    }

    fn get_i32_field_for_reflect(&self) -> &i32 {
        &self.i32_field
    }

    fn mut_i32_field_for_reflect(&mut self) -> &mut i32 {
        &mut self.i32_field
    }

    // float float_field = 3;

    pub fn clear_float_field(&mut self) {
        self.float_field = 0.;
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: f32) {
        self.float_field = v;
    }

    pub fn get_float_field(&self) -> f32 {
        self.float_field
    }

    fn get_float_field_for_reflect(&self) -> &f32 {
        &self.float_field
    }

    fn mut_float_field_for_reflect(&mut self) -> &mut f32 {
        &mut self.float_field
    }
}

impl ::protobuf::Message for TestMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.string_field)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.i32_field = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.float_field = tmp;
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
        if !self.string_field.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.string_field);
        }
        if self.i32_field != 0 {
            my_size += ::protobuf::rt::value_size(2, self.i32_field, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.float_field != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.string_field.is_empty() {
            os.write_string(1, &self.string_field)?;
        }
        if self.i32_field != 0 {
            os.write_int32(2, self.i32_field)?;
        }
        if self.float_field != 0. {
            os.write_float(3, self.float_field)?;
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

impl ::protobuf::MessageStatic for TestMessage {
    fn new() -> TestMessage {
        TestMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string_field",
                    TestMessage::get_string_field_for_reflect,
                    TestMessage::mut_string_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "i32_field",
                    TestMessage::get_i32_field_for_reflect,
                    TestMessage::mut_i32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_field",
                    TestMessage::get_float_field_for_reflect,
                    TestMessage::mut_float_field_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestMessage>(
                    "TestMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestMessage {
    fn clear(&mut self) {
        self.clear_string_field();
        self.clear_i32_field();
        self.clear_float_field();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16src/message/test.proto\"n\n\x0bTestMessage\x12!\n\x0cstring_field\
    \x18\x01\x20\x01(\tR\x0bstringField\x12\x1b\n\ti32_field\x18\x02\x20\x01\
    (\x05R\x08i32Field\x12\x1f\n\x0bfloat_field\x18\x03\x20\x01(\x02R\nfloat\
    Fieldb\x06proto3\
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
