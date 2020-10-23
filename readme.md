SARNEG utils
============

SARNEG - Search And Rescue Numerical Encryption Grid - is simple substition cipher.

We have key of ten characters. Each character is assigned to number from 0 to 9. We then use these characters to encrypt numerical strings.

We use SARNEG to encrypt numerical part of MGRS coordinates.

This is collection of utils to work with SARNEG encryption.

Android
-------

We can run this app on Android in Termux or similar terminal emulator. To build Android binary, call:

`cargo build --target aarch64-linux-android`

Then copy resulting binary using ADB:

`adb push target/aarch64-linux-android/debug/sarneg /storage/self/primary/sarneg`

To run in Termux:

`cp /storage/self/primary/sarneg .`

`chmod u+x sarneg`

`./sarneg encrypt jimekachnu 588470892`