#pragma once

#ifdef KR_PLATFORM_WINDOWS
	#ifdef KR_BUILD_DLL
		#define KAIRON_API __declspec(dllexport)
	#else
		#define KAIRON_API __declspec(dllimport)
	#endif // KR_BUILD_DLL
#else
	#error Kairon only support windows
#endif // KR_PLATFORM_WINDOWS

#ifdef KR_DEBUG
	#define KR_ENABLE_ASSERTS
#endif

#ifdef KR_ENABLE_ASSERTS
	#define KR_ASSERT(x, ...) { if(!(x)){ KR_ERROR("Assertion Failed: {0}", __VA_ARGS__); __debugbreak();}}
	#define KR_CORE_ASSERT(x, ...) { if(!(x)){ KR_CORE_ERROR("Assertion Faeiled: {0}", __VA_ARGS__); __debugbreak();}}
#else
	#define KR_ASSERT(x, ...)
	#define KR_CORE_ASSERT(x, ...)
#endif // KR_ENABLE_ASSERTS


#define BIT(X) (1 << X)

#define KR_BIND_EVENT_FN(fn) std::bind(&fn, this, std::placeholders::_1)
