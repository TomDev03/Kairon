#pragma once

#include "Kairon/Layer.h"

#include "Kairon/Events/ApplicationEvent.h"
#include "Kairon/Events/KeyEvent.h"
#include "Kairon/Events/MouseEvent.h"

namespace Kairon {

	class KAIRON_API ImGuiLayer : public Layer {
	public:
		ImGuiLayer();
		~ImGuiLayer();

		virtual void OnAttach() override;
		virtual void OnDetach() override;
		virtual void OnImGuiRender() override;

		void Begin();
		void End();
	private:
		float m_Time = 0.0f;
	};
}