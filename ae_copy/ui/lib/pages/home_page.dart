import 'package:flutter/material.dart';
import 'package:ui/settings/globals.dart';
import 'package:ui/pages/nav_bar.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String viewStateInput = "";
  bool shownav = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: background,
      body: Row(
        children: [
          Navbar(),
        ],
      ),
    );
  }
}
