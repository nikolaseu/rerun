// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/keypoint_id.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    /// \private
    template <typename T>
    class NumericBuilder;

    class DataType;
    class MemoryPool;
    class UInt16Type;
    using UInt16Builder = NumericBuilder<UInt16Type>;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: A 16-bit ID representing a type of semantic keypoint within a class.
    struct KeypointId {
        uint16_t id;

      public:
        KeypointId() = default;

        KeypointId(uint16_t id_) : id(id_) {}

        KeypointId& operator=(uint16_t id_) {
            id = id_;
            return *this;
        }

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Creates a new array builder with an array of this type.
        static Result<std::shared_ptr<arrow::UInt16Builder>> new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::UInt16Builder* builder, const KeypointId* elements, size_t num_elements
        );
    };
} // namespace rerun::datatypes
