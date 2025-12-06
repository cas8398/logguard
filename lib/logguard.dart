library logguard;

import 'dart:async';
import 'dart:developer' as developer;
import 'dart:io';
import 'package:flutter/foundation.dart' as foundation;
import 'package:flutter/widgets.dart' as widgets;

import 'src/ffi_bridge.dart';

enum LogLevel { info, debug, warning, error }

/// Simple LogGuard
class LogGuard {
  static bool _hooked = false;
  static bool _ffiAvailable = false;
  static Function(String?, {int? wrapWidth})? _originalDebugPrint;

  /// Run Flutter app with LogGuard protection
  static void runApp(
    widgets.Widget app,
  ) async {
    // Initialize FFI
    await _initFFI();

    runZonedGuarded(
      () {
        setupHooks();
        widgets.runApp(app);
      },
      (error, stack) {
        log('Uncaught error: $error',
            name: 'Error', error: error, stackTrace: stack);
      },
      zoneSpecification: ZoneSpecification(
        print: (self, parent, zone, line) {
          if (_ffiAvailable) {
            final safe = sanitize(line);
            parent.print(zone, safe);
            return;
          }
          parent.print(zone, line);
        },
      ),
    );
  }

  /// Run any function with LogGuard protection
  static Future<T> runGuarded<T>(
    T Function() callback,
  ) async {
    await _initFFI();

    return runZoned(
      () {
        setupHooks();
        return callback();
      },
      zoneSpecification: ZoneSpecification(
        print: (self, parent, zone, line) {
          if (_ffiAvailable) {
            final safe = sanitize(line);
            parent.print(zone, safe);
            return;
          }
          parent.print(zone, line);
        },
      ),
    );
  }

  /// Initialize FFI if available
  static Future<void> _initFFI() async {
    // Only initialize FFI on supported platforms
    if (!Platform.isAndroid && !Platform.isLinux) {
      _ffiAvailable = false;
      log('FFI only supported on Android and Linux, using Dart sanitizer');
      return;
    }

    try {
      _ffiAvailable = await LogGuardFFI.init();
      if (LogGuardFFI.isReady) {
        log('FFI initialized successfully');
      }
    } catch (e) {
      _ffiAvailable = false;
      log('FFI not available, using Dart sanitizer: $e');
    }
  }

  /// Setup all log hooks
  static void setupHooks() {
    if (_hooked) return;
    _hookDebugPrint();
    _hooked = true;
  }

  /// Remove all hooks
  static void removeHooks() {
    if (!_hooked) return;
    _unhookDebugPrint();
    _hooked = false;
  }

  /// Central sanitize function
  static String sanitize(String message) {
    try {
      return _sanitizeFFI(message);
    } catch (e) {
      // Fallback to original message if sanitization fails for any reason
      return message;
    }
  }

  /// FFI sanitization
  static String _sanitizeFFI(String message) {
    return LogGuardFFI.sanitize(message);
  }

  /// Log with sanitization
  static void log(
    String message, {
    String? name,
    Object? error,
    StackTrace? stackTrace,
    LogLevel level = LogLevel.info,
    bool toConsole = true,
    bool toDeveloper = false,
  }) {
    final safeMessage = sanitize(message);
    final safeError = (error is String) ? sanitize(error) : error;

    // Always log to developer if enabled
    if (toDeveloper) {
      developer.log(
        safeMessage,
        name: name ?? 'LogGuard',
        error: safeError,
        stackTrace: stackTrace,
        level: _levelToInt(level),
      );
    }

    // Optionally print to console
    if (toConsole) {
      final prefix = '[LOGGUARD ${level.name.toUpperCase()}]';
      safePrint('$prefix $safeMessage');
      if (error != null) {
        safePrint('$prefix Error: $safeError');
      }
    }
  }

  /// Convert LogLevel to int (developer.log expects int)
  static int _levelToInt(LogLevel level) {
    switch (level) {
      case LogLevel.debug:
        return 700; // same as Dart's debug level
      case LogLevel.warning:
        return 900;
      case LogLevel.error:
        return 1000;
      case LogLevel.info:
      default:
        return 800;
    }
  }

  /// Safe print
  static void safePrint(Object? object) {
    print(sanitize(object.toString()));
  }

  /// Safe debug print
  static void safeDebug(String message, {int? wrapWidth}) {
    foundation.debugPrint(sanitize(message), wrapWidth: wrapWidth);
  }

  /// Check if FFI is available
  static bool get isFFIAvailable => _ffiAvailable;

  // Private hooks
  static void _hookDebugPrint() {
    final original = foundation.debugPrint;
    _originalDebugPrint = original;

    foundation.debugPrint = (String? message, {int? wrapWidth}) {
      final safe = sanitize(message ?? '');
      original.call(safe, wrapWidth: wrapWidth);
    };
  }

  static void _unhookDebugPrint() {
    if (_originalDebugPrint != null) {
      foundation.debugPrint = _originalDebugPrint!;
    }
  }
}

/// String extension for easy sanitization
extension LogGuardStringExtension on String {
  String get sanitized => LogGuard.sanitize(this);

  /// Apply specific pattern
  String apply(Pattern pattern, String replacement) {
    return replaceAll(pattern, replacement);
  }
}
