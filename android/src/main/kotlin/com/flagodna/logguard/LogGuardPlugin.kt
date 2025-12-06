package com.flagodna.logguard

import android.util.Log
import io.flutter.embedding.engine.plugins.FlutterPlugin

class LogGuardPlugin : FlutterPlugin {

    override fun onAttachedToEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        try {
            System.loadLibrary("logguard_native")
            Log.d("LogGuard", "Native library loaded successfully")
        } catch (e: UnsatisfiedLinkError) {
            Log.e("LogGuard", "Failed to load native library: ${e.message}")
        }
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        // Cleanup if needed
    }
}
