import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'ffi.dart';

//this needs a provider hooked up to rust functions
//so that justice can use it correctly right away

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  // These futures belong to the state and are only initialized once,
  // in the initState method.
  late Future<Platform> platform;
  late Future<bool> isRelease;
  List<Item> gridItems = [Item.CowButter];
  List<Widget> gridWidgets = [Container()];
  final TextEditingController _controller = TextEditingController();
  late ItemInfo item_info;

  void _set_item_info(Item item) async {
    item_info = await _get_item_info(item);
  }

  List<Widget> _build_widgets_from_list(List<Item> resultList) {
    List<Widget> outputList = [];
    for (Item item in resultList) {
      _set_item_info(item);
      outputList.add(Card(
        child: InkWell(
          splashColor: Colors.blue.withAlpha(30),
          child: SizedBox(
            width: 300,
            height: 100,

            //Item Image and
            child: Row(
              children: [
                Column(
                  children: [
                    Text(item_info.displayName),
                    Text("for every ${item_info.nutrition!.unitWeight}g")
                  ],
                ),
                Image.asset('images/${item_info.fileName}'),
                Text("liter weight: ${item_info.nutrition!.literWeight}"),
                Text("calories: ${item_info.nutrition!.calories}"),
                Text(
                    "calories from fat: ${item_info.nutrition!.caloriesFromFat}"),
                Text(
                    "total_fat_weight: ${item_info.nutrition!.totalFatWeight}"),
                Text(
                    "saturated_fat_weight: ${item_info.nutrition!.saturatedFatWeight}"),
                Text(
                    "polyunsaturated_fat_weight: ${item_info.nutrition!.polyunsaturatedFatWeight}"),
                Text(
                    "monounsaturated_fat_weight: ${item_info.nutrition!.monounsaturatedFatWeight}"),
                Text(
                    "cholesterol_weight: ${item_info.nutrition!.cholesterolWeight}"),
                Text("sodium_weight: ${item_info.nutrition!.sodiumWeight}"),
                Text(
                    "potassium_weight: ${item_info.nutrition!.potassiumWeight}"),
                Text(
                    "carbohydrate_weight: ${item_info.nutrition!.carbohydrateWeight}"),
                Text("fiber_weight: ${item_info.nutrition!.fiberWeight}"),
                Text("sugar_weight: ${item_info.nutrition!.sugarWeight}"),
                Text("protein_weight: ${item_info.nutrition!.proteinWeight}"),

                /*
    

                */
              ],
            ),
          ),
        ),
      ));
    }
    return outputList;
  }

  Future<ItemInfo> _get_item_info(Item item) async {
    return await api.getNutrition(item: item);
  }

  @override
  void initState() {
    super.initState();
    platform = api.platform();
    isRelease = api.rustReleaseMode();
    gridWidgets = _build_widgets_from_list(gridItems);
  }

  void _ask_with_new_string(String string) async {
    var futureItems = await api.searchLocalItems(input: string);
    super.setState(() {
      gridItems = futureItems;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Column(
              children: <Widget>[
                //Search panel, and settings
                TextField(
                  onChanged: (textInput) {
                    setState(() {
                      _ask_with_new_string(textInput);
                      gridWidgets = _build_widgets_from_list(gridItems);
                    });
                  },
                  controller: _controller,
                  decoration: const InputDecoration(
                    hintText: "search items >>",
                    border: OutlineInputBorder(),
                  ),
                )
              ],
            ),
            GridView.count(
              crossAxisCount: 3,
              children: gridWidgets,
            )
          ],
        ),
      ),
    );
  }
}

/*
            const Text("You're running on"),
            // To render the results of a Future, a FutureBuilder is used which
            // turns a Future into an AsyncSnapshot, which can be used to
            // extract the error state, the loading state and the data if
            // available.
            //
            // Here, the generic type that the FutureBuilder manages is
            // explicitly named, because if omitted the snapshot will have the
            // type of AsyncSnapshot<Object?>.
            FutureBuilder<List<dynamic>>(
              // We await two unrelated futures here, so the type has to be
              // List<dynamic>.
              future: Future.wait([platform, isRelease]),
              builder: (context, snap) {
                final style = Theme.of(context).textTheme.headline4;
                if (snap.error != null) {
                  // An error has been encountered, so give an appropriate response and
                  // pass the error details to an unobstructive tooltip.
                  debugPrint(snap.error.toString());
                  return Tooltip(
                    message: snap.error.toString(),
                    child: Text('Unknown OS', style: style),
                  );
                }

                // Guard return here, the data is not ready yet.
                final data = snap.data;
                if (data == null) return const CircularProgressIndicator();

                // Finally, retrieve the data expected in the same order provided
                // to the FutureBuilder.future.
                final Platform platform = data[0];
                final release = data[1] ? 'Release' : 'Debug';
                final text = const {
                      Platform.Android: 'Android',
                      Platform.Ios: 'iOS',
                      Platform.MacApple: 'MacOS with Apple Silicon',
                      Platform.MacIntel: 'MacOS',
                      Platform.Windows: 'Windows',
                      Platform.Unix: 'Unix',
                      Platform.Wasm: 'the Web',
                    }[platform] ??
                    'Unknown OS';
                return Text('$text ($release)', style: style);
              },
            )
*/
/*

    _controller.addListener(() {
      final String text = _controller.text.toLowerCase();
      _controller.value = _controller.value.copyWith(
        text: text,
        selection:
            TextSelection(baseOffset: text.length, extentOffset: text.length),
        composing: TextRange.empty,
      );
      //call futures here to get them going
      //before needing them
    });

*/
