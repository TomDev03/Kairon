#include <Kairon.h>

class ExampleLayer : public Kairon::Layer {
public:
	ExampleLayer() : Layer("Example") {}

	void OnUpdate() override {

		if (Kairon::Input::IsKeyPressed(KR_KEY_TAB))
			KR_TRACE("Tab key is pressed!");
	}

	void OnEvent(Kairon::Event& event) override {
		if (event.GetEventType() == Kairon::EventType::KeyPressed) {
			Kairon::KeyPressedEvent& e = (Kairon::KeyPressedEvent&)event;
			KR_TRACE("{0}", (char)e.GetKeyCode());
		}
	}
};

class SandBox : public Kairon::Application {
public:
	SandBox() {
		PushLayer(new ExampleLayer());
		PushOverlay(new Kairon::ImGuiLayer());
	}
	~SandBox() {
	
	}

private:

};

Kairon::Application* Kairon::createApplication() {

	return new SandBox(); 

}