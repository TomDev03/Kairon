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

group "Dependencies"
	include "Kairon/vendor/GLFW"
	include "Kairon/vendor/Glad"
	include "Kairon/vendor/imgui"

group ""

project "Kairon"
	location "Kairon"
	kind "SharedLib"
	language "C++"
	staticruntime "off"

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
		"%{IncludeDir.ImGui}"
	}

	links{
		"GLFW",
		"Glad",
		"ImGui",
		"opengl32.lib"
	}

	 filter "system:windows"
		cppdialect "C++17"
		systemversion "latest"

		defines{
			"KR_PLATFORM_WINDOWS",
			"KR_BUILD_DLL",
			"GLFW_INCLUDE_NONE"
		}

		postbuildcommands{
			("{COPY} %{cfg.buildtarget.relpath} \"../bin/" .. outputdir .. "/Sandbox/\"")
		}

	filter "configurations:Debug"
		defines "KR_DEBUG"
		runtime "Debug"
		symbols "On"

	filter "configurations:Release"
		defines "KR_RELEASE"
		runtime "Release"
		optimize "On"

	filter "configurations:Dist"
		defines "KR_DIST"
		runtime "Release"
		optimize "On"

project "Sandbox"
	location "Sandbox"
	kind "ConsoleApp"
	language "C++"
	staticruntime "off"

	language "C++"

	targetdir ("bin/" .. outputdir .. "/%{prj.name}")
	objdir ("bin-int/" .. outputdir .. "/%{prj.name}")

	files{
		"%{prj.name}/src/**.h",
		"%{prj.name}/src/**.cpp"
	}

	includedirs{
		"Kairon/vendor/spdlog/include",
		"Kairon/src"
	}

	links{
		"Kairon"
	}

	 filter "system:windows"
		cppdialect "C++17"
		systemversion "latest"

		defines{
			"KR_PLATFORM_WINDOWS"
		}

	filter "configurations:Debug"
		defines "KR_DEBUG"
		runtime "Debug"
		symbols "On"

	filter "configurations:Release"
		defines "KR_RELEASE"
		runtime "Release"
		optimize "On"

	filter "configurations:Dist"
		defines "KR_DIST"
		runtime "Release"
		optimize "On"