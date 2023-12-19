#pragma once

#ifdef KR_PLATFORM_WINDOWS

extern Kairon::Application* Kairon::createApplication();

int main(int argc, char** argv) {

	Kairon::Log::Init();
	KR_CORE_WARN("Initialized Log!");
	int a = 5;
	KR_INFO("Hello! Var={0}", a);

	auto app = Kairon::createApplication();
	app->Run();
	delete app;

}

#endif // KR_PALTFORM_WINDOWS
