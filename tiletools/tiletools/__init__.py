"""Python terrain-tile matching + procedural map generation for C&C Remastered."""

from .join import JoinTable, Material
from .matcher import Strategy, assign

__all__ = ["JoinTable", "Material", "Strategy", "assign"]
