workspace "Kairon"
	architecture "x64"
	startproject "Sandbox"

	configurations{
		"Debug",
		"Release",
		"Dist"
	}

outputdir = "%{cfg.buildcfg}-%{cfg.system}-%{cfg.architecture}"

--Include directories relative to root folder
IncludeDir = {}
IncludeDir["GLFW"] = "Kairon/vendor/GLFW/include"
IncludeDir["Glad"] = "Kairon/vendor/Glad/include"
IncludeDir["ImGui"] = "Kairon/vendor/imgui"
IncludeDir["glm"] = "Kairon/vendor/glm"

group "Dependencies"
	include "Kairon/vendor/GLFW"
	include "Kairon/vendor/Glad"
	include "Kairon/vendor/imgui"

group ""

project "Kairon"
	location "Kairon"
	kind "StaticLib"
	language "C++"
	cppdialect "C++17"
	staticruntime "on"

	targetdir ("bin/" .. outputdir .. "/%{prj.name}")
	objdir ("bin-int/" .. outputdir .. "/%{prj.name}")

	pchheader "hzpch.h"
	pchsource "Kairon/src/hzpch.cpp"

	files{
		"%{prj.name}/src/**.h",
		"%{prj.name}/src/**.cpp"
	}

	includedirs{
		"%{prj.name}/src",
		"%{prj.name}/vendor/spdlog/include",
		"%{IncludeDir.GLFW}",
		"%{IncludeDir.Glad}",
		"%{IncludeDir.ImGui}",
		"%{IncludeDir.glm}"
	}

	links{
		"GLFW",
		"Glad",
		"ImGui",
		"opengl32.lib"
	}

	 filter "system:windows"
		systemversion "latest"

		defines{
			"KR_PLATFORM_WINDOWS",
			"KR_BUILD_DLL",
			"GLFW_INCLUDE_NONE"
		}

	filter "configurations:Debug"
		defines "KR_DEBUG"
		runtime "Debug"
		symbols "on"

	filter "configurations:Release"
		defines "KR_RELEASE"
		runtime "Release"
		optimize "on"

	filter "configurations:Dist"
		defines "KR_DIST"
		runtime "Release"
		optimize "on"

project "Sandbox"
	location "Sandbox"
	kind "ConsoleApp"
	language "C++"
	cppdialect "C++17"
	staticruntime "on"

	targetdir ("bin/" .. outputdir .. "/%{prj.name}")
	objdir ("bin-int/" .. outputdir .. "/%{prj.name}")

	files{
		"%{prj.name}/src/**.h",
		"%{prj.name}/src/**.cpp"
	}

	includedirs{
		"Kairon/vendor/spdlog/include",
		"Kairon/src",
		"Kairon/vendor",
		"%{IncludeDir.glm}"
	}

	links{
		"Kairon"
	}

	 filter "system:windows"
		systemversion "latest"

		defines{
			"KR_PLATFORM_WINDOWS"
		}

	filter "configurations:Debug"
		defines "KR_DEBUG"
		runtime "Debug"
		symbols "on"

	filter "configurations:Release"
		defines "KR_RELEASE"
		runtime "Release"
		optimize "on"

	filter "configurations:Dist"
		defines "KR_DIST"
		runtime "Release"
		optimize "on"