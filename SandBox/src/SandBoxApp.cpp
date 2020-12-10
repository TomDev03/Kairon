#include <Kairon.h>

class SandBox : public Kairon::Application {
public:
	SandBox() {
	
	}
	~SandBox() {
	
	}

private:

};

Kairon::Application* Kairon::createApplication() {

	return new SandBox(); 

}