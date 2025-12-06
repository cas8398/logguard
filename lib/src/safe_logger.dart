import 'package:flutter/foundation.dart';
import 'package:logguard/logguard.dart';

/// Simple logger that automatically uses LogGuard
class SafeLogger {
  final String name;

  const SafeLogger(this.name);

  /// Generic log with optional level
  void log(
    String message, {
    LogLevel level = LogLevel.info,
    Object? error,
    StackTrace? stackTrace,
    bool toConsole = true,
    bool toDeveloper = false,
  }) {
    LogGuard.log(
      message,
      name: '$name [$level]',
      error: error,
      stackTrace: stackTrace,
      toConsole: toConsole,
      toDeveloper: toDeveloper,
    );
  }

  /// Debug level log
  void debug(String message) {
    if (kDebugMode) {
      log(message, level: LogLevel.debug);
    }
  }

  /// Error level log
  void error(String message, {Object? error, StackTrace? stackTrace}) {
    log(message, level: LogLevel.error, error: error, stackTrace: stackTrace);
  }

  /// Optional: warning level
  void warn(String message) {
    log(message, level: LogLevel.warning);
  }
}
