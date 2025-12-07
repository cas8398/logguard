import 'package:flutter/material.dart';
import 'package:logguard/logguard.dart';

void main() {
  // Simple toggle: enable LogGuard in production, disable in debug
  const bool isReleaseMode = bool.fromEnvironment('dart.vm.product');

  LogGuard.runApp(
    MyApp(isReleaseMode: isReleaseMode),
    enable: isReleaseMode, // Enable only in release mode
  );
}

class MyApp extends StatelessWidget {
  final bool isReleaseMode;

  const MyApp({super.key, required this.isReleaseMode});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'LogGuard Example',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: HomeScreen(isReleaseMode: isReleaseMode),
    );
  }
}

class HomeScreen extends StatefulWidget {
  final bool isReleaseMode;

  const HomeScreen({super.key, required this.isReleaseMode});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  int _counter = 0;

  void _incrementCounter() {
    setState(() {
      _counter++;

      // This print will be sanitized in release mode
      print('Counter value: $_counter');

      // Sensitive data example
      final apiKey = 'sk_live_1234567890abcdef';
      final userEmail = 'user@example.com';

      print('API Key: $apiKey');
      print('User Email: $userEmail');
      debugPrint('Debug counter: $_counter');
    });
  }

  void _toggleLogGuard() {
    if (LogGuard.enabled) {
      LogGuard.disable();
    } else {
      LogGuard.enable();
    }
    setState(() {});
  }

  void _testSanitize() {
    final text = 'Hello World';
    print('Original: $text');
    print('Sanitized: ${text.sanitized}');
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('LogGuard Example'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              'Release Mode: ${widget.isReleaseMode}',
              style: const TextStyle(fontSize: 16),
            ),
            const SizedBox(height: 20),
            Text(
              'LogGuard Enabled: ${LogGuard.enabled}',
              style: TextStyle(
                fontSize: 18,
                color: LogGuard.enabled ? Colors.green : Colors.red,
              ),
            ),
            const SizedBox(height: 30),
            const Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            const SizedBox(height: 40),
            ElevatedButton.icon(
              onPressed: _toggleLogGuard,
              icon: Icon(
                LogGuard.enabled ? Icons.toggle_on : Icons.toggle_off,
                size: 30,
              ),
              label: Text(
                LogGuard.enabled ? 'Disable LogGuard' : 'Enable LogGuard',
                style: const TextStyle(fontSize: 16),
              ),
            ),
            const SizedBox(height: 20),
            ElevatedButton.icon(
              onPressed: _testSanitize,
              icon: const Icon(Icons.security),
              label: const Text('Test Sanitization'),
            ),
            const SizedBox(height: 20),
            ElevatedButton.icon(
              onPressed: () {
                LogGuard.log(
                  'Manual log example',
                  level: LogLevel.info,
                  toConsole: true,
                  toDeveloper: true,
                );
              },
              icon: const Icon(Icons.logout),
              label: const Text('Manual Log'),
            ),
            const SizedBox(height: 40),
            Container(
              margin: const EdgeInsets.symmetric(horizontal: 20),
              child: const Text(
                'Check console output:\n'
                '- In debug mode: prints show original text\n'
                '- In release mode: prints are sanitized',
                textAlign: TextAlign.center,
                style: TextStyle(color: Colors.grey),
              ),
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
