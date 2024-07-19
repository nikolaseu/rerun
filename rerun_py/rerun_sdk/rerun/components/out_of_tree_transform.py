# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/out_of_tree_transform.fbs".

# You can extend this class by creating a "OutOfTreeTransformExt" class in "out_of_tree_transform_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["OutOfTreeTransform", "OutOfTreeTransformBatch", "OutOfTreeTransformType"]


class OutOfTreeTransform(datatypes.Bool, ComponentMixin):
    """
    **Component**: If out of tree transform is enabled, a transform does not participate in the transform hierarchy.

    This means transforms on this entity do not affect children.
    It will however, still be affected by transforms on its parents.

    This is automatically enabled if any of the the transform components are present multiple times.
    Setting this to false for a transform that has multiple instances of the same transform component,
    will result in an error.
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of OutOfTreeTransformExt in out_of_tree_transform_ext.py

    # Note: there are no fields here because OutOfTreeTransform delegates to datatypes.Bool
    pass


class OutOfTreeTransformType(datatypes.BoolType):
    _TYPE_NAME: str = "rerun.components.OutOfTreeTransform"


class OutOfTreeTransformBatch(datatypes.BoolBatch, ComponentBatchMixin):
    _ARROW_TYPE = OutOfTreeTransformType()


# This is patched in late to avoid circular dependencies.
OutOfTreeTransform._BATCH_TYPE = OutOfTreeTransformBatch  # type: ignore[assignment]
