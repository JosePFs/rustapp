package org.eixe.patientfront

import io.flutter.embedding.android.FlutterActivity

class MainActivity : FlutterActivity() {
    init {
        System.loadLibrary("mobile_bridge_frb")
    }
}
