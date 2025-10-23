import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

class Navbar extends StatefulWidget {
  const Navbar({super.key});

  @override
  State<Navbar> createState() => _NavbarState();
}

class _NavbarState extends State<Navbar> {
  Color background = const Color.fromARGB(255, 96, 125, 139);
  bool shownav = false;
  bool showlogs = false;

  @override
  Widget build(BuildContext context) {
    final currentWidth = MediaQuery.of(context).size.width;
    final currentHeight = MediaQuery.of(context).size.height;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(
          width: currentWidth * 0.05,
          child: IconButton(
            icon: SvgPicture.asset('assets/dashboardic.svg'),
            onPressed: () {
              setState(() {
                shownav = !shownav;
              });
            },
          ),
        ),
        AnimatedOpacity(
          opacity: shownav ? 1 : 0,
          duration: Duration(milliseconds: 200),
          child: Container(
            height: currentHeight * 0.9,
            width: currentWidth * 0.05,
            color: Colors.transparent,
            child: ListView(
              children: [
                IconButton(
                  icon: SvgPicture.asset('assets/logsic.svg'),
                  onPressed: () => Navigator.pushNamed(context, 'logs'),
                ),
                IconButton(icon: SvgPicture.asset("assets/clearic.svg"), onPressed: () => Navigator.pushNamed(context, "clear"),)
              ],
            ),
          ),
        ),
      ],
    );
  }
}
