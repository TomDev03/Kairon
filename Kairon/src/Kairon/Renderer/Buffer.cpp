#include "hzpch.h"
#include "Buffer.h"

#include "Renderer.h"

#include "Platform/OpenGL/OpenGLBuffer.h"

namespace Kairon {

	/// <summary>
	/// Create a vertex buffer
	/// </summary>
	/// <param name="vertices"></param>
	/// <param name="size"></param>
	/// <returns>VertexBuffer*</returns>
	VertexBuffer* VertexBuffer::Create(float* vertices, uint32_t size)
	{
		switch (Renderer::GetAPI())
		{
			case RendererAPI::None: {
				KR_CORE_ASSERT(false, "RendererAPI::None is currently not supported!");
				return nullptr;
			}
			case RendererAPI::OpenGL: {
				return new OpenGLVertexBuffer(vertices, size);
			}
		}
		
		KR_CORE_ASSERT(false, "Unknown RendererAPI!");

		return nullptr;
	}

	/// <summary>
	/// Create a index buffer
	/// </summary>
	/// <param name="indices"></param>
	/// <param name="size"></param>
	/// <returns>IndexBuffer*</returns>
	IndexBuffer* IndexBuffer::Create(uint32_t* indices, uint32_t size)
	{
		switch (Renderer::GetAPI())
		{
		case RendererAPI::None: {
			KR_CORE_ASSERT(false, "RendererAPI::None is currently not supported!");
			return nullptr;
		}
		case RendererAPI::OpenGL: {
			return new OpenGLIndexBuffer(indices, size);
		}
		}

		KR_CORE_ASSERT(false, "Unknown RendererAPI!");

		return nullptr;
	}

}