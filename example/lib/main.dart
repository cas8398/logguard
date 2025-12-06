import 'dart:io';

import 'package:flutter/material.dart';
import 'package:logguard/logguard.dart';

void main() {
  LogGuard.runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: Text('LogGuard Demo')),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              ElevatedButton(
                child: Text('Test Print'),
                onPressed: () {
                  // This will be sanitized by zone
                  print('Email: john@example.com');
                },
              ),
              SizedBox(height: 16),
              ElevatedButton(
                child: Text('Test DebugPrint'),
                onPressed: () {
                  // This will be sanitized by hook
                  debugPrint('Card: 4111-1111-1111-1111');
                },
              ),
              SizedBox(height: 16),
              ElevatedButton(
                child: Text('Test Stdout'),
                onPressed: () {
                  // This will be sanitized by IOSink wrapper
                  stdout.writeln('Token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9');
                },
              ),
              SizedBox(height: 16),
              ElevatedButton(
                child: Text('Test LogGuard.log'),
                onPressed: () {
                  // Use LogGuard's wrapper for developer.log
                  LogGuard.log('API Key: sk_test_xyz789', name: 'Auth');
                },
              ),
            ],
          ),
        ),
      ),
    );
  }
}
