#include <Kairon.h>

class ExampleLayer : public Kairon::Layer {
public:
	ExampleLayer() : Layer("Example") {}

	void OnUpdate() override {
		KR_INFO("ExampleLayer::Update");
	}

	void OnEvent(Kairon::Event& event) override {
		KR_TRACE("{0}", event);
	}
};

class SandBox : public Kairon::Application {
public:
	SandBox() {
		PushLayer(new ExampleLayer());
	}
	~SandBox() {
	
	}

private:

};

Kairon::Application* Kairon::createApplication() {

	return new SandBox(); 

}