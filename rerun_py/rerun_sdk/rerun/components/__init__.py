# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from .annotation_context import (
        AnnotationContext,
        AnnotationContextArrayLike,
        AnnotationContextBatch,
        AnnotationContextLike,
        AnnotationContextType,
    )
    from .blob import Blob, BlobArrayLike, BlobBatch, BlobLike, BlobType
    from .class_id import ClassId, ClassIdBatch, ClassIdType
    from .clear_is_recursive import (
        ClearIsRecursive,
        ClearIsRecursiveArrayLike,
        ClearIsRecursiveBatch,
        ClearIsRecursiveLike,
        ClearIsRecursiveType,
    )
    from .color import Color, ColorBatch, ColorType
    from .depth_meter import DepthMeter, DepthMeterArrayLike, DepthMeterBatch, DepthMeterLike, DepthMeterType
    from .disconnected_space import (
        DisconnectedSpace,
        DisconnectedSpaceArrayLike,
        DisconnectedSpaceBatch,
        DisconnectedSpaceLike,
        DisconnectedSpaceType,
    )
    from .draw_order import DrawOrder, DrawOrderArrayLike, DrawOrderBatch, DrawOrderLike, DrawOrderType
    from .half_sizes2d import HalfSizes2D, HalfSizes2DBatch, HalfSizes2DType
    from .half_sizes3d import HalfSizes3D, HalfSizes3DBatch, HalfSizes3DType
    from .instance_key import InstanceKey, InstanceKeyArrayLike, InstanceKeyBatch, InstanceKeyLike, InstanceKeyType
    from .keypoint_id import KeypointId, KeypointIdBatch, KeypointIdType
    from .line_strip2d import LineStrip2D, LineStrip2DArrayLike, LineStrip2DBatch, LineStrip2DLike, LineStrip2DType
    from .line_strip3d import LineStrip3D, LineStrip3DArrayLike, LineStrip3DBatch, LineStrip3DLike, LineStrip3DType
    from .material import Material, MaterialBatch, MaterialType
    from .media_type import MediaType, MediaTypeBatch, MediaTypeType
    from .mesh_properties import MeshProperties, MeshPropertiesBatch, MeshPropertiesType
    from .out_of_tree_transform3d import OutOfTreeTransform3D, OutOfTreeTransform3DBatch, OutOfTreeTransform3DType
    from .pinhole_projection import PinholeProjection, PinholeProjectionBatch, PinholeProjectionType
    from .position2d import Position2D, Position2DBatch, Position2DType
    from .position3d import Position3D, Position3DBatch, Position3DType
    from .radius import Radius, RadiusArrayLike, RadiusBatch, RadiusLike, RadiusType
    from .resolution import Resolution, ResolutionBatch, ResolutionType
    from .rotation3d import Rotation3D, Rotation3DBatch, Rotation3DType
    from .scalar import Scalar, ScalarArrayLike, ScalarBatch, ScalarLike, ScalarType
    from .scalar_scattering import (
        ScalarScattering,
        ScalarScatteringArrayLike,
        ScalarScatteringBatch,
        ScalarScatteringLike,
        ScalarScatteringType,
    )
    from .tensor_data import TensorData, TensorDataBatch, TensorDataType
    from .text import Text, TextBatch, TextType
    from .text_log_level import TextLogLevel, TextLogLevelBatch, TextLogLevelType
    from .transform3d import Transform3D, Transform3DBatch, Transform3DType
    from .vector3d import Vector3D, Vector3DBatch, Vector3DType
    from .view_coordinates import (
        ViewCoordinates,
        ViewCoordinatesArrayLike,
        ViewCoordinatesBatch,
        ViewCoordinatesLike,
        ViewCoordinatesType,
    )


module_content: dict[str, str] = {
    "AnnotationContext": "annotation_context",
    "AnnotationContextArrayLike": "annotation_context",
    "AnnotationContextBatch": "annotation_context",
    "AnnotationContextLike": "annotation_context",
    "AnnotationContextType": "annotation_context",
    "Blob": "blob",
    "BlobArrayLike": "blob",
    "BlobBatch": "blob",
    "BlobLike": "blob",
    "BlobType": "blob",
    "ClassId": "class_id",
    "ClassIdBatch": "class_id",
    "ClassIdType": "class_id",
    "ClearIsRecursive": "clear_is_recursive",
    "ClearIsRecursiveArrayLike": "clear_is_recursive",
    "ClearIsRecursiveBatch": "clear_is_recursive",
    "ClearIsRecursiveLike": "clear_is_recursive",
    "ClearIsRecursiveType": "clear_is_recursive",
    "Color": "color",
    "ColorBatch": "color",
    "ColorType": "color",
    "DepthMeter": "depth_meter",
    "DepthMeterArrayLike": "depth_meter",
    "DepthMeterBatch": "depth_meter",
    "DepthMeterLike": "depth_meter",
    "DepthMeterType": "depth_meter",
    "DisconnectedSpace": "disconnected_space",
    "DisconnectedSpaceArrayLike": "disconnected_space",
    "DisconnectedSpaceBatch": "disconnected_space",
    "DisconnectedSpaceLike": "disconnected_space",
    "DisconnectedSpaceType": "disconnected_space",
    "DrawOrder": "draw_order",
    "DrawOrderArrayLike": "draw_order",
    "DrawOrderBatch": "draw_order",
    "DrawOrderLike": "draw_order",
    "DrawOrderType": "draw_order",
    "HalfSizes2D": "half_sizes2d",
    "HalfSizes2DBatch": "half_sizes2d",
    "HalfSizes2DType": "half_sizes2d",
    "HalfSizes3D": "half_sizes3d",
    "HalfSizes3DBatch": "half_sizes3d",
    "HalfSizes3DType": "half_sizes3d",
    "InstanceKey": "instance_key",
    "InstanceKeyArrayLike": "instance_key",
    "InstanceKeyBatch": "instance_key",
    "InstanceKeyLike": "instance_key",
    "InstanceKeyType": "instance_key",
    "KeypointId": "keypoint_id",
    "KeypointIdBatch": "keypoint_id",
    "KeypointIdType": "keypoint_id",
    "LineStrip2D": "line_strip2d",
    "LineStrip2DArrayLike": "line_strip2d",
    "LineStrip2DBatch": "line_strip2d",
    "LineStrip2DLike": "line_strip2d",
    "LineStrip2DType": "line_strip2d",
    "LineStrip3D": "line_strip3d",
    "LineStrip3DArrayLike": "line_strip3d",
    "LineStrip3DBatch": "line_strip3d",
    "LineStrip3DLike": "line_strip3d",
    "LineStrip3DType": "line_strip3d",
    "Material": "material",
    "MaterialBatch": "material",
    "MaterialType": "material",
    "MediaType": "media_type",
    "MediaTypeBatch": "media_type",
    "MediaTypeType": "media_type",
    "MeshProperties": "mesh_properties",
    "MeshPropertiesBatch": "mesh_properties",
    "MeshPropertiesType": "mesh_properties",
    "OutOfTreeTransform3D": "out_of_tree_transform3d",
    "OutOfTreeTransform3DBatch": "out_of_tree_transform3d",
    "OutOfTreeTransform3DType": "out_of_tree_transform3d",
    "PinholeProjection": "pinhole_projection",
    "PinholeProjectionBatch": "pinhole_projection",
    "PinholeProjectionType": "pinhole_projection",
    "Position2D": "position2d",
    "Position2DBatch": "position2d",
    "Position2DType": "position2d",
    "Position3D": "position3d",
    "Position3DBatch": "position3d",
    "Position3DType": "position3d",
    "Radius": "radius",
    "RadiusArrayLike": "radius",
    "RadiusBatch": "radius",
    "RadiusLike": "radius",
    "RadiusType": "radius",
    "Resolution": "resolution",
    "ResolutionBatch": "resolution",
    "ResolutionType": "resolution",
    "Rotation3D": "rotation3d",
    "Rotation3DBatch": "rotation3d",
    "Rotation3DType": "rotation3d",
    "Scalar": "scalar",
    "ScalarArrayLike": "scalar",
    "ScalarBatch": "scalar",
    "ScalarLike": "scalar",
    "ScalarType": "scalar",
    "ScalarScattering": "scalar_scattering",
    "ScalarScatteringArrayLike": "scalar_scattering",
    "ScalarScatteringBatch": "scalar_scattering",
    "ScalarScatteringLike": "scalar_scattering",
    "ScalarScatteringType": "scalar_scattering",
    "TensorData": "tensor_data",
    "TensorDataBatch": "tensor_data",
    "TensorDataType": "tensor_data",
    "Text": "text",
    "TextBatch": "text",
    "TextType": "text",
    "TextLogLevel": "text_log_level",
    "TextLogLevelBatch": "text_log_level",
    "TextLogLevelType": "text_log_level",
    "Transform3D": "transform3d",
    "Transform3DBatch": "transform3d",
    "Transform3DType": "transform3d",
    "Vector3D": "vector3d",
    "Vector3DBatch": "vector3d",
    "Vector3DType": "vector3d",
    "ViewCoordinates": "view_coordinates",
    "ViewCoordinatesArrayLike": "view_coordinates",
    "ViewCoordinatesBatch": "view_coordinates",
    "ViewCoordinatesLike": "view_coordinates",
    "ViewCoordinatesType": "view_coordinates",
}


def __getattr__(name: str) -> Any:
    from importlib import import_module

    if name in module_content:
        module = import_module(f".{module_content[name]}", __name__)
        return getattr(module, name)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")
