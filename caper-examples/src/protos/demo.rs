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
pub struct GreetMessage {
    // message fields
    pub msg: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GreetMessage {}

impl GreetMessage {
    pub fn new() -> GreetMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GreetMessage {
        static mut instance: ::protobuf::lazy::Lazy<GreetMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GreetMessage,
        };
        unsafe {
            instance.get(GreetMessage::new)
        }
    }

    // string msg = 1;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }
}

impl ::protobuf::Message for GreetMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
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
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.msg.is_empty() {
            os.write_string(1, &self.msg)?;
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

impl ::protobuf::MessageStatic for GreetMessage {
    fn new() -> GreetMessage {
        GreetMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GreetMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    GreetMessage::get_msg_for_reflect,
                    GreetMessage::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GreetMessage>(
                    "GreetMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GreetMessage {
    fn clear(&mut self) {
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GreetMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GreetMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrimeRequest {
    // message fields
    pub number: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrimeRequest {}

impl PrimeRequest {
    pub fn new() -> PrimeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrimeRequest {
        static mut instance: ::protobuf::lazy::Lazy<PrimeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrimeRequest,
        };
        unsafe {
            instance.get(PrimeRequest::new)
        }
    }

    // uint64 number = 1;

    pub fn clear_number(&mut self) {
        self.number = 0;
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: u64) {
        self.number = v;
    }

    pub fn get_number(&self) -> u64 {
        self.number
    }

    fn get_number_for_reflect(&self) -> &u64 {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut u64 {
        &mut self.number
    }
}

impl ::protobuf::Message for PrimeRequest {
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
                    let tmp = is.read_uint64()?;
                    self.number = tmp;
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
        if self.number != 0 {
            my_size += ::protobuf::rt::value_size(1, self.number, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.number != 0 {
            os.write_uint64(1, self.number)?;
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

impl ::protobuf::MessageStatic for PrimeRequest {
    fn new() -> PrimeRequest {
        PrimeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrimeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "number",
                    PrimeRequest::get_number_for_reflect,
                    PrimeRequest::mut_number_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrimeRequest>(
                    "PrimeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrimeRequest {
    fn clear(&mut self) {
        self.clear_number();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrimeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrimeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrimeResponse {
    // message fields
    pub number: u64,
    pub is_prime: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrimeResponse {}

impl PrimeResponse {
    pub fn new() -> PrimeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrimeResponse {
        static mut instance: ::protobuf::lazy::Lazy<PrimeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrimeResponse,
        };
        unsafe {
            instance.get(PrimeResponse::new)
        }
    }

    // uint64 number = 1;

    pub fn clear_number(&mut self) {
        self.number = 0;
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: u64) {
        self.number = v;
    }

    pub fn get_number(&self) -> u64 {
        self.number
    }

    fn get_number_for_reflect(&self) -> &u64 {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut u64 {
        &mut self.number
    }

    // bool is_prime = 2;

    pub fn clear_is_prime(&mut self) {
        self.is_prime = false;
    }

    // Param is passed by value, moved
    pub fn set_is_prime(&mut self, v: bool) {
        self.is_prime = v;
    }

    pub fn get_is_prime(&self) -> bool {
        self.is_prime
    }

    fn get_is_prime_for_reflect(&self) -> &bool {
        &self.is_prime
    }

    fn mut_is_prime_for_reflect(&mut self) -> &mut bool {
        &mut self.is_prime
    }
}

impl ::protobuf::Message for PrimeResponse {
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
                    let tmp = is.read_uint64()?;
                    self.number = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_prime = tmp;
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
        if self.number != 0 {
            my_size += ::protobuf::rt::value_size(1, self.number, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.is_prime != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.number != 0 {
            os.write_uint64(1, self.number)?;
        }
        if self.is_prime != false {
            os.write_bool(2, self.is_prime)?;
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

impl ::protobuf::MessageStatic for PrimeResponse {
    fn new() -> PrimeResponse {
        PrimeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrimeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "number",
                    PrimeResponse::get_number_for_reflect,
                    PrimeResponse::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_prime",
                    PrimeResponse::get_is_prime_for_reflect,
                    PrimeResponse::mut_is_prime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrimeResponse>(
                    "PrimeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrimeResponse {
    fn clear(&mut self) {
        self.clear_number();
        self.clear_is_prime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrimeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrimeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15src/protos/demo.proto\"\x20\n\x0cGreetMessage\x12\x10\n\x03msg\x18\
    \x01\x20\x01(\tR\x03msg\"&\n\x0cPrimeRequest\x12\x16\n\x06number\x18\x01\
    \x20\x01(\x04R\x06number\"B\n\rPrimeResponse\x12\x16\n\x06number\x18\x01\
    \x20\x01(\x04R\x06number\x12\x19\n\x08is_prime\x18\x02\x20\x01(\x08R\x07\
    isPrime2[\n\x04Demo\x12(\n\x08greet_to\x12\r.GreetMessage\x1a\r.GreetMes\
    sage\x12)\n\x08is_prime\x12\r.PrimeRequest\x1a\x0e.PrimeResponseb\x06pro\
    to3\
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
