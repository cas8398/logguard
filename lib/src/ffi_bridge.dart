import 'dart:ffi';
import 'dart:io' as io;
import 'package:ffi/ffi.dart';

import 'safe_logger.dart';

/// Simple LogGuard FFI - Just sanitize text
class LogGuardFFI {
  static DynamicLibrary? _lib;
  static bool _loaded = false;

  // Only need these 3 functions
  static final SafeLogger log = SafeLogger('LogGuardFFI');
  static late final void Function() _init;
  static late final Pointer<Utf8> Function(Pointer<Utf8>) _sanitize;
  static late final void Function(Pointer<Utf8>) _freeString;

  /// Initialize once
  static Future<bool> init() async {
    if (_loaded) return true;

    try {
      // Load library
      if (io.Platform.isAndroid) {
        _lib = DynamicLibrary.open('liblogguard_native.so');
      } else {
        return false;
      }

      // Bind functions
      _init = _lib!
          .lookupFunction<Void Function(), void Function()>('logguard_init');
      _sanitize = _lib!.lookupFunction<Pointer<Utf8> Function(Pointer<Utf8>),
          Pointer<Utf8> Function(Pointer<Utf8>)>('logguard_sanitize');
      _freeString = _lib!.lookupFunction<Void Function(Pointer<Utf8>),
          void Function(Pointer<Utf8>)>('logguard_free_string');

      // Initialize
      _init();
      _loaded = true;
      return true;
    } catch (e) {
      log.error('❌ LogGuard FFI error: $e');
      return false;
    }
  }

  /// Simple sanitize function
  static String sanitize(String text) {
    if (!_loaded || text.isEmpty) return text;

    // Convert to C string
    final inputPtr = text.toNativeUtf8();
    Pointer<Utf8> resultPtr = nullptr;

    try {
      // Call native sanitize
      resultPtr = _sanitize(inputPtr);

      // Check result
      if (resultPtr == nullptr || resultPtr.address == 0) {
        return text;
      }

      // Convert back to Dart
      return resultPtr.toDartString();
    } catch (e) {
      log.error('❌ Sanitize error: $e');
      return text;
    } finally {
      // Clean up
      malloc.free(inputPtr);
      if (resultPtr != nullptr && resultPtr.address != 0) {
        _freeString(resultPtr);
      }
    }
  }

  /// Batch sanitize
  static List<String> sanitizeList(List<String> texts) {
    final results = <String>[];
    for (final text in texts) {
      results.add(sanitize(text));
    }
    return results;
  }

  /// Quick check if initialized
  static bool get isReady => _loaded;
}
