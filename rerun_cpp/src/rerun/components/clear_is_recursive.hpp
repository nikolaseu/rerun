// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/clear_is_recursive.fbs".

#pragma once

#include "../data_cell.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    class BooleanBuilder;
    class DataType;
    class MemoryPool;
} // namespace arrow

namespace rerun::components {
    /// **Component**: Configures how a clear operation should behave - recursive or not.
    struct ClearIsRecursive {
        /// If true, also clears all recursive children entities.
        bool recursive;

        /// Name of the component, used for serialization.
        static const char NAME[];

      public:
        ClearIsRecursive() = default;

        ClearIsRecursive(bool recursive_) : recursive(recursive_) {}

        ClearIsRecursive& operator=(bool recursive_) {
            recursive = recursive_;
            return *this;
        }

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Creates a new array builder with an array of this type.
        static Result<std::shared_ptr<arrow::BooleanBuilder>> new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::BooleanBuilder* builder, const ClearIsRecursive* elements, size_t num_elements
        );

        /// Creates a Rerun DataCell from an array of ClearIsRecursive components.
        static Result<rerun::DataCell> to_data_cell(
            const ClearIsRecursive* instances, size_t num_instances
        );
    };
} // namespace rerun::components
