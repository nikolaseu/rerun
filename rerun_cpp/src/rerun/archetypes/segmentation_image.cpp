// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/segmentation_image.fbs".

#include "segmentation_image.hpp"

#include "../component_batch_adapter_builtins.hpp"

namespace rerun::archetypes {
    const char SegmentationImage::INDICATOR_COMPONENT_NAME[] =
        "rerun.components.SegmentationImageIndicator";
}

namespace rerun {

    Result<std::vector<SerializedComponentBatch>> AsComponents<
        archetypes::SegmentationImage>::serialize(const archetypes::SegmentationImage& archetype) {
        using namespace archetypes;
        std::vector<SerializedComponentBatch> cells;
        cells.reserve(2);

        {
            auto result = ComponentBatch<rerun::components::TensorData>(archetype.data).serialize();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.draw_order.has_value()) {
            auto result = ComponentBatch<rerun::components::DrawOrder>(archetype.draw_order.value())
                              .serialize();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        {
            auto result = ComponentBatch<SegmentationImage::IndicatorComponent>(
                              SegmentationImage::IndicatorComponent()
            )
                              .serialize();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
