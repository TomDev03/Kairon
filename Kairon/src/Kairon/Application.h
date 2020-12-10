#pragma once

#include "Core.h"

namespace Kairon {

	class KAIRON_API Application {

	public:
		Application();
		virtual ~Application();

		void Run();
	};

	//to be defined in CLIENT
	Application* createApplication();

}
