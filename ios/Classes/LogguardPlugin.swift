import Flutter
import UIKit

public class LogGuardPlugin: NSObject, FlutterPlugin {
    public static func register(with registrar: FlutterPluginRegistrar) {
        // Create channel but don't implement anything
        let channel = FlutterMethodChannel(
            name: "logguard",
            binaryMessenger: registrar.messenger()
        )
        let instance = LogGuardPlugin()
        registrar.addMethodCallDelegate(instance, channel: channel)
        
        // Load native library (if bundled)
        // Note: For iOS, we typically load through XCFramework
    }
    
    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
        // All calls handled by FFI, return notImplemented
        result(FlutterMethodNotImplemented)
    }
}