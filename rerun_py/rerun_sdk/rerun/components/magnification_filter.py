# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/magnification_filter.fbs".

# You can extend this class by creating a "MagnificationFilterExt" class in "magnification_filter_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from .._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
)

__all__ = [
    "MagnificationFilter",
    "MagnificationFilterArrayLike",
    "MagnificationFilterBatch",
    "MagnificationFilterLike",
    "MagnificationFilterType",
]


from enum import Enum


class MagnificationFilter(Enum):
    """**Component**: Filter used when magnifying an image/texture such that a single pixel/texel is displayed as multiple pixels on screen."""

    Nearest = 1
    """
    Show the nearest pixel value.

    This will give a blocky appearance when zooming in.
    Used as default when rendering 2D images.
    """

    Linear = 2
    """
    Linearly interpolate the nearest neighbors, creating a smoother look when zooming in.

    Used as default for mesh rendering.
    """


MagnificationFilterLike = Union[MagnificationFilter, Literal["Linear", "Nearest", "linear", "nearest"]]
MagnificationFilterArrayLike = Union[MagnificationFilterLike, Sequence[MagnificationFilterLike]]


class MagnificationFilterType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.MagnificationFilter"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.sparse_union([
                pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                pa.field("Nearest", pa.null(), nullable=True, metadata={}),
                pa.field("Linear", pa.null(), nullable=True, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class MagnificationFilterBatch(BaseBatch[MagnificationFilterArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = MagnificationFilterType()

    @staticmethod
    def _native_to_pa_array(data: MagnificationFilterArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (MagnificationFilter, int, str)):
            data = [data]

        types: list[int] = []

        for value in data:
            if value is None:
                types.append(0)
            elif isinstance(value, MagnificationFilter):
                types.append(value.value)  # Actual enum value
            elif isinstance(value, int):
                types.append(value)  # By number
            elif isinstance(value, str):
                if hasattr(MagnificationFilter, value):
                    types.append(MagnificationFilter[value].value)  # fast path
                elif value.lower() == "nearest":
                    types.append(MagnificationFilter.Nearest.value)
                elif value.lower() == "linear":
                    types.append(MagnificationFilter.Linear.value)
                else:
                    raise ValueError(f"Unknown MagnificationFilter kind: {value}")
            else:
                raise ValueError(f"Unknown MagnificationFilter kind: {value}")

        buffers = [
            None,
            pa.array(types, type=pa.int8()).buffers()[1],
        ]
        children = (1 + 2) * [pa.nulls(len(data))]

        return pa.UnionArray.from_buffers(
            type=data_type,
            length=len(data),
            buffers=buffers,
            children=children,
        )
