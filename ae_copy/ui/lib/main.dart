import 'package:flutter/material.dart';
import 'package:ui/pages/empty_page.dart';
import 'package:ui/pages/home_page.dart';
import 'package:ui/pages/log_page.dart';
void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});


  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Anima',
      initialRoute: "home",
      routes: {
        'home': (context) => const MyHomePage(),
        'logs': (context) => const LogPage(),
        'clear': (context) => const EmptyPage(),
      },
    );
  }
}

