#include <Servo.h>

Servo myServo;  // Create a Servo object
int pos;

void setup() {
  myServo.attach(8);  // Attach the servo on pin 9 to the servo object
  Serial.begin(9600); // Start the Serial Monitor
  pos=180;
  myServo.write(pos);
  delay(1000);
}

void loop() {
  String incoming = Serial.readStringUntil('\n');
  Serial.println(incoming);
  if (incoming.equals("SWITCH")){
    if (pos==180){
      pos=90;
    }
    else{
      pos=180;
    }
    myServo.write(pos); 
  }
  delay(1000);
}
