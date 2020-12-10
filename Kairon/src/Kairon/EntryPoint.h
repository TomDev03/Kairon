#pragma once

#ifdef KR_PLATFORM_WINDOWS

extern Kairon::Application* Kairon::createApplication();

int main(int argc, char** argv) {

	auto app = Kairon::createApplication();
	app->Run();
	delete app;

}

#endif // KR_PALTFORM_WINDOWS
