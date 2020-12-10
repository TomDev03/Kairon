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
