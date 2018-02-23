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
pub struct RpcMeta {
    // message fields
    pub request: ::protobuf::SingularPtrField<RpcRequestMeta>,
    pub response: ::protobuf::SingularPtrField<RpcResponseMeta>,
    pub correlation_id: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcMeta {}

impl RpcMeta {
    pub fn new() -> RpcMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcMeta {
        static mut instance: ::protobuf::lazy::Lazy<RpcMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcMeta,
        };
        unsafe {
            instance.get(RpcMeta::new)
        }
    }

    // .RpcRequestMeta request = 1;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: RpcRequestMeta) {
        self.request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut RpcRequestMeta {
        if self.request.is_none() {
            self.request.set_default();
        }
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> RpcRequestMeta {
        self.request.take().unwrap_or_else(|| RpcRequestMeta::new())
    }

    pub fn get_request(&self) -> &RpcRequestMeta {
        self.request.as_ref().unwrap_or_else(|| RpcRequestMeta::default_instance())
    }

    fn get_request_for_reflect(&self) -> &::protobuf::SingularPtrField<RpcRequestMeta> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RpcRequestMeta> {
        &mut self.request
    }

    // .RpcResponseMeta response = 2;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: RpcResponseMeta) {
        self.response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut RpcResponseMeta {
        if self.response.is_none() {
            self.response.set_default();
        }
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> RpcResponseMeta {
        self.response.take().unwrap_or_else(|| RpcResponseMeta::new())
    }

    pub fn get_response(&self) -> &RpcResponseMeta {
        self.response.as_ref().unwrap_or_else(|| RpcResponseMeta::default_instance())
    }

    fn get_response_for_reflect(&self) -> &::protobuf::SingularPtrField<RpcResponseMeta> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RpcResponseMeta> {
        &mut self.response
    }

    // uint64 correlation_id = 4;

    pub fn clear_correlation_id(&mut self) {
        self.correlation_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_correlation_id(&mut self, v: u64) {
        self.correlation_id = v;
    }

    pub fn get_correlation_id(&self) -> u64 {
        self.correlation_id
    }

    fn get_correlation_id_for_reflect(&self) -> &u64 {
        &self.correlation_id
    }

    fn mut_correlation_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.correlation_id
    }
}

impl ::protobuf::Message for RpcMeta {
    fn is_initialized(&self) -> bool {
        for v in &self.request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.correlation_id = tmp;
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
        if let Some(ref v) = self.request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.correlation_id != 0 {
            my_size += ::protobuf::rt::value_size(4, self.correlation_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.request.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.response.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.correlation_id != 0 {
            os.write_uint64(4, self.correlation_id)?;
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

impl ::protobuf::MessageStatic for RpcMeta {
    fn new() -> RpcMeta {
        RpcMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpcRequestMeta>>(
                    "request",
                    RpcMeta::get_request_for_reflect,
                    RpcMeta::mut_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpcResponseMeta>>(
                    "response",
                    RpcMeta::get_response_for_reflect,
                    RpcMeta::mut_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "correlation_id",
                    RpcMeta::get_correlation_id_for_reflect,
                    RpcMeta::mut_correlation_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcMeta>(
                    "RpcMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcMeta {
    fn clear(&mut self) {
        self.clear_request();
        self.clear_response();
        self.clear_correlation_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcRequestMeta {
    // message fields
    pub service_name: ::std::string::String,
    pub method_name: ::std::string::String,
    pub log_id: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcRequestMeta {}

impl RpcRequestMeta {
    pub fn new() -> RpcRequestMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcRequestMeta {
        static mut instance: ::protobuf::lazy::Lazy<RpcRequestMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcRequestMeta,
        };
        unsafe {
            instance.get(RpcRequestMeta::new)
        }
    }

    // string service_name = 1;

    pub fn clear_service_name(&mut self) {
        self.service_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_name(&mut self, v: ::std::string::String) {
        self.service_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_name(&mut self) -> &mut ::std::string::String {
        &mut self.service_name
    }

    // Take field
    pub fn take_service_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service_name, ::std::string::String::new())
    }

    pub fn get_service_name(&self) -> &str {
        &self.service_name
    }

    fn get_service_name_for_reflect(&self) -> &::std::string::String {
        &self.service_name
    }

    fn mut_service_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.service_name
    }

    // string method_name = 2;

    pub fn clear_method_name(&mut self) {
        self.method_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_method_name(&mut self, v: ::std::string::String) {
        self.method_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method_name(&mut self) -> &mut ::std::string::String {
        &mut self.method_name
    }

    // Take field
    pub fn take_method_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.method_name, ::std::string::String::new())
    }

    pub fn get_method_name(&self) -> &str {
        &self.method_name
    }

    fn get_method_name_for_reflect(&self) -> &::std::string::String {
        &self.method_name
    }

    fn mut_method_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.method_name
    }

    // int64 log_id = 3;

    pub fn clear_log_id(&mut self) {
        self.log_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_log_id(&mut self, v: i64) {
        self.log_id = v;
    }

    pub fn get_log_id(&self) -> i64 {
        self.log_id
    }

    fn get_log_id_for_reflect(&self) -> &i64 {
        &self.log_id
    }

    fn mut_log_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.log_id
    }
}

impl ::protobuf::Message for RpcRequestMeta {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.method_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.log_id = tmp;
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
        if !self.service_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.service_name);
        }
        if !self.method_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.method_name);
        }
        if self.log_id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.log_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.service_name.is_empty() {
            os.write_string(1, &self.service_name)?;
        }
        if !self.method_name.is_empty() {
            os.write_string(2, &self.method_name)?;
        }
        if self.log_id != 0 {
            os.write_int64(3, self.log_id)?;
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

impl ::protobuf::MessageStatic for RpcRequestMeta {
    fn new() -> RpcRequestMeta {
        RpcRequestMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcRequestMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service_name",
                    RpcRequestMeta::get_service_name_for_reflect,
                    RpcRequestMeta::mut_service_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method_name",
                    RpcRequestMeta::get_method_name_for_reflect,
                    RpcRequestMeta::mut_method_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "log_id",
                    RpcRequestMeta::get_log_id_for_reflect,
                    RpcRequestMeta::mut_log_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcRequestMeta>(
                    "RpcRequestMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcRequestMeta {
    fn clear(&mut self) {
        self.clear_service_name();
        self.clear_method_name();
        self.clear_log_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcRequestMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcRequestMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcResponseMeta {
    // message fields
    pub error_code: i32,
    pub error_text: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcResponseMeta {}

impl RpcResponseMeta {
    pub fn new() -> RpcResponseMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcResponseMeta {
        static mut instance: ::protobuf::lazy::Lazy<RpcResponseMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcResponseMeta,
        };
        unsafe {
            instance.get(RpcResponseMeta::new)
        }
    }

    // int32 error_code = 1;

    pub fn clear_error_code(&mut self) {
        self.error_code = 0;
    }

    // Param is passed by value, moved
    pub fn set_error_code(&mut self, v: i32) {
        self.error_code = v;
    }

    pub fn get_error_code(&self) -> i32 {
        self.error_code
    }

    fn get_error_code_for_reflect(&self) -> &i32 {
        &self.error_code
    }

    fn mut_error_code_for_reflect(&mut self) -> &mut i32 {
        &mut self.error_code
    }

    // string error_text = 2;

    pub fn clear_error_text(&mut self) {
        self.error_text.clear();
    }

    // Param is passed by value, moved
    pub fn set_error_text(&mut self, v: ::std::string::String) {
        self.error_text = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_text(&mut self) -> &mut ::std::string::String {
        &mut self.error_text
    }

    // Take field
    pub fn take_error_text(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error_text, ::std::string::String::new())
    }

    pub fn get_error_text(&self) -> &str {
        &self.error_text
    }

    fn get_error_text_for_reflect(&self) -> &::std::string::String {
        &self.error_text
    }

    fn mut_error_text_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error_text
    }
}

impl ::protobuf::Message for RpcResponseMeta {
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
                    self.error_code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error_text)?;
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
        if self.error_code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.error_code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.error_text.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error_text);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.error_code != 0 {
            os.write_int32(1, self.error_code)?;
        }
        if !self.error_text.is_empty() {
            os.write_string(2, &self.error_text)?;
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

impl ::protobuf::MessageStatic for RpcResponseMeta {
    fn new() -> RpcResponseMeta {
        RpcResponseMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcResponseMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "error_code",
                    RpcResponseMeta::get_error_code_for_reflect,
                    RpcResponseMeta::mut_error_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_text",
                    RpcResponseMeta::get_error_text_for_reflect,
                    RpcResponseMeta::mut_error_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcResponseMeta>(
                    "RpcResponseMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcResponseMeta {
    fn clear(&mut self) {
        self.clear_error_code();
        self.clear_error_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcResponseMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcResponseMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16src/message/meta.proto\"\x89\x01\n\x07RpcMeta\x12)\n\x07request\
    \x18\x01\x20\x01(\x0b2\x0f.RpcRequestMetaR\x07request\x12,\n\x08response\
    \x18\x02\x20\x01(\x0b2\x10.RpcResponseMetaR\x08response\x12%\n\x0ecorrel\
    ation_id\x18\x04\x20\x01(\x04R\rcorrelationId\"k\n\x0eRpcRequestMeta\x12\
    !\n\x0cservice_name\x18\x01\x20\x01(\tR\x0bserviceName\x12\x1f\n\x0bmeth\
    od_name\x18\x02\x20\x01(\tR\nmethodName\x12\x15\n\x06log_id\x18\x03\x20\
    \x01(\x03R\x05logId\"O\n\x0fRpcResponseMeta\x12\x1d\n\nerror_code\x18\
    \x01\x20\x01(\x05R\terrorCode\x12\x1d\n\nerror_text\x18\x02\x20\x01(\tR\
    \terrorTextb\x06proto3\
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
