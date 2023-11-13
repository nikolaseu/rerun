// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/text_document.fbs".

#include "text_document.hpp"

#include "../component_batch_adapter_builtins.hpp"

namespace rerun::archetypes {
    const char TextDocument::INDICATOR_COMPONENT_NAME[] = "rerun.components.TextDocumentIndicator";
}

namespace rerun {

    Result<std::vector<SerializedComponentBatch>> AsComponents<archetypes::TextDocument>::serialize(
        const archetypes::TextDocument& archetype
    ) {
        using namespace archetypes;
        std::vector<SerializedComponentBatch> cells;
        cells.reserve(2);

        {
            auto result = ComponentBatch<rerun::components::Text>(archetype.text).serialize();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.media_type.has_value()) {
            auto result = ComponentBatch<rerun::components::MediaType>(archetype.media_type.value())
                              .serialize();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        {
            auto result =
                ComponentBatch<TextDocument::IndicatorComponent>(TextDocument::IndicatorComponent())
                    .serialize();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
