# igor-gateway

####Goals
Expose a set of well defined services, abstracting away vendor specific information.

####Supported Primitives
1. BooleanSwitch [on/off]
2. DoubleSwitch [0..1] (dimmers, volume, fans, etc)
3. RGBSwitch
4. CompositeSwitch

####Services
1. EventDispatcher
    - adds ability to react to events (e.g., light turned on)
    - should support use cases (e.g., Event X triggered before/after sunrise)
2. SpeakerProvider, SpeakerGroupProvider
3. LightProvider
4. ThermostatProvider
5. FanProvider
6. CameraProvider
7. CommandService
