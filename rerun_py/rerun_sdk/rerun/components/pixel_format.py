# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/pixel_format.fbs".

# You can extend this class by creating a "PixelFormatExt" class in "pixel_format_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from .._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
)

__all__ = ["PixelFormat", "PixelFormatArrayLike", "PixelFormatBatch", "PixelFormatLike", "PixelFormatType"]


from enum import Enum


class PixelFormat(Enum):
    """
    **Component**: Specifieds a particular format of an [`archetypes.Image`][rerun.archetypes.Image].

    Most images can be described by a [`components.ColorModel`][rerun.components.ColorModel] and a [`components.ChannelDatatype`][rerun.components.ChannelDatatype],
    e.g. `RGB` and `U8` respectively.

    However, some image formats has chroma downsampling and/or
    use differing number of bits per channel, and that is what this [`components.PixelFormat`][rerun.components.PixelFormat] is for.

    All these formats support random access.

    For more compressed image formats, see [`archetypes.ImageEncoded`][rerun.archetypes.ImageEncoded].
    """

    NV12 = 1
    """
    NV12 (aka Y_UV12) is a YUV 4:2:0 chroma downsampled format with 12 bits per pixel and 8 bits per channel.

    First comes entire image in Y in one plane,
    followed by a plane with interleaved lines ordered as U0, V0, U1, V1, etc.
    """

    YUY2 = 2
    """
    YUY2 (aka YUYV or YUYV16), is a YUV 4:2:2 chroma downsampled format with 16 bits per pixel and 8 bits per channel.

    The order of the channels is Y0, U0, Y1, V0, all in the same plane.
    """


PixelFormatLike = Union[PixelFormat, Literal["NV12", "YUY2", "nv12", "yuy2"]]
PixelFormatArrayLike = Union[PixelFormatLike, Sequence[PixelFormatLike]]


class PixelFormatType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.PixelFormat"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.sparse_union([
                pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                pa.field("NV12", pa.null(), nullable=True, metadata={}),
                pa.field("YUY2", pa.null(), nullable=True, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class PixelFormatBatch(BaseBatch[PixelFormatArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = PixelFormatType()

    @staticmethod
    def _native_to_pa_array(data: PixelFormatArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (PixelFormat, int, str)):
            data = [data]

        types: list[int] = []

        for value in data:
            if value is None:
                types.append(0)
            elif isinstance(value, PixelFormat):
                types.append(value.value)  # Actual enum value
            elif isinstance(value, int):
                types.append(value)  # By number
            elif isinstance(value, str):
                if hasattr(PixelFormat, value):
                    types.append(PixelFormat[value].value)  # fast path
                elif value.lower() == "nv12":
                    types.append(PixelFormat.NV12.value)
                elif value.lower() == "yuy2":
                    types.append(PixelFormat.YUY2.value)
                else:
                    raise ValueError(f"Unknown PixelFormat kind: {value}")
            else:
                raise ValueError(f"Unknown PixelFormat kind: {value}")

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
