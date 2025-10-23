import 'package:flutter/material.dart';
import 'package:ui/pages/nav_bar.dart';
import 'package:ui/settings/globals.dart';

class EmptyPage extends StatelessWidget {
  const EmptyPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: background,
      body: Row(
        mainAxisAlignment: MainAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Navbar()
        ],
      ),
    );
  }
}