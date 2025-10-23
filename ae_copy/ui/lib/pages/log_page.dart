// ignore_for_file: non_constant_identifier_names

import 'package:flutter/material.dart';
import 'package:ui/pages/nav_bar.dart';

import 'package:ui/settings/globals.dart';

import 'dart:io';
import 'dart:convert';
import 'dart:async';

class LogPage extends StatefulWidget {
  const LogPage({super.key});

  @override
  State<LogPage> createState() => _LogPageState();
}

class _LogPageState extends State<LogPage> {
  List log_list = [];
  List<String> log_files = [];
  String file = "welcome";
  String command = "r";

  String? edit_string = "";
  bool edit_ready = false;

  final TextEditingController _controller = TextEditingController();

  // ignore: unused_element
  Future<String?> _showOverlay(double height, double width) async {
    final completer = Completer<String?>();
    final TextEditingController editcontroller = TextEditingController();

    late OverlayEntry overlayEntry;

    void removeOverlay() {
      overlayEntry.remove();
      if (!completer.isCompleted) {
        completer.complete(editcontroller.text); // returns the value
      }
    }

    overlayEntry = OverlayEntry(
      builder: (context) => GestureDetector(
        onTap: () {
          removeOverlay();
        },
        behavior: HitTestBehavior.translucent,
        child: Stack(
          children: [
            Positioned(
              top: 0,
              right: 0,
              child: GestureDetector(
                onTap: () {},
                child: Material(
                  color: background,
                  child: Container(
                    decoration: BoxDecoration(
                      border: Border(
                        left: BorderSide(color: Color(0xFFB87333)),
                      ),
                    ),
                    height: height,
                    width: width * 0.15,
                    child: Column(
                      children: [
                        TextField(
                          minLines: 1,
                          maxLines: null,
                          controller: editcontroller,
                          decoration: InputDecoration(
                            hintText: "Enter New Log",
                          ),
                          style: TextStyle(color: Colors.white),
                        ),
                        SizedBox(height: height * 0.01),
                        ElevatedButton(
                          onPressed: () {
                            removeOverlay();
                          },
                          child: Text("Enter"),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );

    Overlay.of(context).insert(overlayEntry);

    return completer.future;
  }

  void command_fetch(
    String fetched_command,
    double height,
    double width,
  ) async {
    setState(() {
      command = fetched_command;
    });

    if (command == "e") {
      // make a popup that lets person enter string for the file
      String? edit_string_value = await _showOverlay(height, width);
      // then run the command
      command_run(file, fetched_command, edit_string_value);
      return;
    }

    command_run(file, fetched_command, edit_string);
  }

  void file_change(String name) {
    setState(() {
      file = name;
    });
  }

  void find_files() async {
    setState(() {
      log_files.clear();
    });
    final dir = Directory('../modules/creation_module/src/creations/');
    final List<FileSystemEntity> files = await dir.list().toList();
    files.forEach((f) {
      final stringed = f.toString();
      final splited = stringed.split("/");
      String name = splited.last;
      name = name.replaceAll('\'', '').trim();
      List file_name = name.split(".");
      name = file_name[0];

      setState(() {
        log_files.add(name);
      });
    });
  }

  void command_run(String file, String command, String? text_input) async {
    // clearing log_list so it doesn't stack
    setState(() {
      log_list.clear();
    });

    // setting dirs
    final execPath = Directory.current.uri
        .resolve('../modules/creation_module/creation')
        .toFilePath();
    final working_dir = File(execPath).parent.path;

    // starting the program
    final log_exec = await Process.start(
      execPath,
      [],
      workingDirectory: working_dir,
    );

    log_exec.stdin.writeln("$file $command");

    if (command == 'e') {
      await log_exec.stdin.flush();
      log_exec.stdin.writeln(text_input);
      log_exec.stdin.writeln("x");
      command_run(file, "r", edit_string);
      return;
    }

    // // command that is to be run

    await log_exec.stdin.flush();
    // start reading the output
    log_exec.stdout.transform(utf8.decoder).transform(LineSplitter()).forEach((
      line,
    ) {
      String cleaned = line.replaceAll(RegExp(r'[^\x20-\x7E]'), '');
      if (cleaned.codeUnits.isEmpty) {
      } else if (cleaned.codeUnitAt(0) == 99) {
        cleaned = cleaned.substring(1);
      }
      setState(() {
        log_list.add(cleaned);
      });
    });

    // exit the program
    log_exec.stdin.writeln("x");
    find_files();
  }

  @override
  void initState() {
    command_run(file, command, edit_string);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final currentWidth =
        (MediaQuery.of(context).size.width) -
        (MediaQuery.of(context).size.width *
            0.05); // this if for finding the correct amount of available distance to draw the buttons
    final buttonWidth = currentWidth / 3.05; // split between three buttons

    final currentHeight = MediaQuery.of(context).size.height;
    final buttonHeight = currentHeight * 0.05;

    return Scaffold(
      backgroundColor: background,
      body: Row(
        mainAxisAlignment: MainAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Navbar(),
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // title of the page
              SizedBox(
                height: currentHeight * 0.03,
                child: Text(
                  "Logs: $file",
                  style: TextStyle(color: Colors.white),
                ),
              ),
              // title of the log being viewed
              SizedBox(
                width: currentWidth,
                height: currentHeight * 0.05,
                child: Row(
                  children: [
                    SizedBox(
                      height: currentHeight * 0.05,
                      width: currentWidth * 0.1,
                      child: TextField(
                        controller: _controller,
                        decoration: InputDecoration(
                          labelStyle: TextStyle(color: Colors.white),
                          border: InputBorder.none,
                          labelText: "File Name",
                        ),
                        style: TextStyle(color: Colors.white),

                        onChanged: (text) {
                          file_change(text);
                          find_files();
                        },
                      ),
                    ),
                    SizedBox(
                      height: currentHeight * 0.05,
                      width: currentWidth * 0.9,
                      child: FileButtons(
                        log_files: log_files,
                        file_change: file_change,
                      ),
                    ),
                  ],
                ),
              ),
              SizedBox(height: currentHeight * 0.02),
              // buttons for editing
              Row(
                children: [
                  LogButtons(
                    buttonWidth: buttonWidth,
                    buttonHeight: buttonHeight,
                    display: "Edit",
                    command: "e",
                    fetch_command: command_fetch,
                  ),
                  LogButtons(
                    buttonWidth: buttonWidth,
                    buttonHeight: buttonHeight,
                    display: "Read",
                    command: "r",
                    fetch_command: command_fetch,
                  ),
                  LogButtons(
                    buttonWidth: buttonWidth,
                    buttonHeight: buttonHeight,
                    display: "New File",
                    command: "n",
                    fetch_command: command_fetch,
                  ),
                ],
              ),
              SizedBox(height: currentHeight * 0.02),
              // the actual spot that views the contents. Shows all messages from program
              SizedBox(
                width: currentWidth,
                height: currentHeight * 0.83,
                child: ListView.builder(
                  itemCount: log_list.length,
                  itemBuilder: (context, index) {
                    return Text(
                      "${log_list[index]}",
                      style: TextStyle(color: Colors.white),
                    );
                  },
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class LogButtons extends StatelessWidget {
  const LogButtons({
    super.key,
    required this.buttonWidth,
    required this.buttonHeight,
    required this.display,
    required this.command,
    required this.fetch_command,
  });
  final String display;
  final double buttonWidth;
  final double buttonHeight;
  final String command;
  final void Function(String, double, double) fetch_command;

  @override
  Widget build(BuildContext context) {
    final currentWidth =
        (MediaQuery.of(context).size.width) -
        (MediaQuery.of(context).size.width * 0.05);
    final currentHeight = MediaQuery.of(context).size.height;

    return Padding(
      padding: const EdgeInsets.fromLTRB(2, 0, 2, 0),
      child: Container(
        decoration: BoxDecoration(
          borderRadius: BorderRadius.all(Radius.circular(10.0)),
          border: Border.all(color: Color(0xFFB87333)),
        ),
        width: buttonWidth,
        height: buttonHeight,
        child: ElevatedButton(
          style: ButtonStyle(
            backgroundColor: WidgetStatePropertyAll<Color>(Colors.transparent),
            shadowColor: WidgetStatePropertyAll<Color>(Colors.transparent),
          ),
          onPressed: () => fetch_command(command, currentHeight, currentWidth),
          child: Text(display, style: TextStyle(color: Colors.white)),
        ),
      ),
    );
  }
}

class FileButtons extends StatelessWidget {
  const FileButtons({
    super.key,
    required this.log_files,
    required this.file_change,
  });
  final List log_files;
  final void Function(String) file_change;

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      scrollDirection: Axis.horizontal,
      itemCount: log_files.length,
      itemBuilder: (context, index) {
        return Padding(
          padding: const EdgeInsets.fromLTRB(2, 0, 2, 0),
          child: Container(
            decoration: BoxDecoration(
              borderRadius: BorderRadius.all(Radius.circular(10.0)),
              border: Border.all(color: Color(0xFFB87333)),
            ),
            child: ElevatedButton(
              style: ButtonStyle(
                backgroundColor: WidgetStatePropertyAll<Color>(
                  Colors.transparent,
                ),
                shadowColor: WidgetStatePropertyAll<Color>(Colors.transparent),
              ),
              onPressed: () {
                file_change(log_files[index]);
              },
              child: Text(
                log_files[index],
                style: TextStyle(color: Colors.white),
              ),
            ),
          ),
        );
      },
    );
  }
}

class EditPopup extends StatefulWidget {
  const EditPopup({super.key});

  @override
  State<EditPopup> createState() => _EditPopupState();
}

class _EditPopupState extends State<EditPopup> {
  OverlayEntry? _overlayEntry;
  final TextEditingController _editcontroller = TextEditingController();
  // ignore: unused_element
  void _showOverlay() {
    _overlayEntry = OverlayEntry(
      builder: (context) => GestureDetector(
        onTap: () {
          _removeOverlay();
        },
        behavior: HitTestBehavior.translucent,
        child: Stack(
          children: [
            Positioned(
              top: 0,
              right: 0,
              child: GestureDetector(
                onTap: () {},
                child: Material(
                  color: Colors.amber,
                  child: SizedBox(
                    child: Column(
                      children: [
                        TextField(
                          controller: _editcontroller,
                          decoration: InputDecoration(
                            hintText: "Type in me mommy",
                          ),
                        ),
                        ElevatedButton(
                          onPressed: () {
                            _removeOverlay();
                          },
                          child: Text("Enter"),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );

    Overlay.of(context).insert(_overlayEntry!);
  }

  void _removeOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = null;
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector();
  }
}
