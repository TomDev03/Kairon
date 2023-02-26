#include <Kairon.h>

#include "imgui/imgui.h"

class ExampleLayer : public Kairon::Layer {
public:
	ExampleLayer() : Layer("Example") {}

	void OnUpdate() override
	{
		if (Kairon::Input::IsKeyPressed(KR_KEY_TAB))
			KR_TRACE("Tab key is pressed (poll)!");
	}

	virtual void OnImGuiRender() override {
		ImGui::Begin("Test");
		ImGui::Text("Hello World");
		ImGui::End();
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
	}
	~SandBox() {
	
	}

private:

};

Kairon::Application* Kairon::createApplication() {

	return new SandBox(); 

}