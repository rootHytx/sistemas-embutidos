// Define the pin connected to the sensor's VOUT
const int sensorPin1 = A0;
const int sensorPin2 = A1;

void setup() {
  Serial.begin(9600);
}

void loop() {
  // Read the analog value from the sensor
  int sensor1Value = analogRead(sensorPin1);
  int sensor2Value = analogRead(sensorPin2);
  String result = "";

  // Convert the analog value to voltage (assuming 5V/3.3V power supply)
  float voltage1 = sensor1Value * (5.0 / 1023.0);
  float voltage2 = sensor2Value * (5.0 / 1023.0);

  // Convert the voltage to distance for GP2Y0A41SK0F
  // Using a typical formula for this sensor: Distance (cm) = 12.08 / (Voltage - 0.25)
  float distance1 = 12.08 / (voltage1 - 0.25);
  float distance2 = 12.08 / (voltage2 - 0.25);
  
  // Print the distance to the Serial Monitor if it's less than 10 cm
    if ((distance1 < 10.0 && distance1 > 0) || (distance2 < 10.0 && distance2 > 0)) {
      Serial.println("proximity_alert"); 
    }
    // Wait before sending the next request
    delay(1000); // Send data every 10 seconds
}
