# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from .affix_fuzzer1 import (
        AffixFuzzer1,
        AffixFuzzer1ArrayLike,
        AffixFuzzer1Batch,
        AffixFuzzer1Like,
        AffixFuzzer1Type,
    )
    from .affix_fuzzer2 import (
        AffixFuzzer2,
        AffixFuzzer2ArrayLike,
        AffixFuzzer2Batch,
        AffixFuzzer2Like,
        AffixFuzzer2Type,
    )
    from .affix_fuzzer3 import (
        AffixFuzzer3,
        AffixFuzzer3ArrayLike,
        AffixFuzzer3Batch,
        AffixFuzzer3Like,
        AffixFuzzer3Type,
    )
    from .affix_fuzzer4 import (
        AffixFuzzer4,
        AffixFuzzer4ArrayLike,
        AffixFuzzer4Batch,
        AffixFuzzer4Like,
        AffixFuzzer4Type,
    )
    from .affix_fuzzer5 import (
        AffixFuzzer5,
        AffixFuzzer5ArrayLike,
        AffixFuzzer5Batch,
        AffixFuzzer5Like,
        AffixFuzzer5Type,
    )
    from .affix_fuzzer20 import (
        AffixFuzzer20,
        AffixFuzzer20ArrayLike,
        AffixFuzzer20Batch,
        AffixFuzzer20Like,
        AffixFuzzer20Type,
    )
    from .affix_fuzzer21 import (
        AffixFuzzer21,
        AffixFuzzer21ArrayLike,
        AffixFuzzer21Batch,
        AffixFuzzer21Like,
        AffixFuzzer21Type,
    )
    from .flattened_scalar import (
        FlattenedScalar,
        FlattenedScalarArrayLike,
        FlattenedScalarBatch,
        FlattenedScalarLike,
        FlattenedScalarType,
    )
    from .primitive_component import (
        PrimitiveComponent,
        PrimitiveComponentArrayLike,
        PrimitiveComponentBatch,
        PrimitiveComponentLike,
        PrimitiveComponentType,
    )
    from .string_component import (
        StringComponent,
        StringComponentArrayLike,
        StringComponentBatch,
        StringComponentLike,
        StringComponentType,
    )


module_content: dict[str, str] = {
    "AffixFuzzer1": "affix_fuzzer1",
    "AffixFuzzer1ArrayLike": "affix_fuzzer1",
    "AffixFuzzer1Batch": "affix_fuzzer1",
    "AffixFuzzer1Like": "affix_fuzzer1",
    "AffixFuzzer1Type": "affix_fuzzer1",
    "AffixFuzzer2": "affix_fuzzer2",
    "AffixFuzzer2ArrayLike": "affix_fuzzer2",
    "AffixFuzzer2Batch": "affix_fuzzer2",
    "AffixFuzzer2Like": "affix_fuzzer2",
    "AffixFuzzer2Type": "affix_fuzzer2",
    "AffixFuzzer20": "affix_fuzzer20",
    "AffixFuzzer20ArrayLike": "affix_fuzzer20",
    "AffixFuzzer20Batch": "affix_fuzzer20",
    "AffixFuzzer20Like": "affix_fuzzer20",
    "AffixFuzzer20Type": "affix_fuzzer20",
    "AffixFuzzer21": "affix_fuzzer21",
    "AffixFuzzer21ArrayLike": "affix_fuzzer21",
    "AffixFuzzer21Batch": "affix_fuzzer21",
    "AffixFuzzer21Like": "affix_fuzzer21",
    "AffixFuzzer21Type": "affix_fuzzer21",
    "AffixFuzzer3": "affix_fuzzer3",
    "AffixFuzzer3ArrayLike": "affix_fuzzer3",
    "AffixFuzzer3Batch": "affix_fuzzer3",
    "AffixFuzzer3Like": "affix_fuzzer3",
    "AffixFuzzer3Type": "affix_fuzzer3",
    "AffixFuzzer4": "affix_fuzzer4",
    "AffixFuzzer4ArrayLike": "affix_fuzzer4",
    "AffixFuzzer4Batch": "affix_fuzzer4",
    "AffixFuzzer4Like": "affix_fuzzer4",
    "AffixFuzzer4Type": "affix_fuzzer4",
    "AffixFuzzer5": "affix_fuzzer5",
    "AffixFuzzer5ArrayLike": "affix_fuzzer5",
    "AffixFuzzer5Batch": "affix_fuzzer5",
    "AffixFuzzer5Like": "affix_fuzzer5",
    "AffixFuzzer5Type": "affix_fuzzer5",
    "FlattenedScalar": "flattened_scalar",
    "FlattenedScalarArrayLike": "flattened_scalar",
    "FlattenedScalarBatch": "flattened_scalar",
    "FlattenedScalarLike": "flattened_scalar",
    "FlattenedScalarType": "flattened_scalar",
    "PrimitiveComponent": "primitive_component",
    "PrimitiveComponentArrayLike": "primitive_component",
    "PrimitiveComponentBatch": "primitive_component",
    "PrimitiveComponentLike": "primitive_component",
    "PrimitiveComponentType": "primitive_component",
    "StringComponent": "string_component",
    "StringComponentArrayLike": "string_component",
    "StringComponentBatch": "string_component",
    "StringComponentLike": "string_component",
    "StringComponentType": "string_component",
}


def __getattr__(name: str) -> Any:
    from importlib import import_module

    if name in module_content:
        module = import_module(f".{module_content[name]}", __name__)
        return getattr(module, name)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")
