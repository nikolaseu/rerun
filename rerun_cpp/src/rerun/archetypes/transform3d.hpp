// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/transform3d.fbs".

#pragma once

#include "../arrow.hpp"
#include "../component_batch.hpp"
#include "../components/transform3d.hpp"
#include "../data_cell.hpp"
#include "../result.hpp"

#include <cstdint>
#include <utility>
#include <vector>

namespace rerun {
    namespace archetypes {
        /// A 3D transform.
        ///
        /// ## Example
        ///
        /// ```cpp,ignore
        /// // Log some transforms.
        ///
        /// #include <rerun.hpp>
        ///
        /// #include <cmath>
        ///
        /// namespace rr = rerun;
        /// namespace rrd = rr::datatypes;
        ///
        /// const float pi = static_cast<float>(M_PI);
        ///
        /// int main() {
        ///     auto rec = rr::RecordingStream("rerun_example_transform3d");
        ///     rec.connect("127.0.0.1:9876").throw_on_failure();
        ///
        ///     auto arrow = rr::Arrows3D::from_vectors({0.0f, 1.0f, 0.0f}).with_origins({0.0f,
        ///     0.0f, 0.0f});
        ///
        ///     rec.log("base", arrow);
        ///
        ///     rec.log("base/translated", rr::Transform3D({1.0f, 0.0f, 0.0f}));
        ///     rec.log("base/translated", arrow);
        ///
        ///     rec.log(
        ///         "base/rotated_scaled",
        ///         rr::Transform3D(
        ///             rrd::RotationAxisAngle({0.0f, 0.0f, 1.0f}, rrd::Angle::radians(pi / 4.0f)),
        ///             2.0f
        ///         )
        ///     );
        ///     rec.log("base/rotated_scaled", arrow);
        /// }
        /// ```
        struct Transform3D {
            /// The transform
            rerun::components::Transform3D transform;

            /// Name of the indicator component, used to identify the archetype when converting to a
            /// list of components.
            static const char INDICATOR_COMPONENT_NAME[];

          public:
            // Extensions to generated type defined in 'transform3d_ext.cpp'

            static const Transform3D IDENTITY;

            /// New 3D transform from translation/matrix datatype.
            Transform3D(const datatypes::TranslationAndMat3x3& translation_and_mat3x3)
                : Transform3D(datatypes::Transform3D::translation_and_mat3x3(translation_and_mat3x3)
                  ) {}

            /// Creates a new 3D transform from translation and matrix provided as 3 columns.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            ///
            /// Implementation note: This overload is necessary, otherwise the array may be
            /// interpreted as bool and call the wrong overload.
            Transform3D(
                const datatypes::Vec3D& translation, const datatypes::Vec3D (&columns)[3],
                bool from_parent = false
            )
                : Transform3D(datatypes::TranslationAndMat3x3(translation, columns, from_parent)) {}

            /// Creates a new 3D transform from translation/matrix.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(
                const datatypes::Vec3D& translation, const datatypes::Mat3x3& matrix,
                bool from_parent = false
            )
                : Transform3D(datatypes::TranslationAndMat3x3(translation, matrix, from_parent)) {}

            /// From translation only.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(const datatypes::Vec3D& translation, bool from_parent = false)
                : Transform3D(datatypes::TranslationAndMat3x3(translation, from_parent)) {}

            /// From 3x3 matrix only.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(const datatypes::Mat3x3& matrix, bool from_parent = false)
                : Transform3D(datatypes::TranslationAndMat3x3(matrix, from_parent)) {}

            /// From 3x3 matrix provided as 3 columns only.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(const datatypes::Vec3D (&columns)[3], bool from_parent = false)
                : Transform3D(datatypes::TranslationAndMat3x3(columns, from_parent)) {}

            /// New 3D transform from translation/rotation/scale datatype.
            Transform3D(const datatypes::TranslationRotationScale3D& translation_rotation_scale3d)
                : Transform3D(datatypes::Transform3D::translation_rotation_scale(
                      translation_rotation_scale3d
                  )) {}

            /// Creates a new 3D transform from translation/rotation/scale.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(
                const datatypes::Vec3D& translation, const datatypes::Rotation3D& rotation,
                const datatypes::Scale3D& scale, bool from_parent = false
            )
                : Transform3D(datatypes::TranslationRotationScale3D(
                      translation, rotation, scale, from_parent
                  )) {}

            /// Creates a new 3D transform from translation/rotation/uniform-scale.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            ///
            /// Implementation note: This explicit overload prevents interpretation of the float as
            /// bool, leading to a call to the wrong overload.
            Transform3D(
                const datatypes::Vec3D& translation, const datatypes::Rotation3D& rotation,
                float uniform_scale, bool from_parent = false
            )
                : Transform3D(datatypes::TranslationRotationScale3D(
                      translation, rotation, uniform_scale, from_parent
                  )) {}

            /// Creates a new rigid transform (translation & rotation only).
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(
                const datatypes::Vec3D& translation, const datatypes::Rotation3D& rotation,
                bool from_parent = false
            )
                : Transform3D(
                      datatypes::TranslationRotationScale3D(translation, rotation, from_parent)
                  ) {}

            /// From translation & scale only.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(
                const datatypes::Vec3D& translation, const datatypes::Scale3D& scale,
                bool from_parent = false
            )
                : Transform3D(datatypes::TranslationRotationScale3D(translation, scale, from_parent)
                  ) {}

            /// From translation & uniform scale only.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            ///
            /// Implementation note: This explicit overload prevents interpretation of the float as
            /// bool, leading to a call to the wrong overload.
            Transform3D(
                const datatypes::Vec3D& translation, float uniform_scale, bool from_parent = false
            )
                : Transform3D(
                      datatypes::TranslationRotationScale3D(translation, uniform_scale, from_parent)
                  ) {}

            /// From rotation & scale.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(
                const datatypes::Rotation3D& rotation, const datatypes::Scale3D& scale,
                bool from_parent = false
            )
                : Transform3D(datatypes::TranslationRotationScale3D(rotation, scale, from_parent)) {
            }

            /// From rotation & uniform scale.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            ///
            /// Implementation note: This explicit overload prevents interpretation of the float as
            /// bool, leading to a call to the wrong overload.
            Transform3D(
                const datatypes::Rotation3D& rotation, float uniform_scale, bool from_parent = false
            )
                : Transform3D(
                      datatypes::TranslationRotationScale3D(rotation, uniform_scale, from_parent)
                  ) {}

            /// From rotation only.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(const datatypes::Rotation3D& rotation, bool from_parent = false)
                : Transform3D(datatypes::TranslationRotationScale3D(rotation, from_parent)) {}

            /// From scale only.
            ///
            /// @param from_parent If true, the transform maps from the parent space to the space
            /// where the transform was logged. Otherwise, the transform maps from the space to its
            /// parent.
            Transform3D(const datatypes::Scale3D& scale, bool from_parent = false)
                : Transform3D(datatypes::TranslationRotationScale3D(scale, from_parent)) {}

          public:
            Transform3D() = default;

            Transform3D(rerun::components::Transform3D _transform)
                : transform(std::move(_transform)) {}

            /// Returns the number of primary instances of this archetype.
            size_t num_instances() const {
                return 1;
            }

            /// Collections all component lists into a list of component collections. *Attention:*
            /// The returned vector references this instance and does not take ownership of any
            /// data. Adding any new components to this archetype will invalidate the returned
            /// component lists!
            std::vector<AnonymousComponentBatch> as_component_batches() const;
        };
    } // namespace archetypes
} // namespace rerun
