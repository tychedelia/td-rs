#pragma once
#include "SOP_CPlusPlusBase.h"
#include <rust/cxx.h>

namespace td_rs_sop {
    struct Position;
    struct Vec3;
    struct RGBA;
    struct UVW;
    enum class GroupType : ::std::uint8_t;
    struct BoundingBox;

    class SopOutput {
    public:
        SopOutput(SOP_Output *output) noexcept;

        // Add a single point at the given position.
        // Returns the point's index.
        virtual int32_t addPoint(const td_rs_sop::Position pos);

        // Add multiple points at specified positions.
        // 'numPoints' is the number of points to be added.
        virtual bool addPoints(const rust::Slice<td_rs_sop::Position> positions);

        // Returns the number of added points at the time of query
        virtual int32_t getNumPoints();

        // Set the normal vector for the point with the 'pointIdx'.
        // The point must already exist by via calling addPoints() or addPoint().
        virtual bool setNormal(const td_rs_sop::Vec3 n, int32_t pointIdx);

        // Set the normal vectors for existing points.
        // Note that has been the points must be already added by calling addPoints() or addPoint().
        // The startPointIdx indicates the start index of the points to set normals for.
        virtual bool setNormals(const rust::Slice<td_rs_sop::Vec3>, int32_t startPointIdx);

        // Returns true if the normal has been set for this geometry.
        virtual bool hasNormal();

        // Set the color value with Color (i.e. r,g,b,a) for the point with 'pointIdx' index.
        // The point must already exist by via calling addPoints() or addPoint().
        virtual bool setColor(const td_rs_sop::RGBA c, int32_t pointIdx);

        // Set the colors for points that are already added.
        // The startPointIdx indicates the start index of the points to set colors for.
        virtual bool setColors(const rust::Slice<td_rs_sop::RGBA> colors, int32_t startPointIdx);

        // Returns true if the color has been set for this geometry.
        virtual bool hasColor();

        // Set texture coordinate data for existing points.
        // the numLayers is the texcoord size and can be from 1 up to 8 for texture layers
        // the pointIdx specifies the point index with the texture coords
        virtual bool setTexCoord(const rust::Slice<td_rs_sop::UVW> uvw, int32_t pointIdx);

        // Set texture coordinate data for existing points.
        // the numLayers is the texCoord size and can be from 1 up to 8 for texCoord layers.
        // The startPointIdx indicates the start index of the points to set texCoord for.
        virtual bool setTexCoords(const rust::Slice<td_rs_sop::UVW> uvw, int32_t numLayers, int32_t startPointIdx);

        // Returns true if the texCoord/textures has been set for this geometry.
        virtual bool hasTexCoord();

        // Returns the number of texcoord layers
        virtual int32_t getNumTexCoordLayers();

        // Set the custom attribute with SOP_CustomAttribData (must have set its name, number of components, and its type)
        // The data param must hold the data for the custom attribute.
        // E.g a custom atrrib with 4 components for each point should holds 4*numPoints values for its data.
        virtual bool setCustomAttribute(const SOP_CustomAttribData *cu, int32_t numPoints);

        // Returns true if the custom attributes has been set for this geometry.
        virtual bool hasCustomAttibutes();

        // Add a triangle using the points at the given 3 indices.
        virtual bool addTriangle(int32_t ptIdx1, int32_t ptIdx2, int32_t ptIdx3);

        // Add multiple triangles using an array of point's indices.
        // The size param represents the number of triangles to be added.
        // 'indices' must contain at least 3 * size elements.
        virtual bool addTriangles(const rust::Slice<rust::Slice<int32_t>> triangles);

        // add particle systems from the points that has been already added. The points can have colors, normals and custom attribs.
        // the startIndex param is the staring index of the points from particle system.
        virtual bool addParticleSystem(int32_t numParticles, int32_t startIndex);

        // Add line strip from the points that has been already added. The points can have colors, normals and custom attribs.
        // the 'indices' contains the indices of vertices, and 'size' is the number of indices for the line strip
        virtual bool addLine(const rust::Slice<int32_t> indices);

        // Add line strips from the points that has been already added.The points can have colors, normals and custom attribs.
        // the 'indices' contains the indices of vertices, 'sizeOfEachLine' contains the number of vertices for each line,
        // 'numOfLines' specifies the number of lines to be drawn.
        // Note that the number of elements in sizeOfEachLine must be equal to numOfLines.
        virtual bool addLines(const rust::Slice<int32_t> indices, rust::Slice<int32_t> sizeOfEachLine);

        // Returns the number of added primitives at the time of query. Currently it is either the number of triangles or particles.
        virtual int32_t getNumPrimitives();

        // Set the bounding box for the whole geometry.
        // Setting the bounding box helps to have exact homing on the viewer.
        // You may set this value at each frame for non static geometries that are translating constantly.
        virtual bool setBoundingBox(const td_rs_sop::BoundingBox bbox);

        // Add a group with input type and name.
        // Returns false if a group with this name already exists.
        virtual bool addGroup(const td_rs_sop::GroupType type, const rust::Str name);

        // Destroy a group with input type and name.
        // Returns false if a group with this name for the specified type does not exists.
        virtual bool destroyGroup(const td_rs_sop::GroupType type, const rust::Str name);

        // Add a point with its index to an already existing group with SOP_GroupType::Point type.
        // Returns false if a point group with this name does not exists Or
        // if a point with that index does not exists.
        virtual bool addPointToGroup(int index, const rust::Str name);

        // Add a primitive with its index to an already existing group with SOP_GroupType::Primitive type.
        // Returns false if a primitive group with this name does not exists Or
        // if a pritimive with that index does not exists.
        virtual bool addPrimToGroup(int index, const rust::Str *name);

        // Add a point/prim index to an already defined group.
        // Returns false if a primitive group with this name does not exists Or
        // if a pritimive/point with that index does not exists.
        virtual bool addToGroup(int index, const td_rs_sop::GroupType type, const rust::Str name);

        // Add a point with its index to an already existing group with SOP_GroupType::Point type.
        // Returns false if a point group with this name does not exists Or
        // if a point with that index does not exists.
        virtual bool discardFromPointGroup(int index, const rust::Str name);

        // Add a primitive with its index to an already existing group with SOP_GroupType::Primitive type.
        // Returns false if a primitive group with this name does not exists Or
        // if a pritimive with that index does not exists.
        virtual bool discardFromPrimGroup(int index, const rust::Str name);

        // Remove a point/prim index from an already defined group.
        // Returns false if a primitive group with this name does not exists Or
        // if a pritimive/point with that index does not exists.
        virtual bool discardFromGroup(int index, const td_rs_sop::GroupType type, const rust::Str name);


    private:
        SOP_Output *output;
    };

}