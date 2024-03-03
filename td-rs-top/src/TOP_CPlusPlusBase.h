/* Shared Use License: This file is owned by Derivative Inc. (Derivative)
 * and can only be used, and/or modified for use, in conjunction with
 * Derivative's TouchDesigner software, and only if you are a licensee who has
 * accepted Derivative's TouchDesigner license or assignment agreement
 * (which also govern the use of this file). You may share or redistribute
 * a modified version of this file provided the following conditions are met:
 *
 * 1. The shared file or redistribution must retain the information set out
 * above and this list of conditions.
 * 2. Derivative's name (Derivative Inc.) or its trademarks may not be used
 * to endorse or promote products derived from this file without specific
 * prior written permission from Derivative.
 */

/*
 * Produced by:
 *
 * 				Derivative Inc
 *				401 Richmond Street West, Unit 386
 *				Toronto, Ontario
 *				Canada   M5V 3A8
 *				416-591-3555
 *
 * NAME:				TOP_CPlusPlusBase.h
 *
 */

/*******
        Do not edit this file directly!
        Make a subclass of TOP_CPlusPlusBase instead, and add your own
data/function

        Derivative Developers:: Make sure the virtual function order
        stays the same, otherwise changes won't be backwards compatible
********/

#ifndef __TOP_CPlusPlusBase__
#define __TOP_CPlusPlusBase__

#include "CPlusPlus_Common.h"
#include "assert.h"

#ifndef VK_HEADER_VERSION
typedef struct objectVkDevice_T *VkDevice;
#endif

#ifndef _WIN32
#ifdef __OBJC__
@class NSOpenGLContext;
#else
class NSOpenGLContext;
#endif
#endif

namespace TD {
class TOP_CPlusPlusBase;
class TOP_Context;

#pragma pack(push, 8)

enum class TOP_ExecuteMode : int32_t {
  // Old Unsupported Value
  Unsupported = 0,

  // CPU memory is filled with data directly, which is then given to the node to
  // upload into a texture for you. This avoids the need to do any GPU work
  // directly, letting TouchDesigner handle all of that for you.
  CPUMem,

  // Unused value
  Reserved,

  // Using CUDA. Textures will be given using cudaArray*, registered with
  // cudaGraphicsRegisterFlagsSurfaceLoadStore flag set. The output
  // texture will be written using a provided cudaArray* as well
  CUDA,
};

// Used to specify if the given CPU data in CPU-mode is
enum class TOP_FirstPixel : int32_t {
  // The first row of pixel data provided will be the bottom row,
  // starting from the left
  BottomLeft = 0,

  // The first row of pixel data provided will be the top row,
  // starting from the left
  TopLeft,

};

// Define for the current API version that this sample code is made for.
// To upgrade to a newer version, replace the files
// TOP_CPlusPlusBase.h
// CPlusPlus_Common.h
// from the samples folder in a newer TouchDesigner installation.
// You may need to upgrade your plugin code in that case, to match
// the new API requirements
const int TOPCPlusPlusAPIVersion = 11;

class TOP_PluginInfo {
public:
  // Must be set to TOPCPlusPlusAPIVersion in FillTOPPluginInfo
  int32_t apiVersion = 0;

  // Set this to control the execution mode for this plugin
  // See the documention for TOP_ExecuteMode for more information
  TOP_ExecuteMode executeMode = TOP_ExecuteMode::CPUMem;

  int32_t reserved[100];

  // Information used to describe this plugin as a custom OP.
  OP_CustomOPInfo customOPInfo;

  int32_t reserved2[20];
};

enum class DepthFormat : int32_t {
  None,
  Fixed, // Will be 16-bit or 24-bit fix, depending on the GPU.
  Float, // Will be 32-bit float generally.
};

// TouchDesigner will select the best pixel format based on the options you give
// Not all possible combinations of channels/bit depth are possible,
// so you get the best choice supported by your card

class TOP_OutputFormat {
public:
  int32_t width = 0;
  int32_t height = 0;

  // The aspect ratio of the TOP's output. If left as 0s, then it'll use the
  // width/height as the aspect ratio.
  float aspectX = 0.0f;
  float aspectY = 0.0f;

  // The pixel format of the data and texture
  OP_PixelFormat pixelFormat;

  // The anti-alias level.
  // 1 means no anti-alaising
  // 2 means '2x', etc., up to 32 right now
  // Only used when executeMode == TOP_ExecuteMode::OpenGL_FBO
  int32_t antiAlias = 1;

  // If you want to use multiple render targets, you can set this
  // greater than one
  // Only used when executeMode == TOP_ExecuteMode::
  int32_t numColorBuffers = 1;

  DepthFormat depthFormat = DepthFormat::None;

  // Set to true if the depth buffer should include stencil bits.
  // Only used when executeMode == TOP_ExecuteMode::Vulkan
  bool stencilBuffer = false;

  int32_t reserved[20];
};

class TOP_GeneralInfo {
public:
  // Set this to true if you want the TOP to cook every frame, even
  // if none of it's inputs/parameters are changing.
  // This is generally useful for cases where the node is outputting to
  // something external to TouchDesigner, such as a network socket or device.
  // It ensures the node cooks every if nothing inside the network is
  // using/viewing the output of this node. Important: If the node may not be
  // viewed/used by other nodes in the file, such as a TCP network output node
  // that isn't viewed in perform mode, you should set cookOnStart = true in
  // OP_CustomOPInfo. That will ensure cooking is kick-started for this node.
  // Note that this fix only works for Custom Operators, not
  // cases where the .dll is loaded into CPlusPlus TOP.
  // DEFAULT: false
  bool cookEveryFrame;

  // Set this to true if you want the CHOP to cook every frame, if asked
  // (someone uses it's output)
  // This is different from 'cookEveryFrame', which causes the node to cook
  // every frame no matter what
  // DEFAULT: false
  bool cookEveryFrameIfAsked;

  // When setting the output texture size using the node's common page
  // if using 'Input' or 'Half' options for example, it uses the first input
  // by default. You can use a different input by assigning a value
  // to inputSizeIndex.
  // This member is ignored if getOutputFormat() returns true.
  // DEFAULT: 0
  int32_t inputSizeIndex;

  int32_t reserved[20];
};

enum class TOP_BufferFlags : int32_t {
  None = 0x0000,
  Readable = 0x0001,
};

class TOP_Buffer : public OP_RefCount {
protected:
  TOP_Buffer() {}
  virtual ~TOP_Buffer() {}

public:
  void *data = nullptr;
  uint64_t size = 0;
  TOP_BufferFlags flags = TOP_BufferFlags::None;

  int32_t reserved[50];

protected:
  virtual void reserved0() = 0;
  virtual void reserved1() = 0;
  virtual void reserved2() = 0;
  virtual void reserved3() = 0;
  virtual void reserved4() = 0;
};

// This class is passed as the OP_Context in the OP_NodeInfo. It remains valid
// for the life of the node.
class TOP_Context : public OP_Context {
protected:
  virtual ~TOP_Context() {}

public:
  // Creates a TOP_Buffer you can fill with data from the CPU. Held in a
  // OP_SmartRef. This function is thread-safe and can be called at any time in
  // any thread, including from multiple threads at the same time. You become
  // owner of the TOP_Buffer, and must give it back to the node via a call in
  // TOP_Output during execute() or call release() on it yourself if you don't
  // need it anymore.
  virtual OP_SmartRef<TOP_Buffer>
  createOutputBuffer(uint64_t size, TOP_BufferFlags flags, void *reserved) = 0;

  // If you don't need a buffer anymore, but want it to be available for a
  // future create* call (avoiding an allocation) You can return it using this
  // function instead of calling release() on it. This allows it to be re-used
  // for another option potentially, avoiding a re-allocation.
  virtual void returnBuffer(OP_SmartRef<TOP_Buffer> *buf) = 0;

protected:
  virtual void reserved0() = 0;
  virtual void reserved1() = 0;
  virtual void reserved2() = 0;
  virtual void reserved3() = 0;
  virtual void reserved4() = 0;
  virtual void reserved5() = 0;
  virtual void reserved6() = 0;
  virtual void reserved7() = 0;
  virtual void reserved8() = 0;
  virtual void reserved9() = 0;
};

/***** FUNCTION CALL ORDER DURING INITIALIZATION ******/
/*
        When the Custom TOP is created, or the C++ TOP creates an instead of
   this class. Functions will be called in this order
        setupParameters(OP_ParameterManager* m);
*/

/***** FUNCTION CALL ORDER DURING A COOK ******/
/*
        When the TOP cooks the functions will be called in this order

        getGeneralInfo()
        getOutputFormat()

        execute()
        getNumInfoCHOPChans()
        for the number of chans returned getNumInfoCHOPChans()
        {
                getInfoCHOPChan()
        }
        getInfoDATSize()
        for the number of rows/cols returned by getInfoDATSize()
        {
                getInfoDATEntries()
        }
        getWarningString()
        getErrorString()
        getInfoPopupString()
*/

class TOP_UploadInfo {
public:
  TOP_UploadInfo() { memset(reserved, 0, sizeof(reserved)); }

  // Byte offset into the TOP_Buffer's data to start reading the data from.
  uint64_t bufferOffset = 0;

  // Describe the texture that should be created from the buffer
  TD::OP_TextureDesc textureDesc;

  // For e2D texDim textures, you can flip the texture vertically by setting
  // this to TopLeft.
  TOP_FirstPixel firstPixel = TOP_FirstPixel::BottomLeft;

  // You can output to any number of color buffers when uploading. Get the > 0
  // buffers using a Render Select TOP. The color buffers can different
  // resolutions, pixel formats and texDims from each other.
  uint32_t colorBufferIndex = 0;

  uint32_t reserved[25];
};

class TOP_CUDAOutputInfo {
public:
  TOP_CUDAOutputInfo() { memset(reserved, 0, sizeof(reserved)); }

  // HACK: pacify autocxx
  void* stream = 0;

  // Describe the texture that should be created from the buffer
  OP_TextureDesc textureDesc;

  // You can output to any number of color buffers when uploading. Get the > 0
  // buffers using a Render Select TOP. The color buffers can different
  // resolutions, pixel formats and texDims from each other.
  uint32_t colorBufferIndex = 0;

  uint32_t reserved[25];
};

class TOP_Output {
protected:
  virtual ~TOP_Output() {}

public:
  // Upload a TOP_Buffer you have filled in. The texture will be allocated at
  // the resolution/format/dimension as defined in 'info'. This function takes
  // ownership of 'buf'. The OP_SmartRef will be empty after calling this. Only
  // usable in TOP_ExecuteMode::CPUMem
  virtual void uploadBuffer(OP_SmartRef<TOP_Buffer> *buf,
                            const TOP_UploadInfo &info, void *reserved) = 0;

  // Only usable in TOP_ExecuteMode::CUDA
  virtual const OP_CUDAArrayInfo *
  createCUDAArray(const TOP_CUDAOutputInfo &info, void *reserved) = 0;

private:
  virtual void reserved0() = 0;
  virtual void reserved1() = 0;
  virtual void reserved2() = 0;
  virtual void reserved3() = 0;
  virtual void reserved4() = 0;
  virtual void reserved5() = 0;
  virtual void reserved6() = 0;
  virtual void reserved7() = 0;
  virtual void reserved8() = 0;
  virtual void reserved9() = 0;
};

/*** DO NOT EDIT THIS CLASS, MAKE A SUBCLASS OF IT INSTEAD ***/
class TOP_CPlusPlusBase {
protected:
  TOP_CPlusPlusBase() { memset(reserved, 0, sizeof(reserved)); }

public:
  virtual ~TOP_CPlusPlusBase() {}

  // BEGIN PUBLIC INTERFACE

  // Some general settings can be assigned here by setting memebers of
  // the TOP_GeneralInfo class that is passed in
  virtual void getGeneralInfo(TOP_GeneralInfo *, const OP_Inputs *,
                              void *reserved1) {}

  virtual void execute(TOP_Output *, const OP_Inputs *, void *reserved1) = 0;

  // Override these methods if you want to output values to the Info CHOP/DAT
  // returning 0 means you dont plan to output any Info CHOP channels

  virtual int32_t getNumInfoCHOPChans(void *reserved1) { return 0; }

  // Specify the name and value for Info CHOP channel 'index',
  // by assigning something to 'name' and 'value' members of the
  // OP_InfoCHOPChan class pointer that is passed in.
  virtual void getInfoCHOPChan(int32_t index, OP_InfoCHOPChan *chan,
                               void *reserved1) {}

  // Return false if you arn't returning data for an Info DAT
  // Return true if you are.
  // Fill in members of the OP_InfoDATSize class to specify the size
  virtual bool getInfoDATSize(OP_InfoDATSize *infoSize, void *reserved1) {
    return false;
  }

  // You are asked to assign values to the Info DAT 1 row or column at a time
  // The 'byColumn' variable in 'getInfoDATSize' is how you specify
  // if it is by column or by row.
  // 'index' is the row/column index
  // 'nEntries' is the number of entries in the row/column
  // Strings should be UTF-8 encoded.
  virtual void getInfoDATEntries(int32_t index, int32_t nEntries,
                                 OP_InfoDATEntries *entries, void *reserved1) {}

  // You can use this function to put the node into a warning state
  // with the returned string as the message.
  virtual void getWarningString(OP_String *warning, void *reserved1) {}

  // You can use this function to put the node into a error state
  // with the returned string as the message.
  virtual void getErrorString(OP_String *error, void *reserved1) {}

  // Use this function to return some text that will show up in the
  // info popup (when you middle click on a node)
  virtual void getInfoPopupString(OP_String *info, void *reserved1) {}

  // Override these methods if you want to define specfic parameters
  virtual void setupParameters(OP_ParameterManager *manager, void *reserved1) {}

  // This is called whenever a pulse parameter is pressed
  virtual void pulsePressed(const char *name, void *reserved1) {}

  // END PUBLIC INTERFACE

  // Reserved for future features
  virtual int32_t reservedFunc6() { return 0; }
  virtual int32_t reservedFunc7() { return 0; }
  virtual int32_t reservedFunc8() { return 0; }
  virtual int32_t reservedFunc9() { return 0; }
  virtual int32_t reservedFunc10() { return 0; }
  virtual int32_t reservedFunc11() { return 0; }
  virtual int32_t reservedFunc12() { return 0; }
  virtual int32_t reservedFunc13() { return 0; }
  virtual int32_t reservedFunc14() { return 0; }
  virtual int32_t reservedFunc15() { return 0; }
  virtual int32_t reservedFunc16() { return 0; }
  virtual int32_t reservedFunc17() { return 0; }
  virtual int32_t reservedFunc18() { return 0; }
  virtual int32_t reservedFunc19() { return 0; }
  virtual int32_t reservedFunc20() { return 0; }

  int32_t reserved[400];
};

#pragma pack(pop)

static_assert(offsetof(TOP_PluginInfo, apiVersion) == 0, "Incorrect Alignment");
static_assert(offsetof(TOP_PluginInfo, executeMode) == 4,
              "Incorrect Alignment");
static_assert(offsetof(TOP_PluginInfo, customOPInfo) == 408,
              "Incorrect Alignment");
static_assert(sizeof(TOP_PluginInfo) == 944, "Incorrect Size");

static_assert(offsetof(TOP_GeneralInfo, cookEveryFrame) == 0,
              "Incorrect Aligment");
static_assert(offsetof(TOP_GeneralInfo, cookEveryFrameIfAsked) == 1,
              "Incorrect Aligment");
static_assert(offsetof(TOP_GeneralInfo, inputSizeIndex) == 4,
              "Incorrect Aligment");
static_assert(sizeof(TOP_GeneralInfo) == 88, "Incorrect Size");

static_assert(offsetof(TOP_OutputFormat, width) == 0, "Incorrect Aligment");
static_assert(offsetof(TOP_OutputFormat, height) == 4, "Incorrect Aligment");
static_assert(offsetof(TOP_OutputFormat, aspectX) == 8, "Incorrect Aligment");
static_assert(offsetof(TOP_OutputFormat, aspectY) == 12, "Incorrect Aligment");
static_assert(offsetof(TOP_OutputFormat, pixelFormat) == 16,
              "Incorrect Aligment");
static_assert(offsetof(TOP_OutputFormat, antiAlias) == 20,
              "Incorrect Aligment");
static_assert(offsetof(TOP_OutputFormat, numColorBuffers) == 24,
              "Incorrect Aligment");
static_assert(offsetof(TOP_OutputFormat, depthFormat) == 28,
              "Incorrect Aligment");
static_assert(offsetof(TOP_OutputFormat, stencilBuffer) == 32,
              "Incorrect Aligment");
static_assert(sizeof(TOP_OutputFormat) == 116, "Incorrect Size");

static_assert(offsetof(TOP_UploadInfo, bufferOffset) == 0,
              "Incorrect Alignment");
static_assert(offsetof(TOP_UploadInfo, textureDesc) == 8,
              "Incorrect Alignment");
static_assert(offsetof(TOP_UploadInfo, firstPixel) == 156 + 8,
              "Incorrect Alignment");
static_assert(offsetof(TOP_UploadInfo, colorBufferIndex) == 156 + 12,
              "Incorrect Alignment");
static_assert(sizeof(TOP_UploadInfo) == 156 + 116, "Incorrect Size");

}; // namespace TD

#endif
