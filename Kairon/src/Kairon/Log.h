#pragma once

#include "Core.h"
#include "spdlog/spdlog.h"
#include "spdlog/fmt/ostr.h"

namespace Kairon {

	class KAIRON_API Log {
	public:
		static void Init();

		inline static std::shared_ptr<spdlog::logger>& GetCoreLogger() { return s_CoreLogger; }
		inline static std::shared_ptr<spdlog::logger>& GetClientLogger() { return s_ClientLogger; }

	private:
		static std::shared_ptr<spdlog::logger> s_CoreLogger;
		static std::shared_ptr<spdlog::logger> s_ClientLogger;
	};

}

// core log macros
#define KR_CORE_TRACE(...)    ::Kairon::Log::GetCoreLogger()->trace(__VA_ARGS__)
#define KR_CORE_INFO(...)     ::Kairon::Log::GetCoreLogger()->info(__VA_ARGS__)
#define KR_CORE_WARN(...)     ::Kairon::Log::GetCoreLogger()->warn(__VA_ARGS__)
#define KR_CORE_ERROR(...)    ::Kairon::Log::GetCoreLogger()->error(__VA_ARGS__)
#define KR_CORE_FATAL(...)    ::Kairon::Log::GetCoreLogger()->fatal(__VA_ARGS__)

// Client log macros
#define KR_TRACE(...)    ::Kairon::Log::GetClientLogger()->trace(__VA_ARGS__)
#define KR_INFO(...)     ::Kairon::Log::GetClientLogger()->info(__VA_ARGS__)
#define KR_WARN(...)     ::Kairon::Log::GetClientLogger()->warn(__VA_ARGS__)
#define KR_ERROR(...)    ::Kairon::Log::GetClientLogger()->error(__VA_ARGS__)
#define KR_FATAL(...)    ::Kairon::Log::GetClientLogger()->fatal(__VA_ARGS__)
