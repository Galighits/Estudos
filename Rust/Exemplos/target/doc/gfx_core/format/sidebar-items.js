window.SIDEBAR_ITEMS = {"enum":[["B8_G8_R8_A8",""],["BC1_R8_G8_B8","Block Compression 1 also known as DXT1, S3TC. See S3TC wiki."],["BC3_R8_G8_B8_A8","Block Compression 3 also known as DXT5, S3TC. See S3TC wiki."],["ChannelSource","Source channel in a swizzle configuration. Some may redirect onto different physical channels, some may be hardcoded to 0 or 1."],["ChannelType","Type of a surface channel. This is how we interpret the storage allocated with `SurfaceType`."],["D16",""],["D24",""],["D24_S8",""],["D32",""],["Float",""],["Inorm",""],["Int",""],["R10_G10_B10_A2",""],["R11_G11_B10",""],["R16",""],["R16_G16",""],["R16_G16_B16",""],["R16_G16_B16_A16",""],["R32",""],["R32_G32",""],["R32_G32_B32",""],["R32_G32_B32_A32",""],["R4_G4",""],["R4_G4_B4_A4",""],["R5_G5_B5_A1",""],["R5_G6_B5",""],["R8",""],["R8_G8",""],["R8_G8_B8_A8",""],["Srgb",""],["SurfaceType","Type of the allocated texture surface. It is supposed to only carry information about the number of bits per each channel. The actual types are up to the views to decide and interpret. The actual components are up to the swizzle to define."],["Uint",""],["Unorm",""]],"struct":[["F16",""],["Format","Complete run-time surface format."],["I16Norm",""],["I8Norm",""],["Swizzle","Channel swizzle configuration for the resource views. Note: It’s not currently mirrored at compile-time, thus providing less safety and convenience."],["U16Norm",""],["U8Norm",""]],"trait":[["BlendChannel","An ability of a channel type to be used for blended render targets."],["BlendFormat","Ability to be used for blended render targets."],["BufferFormat","Ability to be used for vertex buffers."],["BufferSurface","An ability of a surface type to be used for vertex buffers."],["ChannelTyped","Compile-time channel type trait."],["DepthFormat","Ability to be used for depth targets."],["DepthStencilFormat","Ability to be used for depth+stencil targets."],["DepthSurface","An ability of a surface type to be used for depth targets."],["Formatted","Compile-time full format trait."],["RenderChannel","An ability of a channel type to be used for render targets."],["RenderFormat","Ability to be used for render targets."],["RenderSurface","An ability of a surface type to be used for render targets."],["StencilFormat","Ability to be used for vertex buffers."],["StencilSurface","An ability of a surface type to be used for stencil targets."],["SurfaceTyped","Compile-time surface type trait."],["TextureChannel","An ability of a channel type to be used for textures."],["TextureFormat","Ability to be used for textures."],["TextureSurface","An ability of a surface type to be used for textures."]],"type":[["Bgra8","Standard 8bits BGRA format."],["Depth","Standard 24-bit depth format."],["Depth32F","Standard 32-bit floating-point depth format."],["DepthStencil","Standard 24-bit depth format with 8-bit stencil."],["Rgb10a2F","Standard HDR floating-point format with 10 bits for RGB components and 2 bits for the alpha."],["Rgba16F","Standard 16-bit floating-point RGBA format."],["Rgba32F","Standard 32-bit floating-point RGBA format."],["Rgba8","Standard 8bits RGBA format."],["Srgba8","Standard 8bit gamma transforming RGB format."],["Vec1","Abstracted 1-element container for macro internal use"],["Vec2","Abstracted 2-element container for macro internal use"],["Vec3","Abstracted 3-element container for macro internal use"],["Vec4","Abstracted 4-element container for macro internal use"]]};