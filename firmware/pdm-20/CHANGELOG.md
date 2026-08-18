# Changelog

# [Unreleased]

- Stop economisation feeding output heartbeat.
- Apply exponential averaging to analog inputs.
- Use a higher sample rate for analog inputs.
- Avoid panic on timer wraparound.
- Prevent overflow when calculating source adress with offset.
- Respond "Complete" to J1939 memory access request erase.
- Return correct J1939 error indicator when analog reading fails to convert.
- Increase analog input sampling time.

## v0.3.1

- Fix economisation being reset when an output is turned on twice.
- Fix J1939 SLOT scaling.
- Set PWM frequency to 1kHz.

# v0.3.0

- Added output configuration.
- Added heartbeat output configuration.
- Added economisation output configuration.

# v0.2.2

- Fix output 3-4 and 5-6 being swapped.

# v0.2.1

- Fix address inputs not having enough time to settle.

# v0.2.0

- Added output overcurrent blanking.
- Added current sense CAN messages.
- Added output PWM support.
